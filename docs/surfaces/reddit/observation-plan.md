# Reddit direct first-party observation plan

Status: planned, not yet executed.

Goal: convert desk research into dated observations of Reddit's current first-party web client for the operator's own authenticated account.

## Observe before automating

The purpose is protocol documentation, not request replay.

Raw browser captures stay private/local.

## Safe evidence handling

A Reddit HAR/network capture can contain:

- session cookies;
- CSRF values;
- private Saved/Vote history;
- direct messages/Chat;
- account settings;
- IP/security information.

Never commit those captures or credentials to the public repository.

Sanitized public observations may record:

- host/path;
- request method;
- operation name;
- variable/field names;
- pagination cursor structure;
- response envelope shape;
- stable object IDs;
- content type;
- auth/CSRF header **names**;
- error form;
- dated client/build context.

## Priority read-only observations

1. own profile;
2. own Posts;
3. own Comments;
4. Saved;
5. Upvoted;
6. Downvoted;
7. Hidden;
8. joined communities;
9. followed users/friends if available;
10. Inbox/archived private messages only after private-data handling is prepared;
11. Chat only as a separate high-sensitivity research session.

## Questions

For every self-data view:

- Does the page use server-rendered HTML, `/svc/shreddit/graphql`, another `/svc/shreddit/*` endpoint, a legacy/oauth endpoint, or multiple sources?
- What stable ID/fullname appears?
- How does pagination work?
- How far back can the UI/protocol traverse?
- Is there a terminal cursor?
- Does reload change operation shape?
- Is the returned representation complete or summarized?
- How are deleted/removed items represented?
- Does Reddit expose data through the first-party client that Devvit does not?

## Historical-depth probing

Do not generate excessive traffic.

A bounded sample should determine whether self-data listings:

- expose conventional `after` cursors;
- use opaque Shreddit cursors;
- stop at a hard item/history count;
- omit old deleted objects;
- differ from the official export.

## Deliverables

Create sanitized dated notes under:

`docs/surfaces/reddit/observations/YYYY-MM-DD-<topic>.md`

Direct observation should supersede public secondary reverse-engineering claims when they conflict.

Completing this observation plan still does not authorize a private-protocol adapter.
