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
8. MCP is an external projection/interface. Core behavior must not depend on MCP.
9. Remote writes are a separate capability boundary from local reads and should be conspicuous and auditable.
10. Do not silently edit user goals/prompts into a different task. If implementation pressure reveals a design conflict, record the conflict and surface it.

## Engineering rules

- Keep generic domain types in `sociarium-core`; keep platform payloads and endpoint semantics in their adapter crate.
- Prefer stable typed identifiers over raw strings crossing every boundary.
- Preserve remote stable IDs separately from mutable handles/display names.
- Avoid premature universalization: normalize shared semantics, retain surface-specific extensions when semantics do not cleanly match.
- Every durable transformation should be explainable from provenance.
- Heavy I/O, network work, indexing, and parsing must not be coupled to any future UI/render thread.
- New architectural decisions should update docs and, when consequential, add an ADR under `docs/decisions/`.
- A feature is not complete when code works but repository format, CLI semantics, or architectural behavior changed without documentation.

## Current milestone

M0 is one vertical slice:

`configured X profile -> direct Rust X adapter -> raw evidence -> normalized posts -> durable local corpus -> rebuildable query index -> CLI query`

Do not widen M0 to publishing, arbitrary public-X search, a GUI, recommendation feeds, or multiple adapters. The architecture must permit those later without implementing them now.
