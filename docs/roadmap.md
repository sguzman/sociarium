# Roadmap

Sociarium is now a **documentation-first digital-forensics and social-surface access project**.

Code is downstream of research. The project does not measure progress primarily by adapter count. During the research milestones, the main unit of progress is durable knowledge: inventories, dossiers, observations, access matrices, protocol maps, and dated classifications.

The project charter is [project-charter.md](project-charter.md).

The durable research program is the [Surface Atlas](surfaces/README.md).

## R0 — Surface inventory

**Complete.**

Goal: enumerate the social surfaces the human principal actually cares about.

The human principal supplies the list. Do not pad the inventory with fashionable or obvious sites merely to create work.

For each named surface, record only enough initial metadata to establish a research queue:

- surface name;
- canonical domain/operator;
- why it matters to the operator;
- whether an existing Sociarium integration/research artifact already exists;
- current research status;
- provisional tier only when evidence already supports one.

R0 is complete when the initial inventory exists and the human principal agrees that it reflects the sites worth investigating. Issue #10 is the durable execution log for this phase.

**R0 is not complete because code was written.**

## R1 — Surface Atlas baseline dossiers

**Complete for the initial six-surface inventory as of 2026-09-18.**

Investigate each R0 surface using the [research doctrine](surface-research-doctrine.md) and [access tiers](surface-access-tiers.md).

Every baseline dossier should establish, as far as evidence permits:

- documented public and authenticated APIs;
- official export/archive mechanisms;
- public web representations;
- first-party private web/mobile protocol surface;
- stable profile/object identifiers;
- authentication boundaries;
- pagination and historical windows;
- edit/deletion semantics;
- media behavior;
- follows/relationships;
- likes/reactions;
- bookmarks/saves;
- lists/collections;
- private messages where relevant and authorized;
- live channels such as WebSocket/SSE/push;
- monetary cost;
- non-monetary friction;
- rate/automation limits;
- incremental synchronization potential;
- historical completeness;
- protocol volatility;
- explicit unknowns.

Each dossier gets:

1. a dated overall access tier;
2. a per-data-class access matrix;
3. evidence provenance;
4. acquisition candidates;
5. an implementation-admission recommendation, which may be **do not implement**.

Documentation alone is a valid completed result.

## R2 — Adversarial protocol archaeology

**Active research phase. X is first because it is the operator's highest-priority surface. Execution is tracked in issue #12.**

For Tier C surfaces, investigate the protocol actually shipped to the authorized user when sanctioned developer interfaces are materially inadequate.

Research may include:

- first-party web request families;
- GraphQL/RPC operation names;
- endpoint structure;
- request/response schemas;
- feature flags;
- stable identifiers;
- pagination cursors;
- history/backfill behavior;
- authenticated session boundaries;
- live channels;
- error forms;
- dated protocol drift.

This phase is **not** a mandate to bypass meaningful authorization/security boundaries.

Raw HAR files, cookies, bearer tokens, private messages, and other sensitive captures stay out of the public repository. Public dossiers preserve sanitized structural findings and provenance.

R2 can conclude that a surface remains impractical or should stay documentation-only.

## R3 — Comparative access atlas

Once several dossiers exist, maintain a cross-surface matrix covering at least:

- overall tier;
- per-data-class accessibility;
- free versus paid self-data access;
- historical completeness;
- incremental sync;
- export quality;
- stable identifiers;
- private-protocol availability;
- protocol volatility;
- recurring manual friction.

The purpose is to make platform dependence visible and comparable rather than rediscovering constraints one integration at a time.

## I0 — Implementation admission

Implementation resumes only after research identifies a worthwhile acquisition target.

Before new surface-specific code begins, the chosen surface/source should have a current dossier that answers:

- why this acquisition source is preferable;
- what data classes it can retrieve;
- what it costs;
- what authorization it needs;
- what history it exposes;
- what stable IDs it exposes;
- what protocol assumptions may rot;
- what alternatives were rejected and why.

The human principal may deliberately override this gate. Agents may not silently bypass it to maintain coding velocity.

## Existing implementation substrate

The pre-Atlas Rust work remains valuable and preserved:

- surface-independent profile/Post ontology;
- source-neutral acquisition envelopes;
- durable raw + normalized evidence;
- stable remote-profile binding;
- rebuildable SQLite/FTS search;
- profile-scoped synchronization;
- Windows credential storage;
- direct Rust X OAuth2/PKCE + API support;
- partial X archive import work.

The official X API candidate remains frozen on `m0-rc1`.

The X archive importer is **not an active milestone**. It is preserved implementation work that may be resumed later if the X dossier identifies archive ingestion as a useful acquisition source.

## Later software milestones

Only after the research program has admitted implementation targets:

- additional live adapters/importers;
- richer cross-surface identity analysis;
- semantic MCP/agent interfaces over the local corpus;
- media acquisition;
- scheduled synchronization;
- user interfaces;
- explicit remote-write capabilities.

These are intentionally subordinate to the Surface Atlas. A well-researched surface with no adapter can be more valuable to Sociarium than a hastily implemented integration.
