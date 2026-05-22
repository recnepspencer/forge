# Existing Truth

## What This Feature Is

Existing-truth surfaces are the typed runtime capabilities for working against
already authoritative entities or relations instead of creating new truth from
scratch. They cover:

- binding an existing authoritative target
- updating or deleting that target through the ordinary mutation lane
- retaining or backend-verifying assertions about current authoritative truth
- probing current authoritative values without mutating

This is one capability family, not four unrelated feature surfaces.

The probe and verified-existing convenience surfaces are also part of the
covered intent-admission story. They still belong to one existing-truth
capability family; they just delegate through the shared lattice before the
runtime executes the bridge-backed work.

## Why You Use It

- you need to mutate or inspect an already authoritative target
- you want the runtime to preserve binding evidence instead of flattening the
  operation into generic identity strings
- you need backend-verified preconditions before an update or delete executes
- you need current authoritative values as input without executing a mutation

## Stable Entry Points

- `workspace.bind_existing_entity(...)`
- `workspace.bind_existing_relation(...)`
- `workspace.update_existing(...)`
- `workspace.delete_existing(...)`
- `workspace.assert_existing(...)`
- `workspace.verify_existing(...)`
- `workspace.update_existing_verified(...)`
- `workspace.delete_existing_verified(...)`
- `workspace.probe_existing(...)`
- `runtime.probe_existing_intent(...)`
- `workspace.probe_existing_intent(...)`

## Core Mental Model

Existing truth starts with a typed binding, not a caller-owned identity story.
Once a target is bound, the runtime owns the canonical binding digest,
resolved target evidence, and resulting authority story.

There are three distinct lanes:

- ordinary existing-target mutation
- existing-target assertion and backend verification
- existing-target probing without mutation

Do not flatten them together. They answer different questions and produce
different evidence.

Good to know:

- ordinary update and delete existing-target mutation stays on the direct
  mutation surface
- verified existing-target mutation and probe execution are thin convenience
  wrappers over covered intent-admission families
- the binding and resulting evidence still belong to this capability, not to a
  separate intent feature

## Ordinary Existing-Target Mutation

Use the typed binding helpers first:

```rust
let existing_task = workspace
    .bind_existing_entity(
        ForgeQueryExistingEntityTarget::new("authority:task-1", "task-row-1")?
            .in_target_collection("Task")?,
    )?;

let task_receipt = workspace.update_existing(existing_task, |task| {
    task.aspect("title.value", "Updated title")
})?;

let existing_relation = workspace
    .bind_existing_relation(
        ForgeQueryExistingRelationTarget::new("authority:rel-7", "relation-row-7")?
            .in_target_collection("TaskRelation")?,
    )?;

let relation_receipt = workspace.delete_existing(existing_relation)?;
```

Admitted relation rewrites stay on that same surface instead of pretending an
update is a replacement:

```rust
let existing_relation = workspace
    .bind_existing_relation(
        ForgeQueryExistingRelationTarget::new("authority:rel-7", "relation-row-7")?
            .in_target_collection("TaskRelation")?,
    )?;

let relation_receipt = workspace.update_existing(existing_relation, |relation| {
    relation
        .aspect("kind.value", "blocks")
        .aspect("status.value", "closed")
})?;
```

## Assertion And Verification

Use `assert_existing(...)` when the caller wants to retain an authoritative
assertion receipt without asking the backend to prove current stored values.

Use `verify_existing(...)` when the backend must prove those asserted values
now and deny typed and early on mismatch or missing truth.

```rust
let retained = workspace.assert_existing(existing_task.clone(), |task| {
    task.aspect("title.value", "Updated title")
})?;

let verified = workspace.verify_existing(existing_task, |task| {
    task.aspect("title.value", "Updated title")
})?;
```

Verified existing-target mutation keeps that same target-first shape while
adding an immediate backend precondition:

```rust
let update_receipt = workspace.update_existing_verified(
    existing_task.clone(),
    |verify| verify.aspect("status.value", "open"),
    |update| update.aspect("status.value", "closed"),
)?;

let delete_receipt = workspace.delete_existing_verified(
    existing_task,
    |verify| verify.aspect("status.value", "closed"),
    |delete| delete.touch("status.value"),
)?;
```

The verified update stays an ordinary `update` mutation-family receipt. The
verified delete stays an ordinary `delete` mutation-family receipt. Both keep
backend-verified assertion evidence so downstream code can still explain why
the runtime was willing to mutate existing truth.

## Probing

Use `workspace.probe_existing(...)` when the caller needs current
authoritative values for a bound existing target without executing a mutation.

```rust
let binding = workspace.bind_existing_entity(
    ForgeQueryExistingEntityTarget::new(
        "authority:task-123",
        "Task:42",
    )?
    .in_target_collection("Task")?,
)?;

let probe = workspace.probe_existing(
    binding,
    ["identity.id", "title.value", "status.value"],
)?;
```

Typed access stays straightforward:

```rust
assert_eq!(
    probe.field("title.value").unwrap().value_json(),
    "\"Ship authority probe\""
);
```

Use probing when the domain needs current authoritative truth as an input.
Use assertion or verified mutation when the domain is staying in the mutation
lane.

When you need the explicit admitted proof chain, use the intent path directly:

```rust
let review = runtime.probe_existing_intent(request).review()?;
let admitted = review.admit()?;
let result = admitted.execute()?;

let trace = result.receipt().decision_trace_envelope();
let provenance = result.receipt().execution_provenance();
```

## Support And Denials

Before teaching backend-verified update, delete, or probe as ordinary
bridge-backed production lanes, read the support rows:

```rust
let support = workspace.public_authoritative_mutation_evidence_support();
let update_row = support
    .bridge_backed_verification_support_rows()
    .iter()
    .find(|row| {
        row.operation_family() == "update_existing_verified"
            && row.target_binding_family() == "direct_entity_identity"
    })
    .unwrap();
```

The same pattern applies for `delete_existing_verified` and `probe_existing`.

Fail-closed behavior matters here:

- unsupported backends deny typed and early
- missing asserted values deny typed and early
- mismatched asserted values deny typed and early
- preview-local execution does not pretend to mint authoritative verification

## How It Relates To Other Features

- Use [Writes and Intent Boundaries](../execution/writes-and-intents.md) for
  the broader authoritative mutation surface.
- Use [Intent Admission](../execution/intent-admission.md) for the shared
  review, admit, handoff, and provenance model behind the covered verified and
  probe lanes.
- Use [Projection Consumption](projection-consumption.md) when an existing-
  target receipt or probe must become typed consumed facts.
- Use [Inspection](inspection.md) when you need the retained explanation
  surface for the resulting receipts, bindings, or probes.
- Use [Support Matrix And Admission](../foundations/support-matrix-and-admission.md)
  when the runtime family posture itself is what you need to inspect first.


