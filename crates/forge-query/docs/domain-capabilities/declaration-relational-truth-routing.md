# Declaration Relational Truth Routing

## What This Feature Is

Declaration relational truth routing is the Query-owned boundary that turns one
retained declaration envelope into one retained relational authority binding.

This is the first phase where Query crosses from declaration-side proof into a
lower truth-authority family. Query still owns the public artifact and the
orchestration boundary. `forge-relational` still owns relational truth
semantics.

Phase 12 is envelope-backed only on the public lane. It does not start from
raw declarations, canonical declarations, legality evidence, foundational
evidence, route plans, or receipts alone.

## Why You Use It

- route an already-proved declaration into the relational authority family it
  actually needs
- preserve admitted-world identity, route posture, receipt posture, and
  evidence origin while crossing into relational truth
- keep mixed route plans honest by lowering only the relational slice in this
  phase
- get one relational-routing digest that converges when retained truth matches
  and diverges when world, route posture, receipt posture, or truth claim
  changes
- hand later phases one Query-owned artifact instead of making them rediscover
  which relational surface was selected

## Stable Entry Points

- `ForgeQueryDeclarationRelationalRoutingInput`
- `ForgeQueryDeclarationRelationalRouting`
- `ForgeQueryDeclarationRelationalRoutingChecked`
- `ForgeQueryDeclarationRelationalRoutingDeferred`
- `ForgeQueryDeclarationRelationalRoutingDenied`
- `ForgeQueryDeclarationRelationalRoutingFailed`
- `ForgeQueryDeclarationRelationalRoutingTerminalError`
- `ForgeQueryDeclarationEntryRelationalRoutingError`
- `ForgeQueryDeclarationRelationalRoutingClass`
- `ForgeQueryDeclarationRelationalBinding`
- `ForgeQueryDeclarationRelationalTruthContract`
- `ForgeQueryDeclarationRelationalTruthClaim`
- `ForgeQueryDeclarationRelationalAuthorityFamily`
- `ForgeQueryDeclarationRelationalRoutingExplanation`
- `ForgeQueryDeclarationRelationalRoutingSupportReport`
- `ForgeQueryDeclarationRelationalRoutingSupportRow`
- `ForgeQueryDeclarationRelationalTruthRoutingSupportStatus`
- `ForgeQueryAdmittedConfiguredDomainHandle::route_relational_truth(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::route_relational_truth_checked(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::route_relational_truth_from_progressed(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::route_relational_truth_from_progressed_with_intent(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::declare_review_progress_describe_plan_receipt_envelope_and_route_relational_truth(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::relational_truth_support::<I>()`

Good to know:

- the public lane is envelope-backed only
- common-lane helpers exist only for structurally relational declaration
  families
- mixed route plans still route in Phase 12, but only the relational slice is
  lowered now
- deferred, denied, and failed envelopes remain first-class typed outcomes

## API Reference

Relational-routing input constructors:

- `ForgeQueryDeclarationRelationalRoutingInput::enveloped(envelope)`
- `ForgeQueryDeclarationRelationalRoutingInput::deferred(envelope)`
- `ForgeQueryDeclarationRelationalRoutingInput::denied(envelope)`
- `ForgeQueryDeclarationRelationalRoutingInput::failed(envelope)`
- `ForgeQueryDeclarationRelationalRoutingInput::envelope_checked(checked)`

Admitted-handle relational-routing entry points:

- `route_relational_truth(subject) -> Result<ForgeQueryDeclarationRelationalRouting<D, I>, ForgeQueryDeclarationRelationalRoutingTerminalError<D, I>>`
- `route_relational_truth_checked(subject) -> ForgeQueryDeclarationRelationalRoutingChecked<D, I>`
- `route_relational_truth_from_progressed(progressed) -> Result<ForgeQueryDeclarationRelationalRouting<D, I>, ForgeQueryDeclarationRelationalRoutingTerminalError<D, I>>`
- `route_relational_truth_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationRelationalRouting<D, I>, ForgeQueryDeclarationRelationalRoutingTerminalError<D, I>>`
- `declare_review_progress_describe_plan_receipt_envelope_and_route_relational_truth(input) -> Result<ForgeQueryDeclarationRelationalRouting<D, I>, ForgeQueryDeclarationEntryRelationalRoutingError<D, I>>`
- `relational_truth_support::<I>() -> ForgeQueryDeclarationRelationalRoutingSupportReport<D, I>`

Checked relational-routing outcomes:

- `ForgeQueryDeclarationRelationalRoutingChecked::Routed(ForgeQueryDeclarationRelationalRouting<D, I>)`
- `ForgeQueryDeclarationRelationalRoutingChecked::Deferred(ForgeQueryDeclarationRelationalRoutingDeferred<D, I>)`
- `ForgeQueryDeclarationRelationalRoutingChecked::Denied(ForgeQueryDeclarationRelationalRoutingDenied<D, I>)`
- `ForgeQueryDeclarationRelationalRoutingChecked::Failed(ForgeQueryDeclarationRelationalRoutingFailed<D, I>)`

Truth claims:

- `AuthoritativeCurrentTruth`
- `Identity`
- `Lineage`
- `HistoricalTruth`
- `InvariantTruth`
- `GroupedTruth`
- `StrategyTruth`

