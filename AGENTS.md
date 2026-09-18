# Sociarium research and implementation contract

This repository is an ongoing collaboration space. Preserve research provenance, architectural intent, and documentation as first-class project state. During the Surface Atlas phase, documentation outranks implementation velocity.

## Roles

- **Human principal:** owns goals, constraints, credentials, and final product authority.
- **ChatGPT:** project owner/director/research lead/architect/integrator; defines research programs, milestones, architecture, acceptance criteria, and reviews evidence and integration quality.
- **Codex/implementation agents:** implement bounded tasks only after admission by the current project phase. Do not invent coding work during a research freeze or silently redefine project goals.

## Non-negotiable architecture

1. Sociarium is not an X/Twitter client with future generalization bolted on. It is a generic social-data substrate whose first surface integration is X.
2. Surfaces are not acquisition sources and neither is ontology. One surface may have multiple explicit acquisition mechanisms (API, archive import, later capture sources).
3. Profiles are first-class synchronization scopes. Never introduce a hidden global "current account" assumption into core APIs.
4. A remote profile is identified durably by surface + stable remote ID. Handles/display names are mutable observations, not identity keys. One local profile must never silently become a bucket for multiple remote profile IDs.
5. A remote profile is not automatically a real-world person. Cross-profile identity links must be explicit and provenance-preserving.
6. Preserve raw, normalized, and derived layers as distinct concepts.
7. Durable corpus files are authoritative historical records. Indexes/caches must be rebuildable.
8. The software source repository and the operator's corpus repository are separate authority domains. Do not default real corpus data into the public source checkout.
9. Surface integrations are Rust-native. Do not add `xurl`, XMCP, Node, Go, Python, or another platform CLI as a runtime dependency to avoid implementing an adapter properly.
10. Credentials are operational secret state outside the corpus. Persistent credential keys are profile-scoped; do not introduce a global surface credential singleton or store bearer material in TOML/corpus files.
11. Private/authorized social data is not necessarily credential material. A corpus can be secret-free while still inappropriate to publish publicly.
12. Git may version/transport corpus state, but it is not the persistence API. GitHub is never corpus authority.
13. MCP is an external projection/interface. Core behavior must not depend on MCP.
14. Remote writes are a separate capability boundary from local reads and should be conspicuous and auditable.
15. Do not silently edit user goals/prompts into a different task. If implementation pressure reveals a design conflict, record the conflict and surface it.
16. **Research before new surface-specific code.** A new adapter/importer/private-protocol implementation normally requires a current Surface Atlas dossier and an explicit implementation-admission decision.
17. Documentation-only outcomes are valid. Do not manufacture implementation work merely to preserve coding momentum.
18. Tier C — Adversarial means protocol archaeology is a legitimate research task when sanctioned interfaces materially obstruct self-data access. It does not authorize credential theft, impersonation, or bypassing meaningful authorization controls.

## Research rules

- The Surface Atlas is first-class project state, not disposable planning material.
- For material surface claims, distinguish documented fact, direct observation, inference, and unknown.
- Date private/undocumented protocol observations. Do not rewrite a historical observation into an eternal claim when the remote system changes.
- Investigate access per data class; do not let one overall tier hide different conditions for posts, likes, bookmarks, relationships, media, or messages.
- Record monetary cost and recurring manual friction explicitly.
- The official API is one acquisition source, not the definition of a surface.
- On adversarial surfaces, the authorized first-party web/mobile client may be treated as protocol evidence.
- Do not commit live cookies, authorization headers, bearer/refresh tokens, passwords, private HAR captures, or other authority-bearing secrets to this public repository.
- Sanitized endpoint/operation/schema/pagination observations are preferred for public dossiers.
- Before proposing implementation, document the exact acquisition source being recommended and why it is preferable to the alternatives.
- The human principal supplies the surfaces they care about. Do not pad the active research inventory with unrelated platforms merely to make it look comprehensive.

## Engineering rules

