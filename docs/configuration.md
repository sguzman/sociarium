# Configuration

Sociarium configuration describes **what the corpus tracks** and the non-secret settings required to talk to configured surfaces. It does not contain bearer credentials.

The current schema is version `1` and is represented as TOML. See [`sociarium.example.toml`](../sociarium.example.toml).

```toml
schema_version = 1

[surfaces.x]
client_id = "replace-with-your-x-app-client-id"
redirect_uri = "http://127.0.0.1:49152/oauth/x/callback"

[[profiles]]
id = "x-main"
surface = "x"
handle = "sguzman"
ownership = "self_owned"
enabled = true
```

## Local configuration and Git

The repository-root `sociarium.toml` is ignored by default. Copy [`sociarium.example.toml`](../sociarium.example.toml) to `sociarium.toml` for machine/profile-local configuration.

This is a safety and locality default, not permission to put secrets in the file. `sociarium.toml` may contain account handles, profile selection, local callback settings, and other operator-specific state that does not need to become part of the public source repository. If a deployment deliberately wants a configuration file under version control, use an explicitly named tracked configuration and pass it with `--config` rather than weakening the default accidentally.

## Surface settings are adapter-owned

`[surfaces.<surface>]` is a generic map of non-secret adapter settings. Sociarium core does not gain fields such as `x_client_id`; the X adapter/CLI interprets only the settings under `[surfaces.x]`.

For M0 X authorization:

- `client_id` identifies the registered X Developer App and is not treated as a bearer secret;
- `redirect_uri` is the loopback callback URI used by the native OAuth2/PKCE flow and must match the URI registered with X;
- the current CLI requires an `http://127.0.0.1:<port>/...` redirect so the callback listener remains loopback-only.

Future adapters can define their own non-secret settings without changing the profile ontology.

## Profiles are first-class

Each `[[profiles]]` entry is an independent synchronization scope. The same surface may have many configured profiles and a corpus may contain many surfaces at once.

`id` is Sociarium's stable local name for the tracked profile. It must be unique within a corpus.

`surface` selects an adapter such as `x`. The core configuration does not special-case X.

`remote_id`, when known, is the remote surface's stable identifier. Handles are mutable and must not replace it as identity.

`handle` is optional mutable metadata/configuration convenience. Adapters may discover a newer handle while synchronizing.

`ownership` describes the relationship between the corpus operator and the profile. M0 X synchronization is intentionally limited to `self_owned` because it uses authenticated-user context.

`enabled` controls whether broad commands such as a future `sync --all` should include the profile. The current explicit `sync <profile-id>` runner rejects disabled profiles.

## CLI inspection

```text
sociarium --config sociarium.toml config check
sociarium --config sociarium.toml profiles list
```

Neither command contacts a remote surface.

Authorization is a separate operation:

```text
sociarium --config sociarium.toml auth login x-main
sociarium --config sociarium.toml auth status x-main
```

## Secrets do not belong here

Client secrets, access tokens, refresh tokens, passwords, session cookies, PKCE verifiers, authorization codes, and other bearer credentials must not be written into this TOML file or any Git-tracked corpus path.

On Windows, M0 persists profile-scoped OAuth token envelopes in Windows Credential Manager through `sociarium-credentials`. See [`security-and-credentials.md`](security-and-credentials.md).
