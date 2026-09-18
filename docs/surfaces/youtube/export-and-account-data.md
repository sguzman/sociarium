# YouTube export and account-data paths

Research date: 2026-09-18.

## Google Takeout

Google Takeout is the first-party portability mechanism for Google products.

Current Google Account Help says users can export/download data from products they use, explicitly including:

- YouTube videos;
- registration/account activity;
- other selected Google product data.

A user can select products/categories and create an archive.

Google also supports export destinations and recurring-export options in the Takeout system, though exact product-specific scheduling/contents should be validated in the current UI.

## Uploaded video files

Takeout is especially important because YouTube API policy does not permit ordinary API clients to back up audiovisual content.

For self-owned uploaded media, Takeout is therefore the legitimate first-party backup source.

## Your Data in YouTube

YouTube's "Your Data in YouTube" page summarizes content/activity data and provides privacy/data controls.

Current Help specifically mentions management of:

- video privacy;
- playlist privacy;
- subscription privacy;
- purchased videos;
- comments.

This is a human-facing self-data plane separate from Data API coverage.

## Google My Activity

Watch history and search history are tied into Google/YouTube activity controls.

Since the Data API no longer exposes Watch History/Watch Later as traversable special playlists, My Activity/Takeout are critical research targets for consumption-history preservation.

## What a real current Takeout should establish

Do not rely on historical Takeout package folklore.

A current YouTube export should be inspected for:

- uploaded video files;
- video metadata;
- playlists;
- subscriptions;
- liked/disliked history;
- comments;
- watch history;
- search history;
- live-chat history;
- channel metadata;
- memberships/purchases;
- deleted/private/unlisted object traces;
- stable IDs;
- timestamps/timezones;
- JSON/CSV/HTML format split;
- Brand Account boundaries;
- repeat-export behavior.

## Brand Accounts

Google's current Takeout help warns that YouTube data may be associated with a Brand Account and the user may need to switch account context to retrieve the relevant videos/data.

Sociarium must model account/channel identity explicitly rather than assuming one Google login equals one YouTube channel.

## Sociarium position

Takeout is a strong historical/bootstrap source.

It should be compared with the Data API rather than treated as its inferior fallback:

```text
Data API
    live, structured, quota-governed, storage-policy constrained

Takeout
    user portability/archive, broader history/private activity, snapshot-oriented
```

Different sources can legitimately own different parts of the corpus.
