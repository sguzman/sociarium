# Sociarium implementation contract

This repository is an ongoing collaboration space. Preserve architectural intent, provenance, and documentation as first-class project state.

## Roles

- **Human principal:** owns goals, constraints, credentials, and final product authority.
- **ChatGPT:** project owner/director/architect/integrator; defines milestones, architecture, acceptance criteria, and reviews integration quality.
- **Codex/implementation agents:** implement bounded tasks against the repository contract. Do not silently redefine project goals or architecture.

## Non-negotiable architecture

1. Sociarium is not an X/Twitter client with future generalization bolted on. It is a generic social-data substrate whose first adapter is X.
2. Surfaces are adapters, not ontology.
3. Profiles are first-class synchronization scopes. Never introduce a hidden global "current account" assumption into core APIs.
4. A remote profile is not automatically a real-world person. Cross-profile identity links must be explicit and provenance-preserving.
5. Preserve raw, normalized, and derived layers as distinct concepts.
6. Durable corpus files are authoritative historical records. Indexes/caches must be rebuildable.
7. Surface integrations are Rust-native. Do not add `xurl`, XMCP, Node, Go, Python, or another platform CLI as a runtime dependency to avoid implementing an adapter properly.
8. Credentials are operational secret state outside the corpus. Persistent credential keys are profile-scoped; do not introduce a global surface credential singleton or store bearer material in TOML/corpus files.
9. MCP is an external projection/interface. Core behavior must not depend on MCP.
10. Remote writes are a separate capability boundary from local reads and should be conspicuous and auditable.
11. Do not silently edit user goals/prompts into a different task. If implementation pressure reveals a design conflict, record the conflict and surface it.

## Engineering rules

- Keep generic domain types in `sociarium-core`; keep platform payloads, token semantics, and endpoint behavior in their adapter crate.
- Keep persistent secret storage behind `sociarium-credentials`; it stores opaque bytes keyed by generic profile identity and must not know surface token schemas.
- Prefer stable typed identifiers over raw strings crossing every boundary.
- Preserve remote stable IDs separately from mutable handles/display names.
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

## Current milestone

M0 is one vertical slice:

`configured X profile -> native profile-scoped auth -> direct Rust X adapter -> raw evidence -> normalized posts -> durable local corpus -> rebuildable query index -> CLI query`

Repository implementation for M0 is complete. The remaining gate is a live Windows smoke test with a registered X Developer App and authorized account. Do not close M0 until native login, credential persistence, sync, durable checkpointing, index rebuild, and local query succeed against real X data.

Do not widen M0 to publishing, arbitrary public-X search, a GUI, recommendation feeds, or multiple adapters. The architecture must permit those later without implementing them now.
