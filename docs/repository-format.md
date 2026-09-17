# Repository format

Sociarium's repository is intended to outlive any particular executable build.

## Layers

A corpus repository conceptually contains:

```text
config/          user-selected surfaces/profiles/policies
raw/             preserved remote evidence, organized by source
objects/         normalized durable corpus objects
observations/    historical state observations and snapshots
provenance/      acquisition/transformation records
derived/         regenerable human/agent-facing projections
indexes/         disposable local indexes and databases
```

Exact file sharding and naming remain intentionally unsettled until the M0 X slice provides real data pressure.

## Canonical versus disposable

Canonical/durable:

- configuration describing tracked profiles (minus secrets);
- preserved raw evidence selected by retention policy;
- normalized corpus records;
- observation/provenance history;
- schema/migration metadata.

Disposable/rebuildable:

- SQLite databases;
- FTS/vector indexes;
- caches;
- temporary pagination state where it is safe to reconstruct;
- generated Markdown/agent context views.

Deleting indexes should be recoverable. Deleting the corpus should not be.

## Secrets

OAuth access tokens, refresh tokens, client secrets where applicable, and other credentials must never be committed to the corpus repository. Credential storage is an operational concern outside canonical Git-tracked data.

## Git

Git versions the corpus repository; it is not the query engine or persistence API.

A sync should be able to update corpus files without requiring an automatic commit. Auto-commit may later be a policy option rather than an architectural prerequisite.
