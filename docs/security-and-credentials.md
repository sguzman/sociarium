# Security and credentials

Sociarium deliberately separates durable social data from credentials that grant authority over remote accounts.

## Credential boundary

The corpus may contain public or authorized observations acquired from a surface. It must not contain the bearer material used to acquire them.

Never write these into the Git-tracked corpus:

- OAuth access tokens;
- OAuth refresh tokens;
- client secrets;
- passwords;
- session cookies;
- PKCE code verifiers;
- one-time authorization codes.

`XOAuthSession` and `XTokenSet` implement redacted debug formatting so routine diagnostics do not dump their secret fields.

Persistent credential storage is a separate M0 implementation step. Its contract is:

1. storage is outside the corpus repository;
2. lookup is scoped by local profile identity rather than a hidden global account;
3. the adapter may load/refresh credentials without making the secret bytes part of normalized records or raw evidence;
4. deleting/rebuilding indexes or moving the corpus must not copy credentials implicitly;
5. diagnostics must report credential state without printing credential values.

On desktop systems, an OS credential store/keyring is preferred over plaintext files. A fallback file store, if ever added, must be explicit, ignored by Git, permission-restricted where the OS allows it, and clearly documented as lower assurance.

## Temporary M0 environment bridge

The profile sync command is now wired end-to-end, but persistent credential lookup and the local OAuth callback listener are not yet wired into the CLI. For live X testing, the CLI currently accepts an already-authorized user access token through:

```text
SOCIARIUM_X_ACCESS_TOKEN
```

This is intentionally temporary. The environment variable is process operational state, not configuration and not corpus data. The CLI only reports whether it is present; it never prints the value.

The next credential slice replaces this bridge with profile-scoped OS credential-store entries and native OAuth2/PKCE authorization/refresh handling.

## X OAuth policy

The first X flow uses OAuth 2.0 Authorization Code with PKCE as a public/native-client shape. The baseline read scopes are:

```text
tweet.read
users.read
offline.access
```

`offline.access` is requested so a refresh token can support future unattended incremental synchronization. Write scopes are intentionally absent from M0.

The redirect URI is configuration for the registered X Developer App and must match the URI registered with X exactly. The CLI callback listener should bind to loopback only and validate OAuth `state` before exchanging the short-lived authorization code.

## Raw evidence is not credential storage

Preserving raw API response bodies is part of Sociarium's provenance model. HTTP request headers and OAuth token responses are not raw social evidence and must not be archived with those responses.
