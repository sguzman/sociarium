# X / Twitter depth-research priority roadmap

Created: 2026-09-19.

Status: active X research roadmap after completion of the R2 baseline.

The R2/X baseline established the core first-party protocol map. It did **not** exhaust X as a research target.

This roadmap orders the remaining X work by:

1. research leverage;
2. operator effort;
3. evidence safety;
4. likelihood of changing the Sociarium data model or acquisition strategy;
5. dependency on earlier findings.

The order is intentional. Do not skip ahead merely because a later topic is more exotic.

## Deferred — Bookmark folders (paid feature)

Status: **blocked by current zero-spend policy.** The operator does not have X Premium and does not want to pay for it now. X currently places Bookmark Folders in its paid Premium feature set.

Do not ask the operator to purchase Premium for this research. Preserve this item for a future date if the operator independently gains access or explicitly authorizes spending.

Original rationale: smallest extension of an already-mapped private self-data family, with strong expected value and low capture complexity.

Establish:

- current bookmark-folder UI route(s);
- operation name(s), including whether `BookmarkFolderTimeline` is still current;
- stable folder IDs;
- folder metadata;
- Post membership representation;
- pagination and terminal behavior;
- whether moving/adding bookmarks uses distinct read/write operations;
- whether folder reads are viewer-scoped in the same way as ordinary Bookmarks.

Deliverable:

- one sanitized bookmark-folder observation;
- updates to X access matrix/private-protocol notes if the current shape differs from public technical evidence.

## Priority 1 — Lists beyond management

The baseline only mapped `ListsManagementPageTimeline`.

Next map:

- list detail;
- list timeline;
- list members;
- list followers/subscribers;
- ownership;
- memberships;
- pinned-list behavior if exposed;
- stable list/user/Post IDs across those views.

**Why second:** Lists already exposed useful partial-success/error behavior and stable list IDs. Deeper mapping gives high schema leverage without touching high-sensitivity account data.

## Priority 2 — Blocks and mutes

Direct captures already showed `blocking`, `muting`, and relationship-perspective fields incidentally, but not the actual self-data collection operations.

Establish:

- blocked-account collection;
- muted-account collection;
- viewer scoping;
- stable user IDs;
- pagination;
- terminal behavior;
- whether mute/block state is represented solely in relationship perspectives or also in collection-specific payloads.

**Why third:** these are first-class private relationship datasets and likely fit the existing user/relationship ontology cleanly.

## Priority 3 — Search

Directly observe current X Web search.

Map:

- `SearchTimeline` or its current replacement;
- Top/Latest/People/Media selectors;
- query variables;
- cursor semantics;
- Post/user result mixtures;
- result modules;
- visibility wrappers;
- authenticated vs public behavior.

**Why fourth:** search is broad and highly useful, but it is not necessary to understand account-owned state. It comes after the missing self-data collections.

## Priority 4 — Likes and Bookmarks terminal-depth probes

The baseline proved both are pageable but did not reach terminal history.

Establish, with bounded respectful probing:

- whether a terminal page is reachable;
- terminal instruction vs cursor-disappearance behavior;
- practical historical depth on the operator's account;
- duplicate behavior across long pagination runs;
- whether old/private/deleted objects create gaps.

Do **not** infer a universal service-wide ceiling from one account.

## Priority 5 — Deleted, edited, unavailable, and restricted objects

Build a representation taxonomy for objects that do not appear as ordinary Posts/users.

Capture opportunistically where possible:

- deleted Post;
- edited Post/current edit metadata;
- unavailable Post;
- suspended/deactivated user;
- protected/private content not visible to the viewer;
- withheld/restricted content;
- tombstone or visibility-wrapper forms.

**Why here:** the normal happy-path schema should be stable before mapping exception objects.

## Priority 6 — Public vs authenticated boundary matrix

The baseline documents the name-level auth/session boundary. The next question is which **reads actually require which authority class**.

Compare safe read-only cases:

- logged-out public profile/Post reads;
- logged-in public profile/Post reads;
- viewer-scoped private collections;
- relationship state;
- notifications;
- search.

