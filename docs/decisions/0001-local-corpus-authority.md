# ADR 0001: Local corpus as durable record

Status: accepted

## Decision

After remote data is acquired, Sociarium's repository becomes the durable historical record of what was observed. Remote surfaces remain authoritative for their current live state, but Sociarium does not depend on them retaining previously observed state.

## Consequences

- acquired observations may remain after remote deletion or mutation;
- provenance and observation time are first-class concerns;
- destructive overwrite is inappropriate for historically meaningful fields;
- indexes and derived views cannot be the only retained representation.
