# API Overview

This is the public shape of `worth-relational` as a standalone library.

## Public center of gravity

Import through:

```rust
use worth_relational::facade::*;
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

### Publish semantic aspect changes

Use `facade::publication` when another runtime needs the authoritative meaning
of a committed change. `PublishedAuthoritativeAspectChange` retains aspect
identity and revision, Relational binding, change kind, optional field path,
and exact or declared-widening precision.

Runtime Bridge consumes this publication for Query-installed semantic
correspondence. Downstream callers should not derive their own aspect-change
taxonomy from raw patch fields.

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

