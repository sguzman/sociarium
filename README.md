# Sociarium

Sociarium is a Rust-native, user-sovereign social-data substrate that synchronizes selected profiles across arbitrary social surfaces into a durable, provenance-preserving, Git-versioned local corpus, with first-class CLI, search, and agent interfaces.

> **Surfaces are adapters, not ontology. Profiles are first-class synchronization scopes. The local corpus is the durable record.**

## Status

Sociarium is at bootstrap/M0. The first concrete integration target is X, but X does not define the core model.

The M0 vertical slice is intentionally narrow:

1. define a generic social-domain model;
2. define the adapter contract;
3. establish the durable repository format;
4. add an X adapter that talks directly to X from Rust;
5. sync one configured X profile's posts into the local corpus;
6. rebuild a disposable local index from that corpus;
7. query the result from the CLI.

MCP comes after the corpus and query boundaries are stable. It is a projection of Sociarium, not Sociarium's internal API.

## Architectural invariants

- **Local durability:** once a remote observation is acquired, Sociarium can preserve it independently of the remote surface.
- **No hidden singletons:** no assumption that there is one surface, one profile, or one real-world identity.
- **Profile-first synchronization:** synchronization is scoped to a configured remote profile; surface-wide commands are conveniences over profiles.
- **Adapter isolation:** X-, Reddit-, Bluesky-, Mastodon-, and other surface-specific concepts stay behind adapters unless they represent genuinely shared semantics.
- **Three evidence layers:** raw remote evidence, normalized corpus objects, and derived/indexed views remain distinguishable.
- **Rebuildable indexes:** SQLite/search indexes and caches are disposable projections, never the only copy of corpus data.
- **Direct Rust integrations:** surface adapters talk to remote APIs directly from Rust. External platform CLIs such as `xurl` are not runtime dependencies.
- **Conspicuous writes:** reading/querying local data is broad; remote mutation is a separate capability boundary.
- **Inspectable repository:** the corpus should remain understandable even if the Sociarium executable no longer runs.

## Workspace

- `sociarium-core` — surface-independent social ontology and identifiers.
- `sociarium-adapter` — adapter traits, capabilities, sync batches, and adapter errors.
- `sociarium-adapter-x` — first surface adapter. Direct X API integration belongs here.
- `sociarium-store` — durable repository layout and persistence boundary.
- `sociarium-cli` — human-facing CLI orchestration.

Planned later: indexing/search, MCP, media acquisition, additional adapters, and explicit cross-profile identity resolution.

## Documentation

Start with:

- [`docs/architecture.md`](docs/architecture.md)
- [`docs/data-model.md`](docs/data-model.md)
- [`docs/adapters.md`](docs/adapters.md)
- [`docs/repository-format.md`](docs/repository-format.md)
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

No X credentials are required yet; the X adapter is a typed boundary awaiting the first authenticated sync implementation.

## License

No license has been selected yet. Until one is explicitly added, normal copyright restrictions apply.
