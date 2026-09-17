# Sociarium

Sociarium is a Rust-native, user-sovereign social-data substrate that synchronizes selected profiles across arbitrary social surfaces into a durable, provenance-preserving, Git-versioned local corpus, with first-class CLI, search, and agent interfaces.

> **Surfaces are adapters, not ontology. Profiles are first-class synchronization scopes. The local corpus is the durable record.**

## Status

Sociarium is in M0. The first concrete integration target is X, but X does not define the core model.

The generic profile configuration, native X OAuth/API boundary, durable acquisition bundles, crash-recoverable sync state, rebuildable SQLite/FTS search projection, and profile-scoped CLI sync/query path now exist.

The remaining major M0 integration gap is credential UX: the OAuth2/PKCE primitives exist, but the CLI still needs a loopback callback flow plus profile-scoped OS credential-store persistence and refresh handling. Until that lands, live X sync accepts an already-authorized access token only through the temporary `SOCIARIUM_X_ACCESS_TOKEN` process environment bridge.

The M0 vertical slice remains intentionally narrow:

1. define a generic social-domain model;
2. define the adapter contract;
3. establish the durable repository format;
4. add an X adapter that talks directly to X from Rust;
5. sync one configured X profile's posts into the local corpus;
6. rebuild a disposable local index from that corpus;
7. query the result from the CLI;
8. finish native credential persistence and OAuth authorization UX.

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
- `sociarium-adapter-x` — first surface adapter; native X OAuth2/PKCE, HTTP acquisition, cursor semantics, and normalization live here.
- `sociarium-config` — non-secret, surface-agnostic corpus/profile configuration.
- `sociarium-store` — durable repository layout and persistence boundary.
- `sociarium-sync` — generic profile-scoped sync orchestration and crash-resume rules.
- `sociarium-search` — disposable SQLite/FTS projection rebuilt from durable acquisitions.
- `sociarium-cli` — human-facing CLI orchestration.

Planned later: persistent credential-store integration, MCP, media acquisition, additional adapters, and explicit cross-profile identity resolution.

## Configuration

Copy [`sociarium.example.toml`](sociarium.example.toml) and adjust the profile set for a corpus. The file describes synchronization scopes only; OAuth tokens and other secrets do not belong in it.

```text
cargo run -p sociarium-cli -- --config sociarium.toml config check
cargo run -p sociarium-cli -- --config sociarium.toml profiles list
```

For the temporary M0 X credential bridge:

```text
SOCIARIUM_X_ACCESS_TOKEN=<authorized-user-token> \
  cargo run -p sociarium-cli -- --config sociarium.toml --corpus corpus sync x-main

cargo run -p sociarium-cli -- --corpus corpus posts list --profile x-main
cargo run -p sociarium-cli -- --corpus corpus posts search sociarium --profile x-main
```

On PowerShell, set the environment variable using normal PowerShell environment syntax rather than the POSIX inline form above.

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

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p sociarium-cli -- doctor
```

Live X synchronization requires a registered X Developer App plus user authorization. Unit tests and normalization fixtures do not require live X credentials.

## License

No license has been selected yet. Until one is explicitly added, normal copyright restrictions apply.
