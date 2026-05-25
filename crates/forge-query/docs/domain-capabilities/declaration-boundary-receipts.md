# Declaration Boundary Receipts

## What This Feature Is

Declaration boundary receipts are the Query-owned operational artifacts that
record what crossing posture Query is willing to claim after route planning.

They are built from retained declaration truth, not from fresh discovery:

- admitted-world proof from the handle
- retained declaration truth from foundational evidence
- retained route truth from route planning
- foundational boundary-receipt materialization primitives

They are not the same thing as Phase 8 descriptive receipts. Foundational
evidence receipts describe retained declaration truth. Declaration boundary
receipts record the Query crossing posture that followed from that truth.

## Why You Use It

- keep one public artifact for declaration crossing posture
- preserve deferred, denied, and failed crossing outcomes instead of dropping
  them
- carry forward route explanation and typed route-denial causes
- hand later envelope, inspection, and recovery features one stable receipt
  surface
- get one receipt digest that converges when retained truth matches and
  diverges when admitted world or crossing posture changes

## Stable Entry Points

- `ForgeQueryDeclarationReceiptInput`
- `ForgeQueryDeclarationReceipt`
- `ForgeQueryDeclarationReceiptChecked`
- `ForgeQueryDeclarationReceiptDeferred`
- `ForgeQueryDeclarationReceiptDenied`
- `ForgeQueryDeclarationReceiptFailed`
- `ForgeQueryDeclarationReceiptTerminalError`
- `ForgeQueryDeclarationEntryReceiptError`
- `ForgeQueryDeclarationReceiptClass`
- `ForgeQueryDeclarationReceiptKind`
- `ForgeQueryDeclarationReceiptExplanation`
- `ForgeQueryDeclarationReceiptDenialCause`
- `ForgeQueryAdmittedConfiguredDomainHandle::receipt_routes(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::receipt_routes_checked(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::receipt_routes_from_progressed(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::receipt_routes_from_progressed_with_intent(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::declare_review_progress_describe_plan_and_receipt(...)`

Good to know:

- boundary receipts start from retained route truth, not from legality evidence
  or foundational evidence alone
- the admitted handle remains the public entry surface because admitted-world
  identity is part of the retained proof chain
- denied and deferred crossings still produce Query-owned receipt artifacts
- signal-only route plans are not treated as successful receipt kinds in the
  current implementation

## API Reference

Receipt input constructors:

- `ForgeQueryDeclarationReceiptInput::planned(plan)`
- `ForgeQueryDeclarationReceiptInput::deferred(plan)`
- `ForgeQueryDeclarationReceiptInput::denied(plan)`
- `ForgeQueryDeclarationReceiptInput::failed(plan)`
- `ForgeQueryDeclarationReceiptInput::route_checked(checked)`

Admitted-handle receipt entry points:

- `receipt_routes(subject) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptTerminalError<D, I>>`
- `receipt_routes_checked(subject) -> ForgeQueryDeclarationReceiptChecked<D, I>`
- `receipt_routes_from_progressed(progressed) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptTerminalError<D, I>>`
- `receipt_routes_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptTerminalError<D, I>>`
- `declare_review_progress_describe_plan_and_receipt(input) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationEntryReceiptError<D, I>>`

Checked receipt outcomes:

- `ForgeQueryDeclarationReceiptChecked::Issued(ForgeQueryDeclarationReceipt<D, I>)`
- `ForgeQueryDeclarationReceiptChecked::Deferred(ForgeQueryDeclarationReceiptDeferred<D, I>)`
- `ForgeQueryDeclarationReceiptChecked::Denied(ForgeQueryDeclarationReceiptDenied<D, I>)`
- `ForgeQueryDeclarationReceiptChecked::Failed(ForgeQueryDeclarationReceiptFailed<D, I>)`

Receipt classes:

- `CoveredCrossing`
- `DeferredCrossing`
- `DeniedCrossing`
- `FailedCrossing`

Receipt kinds:

- `Relational`
- `Bridge`
- `Mixed`
- `Deferred`
- `Denied`
- `Failed`

Receipt-denial causes:

- `MissingRoutePlan`
- `UnsupportedReceiptKind`
- `ReceiptMaterializationMismatch`
- `RouteIntegrityMismatch`

## Core Mental Model

Think of declaration boundary receipts as the first operational artifact after
planning:

1. progression proves the declaration can continue
2. foundational evidence publishes retained declaration truth
3. route planning decides which lower-authority participation is honestly in
   play
4. receipt materialization records the crossing posture Query is willing to
   claim

That means:

- successful planned route participation becomes an issued crossing receipt
- deferred route participation becomes a deferred receipt artifact
- denied route participation becomes a denied receipt artifact
- failed route participation becomes a failed receipt artifact

The receipt stays Query-owned even when the lower authority has its own local
receipt or evidence language.

Declaration boundary envelopes are the next public crossing-story boundary.

## How It Executes

The advanced lane executes in this order:

1. produce admitted declaration progression
2. describe matching foundational evidence from that progressed declaration
3. produce route truth through route planning
4. wrap planned, deferred, denied, or failed route truth in
   `ForgeQueryDeclarationReceiptInput`
5. call `receipt_routes(...)` or `receipt_routes_checked(...)`
6. Query:
   - reuses the retained foundational evidence
   - reuses the retained route explanation or route-denial cause
   - materializes one foundational boundary receipt surface
   - derives one Query receipt digest from retained proof and crossing posture
