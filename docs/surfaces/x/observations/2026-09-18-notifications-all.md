# X observation: Notifications — All

- Observation date/time UTC: 2026-09-18, approximately 20:52:50-20:52:55
- Browser/version: Microsoft Edge 153 / Chromium 153
- X account context: self-owned authenticated account
- First-party UI route: `https://x.com/notifications`
- Local private capture filename supplied for analysis: `x.com.notifications.har`
- Local private capture SHA-256: `89d29808b992aabdc0463fca3bfe5cb6f2bf4d4f2366a37382e08675daf2d6e1`
- Raw capture committed publicly: **no**
- Interaction type: read-only page observation
- Related R2 issue: #12

## Operator action sequence

1. opened the main X Notifications page;
2. opened/cleared DevTools Network while scoped to that page;
3. reloaded the main Notifications view;
4. scrolled until the small current notification history was exhausted;
5. exported the browser's sanitized HAR privately.

The operator did not switch to the Mentions subview during this capture.

## Direct observations

### Notifications request family

The main Notifications page used the standard X GraphQL family:

`https://x.com/i/api/graphql/{queryId}/NotificationsTimeline`

Observed details:

- method: `GET`;
- operation: `NotificationsTimeline`;
- observed query ID: `MSUPE4KwuxyghDO60Bv5uQ`;
- status: 200 on all three observed requests;
- query parameters: `variables` and `features`;
- no `fieldToggles` query parameter was present;
- requested `count`: 20;
- observed `timeline_type`: `All`.

The initial variables were:

- `timeline_type`;
- `count`.

Pagination requests added:

- `cursor`.

The query ID is a dated observation, not a durable constant.

### Response envelope

The notification timeline response used:

`data.viewer_v2.user_results.result.notification_timeline.timeline.instructions`

The initial response also contained:

`timeline.responseObjects.feedbackActions`

Later bounded pages did not carry that response-object section.

### Notification entry types

The first content-bearing page contained 20 primary timeline items:

- 16 `TimelineNotification` items;
- 4 `TimelineTweet` items.

The second page contained:

- 2 `TimelineNotification` items;
- 0 `TimelineTweet` items.

The terminal third response contained no notification/tweet content entries.

Therefore the Notifications timeline is heterogeneous. A consumer must not assume every notification entry has one common object shape.

#### `TimelineNotification`

Observed structural fields included:

- `id`;
- `notification_icon`;
- `notification_url`;
- `rich_message`;
- `template`;
- `timestamp_ms`.

Observed notification templates were `TimelineNotificationAggregateUserActions`.

Their template structure included:

- `from_users`;
- `target_objects`.

Observed target objects included `TimelineNotificationTweetRef`.

Observed icon categories in this bounded private sample included heart, person, recommendation, and Community Notes/Birdwatch-style icon classes. No notification text, actor identities, target IDs, or URLs are reproduced in this public note.

#### `TimelineTweet`

Some notification entries were represented directly as `TimelineTweet` objects carrying ordinary `tweet_results` Post payloads.

This means notification normalization must preserve the distinction between:

- aggregate/action notification events;
- direct Post timeline items surfaced in Notifications.

### Pagination behavior

Three `NotificationsTimeline` requests were observed:

1. initial request with no cursor;
2. second request with the first response's Bottom cursor;
3. third request with the second response's Bottom cursor.

Both follow-up request cursors exactly matched the prior response's Bottom cursor.

The same `NotificationsTimeline` operation/query ID was reused across all three requests.

Observed content-item counts:

- page 1: 20;
- page 2: 2;
- page 3: 0.

### Terminal behavior by cursor disappearance

The first two responses included both Top and Bottom cursor objects.

The third response contained:

- a Top cursor;
- **no Bottom cursor**;
- zero notification/tweet content entries.

Unlike the profile and Followers captures, this terminal page did **not** carry an explicit `TimelineTerminateTimeline(direction=Bottom)` instruction.

Therefore R2 has now directly observed at least two terminal-pagination patterns:

1. explicit `TimelineTerminateTimeline`;
2. exhaustion where the next/Bottom cursor disappears.

A robust client must support both.

### Unread-state instructions

Every observed Notifications response carried:

- `TimelineClearEntriesUnreadState`;
- `TimelineMarkEntriesUnreadGreaterThanSortIndex`.

The initial response additionally carried `TimelineClearCache`.

These are response-side timeline instructions observed during a read-only page load. No notification-marking mutation request was observed in this capture.

Sociarium should preserve these instructions as protocol/state evidence rather than assuming the notification stream is only a list of content objects.

### Viewer scoping

The request variables did not include `userId`.

The response was rooted under `data.viewer_v2...`.

This strongly indicates authenticated-viewer scoping for Notifications, analogous in principle to Bookmarks and Lists management. That interpretation is an inference from the directly observed request/response shape; the complete Cookie/Authorization boundary remains hidden by sanitized HAR export.

### Incidental supporting operation

The page also issued a GraphQL `ViewerBadgeCounts` query during initial load.

This capture does not establish that `ViewerBadgeCounts` is unique to Notifications, but its presence is consistent with the page's unread/badge UI responsibilities.

### Auth/session-related names

The Notifications GraphQL requests directly contained the familiar X header names:

- `x-client-transaction-id`;
- `x-csrf-token`;
- `x-twitter-active-user`;
- `x-twitter-auth-type`;
- `x-twitter-client-language`.

Values are intentionally omitted.

As in prior Edge 153 captures, the sanitized HAR omitted ordinary Cookie/Authorization request headers but retained non-empty `x-csrf-token` values. The HAR therefore remains private evidence and is not safe to publish as-is.

## Confirmed/refined baseline claims

This capture directly confirms that, for the observed current X web client:

- the main Notifications page uses GraphQL `NotificationsTimeline`;
- `timeline_type=All` selects the main Notifications stream;
- the operation is viewer-scoped in shape and has no observed `userId`;
- the notification stream mixes `TimelineNotification` and `TimelineTweet` content;
- aggregate notification objects carry actors, target references, rich-message presentation, icon class, URL, and timestamp structure;
- Bottom-cursor pagination works through `variables.cursor`;
- terminal exhaustion may be expressed by disappearance of the Bottom cursor rather than a `TimelineTerminateTimeline` instruction;
- unread-state handling appears as explicit timeline instructions;
- no read-state mutation request was observed in this bounded load.

## What this capture does not establish

Still unresolved:

- the Mentions subview and its `timeline_type`/operation behavior;
- Verified-notifications subview behavior;
- notification push/live transport;
- long-term notification history depth on an older/high-volume account;
- query-ID volatility across a future X deployment;
- repost-wrapper representation;
- complete Cookie/Authorization boundary hidden by browser HAR sanitization;
- rate-limit behavior.

Because Mentions was not opened, the combined **Mentions/Notifications** R2 checklist item should remain open until a separate Mentions observation is made.

## Public-safety review

- [x] no cookies;
- [x] no Authorization values;
- [x] no CSRF/token values;
- [x] no session IDs;
- [x] no raw cursor values;
- [x] no notification text;
- [x] no notification actor/target identities or IDs;
- [x] no private message bodies;
- [x] no full HAR.

The only raw-capture linkage published here is the SHA-256 digest.
