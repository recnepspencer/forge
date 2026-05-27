# Continuation Pipeline

## What This Feature Is

The continuation pipeline is the Query-owned surface that turns a retained
continuation binding result into one prepared continuation artifact, and then
optionally executes that prepared artifact through the next lower continuation
boundary.

Use it when you already have one of these:

- a retained declaration envelope
- a continuation binding request from current context
- a prepared continuation artifact that is ready for explicit execution

This feature solves the next step after Phase 25 binding. Binding answers
"which continuation input is the right one?" The continuation pipeline answers
"what can Query prepare from that input, and what can it execute explicitly
without lying about basis, world, or runtime posture?"

## Why You Use It

- prepare continuation without rebuilding bridge, basis, and workspace
  handoff glue yourself
- keep prepared and executed continuation as separate typed states
- preserve wrong-world, wrong-handle, stale, authority-mismatch, and
  basis-mismatch posture as typed outcomes
- inspect which bridge continuation family, truth context, workspace contract,
  and runtime contract Query selected
- keep concise `..._outcome(...)` calls on the same shared ordinary outcome
  vocabulary as the rest of declaration entry

## Stable Entry Points

Prepared/executed continuation types:

- `ForgeQueryPreparedContinuation`
- `ForgeQueryPreparedContinuationChecked`
- `ForgeQueryPreparedContinuationTranscript`
- `ForgeQueryPreparedContinuationOutcome`
- `ForgeQueryContinuationExecution`
- `ForgeQueryContinuationExecutionChecked`
- `ForgeQueryContinuationExecutionTranscript`
- `ForgeQueryContinuationExecutionOutcome`

Prepared continuation contract vocabulary:

- `ForgeQueryPreparedContinuationFamily`
- `ForgeQueryContinuationTruthContext`
- `ForgeQueryContinuationBasisPosture`
- `ForgeQueryContinuationWorkspaceContract`
- `ForgeQueryContinuationRuntimeContract`
- `ForgeQueryPreparedContinuationExecutionMode`
- `ForgeQueryPreparedContinuationSignalPosture`

Request families:

- `ForgeQueryPreparedContinuationRequest`
- `ForgeQueryExecutePreparedContinuationRequest`

The exported execution request type exists for continuation-pipeline plumbing,
but the public admitted-handle execution lane accepts the prepared artifact
directly:

- `execute_prepared_continuation(prepared)`

Admitted-handle preparation entry points:

- `prepare_continuation_from_target(...)`
- `prepare_continuation_from_target_outcome(...)`
- `prepare_continuation_from_target_checked(...)`
- `prepare_continuation_from_target_proof(...)`
- `prepare_continuation_from_context(...)`
- `prepare_continuation_from_context_outcome(...)`
- `prepare_continuation_from_context_checked(...)`
- `prepare_continuation_from_context_proof(...)`

Admitted-handle execution entry points:

- `execute_prepared_continuation(...)`
- `execute_prepared_continuation_outcome(...)`
- `execute_prepared_continuation_checked(...)`
- `execute_prepared_continuation_proof(...)`

Good to know:

- preparation consumes the Phase 25 binding pipeline
- execution consumes only a prepared continuation artifact
- the concise `..._outcome(...)` lane reuses `ForgeQueryOrdinaryOutcome<T>`
- this first shipped slice prepares and executes bridge-backed continuation
  families only

## Core Mental Model

Think about the continuation pipeline as two separate boundaries:

1. prepare continuation
2. execute prepared continuation

Preparation is where Query freezes the retained continuation truth it already
knows:

- which bridge continuation family is in play
- which truth context is in play
- which basis families are required
- which workspace and runtime contract later execution would need
- whether signal posture is compatible, deferred, denied, or failed

Execution is a separate step that consumes that proof-bearing prepared artifact
and checks that it still belongs to the current admitted handle and operating
world.

The important rule is:

- prepared does not mean executed

If you only prepared continuation, Query has not claimed:

- runtime side effects already happened
- workspace mutations already happened
- lower bridge work already ran
- signal execution already ran

## How It Executes

The preparation path is:

1. bind continuation from a retained target or current context
2. recover the retained envelope-backed bridge subject
3. evaluate target-specific signal compatibility
4. route retained bridge continuation
5. derive the prepared continuation contract
6. return a prepared or typed non-success outcome

The execution path is:

1. accept one `ForgeQueryPreparedContinuation`
2. verify handle alignment
3. verify admitted-world alignment
4. derive one execution digest and execution artifact
5. return executed or typed non-success outcome

Preparation does not build a second planning system. It reuses:

- the Phase 25 typed binding pipeline
- retained bridge continuation routing truth
- retained signal compatibility truth

Execution does not accept raw ids, envelopes, or ad hoc bags. It accepts the
prepared artifact directly on the public lane so the type system keeps the
ordering honest.

## Small Example

```rust
let envelope = handle.envelope_routes_from_progressed(
    handle.declare_review_and_progress(
        geometry_session.publish_boundary_change_for_active_face()?,
    )?,
)?;

let prepared = match handle.prepare_continuation_from_target(
    ForgeQueryResolveContinuationFromTargetRequest::new(
        envelope,
        PublishBoundaryChange::aspect_contract(),
    ),
) {
    ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
    other => panic!("unexpected continuation preparation outcome: {:?}", std::mem::discriminant(&other)),
};
```

This is the smallest honest example because it starts from retained envelope
truth and stops at preparation instead of implying later execution happened.

