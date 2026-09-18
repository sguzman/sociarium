# Quora official access and API status

Research date: 2026-09-18.

## Finding

This baseline did **not** identify a current general-purpose official Quora developer API for ordinary user content/self-data such as:

- the authenticated user's questions;
- answers;
- posts;
- comments;
- follows;
- bookmarks;
- messages.

This is a negative research finding, not a proof of nonexistence.

The correct statement is:

> **No general self-data/content API was identified in the official material discovered during this baseline.**

Do not silently upgrade that sentence into "Quora has no API."

## Official API material found

Current Quora for Business material exposes APIs/integrations centered on:

- advertising;
- conversion measurement;
- campaign/reporting partners;
- mobile attribution;
- brand-safety verification.

Examples include Quora's **Conversion API** and API partner ecosystem.

Those are real official APIs, but they do not answer Sociarium's core self-data question.

## Your Content

Quora's official Help says `Your Content` exposes:

- all questions the user has asked or followed;
- answers;
- posts.

Explicit exclusions:

- anonymously authored content;
- privately followed questions.

This is an important machine-access gap:

```text
Quora first-party account UI
    exposes meaningful self-data
        ↓
no general supported self-data developer interface identified
```

## Follows and Bookmarks

Official Help confirms first-party account features for:

- following people;
- following topics;
- followed questions;
- Bookmarks.

These are useful research targets because they are meaningful account state that a user can access interactively even though no current general developer API for them was identified.

## Private Messages

Official Help documents a dedicated Messages feature.

No general supported personal-messaging API was identified in this baseline.

## Public content

Most non-anonymous authored Quora content is intended to be publicly readable, except private message threads and other privacy-limited account state.

Public readability is not equivalent to a complete self-data API:

- public pages may be paginated/limited;
- account-private relationships are omitted;
- anonymous historical contributions are intentionally unlinked;
- edit/history metadata may require internal calls;
- public slugs are not necessarily stable object identity.

## Next official-access research

Before implementation, manually inspect current Quora product/developer surfaces for:

- any newly introduced developer console;
- OAuth/app registration;
- content API documentation;
- export automation;
- account-specific API endpoints.

Because Quora's product and Poe ecosystem evolve, absence must be periodically rechecked.
