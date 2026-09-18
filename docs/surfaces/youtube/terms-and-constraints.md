# YouTube terms and preservation constraints

Research date: 2026-09-18.

This is project/technical research, not legal advice.

## YouTube Terms: automated access

Current YouTube Terms prohibit accessing the Service using automated means such as robots, botnets, or scrapers except for qualifying public search-engine robots.txt use or with YouTube's prior written permission.

The Terms also restrict circumvention of security/usage limitations.

## API Developer Policies: undocumented interfaces

Current YouTube API Developer Policies state that API clients must not:

- use undocumented APIs without express permission;
- reverse engineer undocumented YouTube API services;
- scrape YouTube/Google applications;
- use non-YouTube-API technology to retrieve YouTube API Data.

This creates a bright operational distinction between the supported Data API and first-party Innertube.

## API storage/refresh rules

Most Authorized Data may be retained for no more than 30 days before delete-or-refresh.

Non-Authorized Data has a similar 30-day limit.

Longer-term storage is allowed for certain authorized analytics/reporting/statistical data, but the client must continue verifying authorization/current existence.

Clients must make reasonable efforts to keep stored API Data aligned with current YouTube state.

Historical API data may be displayed if accurately contextualized, but the storage rules still apply.

## Revocation/deletion

When user consent is revoked, API clients must delete associated authorized data according to the policy timelines.

Clients using user data must provide a way for the user to request deletion of stored data.

## Sociarium conflict

Sociarium's preferred evidence model is append-only historical acquisition:

```text
what was observed at time T remains evidence of what was observed at T
```

YouTube API policy instead emphasizes refresh/current-state consistency and deletion on authorization revocation.

This does not necessarily make all historical analysis impossible, but it means the ordinary Sociarium raw-acquisition model cannot simply be applied without policy-aware design.

## Media restriction

YouTube API clients may not use the API to download/import/backup/cache audiovisual content without prior written approval.

Use first-party portability mechanisms such as Takeout for self-owned media backup.

## Current conclusion

YouTube is operationally **Tier B** where the documented API cleanly supports the desired data class.

It becomes **Tier C-like** when:

- the desired self-data exists only in first-party account views;
- preserving immutable API snapshots conflicts with API-data policies;
- the only live technical route would be undocumented Innertube automation.
