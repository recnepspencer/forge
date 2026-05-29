# `worth-topo` Phase 4 Runtime Invariant Closeout

This note closes the specific runtime-invariant rollout hard break that stayed
alive downstream after `worth-schema` removed its public rollout API.

## Final Verdict

- `worth-schema` keeps no public runtime invariant rollout surface.
- `worth-topo` no longer imports or compares against a schema-declared runtime
  invariant plan.
- public/runtime-facing invariant installation is expressed through Query-owned
  registration inputs:
  - `forge_query::facade::ForgeQueryRuntime::builder().custom_invariant(...)`
  - `forge_query::facade::ForgeQueryRuntime::builder().invariant_catalog(...)`
  - `forge_query::facade::ForgeQueryRuntime::builder().register_invariant(...)`
  - `forge_query::facade::ForgeQueryRuntime::builder().invariant_registration_artifact(...)`
- the Worth-owned public input pack for this slice is now
  `worth_topo::runtime_support::milestone_one_invariant_registrations()`

## What Was Deleted

- all downstream `bootstrap_runtime_invariant_plan` imports and uses in the
  `worth-topo` reference-integrity slice
- the parity test that compared the runtime registration pack against a removed
  schema declaration
- test imports of removed schema verification and authority-result surfaces in
  `validation/reference_integrity/tests`

## Public Boundary Decision

`worth-topo` no longer exports the low-level relational runtime-builder helpers
that existed only to mirror the removed schema rollout story:

- `build_milestone_one_runtime`
- `configure_milestone_one_runtime_builder`
- `milestone_one_runtime_builder`
- `MilestoneOneRuntimeSetupError`

Those helpers remain crate-private support code for internal certification and
test scaffolding. The public Worth runtime-support seam is:

- `worth_topo::runtime_support::build_runtime_bridge`
- `worth_topo::runtime_support::TopologyRuntimeBinding`
- `worth_topo::runtime_support::TopologyRuntimeSchemaAdapter`
- `worth_topo::runtime_support::TopologyRuntimeWriteAuthority`
- `worth_topo::runtime_support::topology_runtime`
- `worth_topo::runtime_support::milestone_one_invariant_registrations`

## Query-Owned Story

The intended runtime registration shape is:

```rust
use forge_query::facade::ForgeQueryRuntime;
use worth_topo::runtime_support::{
    build_runtime_bridge,
    milestone_one_invariant_registrations,
    TopologyRuntimeSchemaAdapter,
    TopologyRuntimeWriteAuthority,
};

let registrations = milestone_one_invariant_registrations()?;

let builder = ForgeQueryRuntime::builder()
    .runtime_bridge(build_runtime_bridge(binding.clone())?)
    .schema_adapter(TopologyRuntimeSchemaAdapter)
    .write_authority(TopologyRuntimeWriteAuthority::new(binding));

let builder = registrations.into_iter().fold(builder, |builder, registration| {
    builder.custom_invariant(registration)
});

let runtime = builder.build_backend_from_parts().build()?;
```

This keeps the public registration vocabulary on the Query-owned side while
still letting Worth author its own invariant semantics.
