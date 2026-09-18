# X / Twitter surface dossier

- Surface: X / Twitter
- Operator/company: X Corp.
- Canonical domain: `x.com`
- Research started: 2026-09-18
- Last reviewed: 2026-09-18
- Operator priority: **highest**
- Overall access tier: **provisional Tier C — Adversarial**
- Confidence: high for official API/export facts; direct first-party evidence now covers profile, Post detail, profile pagination, reply/quote/repost relationships, Likes, Bookmarks, Followers, Following, Lists management, Notifications, and Mentions; remaining undocumented subviews are medium-confidence until observed

## Executive summary

X is currently Sociarium's highest-value surface and the first deep Surface Atlas target.

The current documented developer interface is technically capable but commercially hostile to the project's zero-spend self-data goal. X's official API uses prepaid pay-per-use credits. Reads are billed per returned resource. Qualifying **Owned Reads** for the app owner's own posts, bookmarks, followers, likes, lists, and related self-data are discounted to $0.001 per resource, but they are still paid reads.

X separately offers an official account archive through normal account settings. X documents the archive as machine-readable HTML/JSON and says it includes profile information, posts, Direct Messages, attached media, followers, following, Lists, and other account data. This is strong for historical bootstrap and preservation, but it is an asynchronous manual export rather than a continuous acquisition interface.

X Help currently says the ordinary profile timeline displays up to 3,200 of the user's most recent posts and directs users to the archive for older history. The current official `GET /2/users/{id}/tweets` API reference does **not** state that same 3,200 ceiling. Sociarium therefore treats the current API historical ceiling as **unknown pending direct/official evidence** rather than inheriting an older assumption.

Public technical reverse-engineering projects independently document a substantial first-party web protocol under `x.com/i/api/graphql/{queryId}/{operationName}`. Sociarium now has direct current observations for `UserOriginalsTimeline`, `UserByScreenName`, `TweetDetail`, `HomeTimeline`, `Likes`, `Bookmarks`, `Followers`, `Following`, and `ListsManagementPageTimeline`, including timeline pagination, terminal-history behavior, reply/quote/repost relationship payloads, viewer-scoped list management, and partial GraphQL error handling. Both the main Notifications stream and the Mentions subview are now directly observed through the same `NotificationsTimeline` operation with different `timeline_type` selectors.

X's current Terms of Service expressly prohibit scraping and automated access through interfaces other than X's currently available published interfaces unless separately permitted. X's April 2026 Automation Rules also say not to use non-API automation such as scripting the X website. That creates a major implementation constraint even where the private protocol is technically observable.

## Tier assessment

**Provisional Tier C — Adversarial.**

Reasons:

- useful official self-data reads are metered rather than generally free;
- the comprehensive official export is manual/asynchronous and is not a reasonable continuous synchronization primitive;
- the first-party web client appears to have broad live access through an undocumented protocol;
- X's current terms/policies explicitly constrain automated use of non-published interfaces;
- recurring manual export should not be normalized as the operator's permanent job.

This tier is about the **access relationship**, not the quality of X as a social network.

## Acquisition candidates

### Official X API

Technically clean and already partially implemented in Sociarium.

Strengths:

- documented;
- OAuth 2.0 PKCE support;
- stable numeric user/Post IDs;
- pagination and incremental parameters;
- broad endpoint coverage;
- explicit rate limits.

Weaknesses:

- prepaid pay-per-use;
- zero-spend self-data synchronization is not available under the documented standard pricing model;
- pricing is mutable;
- current history ceiling for the user-post endpoint remains unverified in this research pass.

Status: **preserved optional paid backend; not the active implementation target.**

### Official account archive

Strengths:

- first-party;
- machine-readable;
- broad historical/account coverage;
- suitable for recovery/bootstrap and provenance.

Weaknesses:

- manual request;
- asynchronous preparation;
- poor fit for continuous synchronization;
- exact per-file format and completeness by data class should be verified against a real current archive.

Status: **valuable bootstrap/recovery candidate; not an acceptable sole live-acquisition strategy.**

### First-party private web protocol

Strengths indicated by public technical evidence:

- live;
- broad operation set;
- same protocol family used by the shipped web client;
- likely exposes many data classes important to Sociarium.

Weaknesses:

- undocumented;
- operation/query identifiers and feature flags may drift with web builds;
- authenticated requests use live browser-session authority;
- current X terms/policies materially constrain non-published automated access.

Status: **research target, not implementation-admitted.**

### Current repost representation

Direct observation on 2026-09-18 found three reposts in a bounded `HomeTimeline` response. X represented each as a distinct outer Post for the repost event with the original embedded at `legacy.retweeted_status_result.result`.

