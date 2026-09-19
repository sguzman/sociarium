# X first-party private web protocol

Research date: 2026-09-18.

Current evidence class: **public technical evidence plus direct Sociarium first-party observation**.

## Why this matters

The X web application necessarily communicates with X backend services to render authenticated timelines, bookmarks, relationships, post detail, search, and other user-visible state.

Sociarium treats that shipped first-party protocol as a legitimate **research object** when the sanctioned developer API materially obstructs zero-cost self-data access.

This does not automatically admit a private-protocol implementation.

## Publicly documented reverse-engineering pattern

Multiple independent technical projects currently describe a GraphQL request family using URLs shaped like:

`https://x.com/i/api/graphql/{queryId}/{operationName}`

Reported read operation names include:

- `UserByScreenName`;
- `UserTweets`;
- `TweetDetail`;
- `HomeTimeline`;
- `HomeLatestTimeline`;
- `Following`;
- `Followers`;
- `Bookmarks`;
- `BookmarkFolderTimeline`;
- `Likes`;
- List-related timelines/memberships;
- `SearchTimeline`.

Public projects also report legacy/internal REST-like paths under `/i/api/1.1` for some functions.

## Query IDs and feature flags

Public technical sources describe operation URLs as combining a human-readable operation name with a query ID extracted from the current web client bundles.

Several projects explicitly warn that:

- query IDs change with X web releases;
- feature-flag sets are operation-specific;
- clients often rediscover operation/query mappings from X's JavaScript bundles rather than treating hardcoded IDs as durable.

This suggests that **operation semantics may be more durable than operation identifiers**.

That hypothesis requires direct Sociarium observation before being promoted beyond inference.

## Reported authenticated request shape

Public technical evidence describes authenticated web requests as using browser-session state. The current boundary is documented by evidence class in [auth-session-boundary.md](auth-session-boundary.md).

Direct Sociarium captures establish the X-specific request-header names `x-client-transaction-id`, `x-csrf-token`, `x-twitter-active-user`, `x-twitter-auth-type`, and `x-twitter-client-language`. Edge's sanitized HAR removes the ordinary `Cookie` and `Authorization` headers and request-cookie array, so those captures cannot directly expose cookie names.

Multiple current public technical implementations independently identify the authenticated browser cookie names as `auth_token` and `ct0`, with `ct0` mirrored into `x-csrf-token`, and describe an `Authorization` web-client bearer/app-identity header. Sociarium records those names as secondary corroboration rather than mislabeling them as direct observation.

Do **not** put live cookie values, auth tokens, CSRF values, or captured Authorization headers into this public repository.

## What is directly unknown

After the first direct capture, Sociarium still has not directly established:

- which X JavaScript bundles contain operation metadata;
- query-ID volatility across reloads/builds;
- the complete feature/field-toggle behavior across self-data operation families;
- whether all self-data reads require authenticated cookies;
- mature-account history depth and whether any older-history ceiling appears before terminal pagination;
- mature-account profile history depth and bounded terminal depth for `Likes` or `Bookmarks`;
- bookmark-folder behavior;
- DM protocol families;
- live/push notification transport;
- rate-limit behavior and broader failure taxonomy beyond the directly observed partial-success Lists GraphQL field errors;
- how often query IDs/features rotate in practice;
- whether the current web client uses additional anti-automation proof beyond the reported transaction-ID layer.

These remain research targets.

## Direct Sociarium observation — 2026-09-18

The first private browser capture has now been analyzed. See [observations/2026-09-18-profile-a.md](observations/2026-09-18-profile-a.md).

Directly observed in Microsoft Edge 153:

- the GraphQL path family `/i/api/graphql/{queryId}/{operationName}`;
- own-profile operations `UserOriginalsTimeline` and `UserByScreenName`;
- Post-detail operation `TweetDetail`;
- incidental `HomeTimeline` operation with cursor-bearing follow-up requests;
- `x-client-transaction-id`, `x-csrf-token`, `x-twitter-active-user`, `x-twitter-auth-type`, and `x-twitter-client-language` header names;
- stable decimal `rest_id` values alongside distinct opaque GraphQL `id` values for users;
- Post `rest_id` matching `legacy.id_str`;
- quote embedding through `quoted_status_result.result`;
- reply linkage through legacy `in_reply_to_*` fields;
- repost embedding through `legacy.retweeted_status_result.result`, with distinct outer repost and inner original Post identities;
- long-form content through `note_tweet.note_tweet_results.result.text`;
- timeline Top/Bottom cursor entries.

