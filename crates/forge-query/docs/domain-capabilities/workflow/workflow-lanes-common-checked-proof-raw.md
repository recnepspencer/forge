# Workflow Lanes: Common, Checked, Proof, And Raw

## What This Feature Is

Workflow contributions are exposed through a one-lane-at-a-time degradation
story:

- common lane
- checked lane
- proof lane
- raw materializer lane

## Why You Use It

- you want to stay on the ordinary Query path by default
- you need stronger inspection or denial detail for tooling
- you are writing certification or framework code and need direct proof or raw
  materializers

## Stable Entry Points

Common lane:

- `forge_query_domain(...).for_intent(...).inspects_query_preview(...).because(...).materialize()`
- `forge_query_domain(...).for_intent(...).plans_preview_mutation(...).because(...).materialize()`

Checked lane:

- `.try_materialize()`

Proof lane:

- `ForgeQueryWorkflowContributionAuthoring::preview_only_query_inspection(...)`
- `ForgeQueryWorkflowContributionAuthoring::promotion_eligible_mutation_lowering(...)`
- runtime-preflight authoring variants

Raw lane:

- `materialize_query_preview_workflow_artifact(...)`
- `materialize_query_workflow_declaration(...)`

## Core Mental Model

The common lane is for product code.

The checked lane is for code that still wants the public shape but needs a
typed `TransitionOutcome`.

The proof lane is for advanced integration that wants explicit requested,
eligible, admitted, and ready forms.

The raw lane is for materializer-level certification and infrastructure work.

## How It Executes

The same workflow meaning should survive each lane. The difference is how much
of the lifecycle you hold directly.

## Small Example

```rust
let plan = forge_query_domain("worth.spatial")
    .for_intent(&declaration)
    .plans_preview_mutation("topology.preview_mutation", "preview-session:77")
    .because("the edge split should be planned before promotion")
    .materialize()?;
```

## Real Example

```rust
let checked = forge_query_domain("worth.spatial")
    .for_intent(&declaration)
    .plans_preview_mutation("topology.preview_mutation", "preview-session:77")
    .because("the edge split should be planned before promotion")
    .try_materialize();

let outcome = checked.into_transition_outcome();
```

That is the right move when you need denied, stale, or rebind-required metadata
without leaving the public lane entirely.

## How It Relates To Other Features

- [Preview Inspection And Mutation Planning](./preview-inspection-and-mutation-planning.md)
  is the ordinary entry story
- [Runtime-Preflight Workflow Contributions](./runtime-preflight-workflow-contributions.md)
  is the sharper runtime-bound proof story

## Inspection And Debugging

- the checked lane keeps category, target kind, posture, and denial visible
- the proof lane is the right home for certification code that must inspect
  lifecycle progression directly

## Anti-Patterns

- dropping straight from common helpers to raw materializers in ordinary app
  code
- teaching proof-lane APIs as the default developer surface
- assuming checked and proof lanes are different semantic systems

## Current Limits

- some workflow follow-on steps remain intentionally lower-lane
- the common lane does not try to expose every lower-runtime workflow noun

## Related Docs

- [Preview Inspection And Mutation Planning](./preview-inspection-and-mutation-planning.md)
- [Runtime-Preflight Workflow Contributions](./runtime-preflight-workflow-contributions.md)
