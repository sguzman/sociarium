# ADR 0006: Separate social surface from acquisition source

- Status: Accepted
- Date: 2026-09-17

## Context

Sociarium originally proved its first vertical slice through the official X API. That made it easy to speak as though a social surface and an acquisition mechanism were the same thing.

They are not.

X is a social surface. Evidence about an X profile may be acquired through several mechanisms:

- the official X API;
- X's first-party downloadable account archive;
- future explicitly documented capture/import mechanisms.

The distinction became operationally necessary when X's official API moved to prepaid pay-per-use and the project discovered that a surface's sanctioned developer interface could not be treated as the whole access model.

Treating an archive as a fake surface would corrupt the ontology. Forcing archive import through the network-oriented `SocialAdapter` trait would corrupt the acquisition boundary.

## Decision

Model **surface** and **acquisition source** as independent axes.

- `SurfaceId` continues to answer where the social identity/object belongs.
- Acquisition implementations answer how evidence entered Sociarium.
- A normalized X profile remains surface `x` regardless of whether evidence came from the official API or an account archive.
- Stable remote IDs continue to bind local profiles across acquisition sources.
- Importers are not required to implement the network-oriented `SocialAdapter` trait.
- Shared acquisition envelopes/evidence types should live in a source-neutral layer rather than inside a surface adapter.

At the time of this decision, the immediate zero-cost implementation candidate was a dedicated X archive importer. ADR 0007 later superseded that implementation priority: archive import and the official API adapter are now preserved acquisition candidates/evidence while the Surface Atlas researches the full access terrain before further implementation.

## Consequences

- Corpus records from API and archive sources can coexist under one X profile without inventing duplicate identities.
- Provenance must distinguish the acquisition source even when normalized objects share the same surface/profile IDs.
- Storage and query code must not assume every acquisition has pagination, OAuth, or a live remote request.
- Future surfaces can expose multiple acquisition sources without changing the ontology.
- Offline archive import can remain credential-free when/if research later admits it as an implementation target.
- The same distinction supports Surface Atlas research into official APIs, exports, public representations, and authorized first-party private protocols without confusing any one of them with the surface itself.

## Non-decision

This ADR does not define a universal plugin framework for every possible import source. Generalization should stop at the boundaries required by real implementations.
