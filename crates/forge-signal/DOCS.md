# `forge-signal` Docs

This is the human-friendly index for the `forge-signal` surface.

If you want the shortest path:

- Start with [README.md](./README.md)
- Pick a deployment shape from:
  - [Web Development](./QUICKSTART_WEB_DEVELOPMENT.md)
  - [Game Engines](./QUICKSTART_GAME_ENGINES.md)
  - [Fintech](./QUICKSTART_FINTECH.md)
- If you want every important public surface, read [docs/API_SURFACE.md](./docs/API_SURFACE.md)

If you are doing specialized or easy-to-forget work:

- Aspects, conditions, custom condition keys, comparators, and tolerance:
  - [docs/CONDITIONS_AND_COMPARATORS.md](./docs/CONDITIONS_AND_COMPARATORS.md)
- Artifact availability, retained vs reconstructed access, and semantic truth:
  - [docs/ARTIFACT_ACCESS_MATRIX.md](./docs/ARTIFACT_ACCESS_MATRIX.md)
- Snapshots, branch-local evaluation history, and replay inspection:
  - [docs/SNAPSHOTS_BRANCHES_AND_REPLAY.md](./docs/SNAPSHOTS_BRANCHES_AND_REPLAY.md)
- Signal-lineage semantics and artifact evolution:
  - [docs/LINEAGE_MODEL.md](./docs/LINEAGE_MODEL.md)
- Transactions and keyed runtime workflows:
  - [docs/TRANSACTIONS_AND_KEYED_RUNTIME.md](./docs/TRANSACTIONS_AND_KEYED_RUNTIME.md)
- Checkpoint barriers, tier policy, and comparator selection:
  - [docs/CHECKPOINTS_AND_TIERS.md](./docs/CHECKPOINTS_AND_TIERS.md)
- Lifecycle, unregistering nodes, and GC behavior:
  - [docs/LIFECYCLE_AND_GC.md](./docs/LIFECYCLE_AND_GC.md)
- Advanced patterns and niche runtime behavior:
  - [docs/ADVANCED_PATTERNS.md](./docs/ADVANCED_PATTERNS.md)
- Harness scenarios, mutation batches, and parity/certification workflows:
  - [docs/HARNESS_AND_CERTIFICATION.md](./docs/HARNESS_AND_CERTIFICATION.md)
- Low-level tuning, storage profiles, and “I know what I’m doing, mostly” knobs:
  - [LOW_LEVEL_NERDS.md](./LOW_LEVEL_NERDS.md)

## What these docs try not to do

Most docs fail by doing at least one of these:

- hiding available parameters
- showing one generic example with no workload context
- documenting only the happy path
- pretending niche features are unimportant because they are niche

These docs try to do the opposite:

- name the actual entrypoints
- show context-specific examples
- call out retained vs reconstructed behavior explicitly
- document the niche features that AI agents and sleep-deprived humans will otherwise forget
