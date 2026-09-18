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

Repository implementation status: **repository-side M0 complete; live validation pending**.

The pre-live audit produced issues #2–#7, and all are now resolved and CI-verified:

- profile-aware no-network preflight (#2);
- bounded/resilient OAuth loopback callback (#3);
- non-secret-by-construction X/OAuth remote diagnostics (#4);
- current X timeline wire/full-text/relationship fidelity (#5);
- explicit separate Git-safe operator corpus initialization (#6);
- durable stable remote-profile identity binding (#7).

The final M0 gate is one live end-to-end smoke test on Windows with a registered X Developer App and authorized account. It must exercise the real profile-aware preflight, native OAuth login, Windows Credential Manager persistence, acquisition into the dedicated corpus, stable remote identity binding, durable incremental sync state, index rebuild/local query, and a second incremental sync against real X data.

Current M0 implementation includes:

- generic profile and per-surface non-secret configuration;
- explicit `sociarium corpus init` boundary for operator-owned Git-safe corpora;
- profile-aware local no-network preflight with native credential round-trip;
- durable stable remote-profile binding reconstructed from corpus evidence;
- native OAuth2 Authorization Code + S256 PKCE for X;
- profile-scoped Windows Credential Manager persistence;
- refresh-token rollover and automatic refresh before sync;
- direct Rust X API acquisition;
- raw + normalized acquisition bundles;
- generic `Post` reply/quote relationship fields and X normalization support;
- durable incremental/cursor state with crash recovery;
- rebuildable SQLite/FTS index;
- CLI auth, sync, list, and search commands;
- committed reproducible `Cargo.lock`, resolver 3, and explicit MSRV-compatible dependency choices;
- Linux stable, native Windows, and Rust 1.85 locked CI coverage.

M0 corpus-history claim: the first X sync can only bootstrap the history the current user-post timeline exposes (approximately the most recent 3,200 Posts). Sociarium's durable value is preserving acquired evidence and maintaining forward incremental continuity; complete older lifetime history requires another acquisition source such as an export/import or broader remote archive access.

Explicitly not M0: posting, deleting, arbitrary public-X search, GUI, MCP, cross-platform identity resolution, additional adapters, deep thread/context acquisition, and full repost/reblog ontology.

## M1 — X corpus depth

- richer reply/quote/thread context beyond M0's minimal relationship references;
- explicit repost/reblog semantics and acquisition where available;
- richer profile snapshots/history;
- likes/bookmarks for authenticated owned profiles where API access permits;
- following/follower observations;
- more sync diagnostics and recovery tooling;
- media metadata and optional acquisition;
- corpus migrations/schema versioning;
- import/bootstrap paths for existing X archives or older history if useful.

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
