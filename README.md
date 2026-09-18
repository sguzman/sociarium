# Sociarium

Sociarium is a documentation-first, user-sovereign social-data project.

Its long-term software goal is a Rust-native substrate that can preserve selected social data in a durable, provenance-rich local corpus. Its **current primary artifact is the Surface Atlas**: a forensic map of how social platforms let an authorized user access, export, synchronize, and preserve their own data.

> **Research the surface before implementing the adapter. The platform's preferred developer interface is not the ontology of the platform.**

## Current phase: R2 active — X direct observation

The initial Surface Atlas inventory and baseline dossiers are complete for:

- X / Twitter;
- Reddit;
- Bluesky;
- Facebook;
- YouTube;
- Quora.

New surface-specific implementation remains intentionally frozen. Existing code may receive preservation/security/build fixes, but unfinished adapter/importer work does not define the agenda.

**R2 — direct first-party observation / adversarial protocol archaeology is active now.** X is first because it is the human principal's highest-priority surface.

Active execution log: [R2/X — direct first-party observation of authorized web client](https://github.com/sguzman/sociarium/issues/12).

Completed execution logs:

- [R0 — enumerate Surface Atlas inventory](https://github.com/sguzman/sociarium/issues/10)
- [R1 — build baseline Surface Atlas dossiers](https://github.com/sguzman/sociarium/issues/11)

**Stop condition:** no new surface-specific implementation until dossier/observation research supports an explicit implementation-admission decision or the human principal deliberately overrides the freeze.

For every surface, Sociarium asks:

> What mechanisms exist for a user to acquire, preserve, query, and continuously observe their own data, and how much control does the surface retain over each mechanism?

Research covers documented APIs, self-data APIs, official exports, public web representations, first-party private web/mobile protocols, live channels, identifiers, pagination/history limits, authentication, pricing, rate limits, portability, protocol drift, and per-data-class access.

A high-quality dossier is a successful deliverable even if no adapter is ever written. For some surfaces, documentation may remain more valuable than code indefinitely.

Start here:

- [Project charter](docs/project-charter.md)
- [Surface Atlas](docs/surfaces/README.md)
- [Surface research doctrine](docs/surface-research-doctrine.md)
- [Surface access tiers](docs/surface-access-tiers.md)
- [Surface dossier template](docs/surfaces/TEMPLATE.md)
- [ADR 0007: documentation-first Surface Atlas](docs/decisions/0007-documentation-first-surface-atlas.md)

## Access tiers

Sociarium classifies the **access relationship**, not whether a platform is socially good or bad.

- **Tier A — Sovereign-friendly:** strong, practical, user-controlled acquisition paths.
- **Tier B — Workable:** imperfect but usable without recurring payment, degradation, or protocol archaeology as the normal operating model.
- **Tier C — Adversarial:** sanctioned interfaces materially obstruct useful self-data access. Sociarium investigates the private first-party protocol rather than normalizing repeated manual groveling.
- **Tier D — Inaccessible:** useful acquisition currently appears to require crossing a meaningful authorization/security boundary or accepting constraints that make implementation unjustifiable.

A site-wide tier is only a summary. Dossiers also classify individual data classes such as posts, likes, bookmarks, relationships, media, and private messages.

## Adversarial does not mean passive

When a platform exposes useful functionality to its own authorized web/mobile client while withholding or paywalling equivalent developer access, the shipped first-party client becomes a research object.

Sociarium may document:

- endpoints and request families;
- GraphQL/RPC operation names;
- request and response schemas;
- stable IDs;
- pagination cursors;
- feature flags;
- authentication boundaries;
- WebSocket/SSE/live channels;
- protocol drift across dated observations.

This is protocol archaeology, not indiscriminate security bypass. Sociarium does not require credential theft, impersonation, or defeating meaningful access controls to obtain data the operator is not authorized to access.

## Documentation is first-class project state

Surface claims are provenance-heavy and time-bounded.

Sociarium distinguishes:

1. documented facts;
2. directly observed behavior;
3. inference;
4. unknowns.

Prefer a dated statement such as:

> Observed 2026-09-18: the first-party web client used operation X with cursor Y for the authenticated user's own timeline.

over a timeless assertion that a private protocol will always work that way.

Research history should remain visible when a platform changes.

## Existing software substrate

Substantial Rust infrastructure already exists and is preserved.

It includes:

- surface-independent profile/Post ontology;
- source-neutral acquisition envelopes;
- durable raw + normalized acquisition bundles;
- stable remote-profile binding;
- rebuildable SQLite/FTS search;
- profile-scoped synchronization and crash recovery;
- configuration and CLI boundaries;
- Windows Credential Manager integration;
- a direct Rust X OAuth2/PKCE + API adapter;
- work toward first-party X archive import.

The X official API candidate is preserved on `m0-rc1`. Current X API commercial terms made it a poor basis for the project's active milestone, which is one reason the research-first doctrine now exists.

Existing code is **reusable substrate and historical implementation evidence**. It does not obligate Sociarium to keep coding X, complete the archive path immediately, or choose any next surface before research.

## Core invariants

- **Surface != acquisition source.** A social surface may expose an official API, archive, public representation, private first-party protocol, or several of these simultaneously.
- **Research before new implementation.** New surface-specific code normally requires a current dossier and implementation-admission decision.
- **Local durability.** Once evidence is acquired, Sociarium can preserve it independently of the remote surface.
- **Profiles are first-class.** No hidden global current-account singleton.
- **Stable IDs beat mutable handles.** Remote identity is based on durable surface IDs where available.
- **Provenance survives normalization.** Raw/source evidence, normalized records, and derived views remain distinct.
- **Indexes are disposable.** Search/cache projections are rebuildable from durable corpus data.
- **Credentials are not corpus data.** Passwords, cookies, bearer tokens, refresh tokens, and session secrets do not belong in the public research repository or Git-tracked corpus.
- **Private data is not automatically publishable.** Secret-free evidence can still contain sensitive account data.
- **Git is history/transport, not the database.**
- **No site is owed an adapter.** Documentation may be the correct final result for a surface.

## Research and implementation workflow

```text
enumerate surfaces
        ↓
official/documented research
        ↓
public technical research
        ↓
first-party protocol observation where warranted
        ↓
per-data-class access matrix
        ↓
dated tier assessment
        ↓
acquisition candidates
        ↓
implementation admission decision
        ↓
only then: adapter/importer work
```

The Surface Atlas continues to matter after implementation because access policies, prices, APIs, exports, and private protocols change.

## Repository map

Research:

- `docs/project-charter.md` — project purpose, progress model, and research-before-code constitution.
- `docs/surfaces/` — per-surface atlas and dossiers.
- `docs/surface-research-doctrine.md` — evidence/provenance and investigative rules.
- `docs/surface-access-tiers.md` — Tier A/B/C/D definitions.
- `docs/decisions/` — architectural/research decisions.

Software architecture:

- `sociarium-core` — surface-independent social ontology and identifiers.
- `sociarium-acquisition` — source-neutral acquisition evidence/envelopes.
- `sociarium-adapter` — live remote-adapter contract.
- `sociarium-adapter-x` — existing optional X API adapter.
- `sociarium-import-x-archive` — in-progress X archive importer preserved from the pre-atlas phase.
- `sociarium-config` — non-secret configuration.
- `sociarium-credentials` — profile-scoped secret-storage boundary.
- `sociarium-store` — durable corpus persistence.
- `sociarium-sync` — synchronization orchestration.
- `sociarium-search` — rebuildable SQLite/FTS projection.
- `sociarium-cli` — composition root.

## Software repo versus corpus/evidence

`sguzman/sociarium` is public project source and public research documentation.

Real social data, unsanitized browser captures, HAR files, cookies, authorization headers, private messages, and other account-sensitive evidence do **not** belong in this public repository.

A future private/local evidence vault may preserve raw forensic captures when needed. Public dossiers should contain sanitized structural observations and enough provenance to reproduce or verify them without leaking live authority.

## Roadmap

The project no longer treats “finish the next adapter” as the default definition of progress.

See [docs/roadmap.md](docs/roadmap.md) for the research-first milestone sequence.

## Development status

The existing Rust workspace remains buildable project state. Code maintenance, security fixes, and preservation work may continue when needed, but new surface-specific feature implementation is not the active priority during the Surface Atlas research phases.

The declared MSRV remains Rust 1.85 and CI continues to protect the existing substrate.

## License

No license has been selected yet. Until one is explicitly added, normal copyright restrictions apply.
