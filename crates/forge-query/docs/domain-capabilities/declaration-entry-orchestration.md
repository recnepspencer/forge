# Declaration Entry Orchestration

## What This Feature Is

Declaration entry orchestration is the Query-owned front door for the retained
declaration-entry pipeline.

It lets an admitted configured domain handle take one declaration input and run
the canonical declaration-entry sequence through the current public ceiling:
envelope construction.

This is not runtime continuation, signal execution, or contribution automation.
It is the current orchestration surface over the already-shipped
declaration-entry artifacts.

## Why You Use It

- state declaration intent once through one admitted handle
- get the canonical retained declaration-entry artifact on success
- keep deferred, denied, stale, rebind-required, failed, and refused posture
  typed instead of flattening them into one generic error
- inspect the exact stop stage through the checked and proof-visible siblings
- use one stable front door instead of manually stitching together declaration,
  legality, progression, foundational description, route planning, receipt, and
  envelope calls

## Stable Entry Points

- `ForgeQueryAdmittedConfiguredDomainHandle::orchestrate_declaration_entry(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::orchestrate_declaration_entry_checked(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::orchestrate_declaration_entry_proof(...)`
- `ForgeQueryDeclarationEntryOrchestrationChecked`
- `ForgeQueryDeclarationEntryOrchestrationDeferred`
- `ForgeQueryDeclarationEntryOrchestrationDenied`
- `ForgeQueryDeclarationEntryOrchestrationStale`
- `ForgeQueryDeclarationEntryOrchestrationRebindRequired`
- `ForgeQueryDeclarationEntryOrchestrationFailed`
- `ForgeQueryDeclarationEntryOrchestrationRefusal`
- `ForgeQueryDeclarationEntryOrchestrationRefusalClass`
- `ForgeQueryDeclarationEntryOrchestrationTerminalError`
- `ForgeQueryDeclarationEntryOrchestrationProof`
- `ForgeQueryDeclarationEntryOrchestrationStage`
- `ForgeQueryDeclarationEntryOrchestrationStageRecord`

## API Reference

Ordinary orchestration:

- `orchestrate_declaration_entry(input) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEntryOrchestrationTerminalError<D, I>>`

Checked orchestration:

- `orchestrate_declaration_entry_checked(input) -> ForgeQueryDeclarationEntryOrchestrationChecked<D, I>`

Proof-visible orchestration:

- `orchestrate_declaration_entry_proof(input) -> ForgeQueryDeclarationEntryOrchestrationProof<D, I>`

Checked outcome variants:

- `ForgeQueryDeclarationEntryOrchestrationChecked::Enveloped(...)`
- `ForgeQueryDeclarationEntryOrchestrationChecked::Deferred(...)`
- `ForgeQueryDeclarationEntryOrchestrationChecked::Denied(...)`
- `ForgeQueryDeclarationEntryOrchestrationChecked::Stale(...)`
- `ForgeQueryDeclarationEntryOrchestrationChecked::RebindRequired(...)`
- `ForgeQueryDeclarationEntryOrchestrationChecked::Failed(...)`
- `ForgeQueryDeclarationEntryOrchestrationChecked::Refused(...)`

Terminal error variants:

- `ForgeQueryDeclarationEntryOrchestrationTerminalError::Deferred(...)`
- `ForgeQueryDeclarationEntryOrchestrationTerminalError::Denied(...)`
- `ForgeQueryDeclarationEntryOrchestrationTerminalError::Stale(...)`
- `ForgeQueryDeclarationEntryOrchestrationTerminalError::RebindRequired(...)`
- `ForgeQueryDeclarationEntryOrchestrationTerminalError::Failed(...)`
- `ForgeQueryDeclarationEntryOrchestrationTerminalError::Refused(...)`

Refusal classes:

- `UnsupportedAutomation`
- `ExplicitIntentRequired`
- `StrongerProofRequired`
- `AuthorityTransitionRequired`
- `ExpensiveWorkNotAdmittedByDefault`
- `PreparedButNotExecutedContinuation`

Proof-visible accessors:

- `outcome()`
- `stage_records()`
- `orchestration_digest()`

Stage-record accessors:

- `stage()`
- `is_reached()`
- `is_stop()`
- `retained_digest()`

## Core Mental Model

Declaration entry orchestration is one canonical pipeline with three exposure
levels.

The pipeline is:

1. admitted operating world
2. declaration review
3. legality
4. proof-bearing progression
5. foundational description
6. route plan
7. receipt
8. envelope

The three exposures are:

- ordinary: return the retained envelope on success or one typed terminal error
- checked: return the exact stop posture as one typed outcome family
- proof-visible: return the same checked outcome plus exact stage records for
  the canonical lowering path

Good to know:

- orchestration does not bypass the lower declaration-entry surfaces
- orchestration does not re-derive declaration meaning from ambient context
- orchestration stops at the envelope ceiling in the current public phase
- orchestration does not imply signal compatibility, continuation preparation,
  contribution composition, or later execution happened

## How It Executes

For the current public ceiling, Query lowers through the existing
declaration-entry surfaces in order:

- declaration admission and review
- legality review
- proof-bearing progression
- foundational description
- route planning
- receipt issuance
- envelope construction

If one of those phases yields a non-success outcome, orchestration preserves
that distinction instead of forcing it into a single `Err` shape.

If the ordinary surface cannot honestly continue, it returns a typed terminal
error instead of guessing, silently preparing later work, or implying a later
phase already ran.

## Small Example

```rust
let envelope = handle.orchestrate_declaration_entry(
    SplitEdgeAtMidpoint { edge_ref: "edge:42" },
)?;

let _ = envelope.envelope_digest();
```

## Real Example

```rust
match handle.orchestrate_declaration_entry_checked(
    SplitEdgeAtMidpoint { edge_ref: "edge:42" },
) {
    ForgeQueryDeclarationEntryOrchestrationChecked::Enveloped(envelope) => {
        let _ = envelope.envelope_digest();
    }
    ForgeQueryDeclarationEntryOrchestrationChecked::Deferred(outcome) => {
        let _ = outcome.stop_stage();
        let _ = outcome.reason();
    }
    ForgeQueryDeclarationEntryOrchestrationChecked::Denied(outcome) => {
        let _ = outcome.stop_stage();
        let _ = outcome.reason();
    }
    ForgeQueryDeclarationEntryOrchestrationChecked::Stale(outcome) => {
        let _ = outcome.stop_stage();
    }
    ForgeQueryDeclarationEntryOrchestrationChecked::RebindRequired(outcome) => {
        let _ = outcome.stop_stage();
    }
    ForgeQueryDeclarationEntryOrchestrationChecked::Failed(outcome) => {
        let _ = outcome.stop_stage();
    }
    ForgeQueryDeclarationEntryOrchestrationChecked::Refused(outcome) => {
        let _ = outcome.refusal_class();
        let _ = outcome.stop_stage();
        let _ = outcome.reason();
    }
}
```

Proof-visible usage:

```rust
let proof = handle.orchestrate_declaration_entry_proof(
    SplitEdgeAtMidpoint { edge_ref: "edge:42" },
);

let _ = proof.outcome();
let _ = proof.stage_records();
let _ = proof.orchestration_digest();
```

## How It Relates To Other Features

- Use [Canonical Domain Declarations](./canonical-domain-declarations.md) when
  you need to author declarations without lowering them.
- Use [Declaration Progression](./declaration-progression.md) when you need the
  proof-bearing progression artifact directly.
- Use [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
  when you want the retained crossing artifact directly without the orchestration
  front door.
- Use [Declaration Entry Inspection](./declaration-entry-inspection.md) when
  you need one read surface over retained seam truth after orchestration or
  manual lowering.
- Use [Declaration Entry Readiness](./declaration-entry-readiness.md) when you
  need family-level seam support posture rather than one orchestration run.

## Inspection And Debugging

Use the checked surface when you need to know:

- whether the declaration-entry pipeline enveloped, deferred, denied, failed,
  or refused
- the exact stop stage for the current outcome
- the typed refusal class when automation had to stop explicitly

Use the proof-visible surface when you need to know:

- which canonical stages were actually reached
- which stage was the real stop boundary
- which retained digests anchor the reached or stopped stages
- whether an apparent route-stage outcome actually lowered farther into receipt
  posture before stopping

## Anti-Patterns

- do not treat ordinary orchestration as a continuation or execution surface
- do not assume a successful envelope implies signal compatibility or runtime
  readiness
- do not use orchestration when you specifically need one intermediate artifact
  and nothing later
- do not flatten typed terminal errors into one string or generic domain error
- do not infer that dormant refusal classes are unused just because the current
  public ceiling does not exercise every one yet

## Current Limits

- current orchestration success stops at `ForgeQueryDeclarationEnvelope`
- the current front door does not automate signal compatibility
- it does not automate runtime, workspace, or basis continuation
- it does not automate `9.3.7` contribution composition
- the proof-visible surface is intentionally thin: it gives stage records, not
  the richer orchestration transcript family planned for later phases
- the current generic front door exercises the currently reachable
  declaration-entry stop classes, but the stable refusal family is broader than
  that first public ceiling

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Canonical Domain Declarations](./canonical-domain-declarations.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Declaration Entry Inspection](./declaration-entry-inspection.md)
- [Declaration Entry Readiness](./declaration-entry-readiness.md)
