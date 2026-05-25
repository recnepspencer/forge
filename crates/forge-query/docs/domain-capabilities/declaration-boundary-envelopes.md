# Declaration Boundary Envelopes

## What This Feature Is

Declaration boundary envelopes are the public Query-owned artifacts that carry
one complete declaration crossing story forward.

They are receipt-backed only on the public lane. Query does not build them
directly from declarations, legality evidence, foundational evidence, or route
plans alone.

An envelope composes retained truth that already exists:

- retained declaration truth from foundational evidence
- retained route truth from route planning
- retained crossing posture from declaration boundary receipts

This keeps the public story honest:

- foundational evidence still owns retained declaration description
- route plans still own lower-authority participation planning
- receipts still own crossing posture
- envelopes own the one self-describing public artifact over those retained
  truths

## Why You Use It

- hand downstream callers one public artifact instead of making them reassemble
  evidence, route, and receipt truth
- preserve route-denial and receipt-denial topology on non-success paths
- keep admitted-world identity attached to the public crossing story
- get one envelope digest that converges when retained truth matches and
  diverges when world, route posture, receipt posture, or denial topology
  changes
- keep later inspection and recovery work from reopening route or receipt
  reasoning

## Stable Entry Points

- `ForgeQueryDeclarationEnvelopeInput`
- `ForgeQueryDeclarationEnvelope`
- `ForgeQueryDeclarationEnvelopeChecked`
- `ForgeQueryDeclarationEnvelopeDeferred`
- `ForgeQueryDeclarationEnvelopeDenied`
- `ForgeQueryDeclarationEnvelopeFailed`
- `ForgeQueryDeclarationEnvelopeTerminalError`
- `ForgeQueryDeclarationEntryEnvelopeError`
- `ForgeQueryDeclarationEnvelopeClass`
- `ForgeQueryDeclarationEnvelopeEvidenceOrigin`
- `ForgeQueryDeclarationEnvelopeExplanation`
- `ForgeQueryAdmittedConfiguredDomainHandle::envelope_routes(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::envelope_routes_checked(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::envelope_routes_from_progressed(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::envelope_routes_from_progressed_with_intent(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::declare_review_progress_describe_plan_receipt_and_envelope(...)`

Good to know:

- the public envelope lane starts from receipt truth only
- deferred, denied, and failed receipt posture still produce Query-owned
  envelope artifacts
- envelopes preserve foundational evidence origin even though the ordinary lane
  does not begin from foundational evidence directly
- envelopes are Query-owned artifacts; they are not a cloned foundational
  envelope surface

## API Reference

Envelope input constructors:

- `ForgeQueryDeclarationEnvelopeInput::issued(receipt)`
- `ForgeQueryDeclarationEnvelopeInput::deferred(receipt)`
- `ForgeQueryDeclarationEnvelopeInput::denied(receipt)`
- `ForgeQueryDeclarationEnvelopeInput::failed(receipt)`
- `ForgeQueryDeclarationEnvelopeInput::receipt_checked(checked)`

Admitted-handle envelope entry points:

- `envelope_routes(subject) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEnvelopeTerminalError<D, I>>`
- `envelope_routes_checked(subject) -> ForgeQueryDeclarationEnvelopeChecked<D, I>`
- `envelope_routes_from_progressed(progressed) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEnvelopeTerminalError<D, I>>`
- `envelope_routes_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEnvelopeTerminalError<D, I>>`
- `declare_review_progress_describe_plan_receipt_and_envelope(input) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEntryEnvelopeError<D, I>>`

Checked envelope outcomes:

- `ForgeQueryDeclarationEnvelopeChecked::Enveloped(ForgeQueryDeclarationEnvelope<D, I>)`
- `ForgeQueryDeclarationEnvelopeChecked::Deferred(ForgeQueryDeclarationEnvelopeDeferred<D, I>)`
- `ForgeQueryDeclarationEnvelopeChecked::Denied(ForgeQueryDeclarationEnvelopeDenied<D, I>)`
- `ForgeQueryDeclarationEnvelopeChecked::Failed(ForgeQueryDeclarationEnvelopeFailed<D, I>)`