## Real Example

```rust
let envelope = handle.orchestrate_declaration_entry(
    geometry_session.prepare_preview_for_active_face_selection()?,
)?;

let prepared = match handle.prepare_continuation_from_target(
    ForgeQueryResolveContinuationFromTargetRequest::new(
        envelope,
        PreparePreviewForActiveFaceSelection::aspect_contract(),
    )
    .with_bridge_request(
        ForgeQueryDeclarationBridgeContinuationRequest::new(
            ForgeQueryDeclarationBridgeContinuationMode::RuntimeRoute,
            ForgeQueryDeclarationBridgeTruthContext::Current,
        ),
    ),
) {
    ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
    other => panic!("unexpected preparation outcome: {:?}", std::mem::discriminant(&other)),
};

assert_eq!(
    prepared.family(),
    ForgeQueryPreparedContinuationFamily::BridgeRuntimeRoute,
);
assert_eq!(
    prepared.truth_context(),
    ForgeQueryContinuationTruthContext::Current,
);
assert_eq!(
    prepared.runtime_contract(),
    ForgeQueryContinuationRuntimeContract::RuntimeRoute,
);
assert_eq!(
    prepared.workspace_contract(),
    ForgeQueryContinuationWorkspaceContract::RuntimeWorkspace,
);

let executed = match handle.execute_prepared_continuation(prepared) {
    ForgeQueryContinuationExecutionOutcome::Executed(executed) => executed,
    other => panic!("unexpected execution outcome: {:?}", std::mem::discriminant(&other)),
};

let _ = executed.bridge_binding_surface();
let _ = executed.signal_execution_family();
let _ = executed.execution_digest();
```

What this example is showing:

- the envelope is still the authoritative public declaration crossing artifact
- preparation freezes continuation-family, truth-context, basis, workspace,
  runtime, and signal posture on one retained artifact
- execution is explicit and only starts from that prepared artifact
- the final execution artifact still links back to prepared continuation truth

## How It Relates To Other Features

- [Typed Binding Pipeline](./typed-binding-pipeline.md) chooses or denies the
  continuation-ready input before preparation begins.
- [Ordinary Outcomes](./ordinary-outcomes.md) provide the compact public result
  lane for both preparation and execution.
- [Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
  remains the authority for which bridge continuation family and truth context
  Query selected.
- [Declaration Signal Compatibility](./declaration-signal-compatibility.md)
  remains the authority for derived-execution signal posture and basis-family
  requirements.
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
  is the signal-facing composition lane that can reuse retained signal truth
  and optionally stop at `Compatible` before continuation preparation begins.
- [Configured Domain Handles](./configured-domain-handles.md) remain the
  admitted-world boundary that preparation and execution both verify.

Use bridge routing when you want the retained bridge continuation artifact
directly. Use the continuation pipeline when you want Query to turn that
retained truth into one prepared continuation artifact and optional explicit
execution step. Use signal-compatibility orchestration when the public question
is still "compatible or prepared?" rather than "prepare or execute now."

## Inspection And Debugging

Use the prepared artifact when you need to inspect what Query froze before
execution:

- `family()`
- `truth_context()`
- `basis_posture()`
- `workspace_contract()`
- `runtime_contract()`
- `execution_mode()`
- `required_basis_families()`
- `signal_posture()`
- `signal_execution_family()`
- `signal_compatibility_digest()`
- `prepared_digest()`
- `bridge_routing()`
- `declaration_digest()`
- `progression_digest()`
- `route_plan_digest()`
- `receipt_digest()`
- `envelope_digest()`

Use the proof-visible lanes when you need to inspect why preparation or
execution stopped:

- `ForgeQueryPreparedContinuationTranscript::{request, outcome, witness_checks, narrowing_decisions, prepared_digest, linked_artifacts}`
- `ForgeQueryContinuationExecutionTranscript::{request, outcome, witness_checks, execution_digest, linked_artifacts}`

Use the ordinary lane when you want the compact public result but still need a
checked-topology link:

- `prepare_continuation_from_target_outcome(...)`
- `prepare_continuation_from_context_outcome(...)`
- `execute_prepared_continuation_outcome(...)`

## Anti-Patterns

- treating preparation as if it already executed lower bridge or runtime work
- rebuilding continuation-mode, truth-context, or basis rules by hand from the
  envelope instead of using the prepared artifact
- skipping the prepared artifact and trying to execute from raw envelopes or
  ids
- flattening `WrongWorld`, `WrongHandle`, `Stale`, `AuthorityMismatch`, and
  `BasisMismatch` into one generic continuation failure
- using this feature as if it replaced bridge routing or signal compatibility
  authority

## Current Limits

- the first shipped slice prepares and executes bridge-backed continuation
  families only
- signal posture is carried forward into the prepared artifact, but this phase
  does not execute Signal work
- preparation does not create a broad ambient runtime or workspace session by
  itself
- grouped or neighborhood continuation preparation is not part of this slice
- this feature does not replace the explicit lower-authority bridge and signal
  docs when you need their direct artifacts

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Typed Binding Pipeline](./typed-binding-pipeline.md)
- [Ordinary Outcomes](./ordinary-outcomes.md)
- [Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
- [Declaration Signal Compatibility](./declaration-signal-compatibility.md)
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Declaration Entry Orchestration](./declaration-entry-orchestration.md)
- [Domain Capabilities](./README.md)
