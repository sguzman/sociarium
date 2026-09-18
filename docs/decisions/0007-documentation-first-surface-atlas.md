# ADR 0007: Documentation-first Surface Atlas before new implementation

- Status: Accepted
- Date: 2026-09-17

## Context

Sociarium began by implementing an X vertical slice. The implementation produced useful generic infrastructure, but the project discovered late that the current official X API imposes a paid access model for useful live reads.

The immediate reaction was to make X archive import the zero-cost M0 path. That preserved implementation momentum but revealed a deeper problem: implementation was choosing the research agenda.

A user-sovereignty project should know the access terrain before committing engineering effort to a surface.

The project also needs a durable way to reason about surfaces whose documented developer interface does not describe the full protocol exposed to first-party web/mobile clients.

## Decision

Sociarium becomes **documentation-first and research-before-code** for new surface work.

The first-class project artifact is the **Surface Atlas**: dated, provenance-heavy dossiers describing how social surfaces permit an authorized user to acquire and preserve their own data.

New surface-specific implementation is frozen until the current research phase establishes the relevant surface inventory and dossiers.

Each dossier should distinguish documented facts, direct observations, inference, and unknowns; classify access by data class; and record the surface's current overall access tier.

The access tiers are:

- Tier A — Sovereign-friendly;
- Tier B — Workable;
- Tier C — Adversarial;
- Tier D — Inaccessible.

Tier C explicitly authorizes protocol archaeology as a research activity: when sanctioned interfaces materially obstruct self-data access, Sociarium studies the private protocol of the first-party client used by the authorized operator. This does not erase security boundaries or authorize credential theft/access-control bypass.

Existing Rust code is preserved. It is implementation evidence and reusable substrate, not the definition of the project and not a mandate to continue coding.

## Consequences

- Documentation can be valuable even when no adapter is implemented.
- Surface selection is evidence-driven rather than momentum-driven.
- Cost, friction, history limits, and private first-party protocols become first-class research topics.
- Official APIs, archives, public web pages, and private client protocols are treated as acquisition sources rather than as the ontology of a surface.
- A surface can be deliberately left unimplemented after research.
- Existing X API/archive work remains available but is not the active project milestone.
- Agents must not create implementation tasks merely to maintain coding velocity.

## Supersession

This decision changes the active milestone described after ADR 0006. ADR 0006's distinction between surface and acquisition source remains valid and becomes foundational to the Surface Atlas.