7. Query returns one issued, deferred, denied, or failed receipt artifact

The common lane preserves the same structure. It still lowers through:

1. admission
2. canonicalization
3. legality
4. progression
5. foundational evidence
6. route planning
7. receipt materialization

## Small Example

```rust
use forge_query::facade::{
    ForgeQueryDeclarationReceiptChecked, ForgeQueryDeclarationReceiptInput,
};

let progressed = handle.declare_review_and_progress(
    SplitEdgeAtMidpoint { edge_ref: "edge:42" },
)?;

match handle.receipt_routes_checked(
    ForgeQueryDeclarationReceiptInput::planned(
        handle.plan_routes_from_progressed(progressed)?,
    ),
) {
    ForgeQueryDeclarationReceiptChecked::Issued(receipt) => {
        let _ = receipt.boundary_receipt();
    }
    other => panic!("unexpected receipt outcome: {:?}", std::mem::discriminant(&other)),
}
```

This is the smallest honest example because it starts from admitted progression
and finishes with a receipt artifact, but it still keeps the checked receipt
boundary visible.

## Real Example

```rust
use forge_query::facade::{
    ForgeQueryDeclarationFoundationalEvidenceInput, ForgeQueryDeclarationReceiptChecked,
    ForgeQueryDeclarationReceiptInput, ForgeQueryDeclarationRouteIntent,
    ForgeQueryDeclarationRoutePlanInput,
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

match handle.receipt_routes_checked(
    ForgeQueryDeclarationReceiptInput::planned(route_plan),
) {
    ForgeQueryDeclarationReceiptChecked::Issued(receipt) => {
        assert_eq!(receipt.declaration_family_key(), "split-edge");
        let _ = receipt.receipt_digest();
        let _ = receipt.explain();
        let _ = receipt.boundary_receipt();
    }
    ForgeQueryDeclarationReceiptChecked::Deferred(receipt) => {
        let _ = receipt.reason();
    }
    ForgeQueryDeclarationReceiptChecked::Denied(receipt) => {
        let _ = receipt.route_cause();
        let _ = receipt.receipt_cause();
    }
    ForgeQueryDeclarationReceiptChecked::Failed(receipt) => {
        let _ = receipt.reason();
    }
}
```

What this example is showing:

- receipt construction begins from retained route truth
- the receipt retains foundational evidence instead of rematerializing it
- the receipt keeps route explanation and denial topology visible
- the receipt exposes both Query-level inspection and the underlying
  foundational boundary receipt artifact

## How It Relates To Other Features

- [Declaration Progression](./declaration-progression.md) produces the admitted
  declaration proof the receipt pipeline ultimately depends on
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
  publishes the retained declaration truth the receipt must carry forward
- [Declaration Route Plans](./declaration-route-plan.md) produce the crossing
  posture and route explanation receipts consume
- [Configured Domain Handles](./configured-domain-handles.md) retain the
  admitted-world identity receipts must not rediscover

Use route planning when you need to decide what lower-authority participation
is in play. Use boundary receipts when you need the public operational artifact
that records what Query was actually willing to claim from that plan. Use
[Declaration Boundary Envelopes](./declaration-boundary-envelopes.md) when you
need the one self-describing public artifact that carries receipt truth, route
truth, and retained evidence together.

## Inspection And Debugging

Use these surfaces when inspecting a receipt:

- `class()`
- `kind()`
- `declaration_family_key()`
- `handle_identity_digest()`
- `operating_context_identity_digest()`
- `declaration_digest()`
- `progression_digest()`
- `route_plan_digest()`
- `receipt_digest()`
- `foundational_evidence()`
- `route_plan()`
- `route_denial_cause()`
- `explain()`
- `descriptive_receipt()`
- `boundary_receipt()`

Use the terminal receipt wrappers when the crossing did not issue:

- `ForgeQueryDeclarationReceiptDeferred::reason()`
- `ForgeQueryDeclarationReceiptDenied::route_cause()`
- `ForgeQueryDeclarationReceiptDenied::receipt_cause()`
- `ForgeQueryDeclarationReceiptDenied::reason()`
- `ForgeQueryDeclarationReceiptFailed::reason()`

These surfaces help answer:

- whether the receipt records a covered, deferred, denied, or failed crossing
- whether denial came from route truth or from receipt-boundary materialization
- whether two equivalent retained-proof paths converged to the same receipt
  digest
- whether divergence came from admitted world, route posture, or receipt kind

## Anti-Patterns

- attempting receipt construction from legality evidence alone
- attempting receipt construction from foundational evidence without route
  truth
- rebuilding receipt meaning from family labels or payload folklore
- treating foundational descriptive receipts as if they were already Query
  crossing receipts
- using successful receipt APIs as if they were allowed to return free-form
  text or no artifact at all

## Current Limits

Declaration boundary receipts now provide one Query-owned crossing artifact
over retained route truth. They still do not provide:

- public boundary envelopes over that retained receipt truth
- grouped declaration crossing receipts
- successful signal-only receipt kinds
- lower-authority execution internals outside the retained route plan and
  foundational receipt surfaces

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
- [Declaration Route Plans](./declaration-route-plan.md)
- [Declaration Progression](./declaration-progression.md)
- [Domain Capabilities](./README.md)
