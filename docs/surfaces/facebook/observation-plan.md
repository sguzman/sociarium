# Facebook direct observation plan

Status: planned.

Goal: map how the current first-party Facebook web client retrieves the operator's own information without turning observation into automated acquisition.

## Evidence handling

Facebook captures are highly sensitive.

Raw HAR/network captures may contain:

- login/session cookies;
- CSRF tokens;
- access tokens;
- profile/security data;
- friends/connections;
- private group activity;
- Messenger content;
- search/view history.

Keep raw captures private/local.

Never commit credential/token values.

## Priority read-only observations

Observe one user-facing data class at a time:

1. own profile/timeline;
2. own Posts;
3. Activity Log;
4. own Comments;
5. Likes/Reactions history;
6. Saved;
7. Photos;
8. Videos/Reels;
9. Friends/Following;
10. Groups/group activity;
11. Notifications;
12. Search/activity history;
13. Accounts Center / Access Your Information;
14. Messenger only in a separate high-sensitivity session.

## For each request family

Record sanitized:

- host/path;
- method;
- Relay friendly operation name;
- `doc_id` as dated volatile evidence;
- variables field names;
- pagination cursor fields;
- object/profile IDs;
- response envelope structure;
- auth/CSRF **field names only**;
- error shapes;
- build/client context.

## Cross-source identity test

For a few self-owned objects, compare:

- public URL/profile representation;
- Graph API object ID if accessible;
- private GraphQL object ID;
- export-package ID.

This will show whether Sociarium can unify evidence from different Facebook acquisition sources without relying on mutable URLs/text.

## Export comparison

When a real Download Your Information archive is available, compare its records against the same objects observed in the live first-party UI.

Questions:

- Are IDs preserved?
- Are edits represented?
- Are reactions/comments first-class objects?
- Are deleted objects present?
- Is privacy/audience state preserved?
- Are timestamps normalized consistently?

## Implementation gate

Direct observation is evidence collection only.

No replay/private-protocol adapter follows automatically.
