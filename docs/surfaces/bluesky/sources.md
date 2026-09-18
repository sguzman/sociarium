# Bluesky / AT Protocol source ledger

Last reviewed: 2026-09-18.

## Official protocol/developer sources

### AT Protocol repository specification

https://atproto.com/specs/repository

Used for:

- signed account repositories;
- content-addressed Merkle Search Tree model;
- complete CAR exports;
- sync/offline backup/migration use cases;
- record/blob separation.

### Account migration

https://atproto.com/guides/account-migration

Used for:

- repository export/import;
- blob migration;
- preference export/import;
- movement between PDS providers;
- acknowledgement of external service state such as chat.

### Account recovery

https://atproto.com/guides/account-recovery

Used for:

- periodic repo/blob/prefs backups;
- recovery from unavailable/uncooperative PDS;
- rotation-key/self-custody model;
- stable DID through migration.

### AT Protocol overview / identity

https://atproto.com/guides/overview

Used for:

- DIDs as durable identity;
- handles as mutable names;
- PDS/relay/AppView architecture;
- account portability rationale.

### Synchronization / repository event specification

https://atproto.com/specs/sync

Used for:

- commit events;
- create/update/delete operations;
- CAR slices;
- revision/incremental synchronization semantics.

### AT Protocol stack / Tap

https://atproto.com/guides/atproto

Used for:

- repository event streams;
- relays;
- Tap backfill + live firehose filtering;
- service architecture.

### Bluesky HTTP API reference

https://docs.bsky.app/docs/api

Used for:

- `app.bsky.*`, `com.atproto.*`, `chat.bsky.*`, `tools.ozone.*` API families;
- public AppView endpoint model;
- public unauthenticated GET availability through `public.api.bsky.app`.

### Bluesky rate limits

https://docs.bsky.app/docs/advanced-guides/rate-limits

Used for:

- hosted PDS overall request limits;
- account content-write point budgets;
- public API rate-limit guidance.

### AT Protocol OAuth

https://atproto.com/specs/oauth

and related OAuth/permissions documentation.

Used for:

- granular permission model;
- transitional scope evolution;
- separate chat/private permission considerations.

## Official Bluesky policy/product sources

### Bluesky Terms of Service

https://bsky.social/about/support/tos

Used for:

- user content ownership;
- distinction between Bluesky Social and third-party Developer Applications;
- decentralized-copy deletion caveat.

### Bluesky Privacy Policy

https://bsky.social/about/support/privacy-policy

Used for:

- portability/copy rights language;
- account/private-data context.

### Bluesky protocol/company essays and updates

Current Bluesky/AT Protocol posts describing:

- open developer ecosystem;
- data/identity portability;
- provider choice;
- user-controlled web direction.

These are supporting intent evidence. Protocol specifications outrank marketing/philosophical language when they conflict.

## Open-source references

### bluesky-social/atproto

https://github.com/bluesky-social/atproto

Reference implementation and protocol/Lexicon source.

### bluesky-social/social-app

https://github.com/bluesky-social/social-app

Source for the flagship social application.

Open source improves inspectability but direct protocol specifications remain the preferred authority for interoperable behavior.

## Evidence hierarchy

1. current AT Protocol specifications/Lexicons;
2. direct Sociarium observation;
3. current Bluesky developer documentation;
4. open-source reference implementation;
5. Bluesky policy/product explanations;
6. third-party implementation notes;
7. historical memory.

Because the core protocol is open, Sociarium should not substitute reverse-engineering folklore where a normative spec exists.
