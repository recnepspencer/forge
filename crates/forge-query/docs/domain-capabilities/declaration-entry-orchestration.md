# Declaration Entry Orchestration

## What This Feature Is

Declaration entry orchestration is the generic Query front door for the current
declaration-entry pipeline.

Give an admitted configured domain handle one already-assembled declaration
intent, and Query will lower it through the locked public sequence up to the
current ceiling: envelope construction.

This feature does not resolve geometry targets for you, and it does not
continue into runtime execution, workspace entry, signal execution, or `9.3.7`
composition. It starts after your tool or session has already decided what the
user is trying to do.

## Why You Use It

- run one declaration-entry sequence without manually stitching phases together
- keep ordinary, checked, and proof-visible lanes on one canonical pipeline
- preserve deferred, denied, stale, rebind-required, failed, and refused
  posture as typed results
- inspect the exact stop boundary when automation cannot continue
- compare equivalent runs through stable orchestration and outcome digests

## Stable Entry Points

- `ForgeQueryAdmittedConfiguredDomainHandle::orchestrate_declaration_entry(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::orchestrate_declaration_entry_checked(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::orchestrate_declaration_entry_proof(...)`
- `ForgeQueryDeclarationEntryOrchestrationInput`
- `ForgeQueryDeclarationEntryOrchestrationPlan`
- `ForgeQueryDeclarationEntryOrchestrationOutcome`
- `ForgeQueryDeclarationEntryOrchestrationTranscript`
- `ForgeQueryDeclarationEntryOrchestrationExposureLevel`
- `ForgeQueryDeclarationEntryOrchestrationArtifactPolicy`
- `ForgeQueryDeclarationEntryOrchestrationStepRecord`
- `ForgeQueryDeclarationEntryOrchestrationStepDisposition`
- `ForgeQueryDeclarationEntryOrchestrationAutomationBoundary`
- `ForgeQueryDeclarationEntryOrchestrationAutomationStep`
- `ForgeQueryDeclarationEntryOrchestrationAutomationRefusal`
- `ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass`
- `ForgeQueryDeclarationEntryOrchestrationVerbInventory`

Good to know:

- `ForgeQueryDeclarationEntryOrchestrationChecked` is the checked-lane
  compatibility alias for `ForgeQueryDeclarationEntryOrchestrationOutcome`
- `ForgeQueryDeclarationEntryOrchestrationProof` is the proof-visible
  compatibility alias for `ForgeQueryDeclarationEntryOrchestrationTranscript`
- Query owns construction of orchestration inputs, plans, refusals, outcomes,
  and transcripts

## Core Mental Model

There is one generic orchestration feature and one public sequence:

1. admitted handle
2. canonical declaration
3. legality
4. progression
5. foundational evidence
6. route plan
7. receipt
8. envelope

The public grammar over that sequence is intentionally narrow:

- `orchestrate_declaration_entry(...)`
- `orchestrate_declaration_entry_checked(...)`
- `orchestrate_declaration_entry_proof(...)`

Those are not three helper stacks. They are three visibility levels over the
same run:

- ordinary: give me the envelope or one typed terminal error
- checked: give me the typed stop posture
- proof-visible: give me the same posture plus the exact automation transcript

The important system boundary is this:

- your session or tool resolves the user’s targets and assembles declaration
  intent
- Query lowers that intent through the retained declaration-entry pipeline

Do not teach your app team that Query is the target-resolution layer. It is
the orchestration layer after intent already exists.

## How It Executes

The current public ceiling is `EnvelopeCeiling`.

That means the orchestration plan may automate only these steps:

- `AdmittedHandle`
- `CanonicalDeclaration`
- `Legality`
- `Progression`
- `FoundationalEvidence`
- `RoutePlan`
- `Receipt`
- `Envelope`

Query derives one plan at the start of the run and then carries retained proof
forward step by step. The public surfaces do not recompute the declaration
meaning differently for ordinary, checked, and proof-visible lanes.

The plan exposes that sequencing law directly:

- `ceiling_stage()`
- `automation_boundary()`
- `automation_steps()`
- `explicit_caller_handoff_steps()`
- `orchestration_identity_digest()`

In the current shipped boundary, `explicit_caller_handoff_steps()` is still
empty because the public front door stops at envelope construction instead of
advertising a second automated continuation phase.

## Sequencing Law

The locked execution order is:

1. admitted handle
2. canonical declaration
3. legality
4. progression
5. foundational evidence
6. route plan
7. receipt
8. envelope

Public orchestration may not:

- skip directly to a later step
- invent a second planning abstraction
- widen into runtime, workspace, basis, continuation, or signal work
- imply that later execution happened because earlier preparation succeeded

Step records expose the real reached sequence through:

- `stage()`
- `automation_step()`
- `disposition()`
- `retained_digest()`
- `reason()`
- `is_reached()`
- `is_stop()`
- `is_terminal()`

## Where Automation Stops

The current front door stops in one of two broad ways.

First, the retained declaration-entry pipeline can land in an ordinary
non-success posture:

- `Deferred`
- `Denied`
- `Stale`
- `RebindRequired`
- `Failed`

Second, Query can refuse to automate farther even though the declaration-entry
story still has a meaningful typed stop boundary:

- `Refused`

That distinction matters.

`Deferred`, `Denied`, `Stale`, `RebindRequired`, and `Failed` describe what the
pipeline proved. `Refused` describes where Query intentionally stopped
automation.

Use `is_automation_refusal()` when you only need the quick branch. Use the
typed refusal accessors when you need the exact reason:

- `refusal_class()`
- `automation_refusal_class()`
- `stop_stage()`
- `reason()`
- `retained_digest()`
- `orchestration_identity_digest()`
- `automation_boundary()`

