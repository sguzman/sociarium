# Surface Atlas

This directory is the primary research corpus for Sociarium.

The governing project charter is [`../project-charter.md`](../project-charter.md).

Sociarium does not begin with adapters. It begins by understanding how each social surface lets an authorized user access, preserve, and continuously observe their own data.

During the Surface Atlas phases, progress is measured primarily in durable knowledge: inventories, dossiers, evidence, access matrices, protocol maps, dated classifications, and explicit unknowns. Adapter count is not the scorecard.

## Current phase

**R2 adversarial protocol archaeology remains active as a research program; the X baseline is complete. R3 comparative analysis is now active.**

R1 is complete for all six initial surfaces. X completed its first deep R2 sweep under issue #12. New surface-specific implementation remains frozen unless an I0 admission decision or explicit human-principal override authorizes it.

The first R3 cross-surface comparison is maintained in [`../comparative-access-atlas.md`](../comparative-access-atlas.md).

## Inventory

The initial inventory is complete. Do not guess or pad this table merely to make it look complete; later additions are allowed when the human principal actually cares about another surface.

| Surface | Operator priority | Research status | Overall tier | Last reviewed | Dossier |
| --- | --- | --- | --- | --- | --- |
| X / Twitter | **Highest** | R2 baseline complete; implementation not admitted | **Tier C — Adversarial** | 2026-09-18 | [dossier](x/README.md) |
| Reddit | unranked | baseline desk-research dossier complete; direct observation pending | provisional Tier C — Adversarial (Data API has Tier B-like paths) | 2026-09-18 | [dossier](reddit/README.md) |
| Bluesky | unranked | baseline desk-research dossier complete; direct observation pending | **Tier A — Sovereign-friendly** | 2026-09-18 | [dossier](bluesky/README.md) |
| Facebook | unranked | baseline desk-research dossier complete; direct observation pending | provisional Tier C — Adversarial | 2026-09-18 | [dossier](facebook/README.md) |
| YouTube | unranked | baseline desk-research dossier complete; direct observation pending | provisional Tier B — Workable (Tier C pockets) | 2026-09-18 | [dossier](youtube/README.md) |
| Quora | unranked | baseline desk-research dossier complete; direct observation pending | provisional Tier C — Adversarial | 2026-09-18 | [dossier](quora/README.md) |

The initial inventory is intentionally open-ended. New surfaces may be added later without reopening the question of whether the first inventory was valid.

**Operator priority is not an access tier.** Priority records how much the surface matters to the human principal; tier records how the surface treats self-data access. A high-priority surface may be adversarial, and a low-priority surface may be sovereign-friendly.

X remains the highest-priority surface, but its first deep R2 baseline is complete. Direct observation confirmed a broad and coherent first-party protocol while the final access classification remained **Tier C — Adversarial**. Under current conditions, the private protocol is documented but not implementation-admitted.

## Queued surfaces

These surfaces have been explicitly added by the human principal as **low-priority future research targets**. They are inventory entries only: no tier assignment, dossier, desk research, or implementation work is implied yet.

| Surface | Operator priority | Research status |
| --- | --- | --- |
| GitHub | low / queued | not started |
| Hugging Face | low / queued | not started |
| Civitai | low / queued | not started |
| Wikipedia | low / queued | not started |
| TikTok | low / queued | not started |
| Pinterest | low / queued | not started |
| Substack | low / queued | not started |
| Medium | low / queued | not started |
| Wattpad | low / queued | not started |
| SoundCloud | low / queued | not started |
| Sketchfab | low / queued | not started |
| Tumblr | low / queued | not started |
| Royal Road | low / queued | not started |
| Ream | low / queued | not started |
| DeviantArt | low / queued | not started |

Queued surfaces should remain dormant until the human principal promotes one or the active research plan reaches them naturally. The queue exists so these sites are remembered without expanding current scope.

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
