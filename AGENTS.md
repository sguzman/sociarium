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
7. The software source repository and the operator's corpus repository are separate authority domains. Do not default real corpus data into the public source checkout.
8. Surface integrations are Rust-native. Do not add `xurl`, XMCP, Node, Go, Python, or another platform CLI as a runtime dependency to avoid implementing an adapter properly.
9. Credentials are operational secret state outside the corpus. Persistent credential keys are profile-scoped; do not introduce a global surface credential singleton or store bearer material in TOML/corpus files.
10. Private/authorized social data is not necessarily credential material. A corpus can be secret-free while still inappropriate to publish publicly.
11. Git may version/transport corpus state, but it is not the persistence API. GitHub is never corpus authority.
12. MCP is an external projection/interface. Core behavior must not depend on MCP.
13. Remote writes are a separate capability boundary from local reads and should be conspicuous and auditable.
14. Do not silently edit user goals/prompts into a different task. If implementation pressure reveals a design conflict, record the conflict and surface it.

## Engineering rules

- Keep generic domain types in `sociarium-core`; keep platform payloads, token semantics, endpoint behavior, and remote wire names in their adapter crate.
- Generic Sociarium names do not rename remote protocol parameters. For X, names such as `tweet.fields` and `referenced_tweets` remain wire-level X details even though the core type is `Post`.
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
- Successful remote social responses may be durable raw evidence. Failed OAuth/API protocol bodies are diagnostics, not automatically corpus evidence, and must not be dumped verbatim into pasteable errors.

## Current milestone

M0 is one vertical slice:

`configured X profile -> native profile-scoped auth -> direct Rust X adapter -> raw evidence -> normalized posts -> durable local corpus -> rebuildable query index -> CLI query`

The vertical slice is structurally implemented, but a 2026-09-17 pre-live audit found repository-side hardening that must be completed before the deliberate real-X smoke test. M0 is **not repository-complete** while issues #2–#6 remain open.

### Current implementation queue

Implement bounded issues from the repository rather than asking the human principal to relay prompts between agents. Preferred order:

1. **#5 — X timeline wire-contract blocker.** Correct `post.fields` to the documented `tweet.fields`, request `referenced_tweets`, and make M0 repost semantics explicit without flattening unsupported repost relationships.
2. **#4 — safe X remote failure diagnostics.** Stop emitting arbitrary raw OAuth/API failure bodies while preserving useful structured status/category information.
3. **#3 — resilient OAuth loopback callback.** Add bounded waiting and tolerate unrelated local requests without weakening state/error validation.
4. **#6 — separate Git-safe corpus initialization.** Establish the operator corpus as a dedicated durable directory/repository, protect disposable/pending state from accidental Git history, and keep software source separate from social data.
5. **#2 — profile-aware local preflight.** Compose the now-stable config/callback/credential/corpus boundaries into a no-network readiness check safe to paste into issue evidence.

After each bounded issue:

- run formatting, strict clippy, workspace tests, native Windows CI, and Rust 1.85 `--locked` CI;
- update the issue/docs required by that task;
- close the issue only when its acceptance criteria are actually satisfied;
- do not begin the live X smoke test while a pre-live issue remains unresolved.

After #2–#6 are closed and CI is green, the remaining M0 gate is the documented live Windows smoke test with a registered X Developer App and authorized account, using the dedicated corpus boundary. Do not close M0 until native login, credential persistence, real sync, durable checkpointing, index rebuild, local query, and the second incremental sync succeed against real X data.

Do not widen M0 to publishing, arbitrary public-X search, a GUI, recommendation feeds, multiple adapters, deep thread acquisition, or full repost/reblog ontology. The architecture must permit those later without implementing them now.
