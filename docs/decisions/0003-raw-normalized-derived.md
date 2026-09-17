# ADR 0003: Preserve raw, normalized, and derived layers

Status: accepted

## Decision

Sociarium distinguishes raw remote evidence, normalized corpus records, and derived/indexed projections.

## Consequences

- normalizer bugs can be corrected without reacquiring all remote data when raw evidence was retained;
- derived representations may be rebuilt freely;
- provenance can explain how an interpretation was produced;
- platform-specific semantics need not be destroyed merely to satisfy a generic schema.
