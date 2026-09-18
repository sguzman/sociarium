# Surface dossier template

Copy this template into `docs/surfaces/<surface>/` and split it into multiple files when the dossier becomes large.

## Identity

- Surface:
- Operator/company:
- Canonical domains:
- Clients investigated:
- Research started:
- Last reviewed:
- Overall access tier:
- Confidence:

## Executive summary

Summarize how the surface allows the operator to acquire and preserve their own data. State the major opportunities, restrictions, costs, and unknowns.

## Access matrix

| Data class | Documented API | Official export | Public web | Private web protocol | Mobile protocol | Live channel | Cost | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| profile | unknown | unknown | unknown | unknown | unknown | unknown | unknown | |
| authored posts | unknown | unknown | unknown | unknown | unknown | unknown | unknown | |
| replies/quotes/reposts | unknown | unknown | unknown | unknown | unknown | unknown | unknown | |
| likes/reactions | unknown | unknown | unknown | unknown | unknown | unknown | unknown | |
| bookmarks/saves | unknown | unknown | unknown | unknown | unknown | unknown | unknown | |
| follows/followers | unknown | unknown | unknown | unknown | unknown | unknown | unknown | |
| lists/collections | unknown | unknown | unknown | unknown | unknown | unknown | unknown | |
| media | unknown | unknown | unknown | unknown | unknown | unknown | unknown | |
| direct/private messages | unknown | unknown | n/a | unknown | unknown | unknown | unknown | |

## Stable identity

Document profile/account IDs, object IDs, mutable handles, URL shapes, and any evidence that identifiers survive renames or migrations.

## Documented APIs

For each relevant API:

- documentation source and date checked;
- authentication model;
- self-data capabilities;
- rate limits;
- history limits;
- pagination;
- scopes/permissions;
- pricing/billing;
- deletion/edit behavior;
- media behavior;
- important omissions.

Separate current documented facts from interpretation.

## Official export / portability

Document:

- how export is requested;
- whether it costs money;
- preparation/wait time if known;
- format;
- completeness;
- stable identifiers;
- timestamps;
- media;
- private/account-sensitive data;
- repeatability;
- whether export can support incremental acquisition or only bootstrap/recovery.

## Public web surface

Document what can be observed without authentication and whether stable URLs/IDs are exposed.

Do not confuse public rendering with a supported machine interface.

## Private first-party protocol

If research is warranted, record dated observations from the operator's own authorized client.

### Authentication boundary

Describe the shape of authentication without recording live secrets.

### Operations/endpoints

| Observed date | Client | Operation/endpoint | Purpose | Pagination | Notes |
| --- | --- | --- | --- | --- | --- |

### Object schemas

Record stable fields and relationships with sanitized examples.

### Pagination/history

Record cursors, windows, limits, sort order, and whether backfill/incremental traversal appears possible.

### Live channels

Record WebSocket/SSE/push channels and their semantics when relevant.

### Protocol drift

Append dated changes rather than rewriting old observations into timeless claims.

## Cost and friction

Record monetary cost and non-monetary friction separately:

- API charges;
- subscription requirements;
- export wait;
- recurring manual steps;
- captchas/device verification;
- rate limits;
- brittle protocol churn.

## Acquisition candidates

List plausible Sociarium acquisition sources and their tradeoffs.

Do not choose an implementation merely because it is easiest to code.

## Tier assessment

State:

- overall tier;
- per-data-class exceptions;
- evidence supporting the classification;
- uncertainty;
- what future observation could change the tier.

## Implementation recommendation

This section may conclude **do not implement yet**.

If implementation is justified, identify the exact source to target and the assumptions that must remain true.

## Sources and observations

Use dated entries. Distinguish:

- official documentation;
- public technical references;
- direct first-party client observations;
- private local captures not committed to the repository;
- inference.
