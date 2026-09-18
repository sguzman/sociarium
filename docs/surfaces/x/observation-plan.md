# X direct first-party observation plan

Status: active under issue #12; profile initial load, Post detail, Bottom-cursor pagination through terminal history, short-interval query-ID reload comparison, History → Likes, History → Bookmarks, Followers, Following, and Lists management are directly observed. Notifications, bookmark folders, list sub-operations, and repost representation remain outstanding.

Purpose: convert public reverse-engineering claims into **dated Sociarium observations** of the first-party X web client used by the operator's own authorized account.

## Principle

Observe first. Do not automate first.

The initial goal is to understand the protocol shipped to the browser, not to replay it.

## Evidence handling

Raw browser captures can contain:

- session cookies;
- CSRF values;
- Authorization headers;
- private messages;
- private bookmarks;
- account metadata;
- personally sensitive request/response bodies.

Therefore:

- keep raw HAR/network captures local/private;
- never commit them to the public Sociarium repository;
- never paste live cookies, auth tokens, CSRF tokens, authorization codes, or session headers into GitHub issues/docs;
- create sanitized public observations from the raw capture;
- preserve local hashes/timestamps if later provenance linking is useful.

## Observation session metadata

Record:

- UTC timestamp;
- browser and version;
- X web build identifiers if visible;
- account context: self-owned authenticated account, without recording credentials;
- page/action being observed;
- whether the action was read-only or mutating;
- any extensions/proxies that could alter network behavior.

## Priority read-only actions

Observe one action at a time:

1. open own profile;
2. scroll own Posts;
3. open a Post detail/thread;
4. open replies;
5. open Likes;
6. open Bookmarks;
7. open a bookmark folder if present;
8. open Followers;
9. open Following;
10. open Lists;
11. open Mentions/Notifications;
12. open Direct Messages only if/when we are prepared to handle private evidence safely.

## For each operation

Record sanitized structure:

- method;
- host/path pattern;
- operation name;
- query-ID shape, but do not assume durability;
- query/body parameter names;
- feature-flag names;
- pagination cursor locations;
- stable profile/Post IDs;
- response envelope shape;
- next-page behavior;
- error shape;
- whether request requires authenticated session state;
- any client-transaction/proof header names;
- whether the operation changes after a web reload/build change.

## Historical-depth probes

For timeline-like sources, determine:

- page size;
- cursor direction;
- whether repeated pagination reaches beyond the visible profile UI window;
- terminal cursor behavior;
- deleted/unavailable object handling;
- edited Post history behavior if encountered;
- whether older history eventually requires archive/export.

Do not generate excessive traffic just to find a theoretical maximum. Research should be bounded and respectful.

## Capture procedure

Use [capture-protocol.md](capture-protocol.md) for the operator workflow and [capture-sanitization.md](capture-sanitization.md) for the public/private evidence boundary.

Use [observations/TEMPLATE.md](observations/TEMPLATE.md) for each sanitized finding.

## Deliverables

Direct observations are stored under:

`docs/surfaces/x/observations/YYYY-MM-DD-<topic>.md`

Direct observations now include [2026-09-18-profile-a.md](observations/2026-09-18-profile-a.md), [2026-09-18-profile-pagination.md](observations/2026-09-18-profile-pagination.md), [2026-09-18-likes.md](observations/2026-09-18-likes.md), [2026-09-18-bookmarks.md](observations/2026-09-18-bookmarks.md), [2026-09-18-followers.md](observations/2026-09-18-followers.md), [2026-09-18-following.md](observations/2026-09-18-following.md), and [2026-09-18-lists.md](observations/2026-09-18-lists.md).

Each observation should clearly say:

- what was directly seen;
- what remains inferred;
- what changed from earlier observations;
- which private local capture, if any, backs the sanitized note.

## Implementation gate

Completing this observation plan still does not automatically authorize a private-protocol adapter.

It supplies evidence for the later implementation-admission decision.
