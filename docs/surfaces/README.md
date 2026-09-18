# Surface Atlas

This directory is the primary research corpus for Sociarium.

The governing project charter is [`../project-charter.md`](../project-charter.md).

Sociarium does not begin with adapters. It begins by understanding how each social surface lets an authorized user access, preserve, and continuously observe their own data.

During the Surface Atlas phases, progress is measured primarily in durable knowledge: inventories, dossiers, evidence, access matrices, protocol maps, dated classifications, and explicit unknowns. Adapter count is not the scorecard.

## Current phase

The current project phase is **R0 — enumerate the surfaces the human principal cares about**.

No new surface-specific implementation should begin during R0. The immediate task is to build the inventory, then investigate those surfaces systematically.

## Inventory

The human principal will supply the initial list. Do not guess or pad this table merely to make it look complete.

| Surface | Operator priority | Research status | Overall tier | Last reviewed | Dossier |
| --- | --- | --- | --- | --- | --- |
| X / Twitter | **Highest** | baseline desk-research dossier complete; direct first-party observation pending | provisional Tier C — Adversarial | 2026-09-18 | [dossier](x/README.md) |
| Reddit | unranked | queued for baseline dossier | unknown | — | pending |
| Bluesky | unranked | queued for baseline dossier | unknown | — | pending |
| Facebook | unranked | queued for baseline dossier | unknown | — | pending |
| YouTube | unranked | queued for baseline dossier | unknown | — | pending |
| Quora | unranked | queued for baseline dossier | unknown | — | pending |

The initial inventory is intentionally open-ended. New surfaces may be added later without reopening the question of whether the first inventory was valid.

**Operator priority is not an access tier.** Priority records how much the surface matters to the human principal; tier records how the surface treats self-data access. A high-priority surface may be adversarial, and a low-priority surface may be sovereign-friendly.

X is currently the highest-priority research target. Its baseline dossier now establishes the documented API/export terrain, public technical evidence for the private web protocol, and current platform-policy constraints. Its Tier C classification remains provisional until direct first-party observation strengthens the undocumented-protocol evidence.

## Adjacent site-specific projects

Sociarium is not a generic reverse-engineering repository for every website the operator uses.

ChatGPT is currently **out of scope for the Surface Atlas** because its site behavior already has a dedicated project, **Chatarium**, which is the canonical home for ChatGPT-specific site documentation, protocol observations, and client work.

Cross-project references are appropriate when a Chatarium finding becomes relevant to Sociarium's general access doctrine, but the same site should not be independently maintained in both repositories.

## Research lifecycle

```text
enumerate surfaces
      ↓
official/documented research
      ↓
public technical research
      ↓
hands-on first-party observation where warranted
      ↓
per-data-class access matrix
      ↓
dated tier assessment
      ↓
acquisition candidates
      ↓
only then: implementation admission decision
```

Research is continuous. Implemented surfaces keep dossiers because APIs, exports, private protocols, pricing, and access policies change.

See:

- [Surface research doctrine](../surface-research-doctrine.md)
- [Surface access tiers](../surface-access-tiers.md)
- [Dossier template](TEMPLATE.md)
