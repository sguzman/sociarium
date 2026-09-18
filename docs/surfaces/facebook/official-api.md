# Facebook official Graph API

Research date: 2026-09-18.

## Scope of this baseline

Meta's developer documentation is versioned and strongly capability/permission/use-case dependent.

The official developers.facebook.com pages were intermittently inaccessible to the research crawler, so this dossier records:

- current indexed copies of Meta's v26.0 permission/reference material;
- canonical Meta documentation URLs for later direct verification;
- conservative conclusions rather than assuming old Facebook Platform behavior remains current.

## Permissions model

Current Meta permission-reference material says Graph API permissions are granular user-granted authorizations.

Depending on capability/audience, apps may need:

- Meta App Review;
- Advanced Access;
- Business Verification;
- data-handling questions;
- periodic Data Use Checkup.

Permissions unused for a period can require regranting.

This is materially different from a personal-access token that automatically exposes every data class belonging to the logged-in user.

## Personal-profile permissions found in the current reference

### `user_posts`

Current permission material describes `user_posts` as allowing access to posts a user made on their timeline.

Its allowed-use section is not unrestricted. Current examples include:

- enabling people to create physical/digital books or albums of their timelines and share memories;
- certain parental-control/safety-analysis uses.

A Facebook Post reference states that a user's post can be read when the owner has granted `user_posts`.

Sociarium's exact personal archival use case would need current Meta review/permission eligibility analysis rather than assuming permission names equal unconditional access.

### `user_photos`

Current v26.0 indexed reference documents:

`GET /{user-id}/photos?type=uploaded`

with a user access token and `user_photos` permission for a person's uploaded/tagged photos.

### `user_videos`

Current permission reference lists `user_videos`, allowing an app to read videos uploaded by a person, again under specific allowed uses.

## Missing "whole account" contract

This baseline did not find a supported Graph API endpoint that means:

> return the authenticated user's complete Facebook activity/history/account state.

Important account categories such as:

- full reaction/activity history;
- saved items;
- search history;
- personal Messenger inbox;
- broad settings/security history;
- recommendation/view logs;

are instead primarily visible through Facebook's own access/export interfaces.

## Stable IDs

Facebook Graph objects use numeric/object IDs and graph edges.

These are useful for durable identity when exposed, but Sociarium must verify:

- ID stability across exports vs Graph API vs private web protocol;
- profile/page/additional-profile distinctions;
- whether export packages preserve the same canonical IDs.

## API versioning

Graph API is explicitly versioned. Current indexed references show v26.0.

A future adapter must record:

- exact Graph version;
- permission set;
- App Review status;
- fields requested;
- deprecation schedule.

Meta API version drift is acquisition provenance, not an invisible implementation detail.

## Current conclusion

The Graph API offers legitimate supported acquisition for selected Facebook data classes.

It does **not** currently present itself as the universal self-data substrate Sociarium would ideally want.

Any implementation should be capability-specific, not marketed as "Facebook fully synchronized."
