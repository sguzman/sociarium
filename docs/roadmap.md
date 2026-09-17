# Roadmap

## M0 — X vertical slice

Goal: prove the architecture with the smallest end-to-end useful path.

Acceptance target:

```text
configured X profile
  -> Rust-native authentication/API client
  -> acquire own posts
  -> preserve raw response evidence
  -> normalize profile/posts
  -> write durable corpus files
  -> build/rebuild local search index
  -> query from CLI
```

Repository implementation status: complete. The remaining M0 gate is one live end-to-end smoke test with a registered X Developer App and authorized account, exercising native login, persisted credentials, acquisition, durable sync state, index rebuild, and local query against real remote data.

Current M0 implementation includes:

- generic profile and per-surface non-secret configuration;
- native OAuth2 Authorization Code + PKCE for X;
- profile-scoped Windows Credential Manager persistence;
- refresh-token rollover and automatic refresh before sync;
- direct Rust X API acquisition;
- raw + normalized acquisition bundles;
- durable incremental/cursor state with crash recovery;
- rebuildable SQLite/FTS index;
- CLI auth, sync, list, and search commands;
- Linux stable, native Windows, and Rust 1.85 CI coverage.

Explicitly not M0: posting, deleting, arbitrary public-X search, GUI, MCP, cross-platform identity resolution, additional adapters.

## M1 — X corpus depth

- replies/quotes/reposts where available;
- richer profile snapshots/history;
- likes/bookmarks for authenticated owned profiles where API access permits;
- following/follower observations;
- more sync diagnostics and recovery tooling;
- media metadata and optional acquisition;
- corpus migrations/schema versioning;
- import/bootstrap paths for existing X archives if useful.

## M2 — Agent surface

- semantic read-only MCP interface over the local corpus;
- evidence/provenance retrieval tools;
- bounded context/export views for agents;
- no arbitrary SQL/shell escape hatch.

## M3 — Second adapter

Add one non-X surface specifically to test whether the ontology and adapter contract genuinely generalize. Choose based on useful available API access rather than conceptual similarity.

## M4 — Identity and cross-surface analysis

- explicit entities/personas/projects;
- evidence-backed profile links;
- queries spanning surfaces and profiles;
- longitudinal comparison and corpus-level derived views.

## Later

Remote write capabilities, richer media support, UI surfaces, scheduled synchronization, and further adapters are deliberately deferred until the acquisition/preservation core is trustworthy.
