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

Repository implementation status: **structurally complete but still under pre-live hardening**. A current-contract audit performed before the first live X run found repository-side work that must be resolved before the end-to-end smoke test.

Open M0 pre-live work is tracked in issues #2–#6:

- correct the X user-timeline wire request and relationship/repost semantics (#5);
- make X/OAuth failure diagnostics non-secret by construction (#4);
- make OAuth loopback handling bounded and resilient (#3);
- initialize a separate Git-safe operator corpus rather than conflating it with the public software checkout (#6);
- compose the settled local boundaries into a profile-aware no-network preflight (#2).

Preferred implementation order is **#5 -> #4 -> #3 -> #6 -> #2**.

After those are integrated and CI is green, the final M0 gate is one live end-to-end smoke test with a registered X Developer App and authorized account, exercising native login, persisted credentials, acquisition into the dedicated corpus, durable sync state, index rebuild, and local query against real remote data.

Current M0 implementation includes:

- generic profile and per-surface non-secret configuration;
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