The direct capture refines one public-research assumption: the observed profile timeline used `UserOriginalsTimeline`, not `UserTweets`. This does not prove `UserTweets` is absent elsewhere or in other builds.

The first capture did not observe own-profile pagination. A second profile-focused capture later on 2026-09-18 directly observed repeated `UserOriginalsTimeline` Bottom-cursor pagination through `variables.cursor`, 93 unique Posts across five content-bearing pages for the young account, and a sixth zero-Post response carrying `TimelineTerminateTimeline(direction=Bottom)`. The same `UserOriginalsTimeline` query ID remained unchanged across the two separate captures roughly half an hour apart. A later Home timeline capture directly observed three repost wrappers using `legacy.retweeted_status_result.result`, including visibility-wrapped outer and inner Posts. The main Notifications and Mentions streams are both directly observed through viewer-scoped `NotificationsTimeline`, selected by `timeline_type=All` versus `timeline_type=Mentions`, with shared unread-state instructions and cursor-disappearance exhaustion semantics. Likes and Bookmarks are directly observed under the current History UI. Followers and Following are directly observed as `TimelineUser` relationship operations scoped by `userId`. Lists management is now directly observed as viewer-scoped `ListsManagementPageTimeline`, including module separation and partial-success GraphQL errors.

Because the browser's sanitized HAR omitted ordinary Cookie/Authorization headers, direct first-party evidence alone cannot name the session cookies. The boundary is nevertheless documented without values by combining direct header-name observation with current independent technical corroboration for `auth_token`, `ct0`, and the `Authorization` header. See [auth-session-boundary.md](auth-session-boundary.md). Importantly, the sanitized HAR retained non-empty `x-csrf-token` values, so raw/sanitized HAR files remain private evidence.

### Repost representation

A later 2026-09-18 direct `HomeTimeline` capture contained three primary timeline Posts with structural repost wrappers.

After normalizing any outer `TweetWithVisibilityResults`, the reposting Post's `legacy` object embedded the original Post at `retweeted_status_result.result`. The outer repost and embedded original carried distinct stable Post IDs and distinct author IDs. One sample showed that both the outer repost and embedded original may themselves be wrapped in `TweetWithVisibilityResults`, so a parser must normalize that wrapper at both layers.

All three outer repost Posts had viewer-state `legacy.retweeted = false` despite being structurally unambiguous reposts. Sociarium therefore treats `retweeted_status_result` as the relationship marker and `retweeted` as viewer-relative state, not as a Post-type discriminator. The familiar `RT @...` display text was also present but is redundant evidence and should not be parsed to recover the relation.

See [observations/2026-09-18-reposts.md](observations/2026-09-18-reposts.md).

### History → Likes

A later 2026-09-18 direct capture established that the current web UI exposes Likes under `/i/history/likes`.

The liked-Post data itself still uses the GraphQL operation `Likes`, not a distinct History-specific data family. Four observed pages reused one dated query ID and paginated by passing the previous response's opaque Bottom cursor through `variables.cursor`. The capture contained 80 unique primary Posts, all marked `legacy.favorited = true`.

See [observations/2026-09-18-likes.md](observations/2026-09-18-likes.md).

### History → Bookmarks

A later 2026-09-18 direct capture established that the current web UI exposes Bookmarks as the base History view at `/i/history`.

The bookmarked-Post data uses GraphQL operation `Bookmarks`. Three observed pages reused one dated query ID and paginated by passing the previous response's opaque Bottom cursor through `variables.cursor`. The capture contained 60 unique primary Posts, all marked `legacy.bookmarked = true`.

Unlike the directly observed Likes operation, Bookmarks did **not** include a `userId` request variable and returned through `data.bookmark_timeline_v2.timeline.instructions` rather than `data.user.result.timeline.timeline.instructions`. This strongly suggests authenticated-viewer scoping for the private bookmark collection, while the exact complete auth mechanism remains unresolved because the sanitized HAR hides ordinary Cookie/Authorization material.

See [observations/2026-09-18-bookmarks.md](observations/2026-09-18-bookmarks.md).

### Followers

A 2026-09-18 direct capture established the GraphQL operation `Followers`, scoped by `userId`, returning `TimelineUser` entries through the familiar user timeline envelope.

The small current follower set fit in one response. That response carried both `TimelineTerminateTimeline(direction=Top)` and `TimelineTerminateTimeline(direction=Bottom)` while also carrying Top/Bottom cursor objects. Sociarium therefore treats terminal instructions, not cursor presence alone, as authoritative pagination state.

Returned user objects exposed stable `rest_id` values and `relationship_perspectives` including `followed_by`, `following`, blocking, and muting state.

