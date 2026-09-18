# Sociarium implementation contract

This repository is an ongoing collaboration space. Preserve architectural intent, provenance, and documentation as first-class project state.

## Roles

- **Human principal:** owns goals, constraints, credentials, and final product authority.
- **ChatGPT:** project owner/director/architect/integrator; defines milestones, architecture, acceptance criteria, and reviews integration quality.
- **Codex/implementation agents:** implement bounded tasks against the repository contract. Do not silently redefine project goals or architecture.

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

M0 is now the **zero-cost X archive vertical slice** tracked in issue #8.

Hard constraint: do not require paid X API credits for M0. The official API backend is already implemented and preserved as an optional paid integration; do not make the human principal fund it merely to finish M0.

Target:

`X account archive -> archive importer -> raw/source evidence -> normalized self-owned profile + Posts -> durable corpus -> stable profile binding -> rebuildable query index -> CLI query`

### Current implementation queue

1. Implement a dedicated `sociarium-import-x-archive` crate rather than forcing offline import through the network-oriented `SocialAdapter` trait.
2. Support ZIP and extracted-directory input with traversal-safe ZIP handling.
3. Detect historical X/Twitter archive file shapes defensively, including JavaScript-assignment-wrapped JSON.
4. Normalize self-owned profile and authored Posts using stable native X IDs; handles remain mutable.
5. Preserve enough original archive evidence and provenance to explain every normalized record.
6. Make re-import idempotent at the visible/query layer and safe for newer archives.
7. Reuse the existing durable profile-binding guard so a different X account cannot be imported beneath an established local profile.
8. Wire CLI `import x-archive <ZIP_OR_DIR> --profile <id>`, index rebuild, and local list/search.
9. Add a real-archive Windows validation run after synthetic fixtures are green.

Do not add browser scraping, private X protocols, paid third-party APIs, posting, arbitrary public-X search, GUI work, recommendation feeds, multiple new surfaces, or MCP to M0.

The frozen `m0-rc1` branch remains the official-API candidate. Do not mutate its meaning; new zero-cost work proceeds on `main`.
