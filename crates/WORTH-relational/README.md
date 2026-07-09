# worth-relational

`worth-relational` is a standalone truth runtime for graph-shaped state.

It is for systems that need state to be authoritative, transactional,
inspectable, replayable, and durable instead of just "some mutable data
structure plus vibes.

It is meant for more than CRUD apps. The target is stuff like chip simulators,
geometry kernels, planning systems, and other runtimes where history,
determinism, replay, validation, branching, and durable recovery are first-class.

The library is built around a few obvious jobs:

- build runtime
- write truth
- read truth
- manage snapshots and branches when you need controlled views of truth
- inspect what happened
- go to history or replay when you need past truth
- use merge, validation, compiled artifacts, retention, indexes, and recovery
  when the workload gets deep

Start here:

- [`QUICKSTART.md`](./QUICKSTART.md)
- [`DAILY_WORKFLOWS.md`](./DAILY_WORKFLOWS.md)
- [`API_OVERVIEW.md`](./API_OVERVIEW.md)

Examples:

- `cargo run -p worth-relational --example basic_runtime`
- `cargo run -p worth-relational --example snapshots`
- `cargo run -p worth-relational --example history_and_replay`
- `cargo run -p worth-relational --example derived_indexes`
- `cargo run -p worth-relational --example validation_and_durability`

## Minimal shape

```rust
use worth_relational::facade::{
    runtime::RelationalRuntimeApi,
    schema::RelationalSchemaRegistry,
    transactions::{TransactionOptions, WorkerIntentBatch},
};

let mut runtime = RelationalRuntimeApi::builder()
    .schema_registry(RelationalSchemaRegistry::new())
    .build();

let mut tx = runtime.begin_transaction(TransactionOptions::default());
tx.push_batch(WorkerIntentBatch::new("example"));
let _outcome = tx.commit()?;

let _truth = runtime.read_truth();
let _snapshots = runtime.snapshots();
let _history = runtime.history();
let _inspection = runtime.inspect_what_happened();
```

## Mental model

- `RelationalRuntimeApi::builder()` is the setup door
- transactions are the write-truth door
- `read_truth()` is the current-truth door
- `snapshots()` is the controlled-view door
- `inspect_what_happened()` and `publication()` are the readback doors
- `history()` and `replay()` are the past-truth doors
- `validation()`, `compiled_artifacts()`, `retention()`, `durability()`,
  `commit_strategies()`, and indexes are the deep-system doors

If you find yourself reaching into crate internals instead of
[`facade`](./src/facade.rs), you are probably leaving the intended public
surface.

