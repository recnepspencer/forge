# API Overview

This is the public shape of `forge-relational` as a standalone library.

## Public center of gravity

Import through:

```rust
use forge_relational::facade::*;
```

The product center is:

- `runtime`
- `transactions`
- `query`
- `schema`
- `snapshots`
- `history`
- `inspection`
- `publication`

## Main doors

### Setup

- `RelationalRuntimeApi::builder()`
- grouped setup sections on the builder:
  - `runtime_setup(...)`
  - `schema_setup(...)`
  - `storage_setup(...)`
  - `durability_setup(...)`
  - `strategy_setup(...)`

### Write truth

- `runtime.begin_transaction(...)`
- `tx.push_batch(...)`
- `tx.commit()`

### Read truth

- `runtime.read_truth()`
- `runtime.snapshots()`

### Inspect what happened

- `runtime.inspect_what_happened()`
- `runtime.publication()`

### Past truth

- `runtime.history()`
- `runtime.history_authority()`
- `runtime.replay()`

## Contained real lanes

These are real public capabilities, but not the default first-contact story:

- `runtime.validation()`
- `runtime.compiled_artifacts()`
- `runtime.compiled_artifacts_authority()`
- `runtime.retention()`
- `runtime.index_access()`
- `runtime.index_authority()`
- `runtime.durability()`
- `runtime.durability_authority()`
- `runtime.commit_strategies()`
- `runtime.commit_strategies_authority()`
- `runtime.merge()`
- `runtime.replay_authority()`

## Published runtime lanes

The standalone library surface now uses the job-shaped runtime names directly:

- `read_truth()`
- `snapshots()`
- `inspect_what_happened()`
- `publication()`
- `history()`
- `history_authority()`
- `replay()`
- `validation()`
- `compiled_artifacts()`
- `retention()`
- `durability()`
- `merge()`

