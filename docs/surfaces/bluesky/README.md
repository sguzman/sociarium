# Bluesky / AT Protocol surface dossier

- Surface: Bluesky
- Protocol: AT Protocol
- Operator/company: Bluesky, PBC for the flagship service; protocol is open and supports other providers
- Canonical domains: `bsky.app`, `bsky.social`, `atproto.com`
- Research started: 2026-09-18
- Last reviewed: 2026-09-18
- Operator priority: unranked relative to the non-X inventory
- Overall access tier: **Tier A — Sovereign-friendly**
- Confidence: high for public social data/repository portability; medium-high for private/app-specific state because those facilities continue to evolve

## Executive summary

Bluesky is the strongest sovereignty fit in the initial Sociarium inventory.

AT Protocol was designed around portable identity and signed personal data repositories rather than treating one company's developer API as the sole representation of the social surface.

Core public social state has unusually strong properties for Sociarium:

- accounts have stable decentralized identifiers (DIDs) independent of mutable handles;
- user repository records are signed/content-addressed;
- complete repositories can be exported as CAR files;
- repositories support diff/synchronization semantics;
- account migration between Personal Data Servers (PDSes) is an explicit protocol workflow;
- repository event streams support live incremental mirroring;
- public AppView reads are available through a free public API surface;
- protocol schemas (Lexicons) and major implementations are open source;
- third parties can operate PDSes, relays, AppViews, feeds, and other protocol services.

This means Sociarium does not need to treat the Bluesky first-party web application as an adversarial black box in order to understand ordinary public social data. The important interfaces are intentionally documented and inspectable.

## Important caveat: "Bluesky account" is broader than the public repo

Not every user-visible data class is necessarily inside the public signed AT repository.

Current architecture treats some state separately, including:

- private actor preferences;
- bookmarks/private application state;
- chat/Direct Messages;
- service-specific moderation or account state.

Official migration documentation explicitly handles preferences separately from the repo and notes that state may also exist in external services such as centralized chat.

Sociarium should therefore avoid saying "repo export = literally every piece of Bluesky account state."

The public social graph/content layer is strongly Tier A. Some private state is closer to Tier A/B depending on the specific API/export mechanism.

## Why Tier A

Bluesky satisfies nearly every Sociarium sovereignty criterion:

- **Free useful reads:** public AppView GETs are broadly available without authentication.
- **Stable identity:** permanent DID is the durable account identity; handles can change.
- **Portable data:** complete signed repositories can be exported/imported.
- **Incremental synchronization:** repository commit event streams are a first-class protocol mechanism.
- **Open protocol:** Lexicons and protocol specs are public.
- **Open implementation:** major server/client implementations are open source.
- **Migration:** PDS migration and recovery are documented protocol workflows.
- **Self-hosting:** users/providers can operate PDS infrastructure.
- **No pay-per-self-read developer toll:** no X-style per-resource billing was found for ordinary AT Protocol/public Bluesky reads in this baseline.
- **Protocol archaeology is optional rather than foundational:** the core interface is documented.

## Identity sovereignty nuance

A DID is the durable identity anchor.

For `did:plc`, maximum recovery sovereignty depends on custody of the DID rotation key. Official recovery guidance explains that the PDS commonly holds a rotation key by default, while users can self-custody an additional key for recovery/migration.

This is powerful, but it should not be flattened into the claim that every default Bluesky account is automatically independent of its host in every operational sense.

`did:web` is another identity mechanism with different control assumptions.

## Acquisition candidates

### Repository export / direct repo synchronization

Best fit for canonical public account-owned records.

Useful for:

- posts;
- likes;
- follows;
- lists and other repository records;
- exact record CIDs/URIs;
- full signed repository backup;
- migration/provenance.

### Repository event stream / relay / Tap

Best fit for continuous change observation.

The protocol emits repository commit events describing create/update/delete operations, and official tooling supports backfill plus live stream filtering.

### Public AppView API

Best fit for convenient normalized/rendered views.

Useful for:

- profiles;
- post/thread views;
- feeds;
- searches;
- social graph views;
- other `app.bsky.*` queries.

Many GET endpoints are callable through `public.api.bsky.app` without authentication.

### Authenticated account APIs

Needed for private/service-specific state such as preferences, bookmarks, chat, and account operations.

These should be assessed separately because they do not all share the repository's portability semantics.

## Implementation recommendation

**Bluesky is implementation-admissible in principle**, but the current R1 research freeze still applies.

When implementation resumes, Bluesky should probably not be modeled as one monolithic "API adapter." A better acquisition design would distinguish:

1. signed repository acquisition/export;
2. incremental repo-event synchronization;
3. AppView-derived convenience observations;
4. private/authenticated service state.

That preserves the protocol's own authority boundaries instead of collapsing everything into the flagship AppView.

## Files

- [Access matrix](access-matrix.md)
- [Protocol and API](protocol-and-api.md)
- [Export, migration, and recovery](export-and-portability.md)
- [Private/application state](private-state.md)
- [Terms and constraints](terms-and-constraints.md)
- [Direct observation plan](observation-plan.md)
- [Source ledger](sources.md)