Record only structural differences and header/cookie **names**, never values.

Goal: distinguish:

- public first-party reads;
- session-enhanced public reads;
- authenticated viewer reads;
- strictly private self-data reads.

## Priority 7 — Failure and error taxonomy

Expand beyond the directly observed Lists partial-success GraphQL errors.

Research safely and without traffic flooding:

- unavailable/deleted-object errors;
- expired/stale cursor behavior;
- stale query/document ID behavior when naturally encountered or safely reproducible;
- unauthenticated/expired-session failure shape;
- GraphQL partial success;
- HTTP error status families;
- rate-limit headers/errors if encountered naturally.

**Do not intentionally hammer X to manufacture a 429.**

## Priority 8 — Query-ID discovery, JavaScript bundles, features, and field toggles

Once the important operation families are mapped, study how the shipped client discovers/describes them.

Establish:

- which current JS bundles contain operation/query metadata;
- operation-name → query-ID relationship;
- whether IDs can be rediscovered mechanically from shipped assets;
- which feature flags are operation requirements vs ordinary client capability flags;
- which field toggles materially alter response shape;
- whether query IDs rotate while operation semantics remain stable.

This is protocol-mechanics research, not implementation admission.

## Priority 9 — Official archive forensics

Inspect a current real X archive when available.

Map:

- archive file manifest;
- JSON/JS/HTML container forms;
- stable IDs;
- authored Posts;
- replies/quotes/reposts;
- likes/bookmarks;
- relationships;
- Lists;
- Direct Messages;
- media references/files;
- deleted/edit history where present;
- differences from live protocol representation.

Compare archive authority against live first-party observations.

### Parallel note

Because archive preparation is asynchronous, the operator may request a one-time archive earlier in the roadmap. That does **not** make repeated archive requests the synchronization strategy. Analysis belongs here when the package is available.

## Priority 10 — Live notification transport

The read-side `NotificationsTimeline` is mapped. The actual real-time transport is not.

Establish:

- whether X Web currently uses WebSocket, SSE, push, polling, or a hybrid;
- connection/bootstrap endpoints;
- event envelope shapes;
- relationship to `NotificationsTimeline`;
- reconnect/resume behavior;
- whether live events contain canonical objects or only invalidation/update signals.

**Why late:** this is mechanically deeper and less important than completing ordinary read coverage.

## Priority 11 — Direct Messages

**Highest sensitivity; deliberately last.**

Only begin when private-evidence handling is explicitly prepared.

Map separately:

- conversation list;
- message/event IDs;
- participants;
- pagination/history;
- attachments/media;
- edits/deletions if supported;
- conversation metadata;
- live update behavior;
- archive representation.

Raw DM payloads must remain private. Public documentation should contain schemas/field names only unless content is deliberately fabricated/redacted.

## Ongoing overlay — protocol drift watch

After Priority 8, protocol drift becomes a longitudinal maintenance activity rather than a blocking research task.

On later X web builds, compare:

- operation names;
- query IDs;
- feature sets;
- field toggles;
- envelope paths;
- auth/proof headers;
- cursor behavior.

Do not rewrite older observations to match newer builds. Add dated observations.

## Practical execution order

The operator-facing sequence is:

1. Lists deep operations
2. Blocks/mutes
3. Search
4. Likes/Bookmarks terminal depth
5. Deleted/edited/unavailable semantics
6. Public/authenticated boundary matrix
7. Failure/error taxonomy
8. Query-ID/bundle/feature mechanics
9. Archive forensics
10. Live notification transport
11. Direct Messages

Deferred paid-only work:

- Bookmark folders — revisit only if the operator independently has Premium or explicitly authorizes spending.

This order remains subordinate to new evidence. A discovery may promote a dependency, but ChatGPT should record and explain that change rather than silently reshuffling the roadmap.

## Implementation boundary

None of these research priorities automatically authorizes:

- a private-protocol adapter;
- replay automation;
- mutation/write automation;
- credential extraction tooling.

The X private protocol remains a research target. Any implementation still requires a separate I0 admission decision or explicit human-principal override.