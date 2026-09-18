# Facebook access, download, and transfer tools

Research date: 2026-09-18.

## Accounts Center / Access Your Information

Current Facebook Help describes **Access Your Information** as a consolidated view of profile/account information.

Current categories include:

- Your Facebook activity;
- Personal information;
- Connections;
- Logged information;
- Security and login information;
- Apps and websites off Facebook;
- Preferences;
- Ads information.

This breadth is important evidence: Facebook internally exposes substantially more self-data to the human user than a conventional Graph API adapter can necessarily retrieve.

## Activity Log

Facebook describes Activity Log as a history of activity on Facebook.

It includes examples such as:

- posts/comments/likes;
- apps used;
- searches;
- other activity that can be reviewed/managed.

For Sociarium, Activity Log is both a user-access feature and a map of data classes worth investigating in the first-party protocol/export.

## Download Your Information

Current Help says users can:

- download all categories at once;
- select specific categories;
- select date ranges.

Meta has centralized Download Your Information in Accounts Center and supports downloading Facebook and Instagram data through that interface.

Historical Meta material documents HTML and JSON export formats. The current baseline did not obtain a current Help page explicitly confirming today's format choices, so a real 2026 download should verify the exact package format rather than assuming old format/UI details remain unchanged.

## Data logs

Meta engineering says Download Your Information began including additional **data logs** in 2024.

Examples include information about content viewed on Facebook and additional details about recommendations/usage.

This makes the export/access plane richer than a simple "things I posted" archive.

## Transfer a Copy of Your Information

Facebook Help describes a direct transfer mechanism for photos, videos, posts, or other supported information to external services.

Meta's portability work has historically used the open-source Data Transfer Project.

This is a positive portability feature, but it remains user-initiated transfer rather than a general live sync API.

## Research value

A real current export package should establish:

- current container/file formats;
- category hierarchy;
- stable profile/object IDs;
- posts/comments/reaction identifiers;
- audience/privacy metadata;
- edit history;
- deleted-content traces;
- group activity;
- saved items;
- friends/connections;
- Messenger/chat representation;
- media naming and timestamps;
- view/recommendation data logs;
- whether repeated exports are full snapshots or expose any incremental metadata.

## Sociarium position

Facebook's export/access tooling is strong enough to deserve its own acquisition-source research.

However:

> strong manual portability does not by itself make the surface sovereign-friendly for continuous independent synchronization.

A future archive importer could still be useful even if the overall surface remains Tier C.
