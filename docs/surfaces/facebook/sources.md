# Facebook source ledger

Last reviewed: 2026-09-18.

## Official Meta/Facebook sources

### Access or download information / Access Your Information

https://www.facebook.com/help/windows-desktop/506794473544588/

Used for:

- Access Your Information category structure;
- Download Your Information category/date-range selection;
- Transfer a Copy of Your Information;
- Activity Log description.

### Accounts Center data-management update

https://about.fb.com/news/2023/10/manage-your-information-across-apps/

Used for:

- centralization of Download Your Information and Access Your Information in Accounts Center;
- cross-Facebook/Instagram download controls;
- transfer functionality.

### Data logs engineering post

https://engineering.fb.com/2025/02/04/security/data-logs-the-latest-evolution-in-metas-access-tools/

Used for:

- addition of data logs to Download Your Information;
- examples such as content-view/recommendation activity;
- relationship among Access Your Information, Activity Log, and downloads.

### Facebook scraping Help

https://www.facebook.com/help/463983701520800

Used for:

- Meta's definition of scraping;
- anti-scraping enforcement posture;
- distinction between authorized and unauthorized automated collection.

### Automated Data Collection Terms

Canonical location:

https://www.facebook.com/legal/automated_data_collection_terms

The direct current page was difficult for the crawler to render; a current archived rendering was used for structural verification.

Used for:

- express-written-permission requirement;
- enforcement/technical-control terms;
- automated collection definition.

### Meta permissions reference

Canonical location:

https://developers.facebook.com/docs/permissions/reference/

The official developers site rate-limited the research crawler. A current indexed rendering of the v26.0 Meta documentation was used as secondary transport for the official text.

Used for:

- permission/App Review/Advanced Access model;
- `user_posts`;
- `user_videos`;
- data-use review requirements.

### Graph API User Photos reference

Canonical location:

https://developers.facebook.com/docs/graph-api/reference/user/photos/

Current indexed v26.0 rendering used for:

- uploaded-photo edge;
- `user_photos` permission.

### Graph API Post reference

Canonical location:

https://developers.facebook.com/docs/graph-api/reference/post/

Current indexed v26.0 rendering used for:

- user-post readability with `user_posts`.

## Older official Meta portability sources used cautiously

Meta's historical public documents establish that Download Your Information has offered machine-readable JSON as well as human-readable HTML.

Because UI/package formats can change, this is historical context rather than proof of the exact 2026 package format.

Examples:

- Meta/Facebook data portability white papers and regulatory submissions;
- historical Download Your Information announcements.

A real current archive should establish the present format.

## Public technical evidence for private web protocol

### dvygolov Facebook/Meta browser scripting examples

Current/public GitHub Gists demonstrate the first-party web stack calling:

`https://www.facebook.com/api/graphql/`

with Relay/Comet-shaped fields including `doc_id`, `variables`, `fb_dtsg`, `lsd`, and friendly operation names.

These are secondary technical evidence only. Do not copy credential extraction behavior into Sociarium.

## Evidence hierarchy

1. direct Sociarium observation;
2. current official Meta/Facebook Help and developer docs;
3. current indexed mirrors of official docs when Meta's site blocks automated retrieval;
4. current Meta engineering/newsroom material;
5. multiple public current protocol observations;
6. historical Meta documents;
7. historical memory.

Any implementation decision requires a fresh direct check of Meta's official developer console/docs because permission eligibility changes frequently.