- Keep generic domain types in `sociarium-core`; keep platform payloads, token semantics, endpoint behavior, and remote wire names in their adapter crate.
- Generic Sociarium names do not rename remote protocol parameters. For X, names such as `tweet.fields`, `referenced_tweets`, and `note_tweet` remain wire-level X details even though the core type is `Post`.
- A portable normalized field should contain the best complete value the acquired source exposes. In particular, do not knowingly normalize a truncated X `text` preview as `Post.text` when `note_tweet.text` contains the full authored text.
- Keep persistent secret storage behind `sociarium-credentials`; it stores opaque bytes keyed by generic profile identity and must not know surface token schemas.
- Prefer stable typed identifiers over raw strings crossing every boundary.
- Preserve remote stable IDs separately from mutable handles/display names.
- If configuration omits a remote stable ID, an established binding must be recoverable from ordinary durable corpus evidence or equivalent canonical state, not only from SQLite/cache state.
- Reject conflicting stable remote identity before a contradictory acquisition becomes canonical for an already-bound local profile.
- Avoid premature universalization: normalize shared semantics, retain surface-specific extensions when semantics do not cleanly match.
- Every durable transformation should be explainable from provenance.
- Persist each acquisition page before advancing its durable cursor. Pagination state and completed incremental high-water state are not interchangeable.
- Heavy I/O, network work, indexing, and parsing must not be coupled to any future UI/render thread.
- New architectural decisions should update docs and, when consequential, add or amend an ADR under `docs/decisions/`.
- A feature is not complete when code works but repository format, CLI semantics, or architectural behavior changed without documentation.
- Rust 1.85 is the declared MSRV. Keep workspace resolver 3 enabled, preserve MSRV CI, and treat a dependency change that raises the Rust floor as an explicit compatibility decision.
- `Cargo.lock` is committed application build state. Do not delete it to force fresh resolution; dependency updates must deliberately refresh it and pass CI with `--locked` afterward.
- The exact `idna_adapter` and Windows `keyring` constraints are compatibility/backend decisions documented in ADR 0005 and ADR 0004; do not loosen them accidentally during routine dependency cleanup.
- Keep native Windows CI because the primary credential backend is Windows-specific and must not rot behind `#[cfg(windows)]`.
- Successful remote social responses may be durable raw evidence. Failed OAuth/API protocol bodies are diagnostics, not automatically corpus evidence, and must not be dumped verbatim into pasteable errors.

## Current milestone

The active milestone is **R0 — Surface inventory**.

The human principal will enumerate the social sites they care about. Sociarium will then investigate those surfaces systematically before selecting any new implementation target.

### Research-first rules

1. **No new surface-specific code during R0.**
2. Do not resume the X archive importer merely because partial code already exists.
3. Do not begin a second adapter because it seems technically attractive.
4. Do not turn unknowns into assumptions. Record them as unknown.
5. Current public facts about APIs, pricing, exports, access policy, or protocol behavior require fresh research and dated sources.
6. For undocumented/private first-party protocols, distinguish direct observation from inference and date every claim.
7. A dossier that concludes `do not implement` is successful project work.
8. No surface is owed an adapter.
9. Cost and recurring manual friction are architectural facts, not incidental product annoyances.
10. For Tier C / Adversarial surfaces, protocol archaeology is a legitimate research path. Repeated manual export rituals are not normalized as the default solution.
11. Adversarial research does not imply credential theft, access-control bypass, or acquisition of data the operator is not authorized to access.
12. Sensitive raw captures do not belong in the public repository.

### Implementation admission gate

Before new surface-specific code is normally allowed, a current dossier should establish:

- overall access tier;
- per-data-class access matrix;
- documented APIs;
- export/portability behavior;
- first-party private protocol status;
- authentication model;
- cost model;
- stable identity/object IDs;
- pagination/history behavior;
- incremental-sync potential;
- protocol volatility;
- relevant constraints/unknowns;
- the exact acquisition source proposed for implementation and why.

The human principal may deliberately override this gate. ChatGPT/Codex must never silently override it merely to keep coding.

### Existing code

Existing Rust infrastructure remains supported project state and may receive preservation/security/build fixes when needed. It is **not** the active research agenda.

The frozen `m0-rc1` branch preserves the official X API candidate. The in-progress X archive importer is historical/reusable work, not an instruction to continue implementation.

See:

- `docs/surfaces/README.md`
- `docs/surface-research-doctrine.md`
- `docs/surface-access-tiers.md`
- `docs/surfaces/TEMPLATE.md`
- ADR 0007
