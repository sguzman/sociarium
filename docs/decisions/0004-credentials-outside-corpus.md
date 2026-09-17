# ADR 0004: Credentials are outside the corpus

- Status: accepted
- Date: 2026-09-17

## Context

Sociarium preserves durable, Git-versioned social observations. Remote adapters also need bearer credentials whose disclosure would grant authority over accounts. Treating those credentials as corpus data would make normal repository operations dangerous and would couple social history to one machine's authentication state.

Credentials also need a stable ownership model. A hidden global/current-account credential would conflict with Sociarium's profile-first architecture because one corpus may track multiple profiles on the same surface and multiple surfaces at once.

## Decision

Credentials are operational secret state, not corpus state.

OAuth access tokens, refresh tokens, client secrets, PKCE verifiers, authorization codes, passwords, and session cookies must not be stored in raw evidence, normalized objects, derived indexes, configuration files, or other Git-tracked corpus paths.

Persistent credential lookup is keyed by:

```text
(surface, local profile id, credential slot)
```

This preserves profile independence and leaves room for more than one credential class per profile without changing the corpus ontology.

The storage contract lives in the surface-independent `sociarium-credentials` crate. Adapters own authentication protocols and token semantics; the credential crate owns only opaque secret persistence.

M0 provides:

- `MemoryCredentialStore` for deterministic tests;
- `NativeCredentialStore` backed by Windows Credential Manager on Windows;
- a versioned X OAuth token envelope stored as opaque secret bytes;
- explicit `auth login`, `auth status`, and `auth logout` CLI operations;
- automatic refresh before X sync when the stored access token is near expiry.

The Windows backend uses the Rust `keyring` crate's Windows-native implementation. The dependency is exact-pinned to `3.6.3` because Sociarium declares Rust 1.85 as its MSRV and newer major keyring releases have moved beyond that floor. CI separately exercises current stable Rust, native Windows, and Rust 1.85.

`SOCIARIUM_X_ACCESS_TOKEN` is retained as an explicit process-level emergency override. It is not configuration, is never written into the credential store automatically, and must never become corpus evidence.

## Consequences

- A corpus can be copied, inspected, versioned, or published without implicitly copying account authority.
- Restoring a corpus on a new machine requires re-establishing credentials separately.
- Multiple profiles on one surface do not silently share credentials.
- Deleting or rebuilding indexes cannot delete or copy remote-account authority.
- Diagnostics can report credential presence/expiry without exposing bearer values.
- Remote write capability can later be audited independently of local corpus readability.
- Additional operating-system credential backends can be added without changing adapter or corpus contracts.

## Rejected alternatives

### Credentials in `sociarium.toml`

Rejected because configuration is expected to be inspectable and may be Git-tracked. Bearer authority must not ride along with ordinary corpus configuration.

### Credentials inside the corpus repository but gitignored

Rejected because repository-local placement still couples corpus copying/backups to secret state and makes accidental disclosure too easy.

### One global X credential

Rejected because it recreates the hidden-singleton model Sociarium explicitly avoids. Credential ownership follows local profile identity.

### Depend on `xurl` or another external credential manager

Rejected because surface integration is intentionally Rust-native and Sociarium must own its authentication/storage contract rather than inherit another CLI's hidden state layout.
