# X observation: History → Bookmarks

- Observation date/time UTC: 2026-09-18, approximately 18:59:27-18:59:41
- Browser/version: Microsoft Edge 153 / Chromium 153
- X account context: self-owned authenticated account
- First-party UI route: `https://x.com/i/history` with the Bookmarks tab active
- Local private capture filename supplied for analysis: `x.com.bookmarks.har`
- Local private capture SHA-256: `90125e73ac6c27bc0c37e1b75ef7a6ab0824ed74d52725ea50a7028ff437679f`
- Raw capture committed publicly: **no**
- Interaction type: read-only
- Related R2 issue: #12

## Operator action sequence

1. opened X History with Bookmarks selected;
2. opened/cleared DevTools Network while scoped to that view;
3. loaded the Bookmarks timeline;
4. scrolled enough to trigger two additional bookmark pages;
5. exported the browser's sanitized HAR privately.

## Direct observations

### Current UI location

The observed Bookmarks view loaded at:

`https://x.com/i/history`

In this web build, Bookmarks is the default/base History view, while Likes is separately routed under `/i/history/likes`.

This is a dated UI observation, not a promise that the route will remain stable.

### Bookmarks request family

The Bookmarks data used the standard X GraphQL path family:

`https://x.com/i/api/graphql/{queryId}/Bookmarks`

Observed details:

- method: `GET`;
- operation: `Bookmarks`;
- observed query ID: `-dgKZ58Dr9YSJYrcgEb5KA`;
- status: 200 on all three observed requests;
- query parameters: `variables` and `features`;
- no `fieldToggles` query parameter was present in these three Bookmarks requests;
- requested count: 20.

Observed variables on the initial request:

- `count`;
- `includePromotedContent`.

Pagination requests added:

- `cursor`.

Notably, **no `userId` variable was present**.

That absence is directly observed. The interpretation that Bookmarks is implicitly scoped to the authenticated viewer is an inference, albeit a strong one given that the operation returned the operator's private bookmarks.

The query ID is a dated observation, not a durable constant.

### Response envelope

The Bookmarks response root differed from the observed Likes response.

Bookmarks used:

`data.bookmark_timeline_v2.timeline.instructions`

Each of the three observed responses contained one `TimelineAddEntries` instruction.

Each page contained:

- 20 `TimelineTimelineItem` entries carrying primary Post results;
- one Top `TimelineTimelineCursor`;
- one Bottom `TimelineTimelineCursor`.

No terminal instruction was reached in this bounded capture.

### Pagination behavior

Three Bookmarks requests were observed:

1. initial request with no cursor;
2. second request with `variables.cursor`;
3. third request with `variables.cursor`.

For each follow-up request, the request cursor exactly matched the previous response's Bottom cursor.

The same `Bookmarks` operation/query ID was reused across all three pages.

Observed primary Post counts:

- page 1: 20;
- page 2: 20;
- page 3: 20.

Across the three pages:

- 60 primary Post entries were observed;
- all 60 primary Post IDs were unique;
- no cross-page duplicate primary Post IDs were observed.

No attempt was made to exhaust bookmark history, so terminal behavior and total reachable depth remain unknown.

### Semantic confirmation

All 60 primary Post results in this Bookmarks timeline had:

`legacy.bookmarked = true`

This directly confirms that the captured operation is serving the operator's bookmarked-Post collection.

No bookmark text, Post text, Post IDs, author IDs, or raw private content are reproduced in this public note.

### Contrast with Likes

The directly observed Likes request included a `userId` variable and returned data through:

`data.user.result.timeline.timeline.instructions`

The directly observed Bookmarks request did **not** include `userId` and returned data through:

`data.bookmark_timeline_v2.timeline.instructions`

This is a meaningful protocol distinction:

- Likes is represented as a user-associated timeline operation;
- Bookmarks is represented as a dedicated viewer/private collection timeline.

The second sentence is an architectural interpretation of the directly observed request/response shapes, not a claim about X's internal implementation.

### Auth/session-related names

The Bookmarks GraphQL requests directly contained these header names:

- `x-client-transaction-id`;
- `x-csrf-token`;
- `x-twitter-active-user`;
- `x-twitter-auth-type`;
- `x-twitter-client-language`.

The observed `x-twitter-auth-type` header named an OAuth2 session mode. Its value is non-secret protocol metadata, but no authority-bearing values are reproduced here.

As in prior Edge 153 captures:

- ordinary `Cookie` request headers were omitted from the sanitized HAR;
- ordinary `Authorization` request headers were omitted;
- non-empty `x-csrf-token` values were retained.

Therefore the browser's sanitized HAR remains private evidence and is not safe to publish as-is.

## Confirmed/refined baseline claims

This capture directly confirms that, for the observed current X web client:

- Bookmarks live in the current History UI;
- the default Bookmarks History document route is `/i/history`;
- bookmarked Posts are fetched through GraphQL operation `Bookmarks`;
- the operation uses the ordinary `/i/api/graphql/{queryId}/{operationName}` family;
- Bookmarks pagination uses opaque Bottom cursors passed through `variables.cursor`;
- the same operation/query ID is reused across successive pages;
- Bookmarks return ordinary Post objects with stable Post identities;
- returned primary Posts are explicitly marked `legacy.bookmarked = true`;
- the Bookmarks operation has a distinct response root from Likes and does not directly take a `userId` variable in the observed requests.

## Unknowns

Still unresolved:

- terminal Bookmarks cursor behavior;
- total reachable bookmark-history depth;
- bookmark-folder protocol and folder-specific timelines;
- behavior for deleted/unavailable bookmarked Posts;
- query-ID volatility across a future X build;
- Followers;
- Following;
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
- [x] no bookmark contents;
- [x] no private notification/message bodies;
- [x] no full HAR;
- [x] no operator or third-party object IDs.

The only raw-capture linkage published here is the SHA-256 digest.