The outer repost and original retained independent stable Post IDs and authors. `TweetWithVisibilityResults` may wrap either layer, and the viewer-state boolean `legacy.retweeted` was `false` on all three outer reposts, so it is not a repost-type discriminator. Sociarium should use the structural embedded result rather than parsing `RT @...` display text.

See [the repost observation](observations/2026-09-18-reposts.md).

### Current Likes UI

Direct observation on 2026-09-18 found that X Web currently exposes the operator's Likes under **History → Likes** at `/i/history/likes`, rather than as the older profile Likes tab. The underlying data request remains the GraphQL `Likes` operation.

See [the Likes observation](observations/2026-09-18-likes.md).

### Current Bookmarks UI

Direct observation on 2026-09-18 found that X Web exposes Bookmarks as the base **History** view at `/i/history`. The underlying data request is GraphQL `Bookmarks`.

Unlike Likes, the observed Bookmarks request did not include a `userId` variable and the response root was `data.bookmark_timeline_v2...`, consistent with a private viewer-scoped collection rather than an arbitrary user's public/profile-associated timeline.

See [the Bookmarks observation](observations/2026-09-18-bookmarks.md).

### Current Followers UI

Direct observation on 2026-09-18 found that the operator's profile Followers view uses GraphQL `Followers` with a `userId` variable. Returned entries are `TimelineUser` objects with stable `rest_id` identity and explicit relationship perspectives.

The bounded response terminated both Top and Bottom directions even though cursor objects were still present, so consumers must treat `TimelineTerminateTimeline` as authoritative rather than inferring pagination availability from cursor presence alone.

See [the Followers observation](observations/2026-09-18-followers.md).

### Current Following UI

Direct observation on 2026-09-18 found that the operator's profile Following view uses GraphQL `Following` with the same broad user-timeline shape and a `userId` variable.

The capture demonstrated real multi-page Bottom-cursor pagination. It also showed that `count=20` is not a strict page-size guarantee: each observed response contained 50 primary users.

See [the Following observation](observations/2026-09-18-following.md).

### Current Lists UI

Direct observation on 2026-09-18 found that the operator's Lists page uses viewer-scoped GraphQL `ListsManagementPageTimeline` with no observed `userId` variable.

The response separated **Discover new Lists** from **Your Lists** as distinct timeline modules and returned `TimelineTwitterList` objects with stable IDs and list-state fields. The same HTTP 200 response also carried recoverable field-level GraphQL errors for optional banner-media data, proving that non-empty `errors[]` can coexist with useful `data`.

See [the Lists observation](observations/2026-09-18-lists.md).

### Current Notifications UI

Direct observation on 2026-09-18 found that both the main `/notifications` page and `/notifications/mentions` use viewer-scoped GraphQL `NotificationsTimeline`.

The main stream selects `timeline_type=All` and mixes aggregate `TimelineNotification` items with ordinary `TimelineTweet` items. The Mentions subview selects `timeline_type=Mentions`; its bounded sample returned ordinary Posts that directly mentioned/replied to the operator. Both selectors carried unread-state instructions and exhausted by returning an empty page without a Bottom cursor.

See [the All Notifications observation](observations/2026-09-18-notifications-all.md) and [the Mentions observation](observations/2026-09-18-notifications-mentions.md).

## Current R2 status

Direct first-party observation is now active under issue #12. The reply/quote/repost relation gate is complete; auth/session-boundary completion and the final X tier/recommendation re-review remain before the R2 baseline gate can close.

See:

- [Browser capture protocol](capture-protocol.md)
- [Capture sanitization rules](capture-sanitization.md)
- [Observation template](observations/TEMPLATE.md)

## Implementation recommendation

**Do not resume X implementation yet.**

Next X work should be forensic:

1. preserve this documented/public-research baseline;
2. directly observe the current first-party web client from the operator's own authenticated browser;
3. map read-only operations relevant to self-data;
4. record pagination, history depth, response identity semantics, and protocol drift;
5. keep all cookies/tokens/session material outside the public repository;
6. only then make a deliberate implementation-admission decision.

The existing official-API and archive-import code remains preserved substrate and implementation evidence.

## Files

- [Access matrix](access-matrix.md)
- [Official API](official-api.md)
- [Official archive/export](archive.md)
- [Private first-party protocol](private-protocol.md)
- [Terms and operational constraints](terms-and-constraints.md)
- [Direct observation plan](observation-plan.md)
- [Browser capture protocol](capture-protocol.md)
- [Capture sanitization rules](capture-sanitization.md)
- [Observation directory](observations/README.md)
- [Source ledger](sources.md)
