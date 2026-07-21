# Installed Operation Aftermath

## What This Feature Is

Installed operation aftermath connects an executed workflow to its declared
exact inverse or compensation operation. Query admits the relationship only
when both operations belong to the same runtime, generation, and basis and the
candidate actually proves the installed postcondition.

Use this when an operation must undo exact prior truth, settle a business
obligation, restore an invariant, discard provisional work, or state honestly
that rebuild or manual recovery is required.

## Why You Use It

- Keep inverse and compensation behavior in installed operation semantics.
- Prevent a same-named or foreign operation from acting as recovery authority.
- Verify the real candidate effects and business postcondition before minting
  an aftermath relation.
- Preserve partial mutations and the underlying failure when recovery itself
  stops after causing effects.

## Stable Entry Points

Portable operation meaning uses:

- `WorthQueryOperationReversalContract`
- `WorthQueryAftermathPostcondition`
- `WorthQueryOperationFailureClass`

An executed workflow uses:

- `trace.aftermath_posture()`
- `trace.admit_aftermath(candidate)`
- `WorthQueryAftermathAdmission`
- `WorthQueryExactInverseCapability::execute_workflow(...)`
- `WorthQueryCompensationCapability::execute_workflow(...)`
- `WorthQueryAftermathRelationReceipt`

The candidate workflow executor implements:

- `prepare_aftermath_intent(...)`
- `verify_aftermath_postcondition(...)`

These hooks interpret domain meaning. Query still owns admission, execution,
effect-scope checks, proof binding, and the final relation receipt.

## Core Mental Model

Declaring an inverse is not proof that an inverse happened. The original trace
retains the exact effects and lineage that need an aftermath. Query then binds
one separately installed candidate operation and asks it to execute normally.

For an exact inverse, Query first proves that candidate mutation effects target
the same authoritative scope as the original effects. The domain evaluator
then proves restoration of prior truth. For compensation, the domain evaluator
proves the declared invariant or business postcondition.

Only after those checks pass does Query mint a proof-carrying relation between
the original trace and the new candidate trace.

## How It Executes

```text
complete the original effectful workflow
  -> inspect its installed aftermath posture
  -> bind the exact declared candidate operation
  -> admit runtime, generation, basis, operation, lowering, and postcondition
  -> let the candidate executor prepare normalized intent
  -> execute the candidate as a fresh bound workflow
  -> verify effect scope and the domain postcondition
  -> mint the aftermath relation receipt
```

If the candidate stops after causing effects, Query returns
`CandidateExecutionFailed` with exact `partial_effects()`, the original
`candidate_execution_stop()`, and `DomainRecoveryRequired` posture.

## Small Example

Inspect posture before choosing a candidate:

```rust
match original_trace.aftermath_posture() {
    domain::WorthQueryAftermathPosture::ExactInverse { operation, .. } => {
        inspect_declared_inverse(operation);
    }
    domain::WorthQueryAftermathPosture::Compensation { operation, .. } => {
        inspect_declared_compensation(operation);
    }
    domain::WorthQueryAftermathPosture::Irreversible => {
        require_manual_recovery();
    }
    _ => {}
}
```

The posture is descriptive. It does not execute or authorize a candidate.

## Real Example

Bind the candidate through the same operating world and preserve the typed
admission result:

```rust
let admission = original_trace.admit_aftermath(candidate_bound);

let capability = match admission {
    domain::WorthQueryAftermathAdmission::Compensation(capability) => capability,
    domain::WorthQueryAftermathAdmission::Denied {
        denial,
        posture,
        counters,
    } => return handle_aftermath_denial(denial, posture, counters),
    domain::WorthQueryAftermathAdmission::ExactInverse(_) => {
        return handle_unexpected_inverse();
    }
};

let executed = capability.execute_workflow(&mut workspace).unwrap();
let relation = executed.relation();

assert_ne!(
    relation.original_trace_identity(),
    relation.aftermath_execution_identity(),
);
```

The relation retains both operation and binding identities, the original
effect receipts, basis, runtime, generation, postcondition, exact counters,
and descriptive Foundational attachment.

## How It Relates To Other Features

- [Installed Operation Re-Execution And Replay](./installed-operation-reexecution-and-replay.md)
  uses the same normalized workflow execution path but has different authority.
- [Installed Operation Lineage And Promotion](./installed-operation-lineage-and-promotion.md)
  supplies the original lineage identity carried into aftermath inspection.
- Provisional discard is a separate effect-free path; it is not a cheap inverse.

## Inspection And Debugging

Inspect:

- `aftermath_posture()` before admission
- denial kind and exact counters
- `candidate_trace_identity()` after semantic verification failure
- `partial_effects()` and `candidate_execution_stop()` after candidate failure
- `recovery_posture()` before deciding the next repair action
- relation operation, binding, capability, effect-receipt, and basis identities

An exact inverse acting on another entity produces
`ExactInverseScopeMismatch`, keeps its mutation evidence, and mints no relation.
A false business postcondition similarly keeps the candidate trace and effects
but produces `PostconditionNotEstablished`.

## Anti-Patterns

- Treating a reversal declaration as proof that restoration occurred.
- Passing a caller-authored target instead of deriving intent from original
  effect evidence.
- Using a candidate from another runtime, generation, or basis.
- Returning `true` from the domain postcondition evaluator without inspecting
  the original evidence and candidate semantics.
- Dropping partial candidate effects because the recovery attempt failed.

## Current Limits

- Exact inverse and compensation require complete postcondition-bearing
  declarations. Older posture-only variants fail as `DeclarationIncomplete`.
- `Irreversible`, `RebuildRequired`, and `ProvisionalDiscard` do not produce an
  executable inverse or compensation capability.
- Domain-specific repair after `DomainRecoveryRequired` remains domain-owned;
  Query preserves the evidence but does not invent recovery policy.

## Related Docs

- [Runtime-Installed Domains And Operations](./runtime-installed-domains.md)
- [Installed Operation Re-Execution And Replay](./installed-operation-reexecution-and-replay.md)
- [Installed Operation Lineage And Promotion](./installed-operation-lineage-and-promotion.md)
- [Recovery Boundary](./recovery-boundary.md)
