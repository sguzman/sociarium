# X observation: Following

- Observation date/time UTC: 2026-09-18, approximately 20:07:31
- Browser/version: Microsoft Edge 153 / Chromium 153
- X account context: self-owned authenticated account
- First-party UI route: `https://x.com/<operator>/following`
- Local private capture filename supplied for analysis: `x.com.following.har`
- Local private capture SHA-256: `259176a05fb90b9dcd3d3e670a5887550393f93046c3d957809e98deabc9fe95`
- Raw capture committed publicly: **no**
- Interaction type: read-only
- Related R2 issue: #12

## Operator action sequence

1. opened the operator's own Following view;
2. opened/cleared DevTools Network while scoped to that view;
3. loaded the Following timeline;
4. scrolled enough to trigger three additional relationship pages;
5. exported the browser's sanitized HAR privately.

## Direct observations

### Following request family

The Following data used the standard X GraphQL family:

`https://x.com/i/api/graphql/{queryId}/Following`

Observed details:

- method: `GET`;
- operation: `Following`;
- observed query ID: `-Mn4uN7C-vxXBwUKtSwS6A`;
- status: 200 on all four observed requests;
- query parameters included `variables` and `features`;
- no `fieldToggles` query parameter was present on these requests;
- requested `count`: 20.

Observed initial variables:

- `userId`;
- `count`;
- `includePromotedContent`;
- `withGrokTranslatedBio`.

Pagination requests added:

- `cursor`.

The query ID is a dated observation, not a durable constant.

### Response envelope

The response used:

`data.user.result.timeline.timeline.instructions`

The initial page contained:

- `TimelineClearCache`;
- `TimelineTerminateTimeline(direction=Top)`;
- `TimelineAddEntries`.

Later pages contained `TimelineAddEntries`.

Each observed page included Top and Bottom cursor entries.

No Bottom terminal instruction was reached in this bounded capture.

### Pagination behavior

Four Following requests were observed:

1. initial request with no cursor;
2. second request with `variables.cursor`;
3. third request with `variables.cursor`;
4. fourth request with `variables.cursor`.

For every follow-up request, the request cursor exactly matched the previous response's Bottom cursor.

The same `Following` operation/query ID was reused across all four pages.

### Requested count versus returned entries

A notable protocol behavior was directly observed:

- request variable `count` was 20;
- each of the four responses contained **50 primary `TimelineUser` entries**.

Across the four pages:

- 200 primary user entries were observed;
- all 200 stable `rest_id` values were unique;
- no cross-page duplicate primary user IDs were observed.

Therefore `count=20` is not a hard response-size contract for this operation in the observed build. Clients should treat the actual returned entries and cursor/terminal instructions as authoritative.

No attempt was made to exhaust the full Following set, so total reachable depth and Bottom terminal behavior remain unknown.

### Relationship semantics

All 200 primary user entries had:

`relationship_perspectives.following = true`

This semantically confirms that the operation is returning accounts followed by the operator.

Returned user objects exposed the same general relationship/profile structure observed in Followers, including:

- stable decimal `rest_id`;
- opaque GraphQL-style `id`;
- `core.name`;
- `core.screen_name`;
- `privacy.protected`;
- `relationship_counts`;
- `relationship_perspectives`;
- profile/avatar/bio metadata;
- tweet/media counts;
- verification-related fields.

Relationship perspectives included fields such as:

- `following`;
- `followed_by`;
- `blocking`;
- `blocked_by`;
- `muting`;
- `live_following`.

No followed-account identities or account IDs are reproduced in this public note.

### Contrast with Followers

The directly observed `Followers` and `Following` operations share the same broad user-timeline envelope and both accept a `userId`.

The bounded Followers capture terminated both directions in its first response because the result set was small.

The Following capture instead demonstrated multi-page Bottom-cursor pagination. Its initial response terminated only the Top direction, while Bottom cursors continued to produce additional pages.

Together these captures establish that relationship timelines use terminal instructions and cursor objects independently:

- a cursor object may be present even when that direction is terminated;
- an unterminated Bottom direction can be advanced by sending the previous Bottom cursor in `variables.cursor`.

### Auth/session-related names

The Following GraphQL requests directly contained these header names:

- `x-client-transaction-id`;
- `x-csrf-token`;
- `x-twitter-active-user`;
- `x-twitter-auth-type`;
- `x-twitter-client-language`.

Values are intentionally omitted.

As in prior Edge 153 captures, the sanitized HAR omitted ordinary Cookie/Authorization request headers but retained a non-empty `x-csrf-token` value. The HAR therefore remains private evidence and is not safe to publish as-is.

## Confirmed/refined baseline claims

This capture directly confirms that, for the observed current X web client:

- the own-profile Following view uses GraphQL operation `Following`;
- it is scoped by a `userId` variable;
- multi-page relationship pagination passes opaque Bottom cursors through `variables.cursor`;
- the same operation/query ID is reused across successive pages;
- returned entries are `TimelineUser` objects with stable identities and explicit relationship perspectives;
- returned accounts are explicitly marked `relationship_perspectives.following = true`;
- requested `count` is not necessarily the actual number of user entries returned.

## Unknowns

Still unresolved:

- Bottom terminal behavior and total reachable Following depth;
- query-ID volatility across a future X build;
- Lists;
- Mentions/Notifications;
- repost-wrapper representation;
- complete Cookie/Authorization boundary hidden by browser HAR sanitization;
- error/rate-limit behavior.

## Public-safety review

- [x] no cookies;
- [x] no Authorization values;
- [x] no CSRF/token values;
- [x] no session IDs;
- [x] no raw cursor values;
- [x] no followed-account identities or account IDs;
- [x] no private notification/message bodies;
- [x] no full HAR.

The only raw-capture linkage published here is the SHA-256 digest.