See [observations/2026-09-18-followers.md](observations/2026-09-18-followers.md).

### Following

A 2026-09-18 direct capture established the GraphQL operation `Following`, also scoped by `userId`, with multi-page Bottom-cursor pagination through `variables.cursor`.

Four observed pages reused one dated query ID and returned 200 unique `TimelineUser` entries total. Every primary user entry had `relationship_perspectives.following = true`.

A notable wire behavior is that the request sent `count=20` while each response carried 50 primary users. Sociarium therefore does not treat the requested count as a hard page-size contract for this operation.

See [observations/2026-09-18-following.md](observations/2026-09-18-following.md).

### Lists management

A 2026-09-18 direct capture established the viewer-scoped GraphQL operation `ListsManagementPageTimeline`.

The request used `count=100` with no observed `userId`. The response root was `data.viewer.list_management_timeline.timeline.instructions`, with separate **Discover new Lists** and **Your Lists** modules carrying `TimelineTwitterList` items. List objects exposed stable decimal `id_str` values alongside state such as `following`, `is_member`, `muting`, and `pinning`.

The bounded one-list result terminated both Top and Bottom while still carrying a Bottom cursor object, again confirming that terminal instructions outrank cursor presence.

This response also provided the first direct partial-success GraphQL error case: HTTP 200 returned usable list data while top-level `errors[]` reported field-level decode failures for optional banner-media data. Sociarium must preserve successful `data` and record partial errors rather than treating any `errors[]` as total failure.

See [observations/2026-09-18-lists.md](observations/2026-09-18-lists.md).

### Notifications — All

A 2026-09-18 direct capture established viewer-scoped GraphQL `NotificationsTimeline` for the main `/notifications` page with `timeline_type=All`.

Three requests were observed. The first response carried 20 content items, the second 2, and the third 0. The first two pages advanced by passing the previous Bottom cursor through `variables.cursor`. The final empty page retained only a Top cursor and omitted the Bottom cursor, with no explicit `TimelineTerminateTimeline(direction=Bottom)`.

This gives Sociarium a second directly observed terminal pattern: exhaustion can be represented by disappearance of the next-direction cursor rather than a terminal instruction.

The stream is heterogeneous: `TimelineNotification` aggregate/action items coexist with ordinary `TimelineTweet` Post items. Responses also carried `TimelineClearEntriesUnreadState` and `TimelineMarkEntriesUnreadGreaterThanSortIndex` instructions. No separate read-state mutation was observed in the bounded page load.

See [observations/2026-09-18-notifications-all.md](observations/2026-09-18-notifications-all.md).

### Notifications — Mentions

A later 2026-09-18 direct capture established that `/notifications/mentions` reuses the same `NotificationsTimeline` operation and dated query ID, changing the request selector to `timeline_type=Mentions`.

The initial Mentions response contained four ordinary `TimelineTweet` items. All four directly referenced the operator through mention entities and reply linkage. A second request reused the previous Bottom cursor and returned zero content with no Bottom cursor, matching the All stream's cursor-disappearance terminal pattern.

The Mentions route also triggered a simultaneous `timeline_type=All` request, demonstrating that route-level traffic can include adjacent/preloaded notification streams and must be disambiguated by request variables rather than page URL alone.

See [observations/2026-09-18-notifications-mentions.md](observations/2026-09-18-notifications-mentions.md).

## Technical promise

The private web protocol appears technically rich enough to deserve serious forensics.

If direct observations confirm the public technical references, it may expose a broader and more immediate live self-data surface than the official free/export mechanisms.

But technical possibility is only one axis.

## Contractual/operational constraint

X's current Terms of Service prohibit automated access through interfaces other than X's published interfaces unless separately allowed and expressly prohibit scraping without prior written consent.

X's April 2026 Automation Rules also state that non-API automation such as scripting the X website may result in permanent suspension.

Therefore:

- **research/documentation:** active;
- **capturing and analyzing the operator's own browser traffic:** research target;
- **building/operating an automated private-protocol client:** not currently admitted.

Sociarium records the technical surface and the policy boundary separately instead of pretending either one does not exist.

## Public technical references

See [sources.md](sources.md).

Current useful references include:

- `mudrii/gobird` wire-protocol documentation;
- `nsozturk/tweetkit-x` current constants/query-ID commentary;
- `uakihir0/twitter-web-client` endpoint inventory;
- related public clients that rediscover query IDs from X's shipped JavaScript.

These are secondary sources. Direct first-party observation outranks them for claims about current X behavior.