The sequencing-specific refusal classes are:

- `ExplicitIntentRequired`
- `ExpensiveAutomationForbidden`
- `AuthorityTransitionRequired`
- `PreparedButNotExecuted`
- `UnsupportedAutomation`
- `StrongerProofRequired`

## Boundary Honesty

The transcript and checked outcome must report the farthest crossed public
boundary honestly.

Examples:

- if route posture lowers into a deferred receipt before stopping, the stop
  stage is `ReceiptIssued`
- if receipt kind support is the blocker, the stop stage is `ReceiptIssued`,
  not `RoutePlanned`
- if the route phase truly stopped before any receipt crossing, the stop stage
  remains `RoutePlanned`

This is why proof-visible orchestration is more than debug logging. It shows
the exact automation story Query is willing to claim.

## Public Grammar

The grammar is fixed for this feature:

- base verb: `orchestrate_declaration_entry`
- checked suffix: `_checked`
- proof-visible suffix: `_proof`

Do not introduce parallel names like:

- `_transcript`
- `_trace`
- `_debug`
- family-specific orchestration aliases

Inspect the locked grammar directly when you need it:

```rust
let inventory = ForgeQueryDeclarationEntryOrchestrationVerbInventory::current();

for verb in inventory.verbs() {
    let _ = verb.public_name();
    let _ = verb.family();
    let _ = verb.exposure_level();
    let _ = verb.ceiling();
    let _ = verb.canonical_base_name();
}
```

## Small Example

This example shows the intended boundary: the editing session resolved the
targets already, then Query orchestrated the declaration-entry pipeline.

```rust
let trim_request = geometry_session.trim_segment_at_active_intersection()?;
let envelope = handle.orchestrate_declaration_entry(trim_request)?;

let _ = envelope.envelope_digest();
```

## Real Example

Use the checked lane when the UI or collaboration workflow needs to know
exactly why automation stopped.

```rust
let trim_request = geometry_session
    .selection()
    .single_segment()
    .trim_at_highlighted_intersection()?;

match handle.orchestrate_declaration_entry_checked(trim_request) {
    ForgeQueryDeclarationEntryOrchestrationOutcome::Enveloped(envelope) => {
        let _ = envelope.envelope_digest();
    }
    ForgeQueryDeclarationEntryOrchestrationOutcome::Refused(refusal) => {
        let _ = refusal.automation_refusal_class();
        let _ = refusal.stop_stage();
        let _ = refusal.reason();
    }
    other => {
        let _ = other.stop_stage();
        let _ = other.outcome_identity_digest();
    }
}
```

Use the proof-visible lane when you need the exact reached sequence:

```rust
let trim_request = geometry_session.trim_segment_at_active_intersection()?;
let transcript = handle.orchestrate_declaration_entry_proof(trim_request);

let _ = transcript.plan().automation_boundary();
let _ = transcript.plan().automation_steps();
let _ = transcript.outcome().stop_stage();

for record in transcript.step_records() {
    let _ = record.stage();
    let _ = record.automation_step();
    let _ = record.disposition();
    let _ = record.retained_digest();
    let _ = record.reason();
}
```

## How It Relates To Other Features

- Use [Configured Domain Handles](./configured-domain-handles.md) to create the
  admitted world that owns this orchestration run.
- Use [Canonical Domain Declarations](./canonical-domain-declarations.md) when
  you need to author or inspect declaration intent without lowering it.
- Use [Declaration Progression](./declaration-progression.md) when you want the
  proof-bearing progression artifact directly.
- Use [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
  when you already have retained receipt-backed crossing truth and want the
  envelope artifact directly.
- Use [Declaration Entry Inspection](./declaration-entry-inspection.md) when
  you need a read surface over retained seam artifacts after a run.
- Use [Declaration Entry Readiness](./declaration-entry-readiness.md) when you
  need family-level seam support posture instead of one concrete run.

## Inspection And Debugging

Use the checked lane when you need:

- the typed stop posture
- the exact stop stage
- the canonical outcome identity
- the exact automation refusal class when the stop is `Refused`

Use the proof-visible lane when you need:

- the declared automation boundary
- the exact sequence of reached stages
- the farthest crossed public boundary
- retained digests attached to each reached or stopped stage
- parity evidence across equivalent runs

Useful accessors:

- `plan().orchestration_identity_digest()`
- `outcome().outcome_identity_digest()`
- `plan().automation_boundary()`
- `plan().automation_steps()`
- `step_records()`
- `step_record.automation_step()`

## Anti-Patterns

- treating ordinary orchestration as “best effort farther lowering”
- assuming a successful envelope implies later runtime or signal work is ready
- passing raw geometry IDs as the main public mental model for app usage docs
- teaching that Query resolves user selections or highlights for you
- flattening `Stale`, `RebindRequired`, `Denied`, `Failed`, and `Refused` into
  one generic error path
- treating proof-visible transcripts as the same thing as retained seam
  inspection artifacts
- inventing parallel orchestration verb families around the locked trio

## Current Limits

- success still stops at the envelope ceiling
- the current front door does not automate relational routing, bridge routing,
  signal compatibility, runtime entry, workspace entry, or continuation
  preparation
- proof-visible is intentionally the only transcript-bearing public lane
- the automation boundary is currently single-valued: `EnvelopeCeiling`
- the grammar is intentionally generic only; family-specific orchestration
  aliases are out of scope here

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Canonical Domain Declarations](./canonical-domain-declarations.md)
- [Declaration Progression](./declaration-progression.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Declaration Entry Inspection](./declaration-entry-inspection.md)
- [Declaration Entry Readiness](./declaration-entry-readiness.md)
