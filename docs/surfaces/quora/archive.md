# Quora official data archive

Research date: 2026-09-18.

## Request path

Quora officially says users can request a copy of their data by:

- emailing `privacy@quora.com`; or
- using Quora's contact form and selecting the option requesting a copy of their data.

Quora says it will send an **archive of the user's content and personal data** to the account's primary email address.

## Timing

Quora says the archive is typically received within **72 hours after its team confirms receipt of the request**.

That wording matters:

- the clock is not necessarily from the instant the user presses a button;
- staff/process confirmation is part of the flow;
- this is an asynchronous privacy-export mechanism, not a low-latency sync interface.

## Current schema: unknown

The official Help article reviewed in this baseline does not provide:

- archive container type;
- file manifest;
- JSON/CSV/HTML schema;
- per-data-class field list;
- stable-ID guarantees;
- incremental/delta semantics.

Therefore Sociarium should not infer today's archive format from old Quora exports or third-party examples.

A real current archive is required.

## Data classes to verify in a real archive

- profile/account identity;
- questions;
- answers;
- posts;
- comments;
- edit history;
- upvotes;
- Bookmarks;
- followed questions;
- privately followed questions;
- followed people;
- followed topics;
- Spaces;
- private Messages;
- notifications;
- moderation actions;
- account/security data;
- content views/feed history;
- LLM/training/privacy preference state;
- deletion history;
- stable IDs;
- media attachments.

## Anonymous-content limitation

Quora currently says it does not retain identity information linking anonymous questions/answers to the originating account.

Therefore even a comprehensive account archive should **not be assumed** able to reconstruct historical anonymous contributions as belonging to that user.

This is an ontology/provenance limitation, not simply a missing file.

## Sociarium position

The archive is valuable as:

- account bootstrap;
- privacy/data recovery;
- historical source;
- cross-check against public/private protocol IDs.

It is not acceptable as the sole live acquisition mechanism because repeated email/contact requests plus preparation delays would make the human operator the synchronization scheduler.
