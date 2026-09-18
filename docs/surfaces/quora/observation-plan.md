# Quora direct observation plan

Status: planned, not yet executed.

Goal: turn current public reverse-engineering evidence into dated first-party observations from the operator's own authorized Quora session.

## Observe before automating

The initial objective is to map the shipped client protocol, not replay it.

## Evidence handling

Raw Quora HAR/network captures may contain:

- login/session cookies;
- CSRF/formkey values;
- Turnstile tokens;
- private follow state;
- Bookmarks;
- private Messages;
- account settings;
- personally identifying metadata.

Keep raw captures local/private.

Never commit live authority values.

Public notes may preserve:

- host/path;
- request method;
- query name;
- persisted-query hash as dated build evidence;
- variable names;
- response field names;
- pagination structure;
- native IDs;
- auth/header **names only**;
- Quora revision/build identifiers;
- error shape.

## Priority observations

### Authored content

1. own profile;
2. `Your Content`;
3. own Questions;
4. own Answers;
5. own Posts;
6. Comments/activity/edit history.

### Private account state

7. Bookmarks;
8. Questions Followed;
9. privately followed questions if the UI exposes them separately;
10. followed People;
11. followed Topics;
12. Spaces membership/admin state;
13. Messages only in a separate high-sensitivity session.

## Questions for each operation

- Which GraphQL query name is used?
- Is the endpoint still `/graphql/gql_para_POST`?
- What native IDs appear?
- Are `uid`, `qid`, `aid` consistent with public URLs/archive data?
- What is the effective page size?
- What cursor shape is used?
- Does the first-party UI stop before the protocol does?
- What is the terminal-history behavior?
- Are deleted objects represented?
- Does edit history expose prior answer/comment bodies?
- Which headers are necessary vs incidental?
- How often do persisted-query hashes/revisions rotate?

## Bounded historical probing

Do not hammer the service to discover theoretical maxima.

Use modest, intentionally bounded pagination to answer:

- whether the private operation is deeper than the visible UI;
- whether old content is structurally available;
- whether cursor semantics remain stable;
- whether server-side hard windows appear.

## Archive comparison

When a current official archive is available, compare a small sample of objects across:

- archive;
- public URL;
- private GraphQL response.

Verify:

- numeric IDs;
- timestamps;
- rich-text fidelity;
- edit history;
- deletion state;
- relationship to profile identity.

## Deliverables

Sanitized notes under:

`docs/surfaces/quora/observations/YYYY-MM-DD-<topic>.md`

Direct observation outranks current public reverse-engineering claims when they conflict.

## Implementation gate

Completing this plan does not automatically authorize a private-protocol adapter.

It supplies evidence for a later implementation-admission decision.
