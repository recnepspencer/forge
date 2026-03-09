# Forge Harness Roadmap

## Goal

Make `forge-harness` the first-class execution infrastructure crate for Forge runtimes.

The crate should be strong enough that runtimes conform to it instead of quietly reshaping it.

## What Exists Now

- first-class workspace crate at `crates/forge-harness`
- generic adapter and capability contracts
- scenario plans, fixtures, mutation batches, execution requests, and execution profiles
- capture records for runs, snapshots, events, diagnostics, explanations, provenance, and replay
- stable record IDs and schema versioning
- compatibility and replay-compatibility checks
- export helpers and record archives
- time markers, feed batches, execution phases, workload budgets, and budget usage
- event subscriptions and projection helpers
- harness-native DX tooling:
  - `HarnessBench`
  - `ProfileCatalog`
  - `RunMatrix`
  - `ParitySuite`
  - `AdapterDouble`
- first truthful adapter in `forge-signal`

## What Must Be True Before We Call It First-Class

### Contract quality

- public crate docs are complete enough to teach the harness directly
- capabilities remain fail-closed
- record vocabulary and naming remain stable
- adapter contracts stay split and small

### Artifact durability

- replay, archive, and comparison records remain versioned and serializable
- compatibility policy is explicit and tested
- grouped export flows are supported without leaking runtime internals

### Tooling quality

- common run paths are ergonomic through harness-native tooling
- parity and profile-sweep workflows do not require custom runner plumbing
- event timeline tooling is typed and reusable

### Adapter discipline

- adapters expose truthful runtime behavior only
- adapters do not leak storage or planner internals
- `forge-signal` remains a client of the harness contract rather than its hidden source of truth

## Remaining High-Value Work

### Near term

- add richer archive/export sinks for larger attached payloads
- add diagnostics and replay tooling on top of the existing harness substrate
- add more fixture and mutation enrichment hooks where real clients need them
- add crate-level examples that show signal-style and non-signal-style usage

### After that

- add relational adapter with diagnostics, replay, branch/history, and serial-vs-staged-parallel parity suites as first-class acceptance paths
- add bridge adapter
- decide whether kernel-specific tooling should live on top of the harness or beside it

## Non-Goals

- do not turn the core crate into a runtime-specific abstraction bucket
- do not let adapter convenience override capability honesty
- do not freeze a canonical end-state API too early; optimize for clear DX while breaking changes are still cheap
