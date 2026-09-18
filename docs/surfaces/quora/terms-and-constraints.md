# Quora terms and operational constraints

Research date: 2026-09-18.

This is technical/project research, not legal advice.

## Current-source limitation

Quora's canonical Terms of Service page on `quora.com` could not be directly retrieved by the research crawler because Quora blocks automated retrieval from that host.

That means this baseline does **not** claim a fully verified current Terms interpretation.

This is a deliberate evidence-quality limitation.

## Historical/current-indexed Terms pattern

Archived/indexed copies of Quora's Terms have long distinguished:

- conditional crawling/search-engine access that follows `robots.txt` and uses a descriptive user agent;
- scraping/automated extraction outside permitted interfaces;
- access to non-public areas;
- circumvention of security/authentication controls.

However, because the canonical current Terms were not directly readable in this research session, **implementation must re-check the live Terms manually before relying on any historical clause**.

Do not cite this file as proof of a current legal permission/prohibition without that re-verification.

## Stronger operational evidence: protocol friction

Even without relying on Terms interpretation, current public technical evidence shows the first-party web protocol using:

- authenticated browser cookies;
- CSRF/formkey state;
- Quora revision/build identifiers;
- browser-window/broadcast identifiers;
- Cloudflare Turnstile state.

That is meaningful operational evidence that Quora does not present the private GraphQL layer as a stable public developer contract.

## Community ownership model

Quora's current Question and Answer Policies explicitly say:

- questions are viewed as **community property**.

Current Help further explains that:

- question deletion rights can end after activity occurs;
- after account deletion, community-property objects such as questions, topics, and Spaces can remain.

This creates an unusual provenance/ownership model:

```text
authored/intitiated by user
        !=
fully disposable/owned by user forever
```

Sociarium should preserve that distinction.

## Account deletion

Current Help says account deletion:

- begins with a 14-day deactivation/grace period;
- then permanently deletes the account/profile/content associated with the account;
- can leave externally republished copies;
- treats community-property content specially.

A separate Help article says community property such as questions, topics, and Spaces remains after account deletion.

## Anonymous contributions

Current Quora Help states that anonymous question/answer identity links are not retained.

That means user identity cannot later be recovered merely because the user originally authored the content.

This creates a hard limit on account-centric historical reconstruction.

## Current implementation consequence

As of this baseline:

- official archive: permitted user-data path, but manual/asynchronous;
- public pages: useful but incomplete;
- private GraphQL: technically rich but undocumented/session-bound;
- private-protocol automation: **not implementation-admitted** pending direct observation and manual current Terms review.

The correct next move is documentation, not replay automation.
