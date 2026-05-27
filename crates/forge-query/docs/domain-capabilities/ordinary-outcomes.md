# Ordinary Outcomes

## What This Feature Is

Ordinary outcomes are the concise public result surface for the two places
where Query most often needs to stop and explain itself:

- declaration-entry orchestration
- typed binding
- continuation preparation and execution
- signal-compatibility orchestration

Use this feature when you want one compact outcome value that still keeps the
important non-success categories separate.

This is not a generic `Result<T, String>`, and it is not a second checked or
proof pipeline. It is a projection layer that gives ordinary callers one shared
typed vocabulary while still linking back to the checked topology underneath.

## Why You Use It

- keep ordinary handle calls concise without flattening real stop posture
- share one non-success vocabulary across binding, continuation, and
  orchestration
- get a machine-readable next step instead of parsing prose
- inspect which checked topology the ordinary outcome came from
- keep `Denied`, `Refused`, `Stale`, `RebindRequired`, `Unsupported`, and
  `Ambiguous` distinct

## Stable Entry Points

Core ordinary-outcome types:

- `ForgeQueryOrdinaryOutcome<T>`
- `ForgeQueryOrdinaryPosture`
- `ForgeQueryOrdinaryPostureKind`
- `ForgeQueryOrdinaryNextStep`
- `ForgeQueryOrdinaryCheckedTopology`
- `ForgeQueryOrdinaryBindingCheckedTopologyKind`

Admitted-handle orchestration entry points:

- `orchestrate_declaration_entry_outcome(...)`

Admitted-handle binding entry points:

- `bind_declaration_from_context_outcome(...)`
- `bind_route_request_from_context_outcome(...)`
- `bind_receipt_request_from_context_outcome(...)`
- `bind_envelope_request_from_context_outcome(...)`
- `bind_continuation_request_from_context_outcome(...)`
- `bind_route_from_target_outcome(...)`
- `bind_receipt_from_target_outcome(...)`
- `bind_envelope_from_target_outcome(...)`
- `bind_continuation_from_target_outcome(...)`

Admitted-handle continuation entry points:

- `prepare_continuation_from_target_outcome(...)`
- `prepare_continuation_from_context_outcome(...)`
- `execute_prepared_continuation_outcome(...)`

Admitted-handle signal-compatibility orchestration entry points:

- `orchestrate_signal_compatibility_outcome(...)`

## Core Mental Model

Think of ordinary outcomes as a public compatibility layer over stronger truth.

The checked and proof-visible lanes are still authoritative. Ordinary outcomes
exist so callers can write compact code without giving up the distinctions that
matter operationally.

The shape is:

- `Bound(T)` when the operation succeeded
- typed non-success variants when the operation stopped honestly

The important rule is:

- ordinary does not mean flattened

If Query refused to automate farther, the ordinary surface says `Refused`.
If the current world was wrong, it says `WrongWorld`.
If the caller needs to rebind, it says `RebindRequired`.
If the result is only unavailable or ambiguous, it says that directly.

## How It Executes

Query does not compute a second decision tree for ordinary outcomes.

Instead, ordinary outcomes are derived from:

- `ForgeQueryBindingChecked<T>` for typed binding
- `ForgeQueryPreparedContinuationChecked<D, I>` and
  `ForgeQueryContinuationExecutionChecked<D, I>` for continuation preparation
  and execution
- `ForgeQuerySignalCompatibilityOrchestrationChecked<D, I>` for signal-facing
  compatibility orchestration
- `ForgeQueryDeclarationEntryOrchestrationTerminalError<D, I>` for declaration-entry orchestration

That projection keeps one small shared posture surface:

- a category
- a human-readable reason
- a next-step hint
- a checked-topology link

The checked-topology link stays typed:

- orchestration outcomes expose stop stage, retained digest, and refusal class
- binding outcomes expose binding checked-topology kind plus linked retained
  artifacts
- signal-compatibility orchestration outcomes expose signal-orchestration
  checked-topology kind plus linked retained artifacts

## Small Example

