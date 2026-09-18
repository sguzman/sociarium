# Facebook terms and operational constraints

Research date: 2026-09-18.

This is technical/project research, not legal advice.

## Automated collection

Meta's current help material defines scraping as automated data collection from websites or interfaces built for people.

Meta says it maintains a dedicated External Data Misuse effort to detect and make unauthorized scraping harder and more costly.

Meta's Automated Data Collection Terms, effective October 7, 2024, state that automated collection from Meta Company Products requires Meta's express written permission and that merely accepting those terms does not itself grant that permission.

The terms also give Meta broad rights to restrict/terminate automated collection and require compliance with Meta technical signals and controls.

## Operational consequence

Facebook's private web protocol is technically observable because the first-party client uses it.

That does **not** make it an implementation-admitted API.

Sociarium must keep separate:

1. observation;
2. schema/protocol documentation;
3. technical reproducibility;
4. operational/contractual acceptability of automation.

## Graph API permission constraints

The supported Graph API is itself not permissionless.

Meta's current permission materials describe:

- granular user authorization;
- App Review for relevant data/use cases;
- Advanced Access;
- Business Verification for some advanced requests;
- data-handling requirements;
- Data Use Checkup.

Even self-data access can therefore carry developer-program friction beyond ordinary user login.

## Export/access tools

By contrast, Facebook's human-facing access/download tools are broad and explicitly designed to let the user inspect or export account information.

This split is central to the Tier C assessment:

```text
broad human self-access
        +
narrow/reviewed machine access
        +
strong anti-automation restrictions on human interfaces
```

## Current conclusion

The safe current Sociarium position is:

- document supported Graph API capabilities;
- document first-party export/access capabilities;
- directly observe private protocol structure without replay automation;
- do not build a scraping/private-protocol acquisition client during R1.
