# Bluesky / AT Protocol interfaces

Research date: 2026-09-18.

Evidence class: **official documented fact** unless marked otherwise.

## Protocol architecture

AT Protocol separates several responsibilities:

- decentralized identity via DIDs and handles;
- signed user repositories stored by a Personal Data Server (PDS);
- repository synchronization/event streams;
- relays indexing/broadcasting repository events;
- AppViews producing application-specific read models;
- labelers/moderation services;
- custom feeds and other services.

This is materially different from a conventional proprietary social API where one company database and one developer gateway define the whole surface.

## Lexicons

AT Protocol APIs and record schemas are defined through **Lexicons**.

Namespaces include:

- `com.atproto.*` — protocol/account/repository/sync primitives;
- `app.bsky.*` — Bluesky application records and views;
- `chat.bsky.*` — chat/DM service;
- `tools.ozone.*` — moderation tooling.

The Lexicon ecosystem is public and extensible.

## Public reads

Current Bluesky developer documentation encourages public read requests to:

`https://public.api.bsky.app`

Many `app.bsky.*` GET endpoints are callable without authentication.

This is useful for rendered/normalized public views without requiring the operator to surrender account credentials.

## Repository APIs

Important primitives include:

- `com.atproto.sync.getRepo` — obtain repository CAR data;
- `com.atproto.repo.getRecord` — retrieve a repository record;
- repository create/put/delete operations for authenticated writes;
- synchronization/event APIs for commit propagation.

Record retrieval can address repositories by handle or DID, though Sociarium should normalize on DID for durable identity.

## Incremental synchronization

The repository synchronization design includes commit events carrying repository changes.

Commit operations distinguish creates, updates, and deletes, with revision/root information and CAR slices sufficient for consumers to update local mirrors.

The protocol therefore gives Sociarium a first-class incremental primitive rather than forcing timeline polling to stand in for authoritative account change history.

Official tooling/documentation also describes **Tap**, which combines backfill and live event-stream filtering.

## Rate limits

Current Bluesky-hosted PDS documentation lists broad operational rate limits such as:

- 3,000 requests per 5 minutes per IP for overall PDS API requests;
- account content-write point budgets (create/update/delete weighted separately);
- generous public AppView API access.

Third-party providers can set different infrastructure limits.

These are service-protection limits, not pay-per-resource billing.

## Authentication

AT Protocol's current OAuth design supports granular permission scopes/permission sets.

The ecosystem is transitioning away from older broad transitional scopes toward more explicit permissions. Private chat permissions are separately scoped.

Because the auth system is evolving, a future Sociarium integration should implement the current protocol at implementation time rather than freezing today's transitional names as ontology.

## Open-source implementation

Bluesky publishes major AT Protocol implementation code and the social application source.

This greatly improves auditability:

- wire behavior can be compared with source;
- Lexicon definitions are inspectable;
- protocol changes can be tracked in public repositories;
- private first-party reverse engineering is not required for ordinary core protocol semantics.

## Sociarium implication

For Bluesky, the most authoritative self-owned public-social acquisition source is likely the signed repository itself, not a rendered AppView timeline.

AppView data is still useful, but it is a derived application view over protocol state and other indexed information.
