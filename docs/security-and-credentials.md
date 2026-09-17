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

`XOAuthSession`, `XTokenSet`, and `XStoredTokens` implement redacted debug formatting so routine diagnostics do not dump their secret fields.

Credential storage follows these rules:

1. storage is outside the corpus repository;
2. lookup is scoped by `(surface, local profile id, credential slot)` rather than a hidden global account;
3. adapters and orchestration may load/refresh credentials without making secret bytes part of normalized records or raw evidence;
4. deleting/rebuilding indexes or moving the corpus must not copy credentials implicitly;
5. diagnostics report credential state without printing credential values.

The generic boundary lives in `sociarium-credentials`. `MemoryCredentialStore` provides deterministic tests. `NativeCredentialStore` provides the host-native persistence implementation where supported.

## Windows Credential Manager

Windows is the first native credential backend and the primary M0 desktop target. `NativeCredentialStore` uses Windows Credential Manager through the Rust `keyring` crate's Windows-native backend.

Credential account names are derived from the surface, local profile id, and credential slot. User-controlled separators are percent-escaped before the key is constructed, so profiles on the same surface remain independent credential scopes.

The credential value is a versioned secret envelope. For X it currently contains the access token, optional refresh token, token type, receipt time, expiry time, and granted scope. The envelope itself is secret and is stored as the credential value; it is never committed to the corpus.

The dependency is deliberately pinned to `keyring = 3.6.3`. Sociarium declares Rust 1.85 as its minimum supported Rust version, and this known backend line remains compatible with that floor. CI tests both current stable Rust and the declared MSRV.

Native persistent credential storage for other operating systems is deferred. The `CredentialStore` abstraction is intentionally platform-independent so later backends do not change the corpus or adapter contracts.

## CLI lifecycle

Profile authorization is explicit:

```text
sociarium auth login x-main
sociarium auth status x-main
sociarium auth logout x-main
```

`auth login`:

1. resolves the configured profile;
2. reads non-secret X application settings from `[surfaces.x]`;
3. binds the configured loopback redirect before authorization begins;
4. creates an OAuth2 Authorization Code + PKCE session;
5. prints the X authorization URL;
6. waits for the loopback callback;
7. validates the callback path and OAuth `state`;
8. exchanges the short-lived code for tokens;
9. stores the redacted/versioned token envelope under that profile's native credential key.

The callback listener accepts only the configured `http://127.0.0.1:<port>/...` path. It does not archive callback requests or OAuth token responses.

`auth status` reports presence, expiry, refresh-token availability, and emergency environment override state without printing bearer values.

`auth logout` removes only the selected profile's stored credential.

## Refresh behavior

X requests the baseline read scopes:

```text
tweet.read
users.read
offline.access
```

`offline.access` permits a refresh token. Before an X sync, Sociarium loads that profile's stored token envelope. If the access token is within the refresh leeway, it refreshes natively, preserves a previous refresh token if the response does not rotate one, and saves the updated envelope back to the same profile-scoped credential key.

Write scopes are intentionally absent from M0.

## Emergency environment override

`SOCIARIUM_X_ACCESS_TOKEN` remains available as an explicit operational escape hatch. If a nonblank value is present, it overrides the persisted X credential for that process.

This is not the normal login path and is never read from `sociarium.toml`. The CLI reports only whether the override is present, never its value.

The override is useful for recovery, CI-like experiments, or environments where the native credential backend is unavailable. It does not change the security boundary: the environment value must never be persisted as corpus evidence.

## Raw evidence is not credential storage

Preserving raw API response bodies is part of Sociarium's provenance model. HTTP request headers, authorization callbacks, OAuth token responses, and credential-store contents are not raw social evidence and must not be archived with social API responses.