Envelope classes:

- `CoveredCrossing`
- `DeferredCrossing`
- `DeniedCrossing`
- `FailedCrossing`

Evidence-origin values:

- `AdmittedProgression`
- `ProgressionDeferred`
- `ProgressionDenied`
- `ProgressionStale`
- `ProgressionRebindRequired`
- `ProgressionFailed`
- `LegalityEvidence`
- `LegalityDenial`

## Core Mental Model

Think of declaration boundary envelopes as the public crossing-story wrapper
over already-proved declaration truth:

1. progression proves whether the declaration can continue
2. foundational evidence publishes retained declaration truth
3. route planning decides what lower-authority participation is honestly in
   play
4. receipt materialization records the crossing posture Query is willing to
   claim
5. envelope construction publishes one self-describing artifact over that
   receipt-backed truth

That means envelopes do not invent new route or receipt meaning. They preserve
and present retained meaning that earlier phases already proved.

## How It Executes

The advanced lane executes in this order:

1. produce admitted declaration progression
2. materialize one declaration boundary receipt from that retained route truth
3. wrap the issued, deferred, denied, or failed receipt in
   `ForgeQueryDeclarationEnvelopeInput`
4. call `envelope_routes(...)` or `envelope_routes_checked(...)`
5. Query:
   - reuses retained foundational evidence through the receipt
   - reuses retained route explanation or route denial through the receipt
   - reuses retained receipt explanation or receipt denial directly
   - derives one envelope digest from retained proof and denial topology
6. Query returns one enveloped, deferred, denied, or failed envelope artifact

The common lane preserves the same structure. It still lowers through:

1. admission
2. canonicalization
3. legality
4. progression
5. foundational evidence
6. route planning
7. receipt materialization
8. envelope construction

## Small Example

```rust
use forge_query::facade::{
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationEnvelopeInput,
};

let receipt = handle.receipt_routes_from_progressed(
    handle.declare_review_and_progress(
        SplitEdgeAtMidpoint { edge_ref: "edge:42" },
    )?,
)?;

match handle.envelope_routes_checked(
    ForgeQueryDeclarationEnvelopeInput::issued(receipt),
) {
    ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
        let _ = envelope.receipt_digest();
        let _ = envelope.explain();
    }
    other => panic!("unexpected envelope outcome: {:?}", std::mem::discriminant(&other)),
}
```

This is the smallest honest example because it starts from retained receipt
truth and finishes with the public envelope artifact while still keeping the
checked envelope boundary visible.

## Real Example

```rust
use forge_query::facade::{
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationEnvelopeInput,
    ForgeQueryDeclarationFoundationalEvidenceInput, ForgeQueryDeclarationRouteIntent,
    ForgeQueryDeclarationReceiptInput, ForgeQueryDeclarationRoutePlanInput,
};

let progressed = handle.progress_declaration(
    handle.declare_and_review(SplitEdgeAtMidpoint { edge_ref: "edge:42" })?,
)?;

let evidence = handle.describe_foundational(
    ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
        progressed.clone(),
    ),
)?;

let route_plan = handle.plan_routes(
    ForgeQueryDeclarationRoutePlanInput::with_intent(
        progressed,
        evidence,
        ForgeQueryDeclarationRouteIntent::Auto,
    ),
)?;

let receipt = handle.receipt_routes(
    ForgeQueryDeclarationReceiptInput::planned(route_plan),
)?;

match handle.envelope_routes_checked(
    ForgeQueryDeclarationEnvelopeInput::issued(receipt),
) {
    ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
        assert_eq!(envelope.declaration_family_key(), "split-edge");
        let _ = envelope.evidence_origin();
        let _ = envelope.route_plan_digest();
        let _ = envelope.receipt_digest();
        let _ = envelope.envelope_digest();
        let _ = envelope.explain();
    }
    ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
        let _ = envelope.reason();
    }
    ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
        let _ = envelope.route_cause();
        let _ = envelope.receipt_cause();
    }
    ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
        let _ = envelope.reason();
    }
}
```

