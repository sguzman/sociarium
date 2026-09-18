# Quora surface dossier

- Surface: Quora
- Operator/company: Quora, Inc.
- Canonical domain: `quora.com`
- Research started: 2026-09-18
- Last reviewed: 2026-09-18
- Operator priority: unranked relative to the non-X inventory
- Overall access tier: **provisional Tier C — Adversarial**
- Confidence: high for official archive/Your Content behavior; medium for private-protocol structure from current public technical evidence; lower for current automation/Terms details because Quora's canonical Terms page was not directly retrievable by the research crawler

## Executive summary

Quora gives the user meaningful **human-facing access** to their own content and provides an official data archive, but this baseline did not identify a current general-purpose supported developer API for ordinary Quora user content or self-data.

Official user-facing mechanisms include:

- **Your Content**, which shows the user's questions, answers, and posts;
- followed-question views;
- Bookmarks;
- follows/topics/people;
- private Messages;
- a privacy/data-copy request that returns an archive of the user's content and personal data.

The official archive is user-initiated rather than a live synchronization interface. Quora says the archive is typically delivered within 72 hours after its team confirms the request.

Quora's current public API/business material discovered in this baseline is concentrated on advertising, measurement, and conversion integrations rather than a broad "read my own Quora account" developer surface.

At the same time, the first-party Quora web application clearly relies on a substantial undocumented GraphQL protocol. A current public reverse-engineering project documents:

- `POST /graphql/gql_para_POST?q=<QueryName>`;
- profile question queries;
- profile answer queries;
- answer body queries;
- activity/edit-history queries;
- stable-looking numeric `uid`, `qid`, and `aid` identifiers;
- Relay-style persisted-query hashes;
- cursor pagination;
- rich-text payloads;
- session-bound Quora headers and Cloudflare Turnstile state.

This makes Quora a strong **protocol-forensics** target even if no private-protocol adapter is ever admitted.

## Why provisional Tier C

Positive factors:

- official data-copy mechanism exists;
- normal first-party UI exposes much of the user's authored public content;
- user-facing Bookmarks/follows/messages exist;
- private protocol appears to expose deep structured history with stable numeric identifiers.

Adversarial factors:

- no general supported self-data/content API was identified in the current research pass;
- archive acquisition is manual/asynchronous;
- the public `Your Content` view is not complete for every data class;
- anonymous historical contributions are intentionally not tied back to the account;
- privately followed questions are explicitly omitted from `Your Content`;
- the private protocol is undocumented, build-sensitive, and tied to live browser/session/anti-abuse state;
- Quora's own object model treats some authored questions as **community property**, so account ownership and object ownership do not align cleanly.

This is exactly the kind of surface where Sociarium's documentation can be more valuable than immediate code.

## Important ontology finding: questions are not fully "owned" by the asker

Quora's current Question and Answer Policies say questions are viewed as **community property**.

Current Help also says that once enough activity has occurred, the original poster can no longer delete a question.

Account deletion removes content/profile associated with the account except for community property such as questions, topics, and Spaces.

That means Sociarium must not model:

`asked by user` == `fully user-owned object`

For Quora, the relationship is closer to:

`user authored/introduced question` + `question becomes community-governed object`

That distinction should survive normalization.

## Anonymous historical content

Quora stopped supporting anonymous question asking in July 2021 and anonymous answering in November 2021.

Quora currently states that it does not retain identity information connecting anonymous questions/answers to the originating account.

Therefore an account-centric acquisition source cannot be assumed capable of reconstructing all historical anonymous content authored by that person.

This is not merely an export limitation. It is an identity/provenance limitation in the platform's own data model.

## Acquisition candidates

### Official data archive

Useful for:

- account bootstrap;
- privacy/data recovery;
- cross-checking IDs and historical content;
- discovering categories not exposed in `Your Content`.

Weaknesses:

- manual request;
- asynchronous preparation;
- current archive schema not documented in the material reviewed;
- repeated requests are unsuitable as a live synchronization primitive.

### Public/profile web

Useful for public authored content and object URLs.

Weaknesses:

- not guaranteed complete;
- infinite-scroll/history depth may be UI-limited;
- private account state is absent.

### First-party private GraphQL protocol

Technically rich enough to warrant serious forensics.

Public current evidence shows operations for questions, answers, answer bodies, and activity/edit history.

Weaknesses:

- undocumented;
- persisted query hashes/revisions can change;
- authenticated browser/session state is required;
- current examples include Cloudflare Turnstile and other anti-abuse/session headers;
- direct Sociarium observation has not yet confirmed current behavior.

Status: **research target, not implementation-admitted.**

## Implementation recommendation

**Do not implement Quora yet.**

Next Quora work should be:

1. inspect a real current account archive;
2. map exact file formats and data classes;
3. directly observe `Your Content`, Bookmarks, follows, Messages, and profile history in the operator's own browser;
4. validate current GraphQL operation families and pagination;
5. compare private-protocol IDs against archive/public URLs;
6. manually re-check Quora's current Terms/automation restrictions before any implementation-admission decision.

## Files

- [Access matrix](access-matrix.md)
- [Official access and API status](official-access.md)
- [Archive and account data](archive.md)
- [Private first-party protocol](private-protocol.md)
- [Terms and constraints](terms-and-constraints.md)
- [Direct observation plan](observation-plan.md)
- [Source ledger](sources.md)
