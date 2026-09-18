# Bluesky private and application-specific state

Research date: 2026-09-18.

The public signed repository is excellent, but it is not the only state associated with a Bluesky account.

## Preferences

Actor preferences are available through authenticated application RPCs.

Official migration documentation treats preferences separately from the repository and demonstrates export/import handling.

Examples include application settings that should not necessarily be globally public records.

## Bookmarks

Bluesky supports bookmarks as private application state.

Current APIs include bookmark operations such as creating/deleting/getting bookmarks, and rendered post views can expose bookmark-related state in authorized contexts.

This state should **not** be assumed to be part of the public signed repository merely because Likes and Follows are.

Questions for later direct/API validation:

- What service is authoritative for bookmark state?
- Is there a complete export endpoint?
- Is pagination unbounded/practically complete?
- Does bookmark state migrate across PDS/provider changes automatically?
- Can another AppView implement compatible bookmark storage?

## Direct Messages / Chat

Bluesky chat uses the `chat.bsky.*` service family.

Chat is not part of the public account repository. Official account-migration material notes that some data may live in external services such as centralized chat.

Current Lexicons expose conversation APIs such as listing conversations and message operations.

This is still much more inspectable than a wholly undocumented proprietary DM protocol, but its portability properties are weaker than those of public repo records.

A future private-state dossier pass should verify:

- complete chat export;
- retention/history limits;
- stable conversation/message identifiers;
- whether chat can be moved between service providers;
- whether end-to-end encryption exists/changes the model;
- deletion semantics.

## Permission model

The OAuth/permission system is moving toward granular permissions.

Private data classes should receive their own permission analysis rather than assuming that access to the public repository implies access to bookmarks/preferences/chat.

## Tier implication

The main public social layer is clear Tier A.

Private/application state is better described per class:

- preferences: Tier A/B;
- bookmarks: likely Tier A/B, portability details still incomplete;
- chat: Tier B-ish relative to the signed-repo model because it remains external/centralized service state.

These caveats do not overturn the overall Tier A classification because the platform's foundational user-owned social data and identity architecture is exceptionally sovereignty-friendly.
