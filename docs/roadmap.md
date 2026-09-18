# Roadmap

## M0 — zero-cost X archive vertical slice

Goal: prove Sociarium's acquisition/preservation architecture without requiring the operator to purchase X API credits.

Hard product constraint: **M0 must cost $0 in X API spend.**

The official X API backend is already implemented and CI-verified (issues #1–#7) and is preserved as an optional paid backend. X's current official developer model is pay-per-use, so its live smoke test is not an M0 completion requirement.

Active acceptance target (#8):

```text
first-party X account archive ZIP/directory
  -> inspect/detect archive layout
  -> identify self-owned X profile
  -> preserve source evidence
  -> normalize profile + authored Posts
  -> write durable corpus files
  -> establish/validate stable remote profile binding
  -> build/rebuild local search index
  -> query from CLI
```

M0 must not require:

- an X Developer App;
- X API credits;
- a paid third-party data provider;
- the user's X password, cookies, bearer token, or browser session;
- undocumented private X protocols.

Architectural consequence: **surface and acquisition source are different axes**. The surface remains `x`; evidence may come from the official API, X's account archive, or later acquisition mechanisms. Offline archive import should be implemented as an importer/acquisition source rather than pretending the archive is a new social surface.

Zero-cost M0 is complete when a real account archive imports successfully on Windows into a dedicated corpus, preserves raw/source evidence plus normalized Posts, establishes/validates stable profile identity, rebuilds the disposable search index, and returns the user's real Posts via local list/search.

The official API candidate remains frozen at `m0-rc1` for later deliberate paid validation.

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
