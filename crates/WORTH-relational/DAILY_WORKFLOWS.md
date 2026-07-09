# Daily Workflows

This library gets much easier if you think in jobs instead of modules.

## Build runtime

Use:

- `RelationalRuntimeApi::builder()`
- `profile(...)` if you want a predefined setup shape
- `runtime_setup(...)`
- `schema_setup(...)`
- `storage_setup(...)`
- `durability_setup(...)`
- `strategy_setup(...)`
- `build()`

## Write truth

Use:

- `begin_transaction(...)`
- `push_batch(...)`
- `commit()`

For larger staged writes, `plan_bulk_mutation_batch(...)` is the main advanced
helper.

The lower-level `admit_*` methods are still real, but they are not the default
story.

## Read truth

Use:

- `read_truth()`
- `snapshots()`
- query APIs when you need larger or more selective reads

Read current truth first.

Do not start with history, replay, or storage helpers unless that is actually
the job.

When you need a pinned read basis, create a snapshot first and then read from
that snapshot handle.

## Inspect what happened

Use:

- `inspect_what_happened()`
- `publication()`

Think of this as the operator readback zone:

- what changed?
- what published?
- what is wrong?
- what is retained?

## Go to history

Use:

- `history()`
- `history_authority()` when you need branch creation or branch-head control

This is the next door after current truth when the question becomes "what did
truth look like before?"

## Go to replay

Use:

- `replay()`

This is the deeper verification lane after history.

## Deep lanes

Use these only when the job really calls for them:

- `validation()`
- `compiled_artifacts()`
- `retention()`
- `index_access()` / `index_authority()`
- `merge()`
- `durability()`
- `durability_authority()`
- `commit_strategies()`
- `commit_strategies_authority()`

That is not because they are hidden.

It is because they are real high-power lanes and should feel intentional.
