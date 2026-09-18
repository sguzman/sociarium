# Quora first-party private web protocol

Research date: 2026-09-18.

Evidence class: **current public technical reverse-engineering evidence**, not yet direct Sociarium observation.

## Endpoint family

A current public reverse-engineering project documents the Quora web frontend using:

`POST https://www.quora.com/graphql/gql_para_POST?q=<QueryName>`

for multiple profile/content operations.

Observed query families include:

- `UserProfileQuestionsList_Questions_Query`;
- `UserProfileAnswersMostRecent_RecentAnswers_Query`;
- `AnswerComponentBaseQuery`;
- `UserProfileEditsQuery`;
- comment-area queries.

## Persisted-query model

Requests include:

- a human-readable `queryName`;
- GraphQL variables;
- an `extensions.hash` persisted-query identifier.

These hashes should be treated as build/version artifacts, not stable API identifiers.

## Stable-looking native object IDs

The public technical evidence exposes numeric:

- `uid` for users;
- `qid` for questions;
- `aid` for answers;
- `opid` for activity/edit operations.

Question responses may also contain Relay-style encoded IDs.

For Sociarium, these numeric IDs are promising candidates for native stable identity, but direct observation must confirm them.

## Pagination

The public reverse-engineering source documents cursor/page-info structures such as:

- `pageInfo.hasNextPage`;
- `pageInfo.endCursor`;
- `after` cursors.

Its observed profile question/answer connections used small server-enforced page sizes even when larger `first` values were requested.

Do not promote those page sizes to a universal Quora contract.

## Rich-text format

Question titles and answer bodies are reported as JSON-encoded rich-text structures containing sections/spans/modifiers rather than plain strings.

This is important for lossless archival:

- links;
- formatting;
- images;
- quoted blocks;
- directionality;
- structure

may be lost if Sociarium keeps only flattened text.

## Activity/edit history

The same current public research documents `UserProfileEditsQuery`, with operation types for:

- adding comments;
- editing comments;
- editing answer content;
- creating/attaching answers.

That is particularly interesting for provenance because the public page object is not necessarily the only available historical evidence.

The external researcher observed a much smaller history window for this activity-log connection than for profile questions/answers on one tested account.

Treat that as a dated per-account observation, not a universal limit.

## Auth/session boundary

The public technical source reports request state including names such as:

- `Quora-Formkey` — CSRF-like token;
- `Quora-Window-Id`;
- `Quora-Revision`;
- `Quora-Broadcast-Id`;
- `Quora-Page-Creation-Time`;
- `Quora-Turnstile-Token`;
- cookies sent with the authenticated browser session.

The exact requirements remain direct-observation targets.

Never commit actual values for:

- cookies;
- formkeys;
- Turnstile tokens;
- session identifiers;
- broadcast tokens;
- private message bodies.

## Service worker behavior

The same secondary source reports Quora's service worker intercepting fetch requests and complicating page-level request inspection.

That is relevant operational evidence for forensics, but Sociarium should not adopt the source's interception/bypass technique as an implementation path.

Our immediate goal is understanding request structure, not defeating browser/security mechanisms.

## Current implementation status

**Research only.**

The private protocol appears technically capable of deep self-content retrieval, but:

- it is undocumented;
- session/build/persisted-query state can change;
- anti-abuse state is involved;
- current Quora Terms/automation rules require a fresh manual review before any implementation admission.

No adapter should be built from this dossier alone.
