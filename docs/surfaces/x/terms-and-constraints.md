# X terms and operational constraints

Research date: 2026-09-18.

This file records current platform constraints relevant to implementation admission. It is technical/project research, **not legal advice**.

## Terms of Service

The current X Terms of Service for users outside the EU/EFTA/UK state that users may not access the Services through means other than X's currently available published interfaces unless separately allowed.

The current Terms also expressly prohibit scraping without prior written consent and restrict working around technical limitations.

They additionally contain a reverse-engineering restriction, subject to whatever exceptions applicable law may require.

For Sociarium this means the existence of an observable private protocol does **not** by itself mean an automated client should be implemented.

## Automation Rules

X's Automation Rules, updated April 2026, explicitly say not to use non-API-based forms of automation such as scripting the X website and warn that these techniques may lead to permanent suspension.

These rules are especially relevant to any proposal that would continuously replay browser-private endpoints outside the first-party client.

## Separation of questions

Sociarium should keep four questions separate:

1. **Can the protocol be observed?**
2. **Can its schemas/operations be documented?**
3. **Can requests be technically reproduced?**
4. **Should Sociarium operate an automated client against it?**

A "yes" to the first three does not automatically produce a "yes" to the fourth.

## Current implementation-admission consequence

As of 2026-09-18:

- official X API: implementation exists but live use is paid;
- archive import: technically/operationally plausible but manual/asynchronous;
- private protocol: research-worthy and likely technically capable, but automation is contractually/operationally high-risk under current X policies.

The current action is therefore **forensics first, no new X acquisition code**.

## Re-review triggers

Revisit this file when:

- X changes its Terms;
- X changes its Automation Rules;
- X introduces a free self-data API;
- X changes archive/export behavior;
- a jurisdiction-specific portability/access right materially changes the practical boundary;
- the human principal deliberately requests a fresh implementation-admission review.
