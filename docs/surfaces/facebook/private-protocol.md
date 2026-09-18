# Facebook first-party private web protocol

Research date: 2026-09-18.

Evidence class: **public technical evidence**, pending direct Sociarium observation.

## Current family

Public technical examples from the current Facebook web application repeatedly show requests to:

`https://www.facebook.com/api/graphql/`

or equivalent Facebook subdomain paths.

The request shape is associated with Meta's Relay/Comet web stack.

Reported fields/headers include structural names such as:

- `doc_id`;
- `variables`;
- `fb_api_caller_class=RelayModern`;
- `fb_api_req_friendly_name`;
- `fb_dtsg`;
- `lsd`;
- `x-fb-lsd`;
- current user/profile IDs;
- Comet request metadata.

These observations are sufficient to identify a substantial undocumented first-party protocol family, but not to claim any one document ID or operation is durable.

## Document IDs

Public scripts/examples show operations identified by numeric `doc_id` values and human-readable friendly names.

The numeric identifiers are implementation/build artifacts and should be assumed volatile until direct observation establishes otherwise.

A future protocol dossier should preserve:

- observed date;
- friendly operation name;
- document-ID shape/value only as dated evidence;
- variables schema;
- response schema;
- pagination;
- auth/session boundary.

## Why this matters

Facebook's first-party client can display:

- Activity Log;
- Saved;
- groups;
- reactions;
- comments;
- connections;
- notifications;
- settings;
- security/account data;
- messaging and many other private views.

Much of that state does not map to a broad supported public self-data API.

Therefore the first-party protocol is essential for understanding the real surface even if Sociarium never automates it.

## Sensitive authority

Public technical examples use browser-session material such as CSRF/session tokens.

Sociarium must never commit:

- cookies;
- `fb_dtsg` values;
- LSD tokens;
- Authorization/access tokens;
- private response bodies;
- Messenger conversations.

Only structural names/semantics belong in public docs.

## Anti-automation boundary

Meta explicitly defines and combats unauthorized scraping/automated collection.

Its current Automated Data Collection Terms require express written permission for automated collection and reserve technical enforcement rights.

Therefore:

- **observe first-party browser traffic:** active research target;
- **document operation/schema behavior:** active research target;
- **automatically replay private endpoints as a live adapter:** not admitted;
- **evade anti-bot/security systems:** outside ordinary Sociarium acquisition doctrine.

## Direct unknowns

Sociarium has not yet directly established:

- current operation names for own timeline/activity/Saved/etc.;
- current pagination shape;
- history depth per data class;
- exact stable ID correspondence across private protocol/export/Graph API;
- current Messenger protocol split;
- operation/document-ID rotation frequency;
- client integrity/anti-automation headers beyond ordinary session/CSRF fields.

These remain direct-observation tasks.
