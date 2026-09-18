# X official account archive

Research date: 2026-09-18.

Evidence class: **documented fact** unless marked otherwise.

## Access path

X documents the archive under the normal account settings:

`Settings and privacy -> Your account -> Download an archive of your data -> Request archive`

The user confirms the account and requests preparation. X sends an email/in-app notification when the archive is ready.

No developer app is part of this workflow.

The current help page does not state a fee for requesting the archive.

## Documented contents

X describes the download as a machine-readable archive containing HTML and JSON files.

The current help page explicitly lists:

- profile information;
- posts;
- Direct Messages;
- Moments;
- attached media, including images, videos, and GIFs from posts/DMs/Moments;
- followers;
- accounts followed;
- address book;
- Lists created, joined, or followed;
- inferred interest/demographic information;
- advertising interaction information;
- more account information.

X separately describes the user's "entire post history" as accessible through X Data / the archive path.

## Historical value

X Help says the ordinary profile timeline displays up to 3,200 recent posts. For older posts it directs the user to the archive, which it describes as beginning with the first post.

For Sociarium this makes the archive potentially superior to the live UI for historical bootstrap.

## Operational friction

The archive is not a continuous synchronization interface.

It requires:

1. a manual request;
2. server-side preparation;
3. waiting for notification;
4. downloading a generated package;
5. repeating the process for a newer snapshot.

A separate current X Help article notes that archive preparation may take 24 hours or longer.

Even if this mechanism is monetarily free, that recurring human labor is a first-class access cost under the Sociarium charter.

## Current Sociarium position

The archive is valuable as:

- historical bootstrap;
- disaster recovery;
- format/provenance research;
- cross-check against other acquisition sources.

It is not accepted as the sole long-term X acquisition model.

The existing partial X archive importer remains paused until the X dossier completes enough research to justify implementation.

## Unknowns requiring a real current archive

Do not inherit old Twitter archive assumptions without evidence.

A current real archive should eventually establish:

- exact file names/layout;
- whether payloads remain JavaScript-assignment-wrapped JSON, plain JSON, or mixed;
- stable account ID presence;
- full Post IDs and relationship IDs;
- likes/bookmarks inclusion;
- block/mute inclusion;
- DM completeness;
- edit history;
- deleted-object traces;
- media filename/link semantics;
- whether large datasets are split across multiple parts;
- whether repeat archives are full snapshots or contain any incremental metadata.

Any real archive is private/account-sensitive evidence and should not be committed to the public Sociarium repository.
