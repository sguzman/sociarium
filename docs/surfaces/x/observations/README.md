# X direct observations

This directory contains **sanitized, dated observations** of the first-party X client used by the operator's own authorized account.

It is not a raw-capture vault.

## What belongs here

- operation names;
- request-path patterns;
- variable names;
- feature/field-toggle names;
- response envelope shapes;
- pagination cursor structure;
- stable public/self-owned identifiers;
- authentication/header **names** without values;
- browser/build metadata;
- dated conclusions;
- explicit unknowns;
- hashes of private local captures where useful.

## What does not belong here

Never commit:

- HAR files;
- browser cookies;
- Authorization values;
- CSRF values;
- OAuth tokens;
- session identifiers;
- bookmark contents unless deliberately sanitized;
- Direct Messages;
- private notification bodies;
- any other raw private account payload.

## Observation lifecycle

1. Capture one bounded first-party interaction locally.
2. Keep the raw capture private.
3. Hash the raw file if provenance linkage is useful.
4. Extract only structural findings.
5. Write a dated observation from [TEMPLATE.md](TEMPLATE.md).
6. Update the main X dossier if direct observation changes a baseline claim.
7. Preserve disagreements rather than rewriting history.

## Recorded observations

- [2026-09-18 profile/timeline capture A](2026-09-18-profile-a.md) — first direct Edge/X observation; confirms current profile/detail operations, identity fields, reply/quote and `note_tweet` structures.
- [2026-09-18 profile pagination to terminal history](2026-09-18-profile-pagination.md) — confirms Bottom-cursor pagination, 93 unique Posts in this young account, explicit `TimelineTerminateTimeline(direction=Bottom)`, and short-interval query-ID stability.
- [2026-09-18 History → Likes](2026-09-18-likes.md) — confirms current `/i/history/likes` UI location, GraphQL `Likes`, Bottom-cursor pagination, and 80 unique liked Posts across four pages.
- [2026-09-18 History → Bookmarks](2026-09-18-bookmarks.md) — confirms current `/i/history` Bookmarks view, GraphQL `Bookmarks`, Bottom-cursor pagination, 60 unique bookmarked Posts across three pages, and viewer-scoped request shape without `userId`.
- [2026-09-18 Followers](2026-09-18-followers.md) — confirms GraphQL `Followers`, `TimelineUser` relationship payloads, and explicit Top/Bottom termination coexisting with cursor objects.
- [2026-09-18 Following](2026-09-18-following.md) — confirms GraphQL `Following`, multi-page Bottom-cursor relationship pagination, 200 unique users across four pages, and the observed `count=20` versus 50 returned users-per-page mismatch.
- [2026-09-18 Lists](2026-09-18-lists.md) — confirms `ListsManagementPageTimeline`, viewer-scoped management modules, stable list IDs/state, bounded termination, and HTTP 200 partial GraphQL errors alongside usable data.

## Naming

Use:

`YYYY-MM-DD-<topic>.md`

Examples:

- `2026-09-18-own-profile-timeline.md`
- `2026-09-18-bookmarks.md`
- `2026-09-18-followers-following.md`

## Evidence priority

Within the X dossier:

1. direct current first-party observation;
2. current official X documentation;
3. multiple current public technical sources;
4. single public reverse-engineering source;
5. historical assumptions.

A direct observation can falsify a public reverse-engineering claim for the observed client/build without proving every X client behaves identically.
