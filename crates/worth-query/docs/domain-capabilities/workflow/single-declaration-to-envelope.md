# Single Declaration To Envelope

## What This Workflow Is

This workflow is the shortest honest path from one declaration input to one
public envelope artifact.

Use it when your app already knows the declaration it wants to run and you want
Query to own the declaration-entry pipeline through the envelope ceiling.

## Why You Use It

- keep one declaration run on the canonical declaration-entry path
- get one public envelope artifact instead of stitching route, receipt, and
  envelope steps together yourself
- preserve typed non-success posture if the run defers, denies, or fails
- keep a clean handoff into later signal, continuation, or inspection work

## Stable Entry Points

- `orchestrate_declaration_entry(...)`
- `orchestrate_declaration_entry_outcome(...)`
- `orchestrate_declaration_entry_checked(...)`
- `orchestrate_declaration_entry_proof(...)`
- `declare_review_progress_describe_plan_receipt_and_envelope(...)`

## Core Mental Model

Think of this as the default single-declaration lane:

1. your app assembles one declaration input
2. Query lowers it through declaration entry
3. Query stops at one envelope artifact or one typed non-success result

This is the right lane when Query already has the declaration meaning and your
main question is "carry this through the public declaration pipeline."

## How It Executes

The canonical sequence is:

1. canonical declaration
2. legality
3. progression
4. foundational evidence
5. route plan
6. receipt
7. envelope

The ordinary lane gives you the compact result. The checked and proof lanes
keep more retained stop and transcript context.

## Small Example

```rust
let envelope = handle.orchestrate_declaration_entry(
    geometry_session.prepare_preview_for_active_face_selection()?,
)?;
```

Use this when one declaration stands on its own and you want the public
crossing artifact with the fewest moving parts.

## Real Example

```rust
let proof = handle.orchestrate_declaration_entry_proof(
    geometry_session.attach_material_for_active_face_selection()?,
);

let _ = proof.plan();
let _ = proof.outcome();
let _ = proof.step_records();
let _ = proof.orchestration_digest();
```

Use the proof-visible lane when you need to know exactly where the declaration
run stopped or what it retained on success.

## How It Relates To Other Features

- [Typed Binding Pipeline](../typed-binding-pipeline.md) is the next step when
  later work needs Query to bind a retained artifact into the next explicit
  input.
- [Declaration Boundary Envelopes](../declaration-boundary-envelopes.md) go
  deeper on what the envelope artifact preserves.
- [Signal Compatibility Orchestration](../signal-compatibility-orchestration.md)
  is the next step when the envelope needs a signal-facing classification.
- [Continuation Pipeline](../continuation-pipeline.md) is the next step when
  the envelope needs explicit continuation preparation or execution.

## Inspection And Debugging

Use the checked or proof lane when you need:

- the exact stop stage
- the retained orchestration plan
- route, receipt, or envelope aspect publication
- materialization or cost posture

Use [Stop To Recovery](./stop-to-recovery.md) when the next question is repair,
not inspection.

## Anti-Patterns

- using this lane when you do not yet know the declaration input
- reopening route or receipt steps by hand when the compact orchestration lane
  already matches the job
- treating a successful envelope as proof that signal or continuation work
  already happened

## Current Limits

- this workflow stops at the envelope ceiling
- declaration-scoped contribution authoring belongs on the separate
  contribution-composed lane
- target resolution still belongs to your app or to the typed binding pipeline,
  not to declaration entry orchestration

## Related Docs

- [Declaration Entry Orchestration](../declaration-entry-orchestration.md)
- [Declaration Boundary Envelopes](../declaration-boundary-envelopes.md)
- [Retained Artifact To Next Step](./retained-artifact-to-next-step.md)
