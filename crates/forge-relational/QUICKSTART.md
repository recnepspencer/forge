# Quickstart

This is the fastest honest path through `forge-relational`.

## 1. Build a runtime

```rust
use forge_relational::facade::{
    runtime::RelationalRuntimeApi,
    schema::RelationalSchemaRegistry,
};

let mut runtime = RelationalRuntimeApi::builder()
    .runtime_setup(|runtime| {
        runtime.runtime_name("quickstart");
    })
    .schema_setup(|schema| {
        schema.schema_registry(RelationalSchemaRegistry::new());
    })
    .build();
```

The normal setup shape is:

- start with `RelationalRuntimeApi::builder()`
- optionally shape grouped setup sections
- provide a schema registry through `schema_setup(...)`
- add storage / durability / strategy setup only when you actually need them
- `build()`

## 2. Write truth

```rust
use forge_relational::facade::transactions::{TransactionOptions, WorkerIntentBatch};

let mut tx = runtime.begin_transaction(TransactionOptions::default());
tx.push_batch(WorkerIntentBatch::new("seed"));
let commit = tx.commit()?;
```

That is the default write story:

- begin transaction
- push batch
- commit

## 3. Read truth

```rust
let truth = runtime.read_truth();
```

`read_truth()` is the current-truth door.

If you need a stable pinned view instead of "whatever is current right now",
use `snapshots()` to create and release snapshot handles.

## 4. Inspect what happened

```rust
let inspection = runtime.inspect_what_happened();
let publication = runtime.publication();
let history = runtime.history();
let replay = runtime.replay();
```

Use:

- `inspect_what_happened()` for inspection
- `publication()` for publication-facing outputs
- `history()` when you need past truth
- `replay()` when you need verification or reconstruction
- `history_authority()` when you need to create or manage branches

## 5. Reach for deeper lanes only when needed

```rust
let validation = runtime.validation();
let compiled = runtime.compiled_artifacts();
let retention = runtime.retention();
let indexes = runtime.index_access();
```

Those are real public lanes.

They are just not the first five minutes of normal usage.

After this file, the fastest concrete references are the runnable examples:

- `basic_runtime`
- `snapshots`
- `history_and_replay`
- `derived_indexes`
- `validation_and_durability`

