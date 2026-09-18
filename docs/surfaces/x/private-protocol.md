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

Public technical evidence describes authenticated web requests as using browser-session state, commonly including:

- a logged-in session cookie;
- a CSRF cookie/value mirrored in an `x-csrf-token` header;
- a web-client bearer/app identity;
- X-specific client/auth headers;
- increasingly, an `x-client-transaction-id` or equivalent client-generated request identifier.

Do **not** put live cookie values, auth tokens, CSRF values, or captured Authorization headers into this public repository.

The transaction-ID and CSRF-related header names are now directly observed, but the complete authentication requirement remains unresolved because the sanitized HAR removed ordinary Cookie/Authorization material.

## What is directly unknown

After the first direct capture, Sociarium still has not directly established:

- which X JavaScript bundles contain operation metadata;
- query-ID volatility across reloads/builds;
- the complete feature/field-toggle behavior across self-data operation families;
- whether all self-data reads require authenticated cookies;
- mature-account history depth and whether any older-history ceiling appears before terminal pagination;
- history depth for `UserTweets`, `Likes`, or `Bookmarks`;
- bookmark-folder behavior;
- DM protocol families;
- live notification transport;
- failure/rate-limit behavior;
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
- long-form content through `note_tweet.note_tweet_results.result.text`;
- timeline Top/Bottom cursor entries.

The direct capture refines one public-research assumption: the observed profile timeline used `UserOriginalsTimeline`, not `UserTweets`. This does not prove `UserTweets` is absent elsewhere or in other builds.

The first capture did not observe own-profile pagination. A second profile-focused capture later on 2026-09-18 directly observed repeated `UserOriginalsTimeline` Bottom-cursor pagination through `variables.cursor`, 93 unique Posts across five content-bearing pages for the young account, and a sixth zero-Post response carrying `TimelineTerminateTimeline(direction=Bottom)`. The same `UserOriginalsTimeline` query ID remained unchanged across the two separate captures roughly half an hour apart. A repost wrapper and the Likes/Bookmarks/Followers/Following/Lists/Notifications operation families remain unobserved.

Because the browser's sanitized HAR omitted ordinary Cookie/Authorization headers, the exact complete authentication boundary is still not established. Importantly, the sanitized HAR did retain non-empty `x-csrf-token` values, so raw/sanitized HAR files remain private evidence.

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
