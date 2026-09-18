# X observation: Notifications — Mentions

- Observation date/time UTC: 2026-09-18, approximately 22:23:47-22:23:52
- Browser/version: Microsoft Edge 153 / Chromium 153
- X account context: self-owned authenticated account
- First-party UI route: `https://x.com/notifications/mentions`
- Local private capture filename supplied for analysis: `x.com.mentions.har`
- Local private capture SHA-256: `5bc136afa436c09901f1878de2e0a8a1430f0f194f57ad28b8e22b961930a2e6`
- Raw capture committed publicly: **no**
- Interaction type: read-only page observation
- Related R2 issue: #12

## Operator action sequence

1. opened the Mentions subview under Notifications;
2. opened/cleared DevTools Network while scoped to that route;
3. reloaded the Mentions page;
4. allowed the small available Mentions history to exhaust naturally;
5. exported the browser's sanitized HAR privately.

The operator believed there were no current mentions, but the first Mentions response contained four Post items addressed/replied to the operator. No content text or identities are reproduced in this public note.

## Direct observations

### Mentions reuses NotificationsTimeline

The Mentions subview did **not** use a separate GraphQL operation.

It used:

`https://x.com/i/api/graphql/{queryId}/NotificationsTimeline`

with the same observed query ID as the earlier main Notifications capture:

`MSUPE4KwuxyghDO60Bv5uQ`

The stream selector changed through the request variable:

`timeline_type = Mentions`

The main Notifications stream had previously used:

`timeline_type = All`

This is direct evidence that the current web client treats All and Mentions as variants of one `NotificationsTimeline` protocol family rather than separate operation families.

### Initial route behavior

Loading `/notifications/mentions` caused the client to issue both:

- one `NotificationsTimeline` request with `timeline_type=All`;
- one `NotificationsTimeline` request with `timeline_type=Mentions`.

The Mentions request is the relevant target for this observation. The simultaneous All request appears to be page-shell/prefetch/state behavior and should not be mistaken for Mentions content.

### Mentions request shape

Observed Mentions requests:

- method: `GET`;
- operation: `NotificationsTimeline`;
- status: 200;
- query parameters: `variables` and `features`;
- no `fieldToggles` parameter;
- requested `count`: 20;
- no `userId` variable.

Initial Mentions variables:

- `timeline_type = Mentions`;
- `count = 20`.

The follow-up request added:

- `cursor`.

### Response envelope

The Mentions response used the same viewer-scoped envelope as the All stream:

`data.viewer_v2.user_results.result.notification_timeline.timeline.instructions`

Observed instruction types on the initial Mentions page:

- `TimelineClearCache`;
- `TimelineAddEntries`;
- `TimelineClearEntriesUnreadState`;
- `TimelineMarkEntriesUnreadGreaterThanSortIndex`.

The follow-up terminal page contained:

- `TimelineAddEntries`;
- `TimelineClearEntriesUnreadState`;
- `TimelineMarkEntriesUnreadGreaterThanSortIndex`.

### Content representation

The first Mentions response contained:

- four primary `TimelineTweet` items;
- one Top cursor;
- one Bottom cursor.

No `TimelineNotification` aggregate objects were present in this bounded Mentions sample.

All four Posts directly mentioned the operator account in their legacy user-mention entities, and each was a reply whose `in_reply_to_user_id_str` pointed to the operator.

This confirms that the captured `timeline_type=Mentions` stream was semantically serving Posts that mention/reply to the operator, not merely a generic notification subset.

No Post text, author identity, Post ID, or mention target ID is reproduced in this public note.

### Pagination and terminal behavior

Two Mentions requests were observed:

1. initial request without cursor;
2. follow-up request carrying the initial response's Bottom cursor.

The second request cursor exactly matched the first response's Bottom cursor.

Observed content counts:

- page 1: 4 `TimelineTweet` items;
- page 2: 0 content items.

The terminal second response contained:

- one Top cursor;
- **no Bottom cursor**;
- no explicit `TimelineTerminateTimeline(direction=Bottom)`.

This matches the terminal pattern directly observed in the All Notifications capture: exhaustion can be represented by disappearance of the Bottom cursor rather than an explicit terminal instruction.

### Contrast with Notifications — All

The All and Mentions streams share:

- operation name `NotificationsTimeline`;
- the same dated query ID;
- viewer-scoped response envelope;
- count/cursor pagination variables;
- unread-state instructions;
- Bottom-cursor exhaustion semantics.

They differ in stream selector and bounded content shape:

- All used `timeline_type=All` and mixed `TimelineNotification` with `TimelineTweet`;
- Mentions used `timeline_type=Mentions` and, in this sample, returned only `TimelineTweet` items.

The bounded Mentions sample is not sufficient to claim that `TimelineNotification` can never appear there.

### Auth/session-related names

The Mentions GraphQL requests directly contained these header names:

- `x-client-transaction-id`;
- `x-csrf-token`;
- `x-twitter-active-user`;
- `x-twitter-auth-type`;
- `x-twitter-client-language`.

Values are intentionally omitted.

As in prior Edge 153 captures, the sanitized HAR omitted ordinary Cookie/Authorization request headers but retained non-empty `x-csrf-token` values. The HAR therefore remains private evidence and is not safe to publish as-is.

## Confirmed/refined baseline claims

This capture directly confirms that, for the observed current X web client:

- the Mentions subview route is `/notifications/mentions`;
- Mentions reuses `NotificationsTimeline` rather than introducing a separate operation;
- `timeline_type` selects between at least `All` and `Mentions`;
- the same dated query ID was used for both selectors;
- Mentions is viewer-scoped in shape and has no observed `userId` variable;
- Mention content can be delivered as ordinary `TimelineTweet` Posts;
- Posts in the bounded sample directly referenced the operator in mention entities and reply linkage;
- Bottom-cursor pagination and cursor-disappearance exhaustion match the All stream;
- unread-state instructions are shared across both selectors.

This observation is sufficient to close the R2 combined **Mentions/Notifications operation observed** baseline gate.

## Still unresolved

- Verified-notifications selector/subview behavior;
- push/live notification transport;
- long-term notification/mention history depth on an older or high-volume account;
- query-ID volatility across a future X deployment;
- repost-wrapper representation;
- complete Cookie/Authorization boundary hidden by browser HAR sanitization;
- rate-limit behavior.

## Public-safety review

- [x] no cookies;
- [x] no Authorization values;
- [x] no CSRF/token values;
- [x] no session IDs;
- [x] no raw cursor values;
- [x] no mention/Post text;
- [x] no author, Post, or target identities/IDs;
- [x] no private message bodies;
- [x] no full HAR.

The only raw-capture linkage published here is the SHA-256 digest.