```rust
let outcome = handle.bind_declaration_from_context_outcome(
    ForgeQueryDeclarationBindingRequest::<AttachFaceMaterial>::new(
        vec![],
        AttachFaceMaterial::aspect_contract(),
        vec![ForgeQueryBindingSourceKind::ExplicitSelection],
    ),
);

match outcome {
    ForgeQueryOrdinaryOutcome::Bound(declaration) => {
        let _ = declaration.declaration_digest();
    }
    ForgeQueryOrdinaryOutcome::Unavailable(posture) => {
        let _ = posture.reason();
        let _ = posture.next_step();
    }
    other => panic!("unexpected outcome kind: {:?}", std::mem::discriminant(&other)),
}
```

## Real Example

```rust
let outcome = handle.orchestrate_declaration_entry_outcome(
    geometry_session.publish_boundary_change_for_active_face()?,
);

match outcome {
    ForgeQueryOrdinaryOutcome::Bound(envelope) => {
        let _ = envelope.envelope_digest();
    }
    ForgeQueryOrdinaryOutcome::Refused(posture) => {
        let _ = posture.kind();
        let _ = posture.reason();
        let _ = posture.next_step();
        let _ = posture.checked_topology().orchestration_stop_stage();
        let _ = posture.checked_topology().orchestration_refusal_class();
    }
    ForgeQueryOrdinaryOutcome::Denied(posture) => {
        let _ = posture.checked_topology().orchestration_stop_stage();
    }
    other => {
        let _ = other;
    }
}
```

## How It Relates To Other Features

- [Configured Domain Handles](./configured-domain-handles.md) expose the
  admitted-handle ordinary entry points.
- [Typed Binding Pipeline](./typed-binding-pipeline.md) owns the checked and
  proof-visible binding outcomes that ordinary binding projects from.
- [Continuation Pipeline](./continuation-pipeline.md) owns the checked and
  proof-visible prepared/executed continuation outcomes that ordinary
  continuation projects from.
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
  owns the checked and proof-visible signal-facing compatibility outcomes that
  ordinary signal orchestration projects from.
- [Declaration Entry Orchestration](./declaration-entry-orchestration.md) owns
  the checked and proof-visible orchestration stop posture that ordinary
  orchestration projects from.

## Inspection And Debugging

Use `ForgeQueryOrdinaryPosture` when you need the compact public surface:

- `reason()`
- `kind()`
- `next_step()`
- `checked_topology()`

Use `ForgeQueryOrdinaryCheckedTopology` when you need to know what the ordinary
result maps back to:

- `orchestration_stop_stage()`
- `orchestration_retained_digest()`
- `orchestration_refusal_class()`
- `binding_kind()`
- `binding_linked_artifacts()`
- `signal_compatibility_orchestration_kind()`
- `signal_compatibility_orchestration_linked_artifacts()`

Binding linked artifacts can expose:

- declaration digest
- progression digest
- route-plan digest
- receipt digest
- envelope digest
- orchestration digest when one exists

## Anti-Patterns

- treating ordinary outcomes as if they replaced checked or proof-visible lanes
- collapsing `Refused` and `Denied` into the same app behavior
- treating `Stale` and `RebindRequired` as interchangeable
- parsing `reason()` when `kind()` or `next_step()` already carry the machine
  decision you need
- assuming ordinary binding, continuation, and orchestration are unrelated
  error vocabularies

## Current Limits

- ordinary outcomes are currently shipped for typed binding,
  continuation preparation/execution, signal-compatibility orchestration, and
  declaration-entry orchestration
- the ordinary surface is intentionally projection-only; it does not own new
  execution or binding logic
- ordinary outcomes do not expose full transcripts; switch to checked or
  proof-visible lanes when you need the full diagnostic surface

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Typed Binding Pipeline](./typed-binding-pipeline.md)
- [Continuation Pipeline](./continuation-pipeline.md)
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
- [Declaration Entry Orchestration](./declaration-entry-orchestration.md)
- [Domain Capabilities](./README.md)
