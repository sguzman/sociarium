# X observation: Followers

- Observation date/time UTC: 2026-09-18, approximately 20:00:36
- Browser/version: Microsoft Edge 153 / Chromium 153
- X account context: self-owned authenticated account
- First-party UI route: `https://x.com/<operator>/followers`
- Local private capture filename supplied for analysis: `x.com.followers.har`
- Local private capture SHA-256: `2ab54e951e566afc9381feead5aa2e56685b8ff4dab24acb535ee3faf1c9463d`
- Raw capture committed publicly: **no**
- Interaction type: read-only
- Related R2 issue: #12

## Operator action sequence

1. opened the operator's own Followers view;
2. opened/cleared DevTools Network while scoped to that view;
3. loaded the Followers view;
4. the small current follower set fit in the initial response, so no manual pagination was available;
5. exported the browser's sanitized HAR privately.

## Direct observations

### Followers request family

The follower data used the standard X GraphQL family:

`https://x.com/i/api/graphql/{queryId}/Followers`

Observed details:

- method: `GET`;
- operation: `Followers`;
- observed query ID: `fVGYs5W9kNUuoUrZwYZQpQ`;
- status: 200;
- query parameters included `variables` and `features`;
- requested count: 20.

Observed variables:

- `userId`;
- `count`;
- `includePromotedContent`;
- `withGrokTranslatedBio`.

No pagination request was generated because the current result set terminated in the initial response.

The query ID is a dated observation, not a durable constant.

### Response envelope

The response used:

`data.user.result.timeline.timeline.instructions`

The instruction sequence directly included:

- `TimelineClearCache`;
- `TimelineTerminateTimeline(direction=Top)`;
- `TimelineTerminateTimeline(direction=Bottom)`;
- `TimelineAddEntries`.

The AddEntries instruction contained:

- four `TimelineTimelineItem` user entries;
- one Bottom `TimelineTimelineCursor`;
- one Top `TimelineTimelineCursor`.

### Terminal behavior

This capture is important because **cursor objects and terminal instructions coexisted in the same response**.

The response contained Top and Bottom cursor entries, while separately declaring termination in both directions.

Therefore a consumer must not infer "more pages exist" merely because a cursor object is present. The terminal instruction is authoritative evidence that this bounded timeline has no additional page in the stated direction.

No follow-up `Followers` request was observed.

### User identity and relationship shape

Each primary entry was a `TimelineUser` carrying:

`user_results.result`

Observed stable identity fields included:

- opaque GraphQL-style `id`;
- decimal-string `rest_id`.

Observed user/profile structure included:

- `core.name`;
- `core.screen_name`;
- `privacy.protected`;
- `relationship_counts.followers`;
- `relationship_counts.following`;
- `relationship_perspectives`;
- profile/avatar/bio metadata;
- tweet/media counts;
- verification-related fields.

The directly observed relationship-perspective fields included:

- `followed_by`;
- `following`;
- `blocked_by`;
- `blocking`;
- `muting`;
- `live_following`.

All four primary user entries had `relationship_perspectives.followed_by = true`, semantically confirming that the returned users are followers of the operator.

At least one returned follower was also marked `following = true`, demonstrating that the same response expresses mutual-follow state without requiring a separate relation object.

No follower identities or account IDs are reproduced in this public note.

### Auth/session-related names

The Followers GraphQL request directly contained these header names:

- `x-client-transaction-id`;
- `x-csrf-token`;
- `x-twitter-active-user`;
- `x-twitter-auth-type`;
- `x-twitter-client-language`.

Values are intentionally omitted.

As in prior Edge 153 captures, the sanitized HAR omitted ordinary Cookie/Authorization request headers but retained a non-empty `x-csrf-token` value. The HAR therefore remains private evidence and is not safe to publish as-is.

## Confirmed/refined baseline claims

This capture directly confirms that, for the observed current X web client:

- the own-profile Followers view uses GraphQL operation `Followers`;
- it is scoped by a `userId` variable;
- user entries expose stable decimal `rest_id` values plus richer user/profile metadata;
- relationship direction is represented directly in `relationship_perspectives`;
- the response can explicitly terminate both Top and Bottom directions;
- terminal instructions may coexist with cursor objects.

The last point is an important parser constraint: cursor presence and pagination availability are not equivalent.

## Unknowns

Still unresolved:

- multi-page Followers pagination behavior on an account with more than one requested page;
- query-ID volatility across a future X build;
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
- [x] no follower identities or account IDs;
- [x] no private notification/message bodies;
- [x] no full HAR.

The only raw-capture linkage published here is the SHA-256 digest.