Authority families:

- `Runtime`
- `History`
- `GroupedTruth`
- `CommitStrategies`
- `BridgeSource`

## Core Mental Model

Think of relational truth routing as the binding phase between declaration proof
and relational authority:

1. envelopes prove one public crossing story already exists
2. relational routing asks whether that envelope still admits a relational slice
3. Query selects one relational truth claim and one relational authority family
4. Query returns one retained routing artifact that points at the real lower
   relational surface without pretending Query owns relational semantics

That means the routing artifact is not a live relational runtime. It is the
proof-bearing Query boundary artifact that says which relational authority lane
this declaration now binds to.

## How It Executes

The advanced lane executes in this order:

1. start from `ForgeQueryDeclarationRelationalRoutingInput`
2. call `route_relational_truth(...)` or `route_relational_truth_checked(...)`
3. Query:
   - verifies the envelope is covered crossing truth
   - verifies the retained route plan still includes a relational slice
   - reads the declaration family's relational truth contract
   - binds that truth contract to one real lower surface family:
     - `forge_relational::facade::runtime`
     - `forge_relational::facade::history`
     - `forge_relational::facade::grouped_truth`
     - `forge_relational::facade::commit_strategies`
     - `forge_relational::facade::bridge::RuntimeBridgeRelationalSource`
   - derives one relational-routing digest from retained proof
4. Query returns a routed, deferred, denied, or failed relational-routing
   artifact

The common lane preserves the same structure. It still lowers through:

1. admission
2. canonicalization
3. legality
4. progression
5. foundational evidence
6. route planning
7. receipt materialization
8. envelope construction
9. relational truth routing

## Small Example

```rust
use forge_query::facade::{
    ForgeQueryDeclarationRelationalRoutingChecked,
    ForgeQueryDeclarationRelationalRoutingInput,
};

let envelope = handle.envelope_routes_from_progressed(
    handle.declare_review_and_progress(
        SplitEdgeAtMidpoint { edge_ref: "edge:42" },
    )?,
)?;

match handle.route_relational_truth_checked(
    ForgeQueryDeclarationRelationalRoutingInput::enveloped(envelope),
) {
    ForgeQueryDeclarationRelationalRoutingChecked::Routed(routing) => {
        let _ = routing.binding();
        let _ = routing.explain();
    }
    other => panic!("unexpected routing outcome: {:?}", std::mem::discriminant(&other)),
}
```

This is the smallest honest example because it starts from retained envelope
truth and ends at the first lower-authority truth-routing artifact without
hiding the checked boundary.

## Real Example

```rust
let routing = handle
    .declare_review_progress_describe_plan_receipt_envelope_and_route_relational_truth(
        SplitEdgeAtMidpoint { edge_ref: "edge:42" },
    )?;

assert_eq!(
    routing.truth_claim(),
    ForgeQueryDeclarationRelationalTruthClaim::GroupedTruth,
);
let _ = routing.authority_family();
let _ = routing.binding();
let _ = routing.relational_routing_digest();
let _ = routing.explain();
```

What this example is showing:

- the public lane still lowers through the full declaration proof chain
- the routed artifact tells you which relational truth claim was selected
- the routed artifact tells you which lower authority family now owns truth
  semantics
- later phases can keep using retained proof instead of rediscovering route or
  receipt meaning

## How It Relates To Other Features

- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md) are
  the required public input to Phase 12
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md) still own
  crossing posture; routing does not replace them
- [Declaration Route Plans](./declaration-route-plan.md) still own route
  selection; routing only consumes the retained relational slice
- [Configured Domain Handles](./configured-domain-handles.md) still own the
  public orchestration lane and support snapshot

Use envelopes when you need one public crossing artifact. Use relational truth
routing when you need to bind that crossing artifact to real relational truth
authority.

## Inspection And Debugging

Use these surfaces when inspecting a routed artifact:

- `class()`
- `truth_claim()`
- `authority_family()`
- `binding()`
- `declaration_family_key()`
- `handle_identity_digest()`
- `operating_context_identity_digest()`
- `declaration_digest()`
- `progression_digest()`
- `route_plan_digest()`
- `receipt_digest()`
- `envelope_digest()`
- `relational_routing_digest()`
- `envelope()`
- `route_denial_cause()`
- `receipt_denial_cause()`
- `evidence_origin()`
- `explain()`

Use `relational_truth_support::<I>()` when you need the family-scoped readiness
row before attempting routing.

## Anti-Patterns

- trying to start relational truth routing from receipts without envelope truth
- trying to reconstruct relational target selection from family labels or route
  folklore
- treating the routed artifact as a live relational runtime
- assuming mixed route plans become pure relational successes in this phase
- using common-lane helpers on non-relational families

## Current Limits

Declaration relational truth routing now binds retained envelope truth to one
real relational authority family. It still does not provide:

- bridge continuation routing for the mixed-authority slice; that begins in the
  next boundary
- signal execution
- signal compatibility classification
- grouped declaration batching
- a later unified inspection surface over routed declaration artifacts
- direct execution of lower relational semantics inside Query

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
- [Declaration Route Plans](./declaration-route-plan.md)
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
- [Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
- [Declaration Signal Compatibility](./declaration-signal-compatibility.md)
- [Domain Capabilities](./README.md)