What this example is showing:

- the envelope starts from retained receipt truth
- foundational evidence origin is still visible on the public story
- route posture and receipt posture stay distinct
- the envelope digest is separate from the receipt digest

## How It Relates To Other Features

- [Declaration Boundary Receipts](./declaration-boundary-receipts.md) provide
  the retained crossing posture the envelope wraps
- [Declaration Route Plans](./declaration-route-plan.md) provide the retained
  route truth the receipt and envelope carry forward
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
  provides the retained declaration truth and evidence-origin posture the
  envelope preserves
- [Configured Domain Handles](./configured-domain-handles.md) retain the
  admitted-world identity the envelope must not rediscover

Use receipts when you need the operational crossing artifact itself. Use
envelopes when you need the one self-describing public artifact that keeps
retained evidence, route truth, and receipt truth together for later
inspection or recovery work. Use
[Declaration Relational Truth Routing](./declaration-relational-truth-routing.md)
when that same public crossing story needs to bind into relational truth
authority. Use
[Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
when it needs to bind into bridge continuation authority. Use
[Declaration Signal Compatibility](./declaration-signal-compatibility.md)
when you need to freeze whether that retained crossing story can later
continue into Signal-backed derived execution.
when that public envelope now needs to bind to one lower relational truth
authority family. Use
[Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
when that same public envelope now needs to bind to one lower bridge
continuation family.

## Inspection And Debugging

Use these surfaces when inspecting an envelope:

- `class()`
- `declaration_family_key()`
- `handle_identity_digest()`
- `operating_context_identity_digest()`
- `declaration_digest()`
- `progression_digest()`
- `route_plan_digest()`
- `receipt_digest()`
- `envelope_digest()`
- `foundational_evidence()`
- `receipt()`
- `route_plan()`
- `route_denial_cause()`
- `receipt_denial_cause()`
- `evidence_origin()`
- `explain()`

Use the terminal envelope wrappers when the public crossing story did not land
as a covered crossing:

- `ForgeQueryDeclarationEnvelopeDeferred::reason()`
- `ForgeQueryDeclarationEnvelopeDenied::route_cause()`
- `ForgeQueryDeclarationEnvelopeDenied::receipt_cause()`
- `ForgeQueryDeclarationEnvelopeDenied::reason()`
- `ForgeQueryDeclarationEnvelopeFailed::reason()`

These surfaces help answer:

- whether the envelope records a covered, deferred, denied, or failed crossing
- whether divergence came from admitted world, route posture, or receipt
  posture
- whether route denial or receipt denial is the real public blocker
- whether the public envelope preserved the same retained truth as the receipt

## Anti-Patterns

- attempting envelope construction from route plans without receipt truth
- attempting envelope construction from foundational evidence alone
- rebuilding route or receipt meaning from family labels or payload folklore
- treating envelopes as if they were allowed to hide denial topology
- using public crossing-story APIs as if they were allowed to return free-form
  text or no artifact at all

## Current Limits

Declaration boundary envelopes now provide one self-describing public artifact
over retained receipt truth. They still do not provide:

- direct construction from foundational evidence or route plans on the public
  lane
- relational truth routing on their own; that begins in the next boundary
- bridge continuation routing on their own; that begins in the next boundary
- grouped declaration envelopes
- a later unified inspection surface over multiple declaration envelopes
- lower-authority execution internals beyond the retained route and receipt
  artifacts they compose

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
- [Declaration Relational Truth Routing](./declaration-relational-truth-routing.md)
- [Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
- [Declaration Route Plans](./declaration-route-plan.md)
- [Declaration Progression](./declaration-progression.md)
- [Domain Capabilities](./README.md)
