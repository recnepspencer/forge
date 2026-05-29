# Seed And Fixture Lane

## What This Feature Is

This page covers the curated seed helpers and primitive corpus that live under
`worth_schema::facade::topology_authoring`.

Examples include:

- `seed_minimal_topology(...)`
- `seed_milestone_one_primitive(...)`
- `milestone_one_default_primitive_corpus()`

## Why You Use It

Use this lane when you need:

- a quick known-good topology shape
- fixture setup for tests
- certification support inputs
- readable examples without hand-authoring every mutation

## Stable Entry Points

From [topology_authoring.rs](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/topology_authoring.rs:1):

- `seed_minimal_topology(...)`
- `seed_milestone_one_primitive(...)`
- `seed_milestone_one_primitive_on_branch(...)`
- `milestone_one_default_primitive_corpus()`
- milestone-one scenario helpers

## Core Mental Model

This is a support lane for known shapes.

Use it when the question is:

- "give me a real small topology"
- "give me a primitive scenario corpus"

Do not use it when the question is:

- "what is the long-term public runtime entry?"
- "what is the stable public runtime result shape?"

## How It Executes

These helpers materialize known topology shapes for you through internal
runtime-backed support code.

That means they are great for:

- tests
- fixtures
- examples
- certification support

and deliberately not the same thing as Query-owned runtime orchestration.

## Small Example

```rust
use forge_relational::facade::runtime::RelationalRuntimeApi;
use worth_schema::facade::{bootstrap_schema_registry, topology_authoring::seed_minimal_topology};

let mut runtime = RelationalRuntimeApi::builder()
    .schema_registry(bootstrap_schema_registry()?)
    .build();

let fixture_seed = seed_minimal_topology(&mut runtime, "example.seed")?;
```

## Real Example

```rust
use forge_relational::facade::runtime::RelationalRuntimeApi;
use worth_schema::facade::{
    bootstrap_schema_registry,
    topology_authoring::{milestone_one_default_primitive_corpus, seed_milestone_one_primitive},
};

let mut runtime = RelationalRuntimeApi::builder()
    .schema_registry(bootstrap_schema_registry()?)
    .build();

let scenario = milestone_one_default_primitive_corpus()
    .into_iter()
    .next()
    .expect("primitive scenario");

let fixture_case = seed_milestone_one_primitive(&mut runtime, "example.primitive", &scenario.case)?;
```

## How It Relates To Other Features

- Use [Create Batch Builder](./create-batch-builder.md) when you want to author
  the batch yourself.
- Use [Verification](./verification.md) when you want the narrow explicit
  verification lane.

## Inspection And Debugging

If a seed helper is too opinionated for your task, that is usually a sign you
should switch to `TopologyCreateBatchBuilder` instead of stretching the seed
lane.

If you find yourself depending on fields like `persisted_truth`, `read_basis`,
or `certified_interpretation` from `MinimalTopologySeed`, treat that as
leaving the fixture/support lane, not as the stable public runtime story.

For primitive commits, the stable support result is now
`SeededTopologyCommit`, not `VerifiedTopologyCommit`.

## Anti-Patterns

- Do not teach this as the normal runtime entry story.
- Do not use milestone corpus helpers as a substitute for stable app-facing
  runtime APIs.
- Do not add new broad runtime policy to this namespace.
- Do not build new consumer code around schema-owned verified-commit products.
- Do not treat `platform::authority` as the place to import read-basis or
  post-execution support artifacts.

## Current Limits

- This lane is intentionally biased toward fixtures and support.
- The helper names reflect milestone-one primitive corpus history because that
  historical shape is the supported corpus boundary.
- `MinimalTopologySeed` and `SeededTopologyCommit` still expose fixture-oriented
  support artifacts because certification and support code depend on them. They
  are not the public runtime result story.

## Related Docs

- [Topology Authoring](./README.md)
- [Create Batch Builder](./create-batch-builder.md)
- [Verification](./verification.md)
