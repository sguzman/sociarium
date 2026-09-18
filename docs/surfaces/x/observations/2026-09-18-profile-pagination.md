# X observation: own-profile pagination to terminal history

- Observation date/time UTC: 2026-09-18, approximately 17:19:11-17:19:23
- Browser/version: Microsoft Edge 153 / Chromium 153
- X account context: self-owned authenticated account
- Local private capture filename supplied for analysis: `x.com.2.har`
- Local private capture SHA-256: `9a7fab667791510b97ed7fabfc0a55801a17b4c706ad011162def4d1526df330`
- Raw capture committed publicly: **no**
- Interaction type: read-only
- Related R2 issue: #12

## Operator action sequence

1. navigated to the operator's own profile;
2. cleared/started the Network capture on that profile;
3. loaded the profile timeline;
4. continuously scrolled downward until the young account had no more Posts to load;
5. exported the browser's sanitized HAR privately.

## Direct observations

### Profile timeline request family

All profile timeline pages used:

- method: `GET`;
- host/path family: `x.com/i/api/graphql/{queryId}/UserOriginalsTimeline`;
- operation: `UserOriginalsTimeline`;
- same observed query ID throughout this capture: `Yr8749ieoUptxRqQv766Fw`;
- requested `count`: 20.

The initial request did not carry a cursor.

Every subsequent profile-page request reused the same operation/query ID and added a `variables.cursor` field containing the opaque Bottom cursor returned by the prior page.

No cursor values or account IDs are published here.

### Bounded pagination sequence

The capture contained six `UserOriginalsTimeline` requests.

Observed normalized Post-entry counts by request:

1. 20 Posts;
2. 19 Posts;
3. 19 Posts;
4. 20 Posts;
5. 15 Posts;
6. 0 Posts.

Across the first five responses, the 93 observed Post IDs were all unique.

The sixth response carried no Post entries and included:

- a Top cursor;
- a Bottom cursor;
- `TimelineTerminateTimeline` with `direction = Bottom`.

This is direct evidence of the end-of-history signal for this particular account at this point in time.

It must **not** be generalized into a universal 93-Post limit or a claim about older/larger X accounts.

### Response instructions and cursor structure

The initial timeline response directly contained:

- `TimelineClearCache`;
- `TimelinePinEntry`;
- `TimelineAddEntries`.

Later pages used:

- `TimelinePinEntry`;
- `TimelineAddEntries`.

The terminal page additionally used:

- `TimelineTerminateTimeline` with Bottom direction.

Each content-bearing response exposed opaque Top and Bottom `TimelineTimelineCursor` objects.

The client used the previous Bottom cursor as the next request's `variables.cursor`.

### Entry heterogeneity

Timeline payloads were not Post-only streams.

Across pages the response also contained modules/cursors in addition to `TimelineTweet` entries. The terminal response itself still contained onboarding/pinned module material plus cursors despite containing zero Posts.

A Sociarium parser must therefore consume typed timeline instructions/entries rather than assuming each entry is a Post.

### Authorship and operation semantics

All 93 directly observed Post results in this `UserOriginalsTimeline` sequence resolved to the operator's own stable remote user ID.

No repost-wrapper object was observed in this bounded corpus.

This is consistent with the operation name `UserOriginalsTimeline`, but it is not enough to establish the protocol's general repost representation.

### Relationship and long-form fields

Within this larger 93-Post sample, the capture again directly contained:

- reply linkage via legacy `in_reply_to_status_id_str`, `in_reply_to_user_id_str`, and `in_reply_to_screen_name`;
- quote linkage via `quoted_status_result` and legacy quote ID/permalink fields;
- long-form Note Tweet objects via `note_tweet.note_tweet_results`.

This independently reinforces the first profile capture's structural findings.

## Query-ID reload comparison

The same `UserOriginalsTimeline` query ID was observed in the earlier profile capture and again in this separate later profile capture roughly half an hour later.

Therefore:

- reload/separate-capture stability over this short interval: directly observed;
- long-term/build-to-build query-ID stability: still unknown.

Sociarium should continue treating query IDs as dated protocol observations rather than durable constants.

## Historical-depth interpretation

For this newly created account, bounded scrolling reached the server-provided Bottom termination condition after 93 unique observed Posts.

This is unusually useful because it verifies that:

- repeated Bottom-cursor pagination can exhaust the available `UserOriginalsTimeline` history for this account;
- X supplies an explicit terminal instruction rather than requiring the client to infer completion only from an empty page;
- the terminal response may still contain cursors/modules even though Post history is exhausted.

It does **not** establish:

- how deep a mature account can paginate;
- whether X imposes a hidden older-history ceiling;
- whether account age, product tier, deletion state, or other factors alter depth;
- whether another profile/timeline operation exposes a different history set.

## Confirmed/refined baseline claims

This capture directly confirms and strengthens:

- `UserOriginalsTimeline` is the current own-profile operation for the observed Edge/X build;
- pagination reuses the same operation/query ID and passes an opaque Bottom cursor through `variables.cursor`;
- Top/Bottom cursors are response objects, not Post entries;
- `TimelineTerminateTimeline(direction=Bottom)` is a directly observed terminal-history signal;
- ordinary response pages may contain fewer Posts than the requested count because the timeline includes typed non-Post structures;
- the observed query ID remained unchanged across two separate captures within the same session/build interval.

## Unknowns

Still unresolved:

- general mature-account history depth;
- long-term query-ID volatility across X web deployments;
- repost-wrapper representation;
- Likes;
- Bookmarks;
- Followers;
- Following;
- Lists;
- Mentions/Notifications;
- complete cookie/Authorization boundary hidden by browser HAR sanitization;
- error/rate-limit behavior.

## Public-safety review

- [x] no cookies;
- [x] no Authorization values;
- [x] no CSRF/token values;
- [x] no session IDs;
- [x] no raw cursor values;
- [x] no private message/bookmark/notification bodies;
- [x] no full HAR;
- [x] no operator or third-party object IDs.

The only raw-capture linkage published here is the SHA-256 digest.
