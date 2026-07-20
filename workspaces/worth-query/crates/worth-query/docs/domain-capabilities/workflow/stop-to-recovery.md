# Stop To Recovery

## What This Workflow Is

This workflow starts when a Query run did not finish and your next question is
"what do I do now?"

Use it when you need one typed repair answer instead of ad hoc stop handling in
each feature family.

## Why You Use It

- keep ordinary, checked, and proof-visible stops on one recovery surface
- preserve whether the stop came from declaration entry, signal,
  continuation, contribution-composed, or grouped work
- keep stale basis, basis mismatch, wrong world, wrong handle, unsupported,
  and manual-inspection posture distinct
- choose a next action without rereading lower artifacts directly

## Stable Entry Points

- `recover_from_outcome(...)`
- `recover_from_declaration_entry_checked(...)`
- `recover_from_declaration_entry_proof(...)`
- `recover_from_signal_compatibility_checked(...)`
- `recover_from_signal_compatibility_proof(...)`
- `recover_from_prepared_continuation_checked(...)`
- `recover_from_prepared_continuation_proof(...)`
- `recover_from_continuation_execution_checked(...)`
- `recover_from_continuation_execution_proof(...)`
- `recover_from_contribution_composed_checked(...)`
- `recover_from_contribution_composed_proof(...)`
- `recover_from_grouped_orchestration_checked(...)`
- `recover_from_grouped_orchestration_proof(...)`

## Core Mental Model

Recovery is not a retry engine. It is a typed next-step answer built from the
same stop proof you already have.

The shortest decision path is:

1. ask which feature family stopped
2. ask what kind of stop it was
3. ask who owns the fix
4. ask what action Query recommends next

## How It Executes

Recovery accepts one of three inputs:

1. an ordinary outcome
2. a checked result
3. a proof-visible transcript

It returns one recovery brief that preserves:

- stop family
- stop kind
- authority surface
- recommended action
- basis posture
- aspect posture
- evidence strength

Checked and proof entry points keep stronger source-family context when it
exists.

## Small Example

```rust
let outcome = handle.orchestrate_declaration_with_contributions_outcome(input);

if let Some(recovery) = handle.recover_from_outcome(&outcome) {
    let _ = recovery.recommended_action();
    let _ = recovery.authority_surface();
}
```

Use this when you are already on the compact ordinary lane and just need the
next step.

## Real Example

```rust
let proof = handle.orchestrate_grouped_proof(declaration);

let recovery = handle
    .recover_from_grouped_orchestration_proof(proof)
    .expect("grouped stop should yield recovery");

let _ = recovery.source_family();
let _ = recovery.recommended_action();
let _ = recovery.explanation().grouped_member_context();
```

Use the stronger grouped checked/proof recovery lane when one neighborhood
member stopped and you need the retained member-local context.

## How It Relates To Other Features

- [Recovery Boundary](../recovery-boundary.md) is the main feature reference
  for recovery types and request meanings.
- [Ordinary Outcomes](../ordinary-outcomes.md) are the compact lane recovery
  often starts from.
- [Grouped Neighborhood Workflow](./grouped-neighborhood-workflow.md) is the
  most common workflow that needs member-local grouped recovery context.

## Inspection And Debugging

Use the recovery brief when you need:

- `stop_family()`
- `stop_kind()`
- `authority_surface()`
- `recommended_action()`

Use the explanation when you need:

- `basis_posture()`
- `aspect_posture()`
- `evidence_strength()`
- `grouped_member_context()`
- `contribution_intent_descriptor()`

## Anti-Patterns

- flattening every stop into one generic "retry later" branch
- treating recovery as if it reruns work for you
- using the compact ordinary recovery lane when you actually need grouped,
  signal, or continuation proof context

## Current Limits

- recovery recommends the next action but does not execute it
- grouped routes, receipts, envelopes, and grouped contributions are still
  primarily inspection surfaces today; grouped orchestration is the richest
  grouped recovery lane

## Related Docs

- [Recovery Boundary](../recovery-boundary.md)
- [Ordinary Outcomes](../ordinary-outcomes.md)
- [Grouped Authoring](../grouped-authoring.md)
