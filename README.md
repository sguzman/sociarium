# Sociarium

Sociarium is a Rust-native, user-sovereign social-data substrate that synchronizes selected profiles across arbitrary social surfaces into a durable, provenance-preserving, Git-versioned local corpus, with first-class CLI, search, and agent interfaces.

> **Surfaces are adapters, not ontology. Profiles are first-class synchronization scopes. The local corpus is the durable record.**

## Status

Sociarium is in M0. The first concrete integration target is X, but X does not define the core model.

The first vertical slice is now implementation-complete in the repository:

1. generic profile and surface configuration;
2. native X OAuth2 Authorization Code + PKCE;
3. profile-scoped Windows Credential Manager persistence;
4. automatic X token refresh;
5. direct Rust X API acquisition;
6. crash-resumable and incremental profile synchronization;
7. durable raw + normalized acquisition bundles;
8. rebuildable SQLite/FTS search projection;
9. CLI authorization, sync, list, and search paths.

The remaining M0 validation gate is a live smoke test against a real registered X Developer App and authorized account. Unit/fixture coverage and CI validate the implementation without requiring live credentials, but they do not substitute for one real end-to-end authorization and synchronization run.

MCP comes after the corpus and query boundaries are stable. It is a projection of Sociarium, not Sociarium's internal API.

## Architectural invariants

- **Local durability:** once a remote observation is acquired, Sociarium can preserve it independently of the remote surface.
- **No hidden singletons:** no assumption that there is one surface, one profile, or one real-world identity.
- **Profile-first synchronization:** synchronization is scoped to a configured remote profile; surface-wide commands are conveniences over profiles.
- **Adapter isolation:** X-, Reddit-, Bluesky-, Mastodon-, and other surface-specific concepts stay behind adapters unless they represent genuinely shared semantics.
- **Three evidence layers:** raw remote evidence, normalized corpus objects, and derived/indexed views remain distinguishable.
- **Rebuildable indexes:** SQLite/search indexes and caches are disposable projections, never the only copy of corpus data.
- **Direct Rust integrations:** surface adapters talk to remote APIs directly from Rust. External platform CLIs such as `xurl` are not runtime dependencies.
- **Credential separation:** bearer credentials are operational secret state outside the Git-tracked corpus.
- **Conspicuous writes:** reading/querying local data is broad; remote mutation is a separate capability boundary.
- **Inspectable repository:** the corpus should remain understandable even if the Sociarium executable no longer runs.

## Workspace

- `sociarium-core` — surface-independent social ontology and identifiers.
- `sociarium-adapter` — adapter traits, capabilities, sync batches, and adapter errors.
- `sociarium-adapter-x` — first surface adapter; native X OAuth2/PKCE, HTTP acquisition, cursor semantics, token envelopes, and normalization live here.
- `sociarium-config` — non-secret corpus/profile configuration plus generic per-surface settings.
- `sociarium-credentials` — profile-scoped credential-store abstraction with an in-memory test backend and Windows Credential Manager backend.
- `sociarium-store` — durable repository layout and persistence boundary.
- `sociarium-sync` — generic profile-scoped sync orchestration and crash-resume rules.
- `sociarium-search` — disposable SQLite/FTS projection rebuilt from durable acquisitions.
- `sociarium-cli` — human-facing auth, sync, index, and query orchestration.

Planned later: MCP, media acquisition, additional adapters, broader X corpus objects, and explicit cross-profile identity resolution.

## Configuration

Copy [`sociarium.example.toml`](sociarium.example.toml), register an X Developer App, and set its non-secret client configuration:

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

Bearer credentials do not belong in this file.

Validate configuration and inspect profiles:

```text
cargo run -p sociarium-cli -- --config sociarium.toml config check
cargo run -p sociarium-cli -- --config sociarium.toml profiles list
```

On Windows, authorize the profile once through the native loopback OAuth flow:

```text
cargo run -p sociarium-cli -- --config sociarium.toml auth login x-main
cargo run -p sociarium-cli -- --config sociarium.toml auth status x-main
```

The CLI prints the X authorization URL, validates the loopback callback, exchanges the authorization code, and stores the resulting token envelope in Windows Credential Manager. Subsequent syncs load and refresh that profile's credential automatically.

Then synchronize and query the local corpus:

```text
cargo run -p sociarium-cli -- --config sociarium.toml --corpus corpus sync x-main
cargo run -p sociarium-cli -- --corpus corpus posts list --profile x-main
cargo run -p sociarium-cli -- --corpus corpus posts search sociarium --profile x-main
```

`SOCIARIUM_X_ACCESS_TOKEN` remains available only as an emergency process-level override; it is not the normal authentication path and is never persisted into the corpus.

## Documentation

Start with:

- [`docs/architecture.md`](docs/architecture.md)
- [`docs/data-model.md`](docs/data-model.md)
- [`docs/adapters.md`](docs/adapters.md)
- [`docs/configuration.md`](docs/configuration.md)
- [`docs/synchronization.md`](docs/synchronization.md)
- [`docs/security-and-credentials.md`](docs/security-and-credentials.md)
- [`docs/repository-format.md`](docs/repository-format.md)
- [`docs/search-index.md`](docs/search-index.md)
- [`docs/roadmap.md`](docs/roadmap.md)
- [`docs/decisions/`](docs/decisions/) for architectural decision records
- [`AGENTS.md`](AGENTS.md) for the implementation contract

## Development

Current CI enforces formatting, strict clippy, workspace tests, native Windows compilation/tests, and the declared Rust 1.85 minimum supported version.

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo +1.85.0 check --workspace --all-targets
cargo run -p sociarium-cli -- doctor
```

Live X synchronization requires a registered X Developer App plus user authorization. Unit tests and normalization fixtures do not require live X credentials.

## License

No license has been selected yet. Until one is explicitly added, normal copyright restrictions apply.
