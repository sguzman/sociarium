# Reddit official data export and account-access paths

Research date: 2026-09-18.

## Data request

Reddit documents a user data-request flow at:

`reddit.com/settings/data-request`

The user logs into the relevant account and submits the request.

Reddit says preparation may take **up to 30 days**. When ready, Reddit sends a notification to the account inbox with a download link and may in some cases use the verified email address.

This is substantially too slow/manual to serve as Sociarium's normal synchronization mechanism.

## Data accessible without waiting

Reddit also documents many account data classes as directly viewable through the logged-in account, including:

- posts the user made;
- comments the user made;
- upvoted posts/comments;
- downvoted posts/comments;
- saved posts/comments;
- hidden posts/comments;
- recently viewed posts;
- joined communities;
- moderated communities;
- archived private messages;
- Chats;
- mod mail;
- followed people;
- friends;
- blocked/approved users;
- recent IP addresses;
- preferences;
- authorized third-party applications.

This is useful evidence that these data classes exist in first-party user interfaces even where current public developer surfaces do not expose them.

## What is not yet established

The current Help article describes requesting a "copy" of Reddit account data but does not, in the material reviewed for this baseline, provide a precise current file manifest/schema for the downloadable package.

A real current export should therefore establish:

- archive container type;
- file formats;
- exact data-class inclusion;
- stable IDs;
- timestamps;
- whether saved/upvoted/downvoted/hidden items are included;
- subscriptions/follows;
- private messages vs Chat;
- deletion history;
- media references;
- pagination/splitting for large accounts;
- whether later requests are full snapshots.

Do not infer the current export schema from old third-party samples.

## Sociarium position

The export is a useful:

- portability mechanism;
- recovery/bootstrap source;
- cross-check against API/private-protocol observations.

It is **not** an acceptable sole live acquisition strategy because a repeated process with a possible 30-day wait makes the human operator the synchronization daemon.
