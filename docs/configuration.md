# Configuration

Sociarium configuration describes **what the corpus tracks**, not secret credentials used to access remote surfaces.

The current schema is version `1` and is represented as TOML. See [`sociarium.example.toml`](../sociarium.example.toml).

```toml
schema_version = 1

[[profiles]]
id = "x-main"
surface = "x"
handle = "sguzman"
ownership = "self_owned"
enabled = true
```

## Profiles are first-class

Each `[[profiles]]` entry is an independent synchronization scope. The same surface may have many configured profiles and a corpus may contain many surfaces at once.

`id` is Sociarium's stable local name for the tracked profile. It must be unique within a corpus.

`surface` selects an adapter such as `x`. The core configuration does not special-case X.

`remote_id`, when known, is the remote surface's stable identifier. Handles are mutable and must not replace it as identity.

`handle` is optional mutable metadata/configuration convenience. Adapters may discover a newer handle while synchronizing.

`ownership` describes the relationship between the corpus operator and the profile. M0 X synchronization is intentionally limited to `self_owned` because it uses authenticated-user context.

`enabled` controls whether broad commands such as a future `sync --all` should include the profile. Explicit profile operations may still address disabled profiles when that behavior is documented.

## CLI inspection

```text
sociarium --config sociarium.toml config check
sociarium --config sociarium.toml profiles list
```

Neither command contacts a remote surface.

## Secrets do not belong here

Client secrets, access tokens, refresh tokens, passwords, session cookies, and other bearer credentials must not be written into this TOML file or any Git-tracked corpus path. See [`security-and-credentials.md`](security-and-credentials.md).
