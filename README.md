# Sociarium

Sociarium is a Rust-native, user-sovereign social-data substrate that synchronizes selected profiles across arbitrary social surfaces into a durable, provenance-preserving, Git-versioned local corpus, with first-class CLI, search, and agent interfaces.

> **Surfaces are adapters, not ontology. Profiles are first-class synchronization scopes. The local corpus is the durable record.**

## Status

Sociarium is in M0. The first concrete integration target is X, but X does not define the core model.

The first vertical slice is structurally implemented:

1. generic profile and surface configuration;
2. native X OAuth2 Authorization Code + S256 PKCE;
3. profile-scoped Windows Credential Manager persistence;
4. automatic X token refresh;
5. direct Rust X API acquisition;
6. crash-resumable and incremental profile synchronization;
7. durable raw + normalized acquisition bundles;
8. rebuildable SQLite/FTS search projection;
9. CLI authorization, sync, list, and search paths.

A pre-live audit against the current X API and local repository boundaries found remaining repository-side M0 hardening. Issues #2–#6 track that work. The confirmed live blocker is #5: the current user-timeline request uses `post.fields` where X documents `tweet.fields`, does not request `referenced_tweets`, and does not request/use `note_tweet`; without that fix, live reply/quote relationships can be absent and long Posts can normalize from a truncated `text` representation instead of the full authored `note_tweet.text`. The other issues harden remote-error safety, loopback handling, dedicated corpus initialization, and a final profile-aware local preflight.

Preferred implementation order is **#5 -> #4 -> #3 -> #6 -> #2**.

**Do not treat the live X smoke test as the next step until issues #2–#6 are resolved and CI is green.** After that, the final M0 validation gate is a real Windows authorization + synchronization run using a registered X Developer App, an authorized account, and a dedicated operator-owned corpus repo/directory.

Build resolution is reproducible: Sociarium commits `Cargo.lock`, uses Cargo resolver 3, selects an MSRV-compatible IDNA backend explicitly, and verifies the locked graph on stable Linux, native Windows, and Rust 1.85.

MCP comes after the corpus and query boundaries are stable. It is a projection of Sociarium, not Sociarium's internal API.

## Architectural invariants

- **Local durability:** once a remote observation is acquired, Sociarium can preserve it independently of the remote surface.
- **No hidden singletons:** no assumption that there is one surface, one profile, or one real-world identity.
- **Profile-first synchronization:** synchronization is scoped to a configured remote profile; surface-wide commands are conveniences over profiles.
- **Adapter isolation:** X-, Reddit-, Bluesky-, Mastodon-, and other surface-specific concepts stay behind adapters unless they represent genuinely shared semantics.
- **Three evidence layers:** raw remote evidence, normalized corpus objects, and derived/indexed views remain distinguishable.
- **Rebuildable indexes:** SQLite/search indexes and caches are disposable projections, never the only copy of corpus data.
- **Direct Rust integrations:** surface adapters talk to remote APIs directly from Rust. External platform CLIs such as `xurl` are not runtime dependencies.
- **Credential separation:** bearer credentials are operational secret state outside the Git-tracked corpus and configuration.
- **Source/corpus separation:** the Sociarium software checkout and the operator's durable social corpus are separate authority domains.
- **Private data is not the same as a credential:** a secret-free corpus can still contain authorized/private observations and should not be assumed safe to publish.
- **Git is history/transport:** Git may version a corpus; it is not the persistence/query API and GitHub is not corpus authority.
- **Best available normalized value:** portable fields such as `Post.text` should use the best complete value the acquired source exposes, not a known truncated preview.
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

## Software repo versus corpus repo

`sguzman/sociarium` is the software source repository. Real social data belongs in a separate operator-owned corpus directory/repository.

```text
sguzman/sociarium/        # Rust software, docs, tests
<operator corpus>/        # acquisitions, normalized records, provenance, local config
```

Issue #6 owns the explicit Git-safe corpus initialization command/policy. Until it lands, do not use a `corpus/` subdirectory inside this public source checkout for a real account merely because the development examples use `cargo run` from here.

If a corpus is pushed to a Git remote, private visibility is the conservative default because future capabilities may include private/authorized observations such as bookmarks. Credentials remain forbidden from the corpus either way.

## Configuration

For source-tree development, copy [`sociarium.example.toml`](sociarium.example.toml) to the repository-root `sociarium.toml`. That source-root filename is ignored by default so machine/profile-local development configuration is not accidentally published. It is still **not a secret store**.

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

Bearer credentials and client secrets do not belong in this file.

Validate configuration and inspect profiles:

```text
cargo run -p sociarium-cli --locked -- --config sociarium.toml config check
cargo run -p sociarium-cli --locked -- --config sociarium.toml profiles list
```

The intended Windows authorization path is:

```text
cargo run -p sociarium-cli --locked -- --config sociarium.toml auth login x-main
cargo run -p sociarium-cli --locked -- --config sociarium.toml auth status x-main
```

The CLI prints the X authorization URL, validates the loopback callback, exchanges the authorization code, and stores the resulting token envelope in Windows Credential Manager. Subsequent syncs load and refresh that profile's credential automatically.

The intended synchronization/query path is:

```text
cargo run -p sociarium-cli --locked -- --config <CORPUS_CONFIG> --corpus <CORPUS_ROOT> sync x-main
cargo run -p sociarium-cli --locked -- --corpus <CORPUS_ROOT> posts list --profile x-main
cargo run -p sociarium-cli --locked -- --corpus <CORPUS_ROOT> posts search sociarium --profile x-main
```

These live commands describe the M0 path, but the deliberate real-X smoke run is currently blocked on issues #2–#6 above.

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
- [`docs/x-live-smoke-test.md`](docs/x-live-smoke-test.md)
- [`docs/decisions/`](docs/decisions/) for architectural decision records
- [`AGENTS.md`](AGENTS.md) for the implementation contract

## Development

Current CI enforces formatting, strict clippy, workspace tests, native Windows compilation/tests, and the declared Rust 1.85 minimum supported version against the committed lockfile.

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo +1.85.0 check --workspace --all-targets --locked
cargo run -p sociarium-cli --locked -- doctor
```

Dependency updates should deliberately refresh `Cargo.lock` and then pass the full matrix. See ADR 0005 for the MSRV and dependency-resolution policy.

Live X synchronization requires a registered X Developer App plus user authorization and whatever current API credits/entitlement X requires. Unit tests and normalization fixtures do not require live X credentials or paid API access.

## License

No license has been selected yet. Until one is explicitly added, normal copyright restrictions apply.
