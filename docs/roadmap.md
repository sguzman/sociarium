# Roadmap

Sociarium is now research-first.

The project does not treat implementation velocity as its primary measure of progress. Documentation, provenance, surface access mapping, and protocol archaeology are first-class deliverables.

## R0 — Surface inventory

**Current phase.**

Goal: enumerate the social surfaces the human principal actually cares about.

Deliverables:

- an explicit surface inventory under `docs/surfaces/`;
- no speculative padding with platforms the operator did not ask to investigate;
- a provisional research status for each surface;
- any immediately known constraints recorded as provisional rather than silently treated as final.

Implementation policy:

- no new surface-specific adapter/importer work during R0;
- preserve existing code and keep it buildable;
- do not invent implementation tasks merely to maintain coding momentum.

R0 completes when the principal's initial surface list has been captured and the research queue is explicit.

## R1 — Surface dossiers

Goal: produce healthy, abundant, provenance-heavy documentation for the surfaces in the inventory.

For each surface, investigate:

- documented/public APIs;
- authenticated self-data APIs;
- official exports/archives;
- public web access;
- first-party private web/mobile protocols where warranted;
- stable identifiers;
- pagination/history windows;
- edits/deletions;
- media;
- likes/reactions;
- bookmarks/saves;
- follows/followers;
- lists/collections;
- private messages when authorized and relevant;
- live-update channels;
- authentication boundaries;
- rate limits;
- pricing/billing;
- portability;
- incremental acquisition;
- historical completeness;
- protocol drift.

Each dossier should distinguish documented fact, direct observation, inference, and unknowns.

### R1 acceptance

A surface dossier is mature enough for classification when it contains:

- a dated executive summary;
- an access matrix by data class and acquisition mechanism;
- cost/friction analysis;
- stable-identity notes;
- acquisition candidates;
- explicit unknowns;
- provenance for material claims.

Research may conclude that a surface should not be implemented.

## R2 — Tiering and comparative Surface Atlas

Goal: compare surfaces without collapsing important differences.

Overall tiers:

- **Tier A — Sovereign-friendly**
- **Tier B — Workable**
- **Tier C — Adversarial**
- **Tier D — Inaccessible**

A tier is a dated summary of the access relationship, not a rating of the platform itself.

Per-data-class exceptions remain explicit. A site may be workable for public posts and adversarial for bookmarks or message history.

### Adversarial research track

Tier C activates protocol archaeology.

When sanctioned interfaces materially obstruct self-data access, Sociarium investigates the first-party client protocol as evidence, including:

- endpoints/operations;
- request and response schemas;
- pagination;
- stable IDs;
- feature flags;
- auth/session boundaries;
- WebSocket/SSE/live channels;
- change history.

This does not authorize credential theft, impersonation, or defeating meaningful access controls.

## R3 — Implementation admission

Goal: decide which researched acquisition sources deserve code.

A new surface-specific implementation should normally require:

- current dossier;
- dated tier assessment;
- per-data-class access matrix;
- authentication model;
- cost model;
- stable IDs;
- history/pagination behavior;
- protocol volatility notes;
- explicit acquisition-source choice;
- rationale for why implementation is worthwhile.

Possible outcomes include:

- implement documented API adapter;
- implement archive/import bootstrap;
- implement an authorized private-protocol adapter;
- combine multiple acquisition sources;
- defer;
- document only.

**“Do not implement” is a valid outcome.**

## M0 — First post-atlas implementation target

Only after R0–R3 produce an implementation admission decision should Sociarium resume active surface-specific implementation.

The exact target is intentionally **not predetermined**.

The existing X work does not automatically win this slot merely because it already has code.

M0 should prove that the research process can drive a technically useful acquisition path into the existing durable corpus substrate.

## M1 — Corpus depth and cross-source reconciliation

After a post-atlas vertical slice is proven:

- richer object semantics;
- reconciliation of observations from multiple acquisition sources for one surface;
- media metadata/acquisition where justified;
- historical snapshots;
- edit/deletion observations;
- relationship/history depth;
- schema migrations.

## M2 — Agent surface

- semantic read-only MCP interface over the local corpus;
- evidence/provenance retrieval;
- bounded context/export views;
- no arbitrary SQL/shell escape hatch.

## M3 — Cross-surface identity and analysis

- explicit entities/personas/projects;
- evidence-backed links between remote profiles;
- cross-surface queries;
- longitudinal comparisons;
- corpus-level derived views.

## Existing pre-atlas implementation

Before ADR 0007, Sociarium built substantial X-oriented infrastructure:

- official X OAuth2/PKCE + API adapter;
- Windows credential persistence;
- profile binding;
- durable acquisition bundles;
- sync/checkpoint machinery;
- search/indexing;
- partial X archive-import work.

That work remains preserved and useful.

It is not deleted, and it is not the active roadmap.

The official API candidate is preserved on `m0-rc1`. Archive-import work on `main` is frozen pending Surface Atlas research. Neither is allowed to define the next milestone by inertia.

## Permanent rule

**Research chooses implementation. Implementation does not choose the research agenda.**
