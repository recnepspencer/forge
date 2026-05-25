# Declaration Bridge Continuation Routing

## What This Feature Is

Declaration bridge continuation routing is the Query-owned boundary that turns
one retained declaration envelope into one retained bridge-continuation
binding.

This is the first phase where Query crosses from the public declaration
crossing story into the real bridge authority family. Query still owns the
public artifact and orchestration boundary. `forge-runtime-bridge` still owns
bridge continuation semantics.

Phase 13 is envelope-backed only on the public lane. It does not start from
raw declarations, canonical declarations, legality evidence, foundational
evidence, route plans, or receipts alone.

## Why You Use It

- route an already-proved declaration into the bridge continuation family it
  actually needs
- preserve admitted-world identity, route posture, receipt posture, and
  evidence origin while crossing into bridge authority
- keep mixed route plans honest by lowering only the bridge slice in this
  phase
- get one bridge-routing digest that converges when retained truth matches and
  diverges when world, truth context, continuation mode, or denial topology
  changes
- hand later phases one Query-owned continuation artifact instead of making
  them rediscover which bridge surface was selected

## Stable Entry Points

- `ForgeQueryDeclarationBridgeRoutingInput`
- `ForgeQueryDeclarationBridgeRouting`
- `ForgeQueryDeclarationBridgeRoutingChecked`
- `ForgeQueryDeclarationBridgeRoutingDeferred`
- `ForgeQueryDeclarationBridgeRoutingDenied`
- `ForgeQueryDeclarationBridgeRoutingFailed`
- `ForgeQueryDeclarationBridgeRoutingTerminalError`
- `ForgeQueryDeclarationEntryBridgeRoutingError`
- `ForgeQueryDeclarationBridgeRoutingClass`
- `ForgeQueryDeclarationBridgeBinding`
- `ForgeQueryDeclarationBridgeContinuationContract`
- `ForgeQueryDeclarationBridgeContinuationFamily`
- `ForgeQueryDeclarationBridgeContinuationMode`
- `ForgeQueryDeclarationBridgeContinuationRequest`
- `ForgeQueryDeclarationBridgeTruthContext`
- `ForgeQueryDeclarationBridgeRoutingExplanation`
- `ForgeQueryDeclarationBridgeRoutingSupportReport`
- `ForgeQueryDeclarationBridgeRoutingSupportRow`
- `ForgeQueryDeclarationBridgeRoutingSupportStatus`
- `ForgeQueryAdmittedConfiguredDomainHandle::route_bridge_continuation(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::route_bridge_continuation_checked(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::route_bridge_continuation_from_progressed(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::route_bridge_continuation_from_progressed_with_intent(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::declare_review_progress_describe_plan_receipt_envelope_and_route_bridge_continuation(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::bridge_continuation_support::<I>()`

Good to know:

- the public lane is envelope-backed only
- common-lane helpers exist only for structurally bridge-capable declaration
  families
- mixed route plans still route in Phase 13, but only the bridge slice is
  lowered now
- deferred, denied, and failed envelopes remain first-class typed outcomes

## API Reference

Bridge-routing input constructors:

- `ForgeQueryDeclarationBridgeRoutingInput::enveloped(envelope)`
- `ForgeQueryDeclarationBridgeRoutingInput::deferred(envelope)`
- `ForgeQueryDeclarationBridgeRoutingInput::denied(envelope)`
- `ForgeQueryDeclarationBridgeRoutingInput::failed(envelope)`
- `ForgeQueryDeclarationBridgeRoutingInput::envelope_checked(checked)`

Admitted-handle bridge-routing entry points:

- `route_bridge_continuation(subject) -> Result<ForgeQueryDeclarationBridgeRouting<D, I>, ForgeQueryDeclarationBridgeRoutingTerminalError<D, I>>`
- `route_bridge_continuation_checked(subject) -> ForgeQueryDeclarationBridgeRoutingChecked<D, I>`
- `route_bridge_continuation_from_progressed(progressed) -> Result<ForgeQueryDeclarationBridgeRouting<D, I>, ForgeQueryDeclarationBridgeRoutingTerminalError<D, I>>`
- `route_bridge_continuation_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationBridgeRouting<D, I>, ForgeQueryDeclarationBridgeRoutingTerminalError<D, I>>`
- `declare_review_progress_describe_plan_receipt_envelope_and_route_bridge_continuation(input) -> Result<ForgeQueryDeclarationBridgeRouting<D, I>, ForgeQueryDeclarationEntryBridgeRoutingError<D, I>>`
- `bridge_continuation_support::<I>() -> ForgeQueryDeclarationBridgeRoutingSupportReport<D, I>`

Checked bridge-routing outcomes:

- `ForgeQueryDeclarationBridgeRoutingChecked::Routed(ForgeQueryDeclarationBridgeRouting<D, I>)`
- `ForgeQueryDeclarationBridgeRoutingChecked::Deferred(ForgeQueryDeclarationBridgeRoutingDeferred<D, I>)`
- `ForgeQueryDeclarationBridgeRoutingChecked::Denied(ForgeQueryDeclarationBridgeRoutingDenied<D, I>)`
- `ForgeQueryDeclarationBridgeRoutingChecked::Failed(ForgeQueryDeclarationBridgeRoutingFailed<D, I>)`

Support-report inspection:

- `ForgeQueryDeclarationBridgeRoutingSupportReport::declaration_family_key()`
- `ForgeQueryDeclarationBridgeRoutingSupportReport::rows()`
- `ForgeQueryDeclarationBridgeRoutingSupportReport::support_digest()`
- `ForgeQueryDeclarationBridgeRoutingSupportRow::continuation_mode()`
- `ForgeQueryDeclarationBridgeRoutingSupportRow::truth_context()`
- `ForgeQueryDeclarationBridgeRoutingSupportRow::family()`
- `ForgeQueryDeclarationBridgeRoutingSupportRow::status()`
- `ForgeQueryDeclarationBridgeRoutingSupportRow::reason()`

Support statuses:

- `Admitted`
- `Unsupported`
- `InvalidContext`

Continuation modes:

- `RuntimeRoute`
- `TruthView`
- `PreviewSession`
- `PreviewPromotion`
- `SubscriptionPreparation`
- `WritebackPreparation`

Truth contexts:

- `Current`
- `Historical`
- `Preview`

Bridge continuation families:

- `RuntimeRoute`
- `TruthView`
- `PreviewSession`
- `PreviewPromotion`
- `SubscriptionPreparation`
- `WritebackPreparation`
- `MixedBridgeContinuation`

## Core Mental Model

Think of bridge continuation routing as the binding phase between the public
declaration crossing story and bridge authority:

1. envelopes prove one public crossing story already exists
2. bridge routing asks whether that envelope still admits a bridge slice
3. Query selects one continuation request, one truth context, and one bridge
   continuation family
4. Query returns one retained routing artifact that points at the real lower
   bridge surface without pretending Query owns bridge semantics

That means the routing artifact is not a live bridge runtime. It is the
proof-bearing Query boundary artifact that says which bridge continuation lane
this declaration now binds to.

## How It Executes

The advanced lane executes in this order:

1. start from `ForgeQueryDeclarationBridgeRoutingInput`
2. call `route_bridge_continuation(...)` or
   `route_bridge_continuation_checked(...)`
3. Query:
   - verifies the envelope is covered crossing truth
   - verifies the retained route plan still includes a bridge slice
   - verifies the envelope belongs to the current admitted handle and world
   - reads the declaration family's bridge continuation contract
   - binds that contract to one real lower surface family:
     - `forge_runtime_bridge::facade::BridgeRouteRequest`
     - `forge_runtime_bridge::facade::BridgeTruthViewEvaluationRequest`
     - `forge_runtime_bridge::facade::BridgeSpeculativeSessionRequest`
     - `forge_runtime_bridge::facade::BridgeSpeculativePromotionRequest`
     - `forge_runtime_bridge::facade::BridgeSubscriptionContinuationCandidateInput`
     - `forge_runtime_bridge::facade::TruthWritebackRequest`
   - derives one bridge-routing digest from retained proof
4. Query returns a routed, deferred, denied, or failed bridge-routing artifact

The common lane preserves the same structure. It still lowers through:

1. admission
2. canonicalization
3. legality
4. progression
5. foundational evidence
6. route planning
7. receipt materialization
8. envelope construction
9. bridge continuation routing

## Small Example

```rust
use forge_query::facade::{
    ForgeQueryDeclarationBridgeRoutingChecked,
    ForgeQueryDeclarationBridgeRoutingInput,
};

let envelope = handle.envelope_routes_from_progressed(
    handle.declare_review_and_progress(
        SplitEdgeAtMidpoint { edge_ref: "edge:42" },
    )?,
)?;

match handle.route_bridge_continuation_checked(
    ForgeQueryDeclarationBridgeRoutingInput::enveloped(envelope),
) {
    ForgeQueryDeclarationBridgeRoutingChecked::Routed(routing) => {
        let _ = routing.binding();
        let _ = routing.explain();
    }
    other => panic!("unexpected routing outcome: {:?}", std::mem::discriminant(&other)),
}
```

This is the smallest honest example because it starts from retained envelope
truth and ends at the first bridge-continuation artifact without hiding the
checked boundary.

## Real Example

```rust
let routing = handle
    .declare_review_progress_describe_plan_receipt_envelope_and_route_bridge_continuation(
        SplitEdgeAtMidpoint { edge_ref: "edge:42" },
    )?;

assert_eq!(
    routing.continuation_family(),
    ForgeQueryDeclarationBridgeContinuationFamily::PreviewSession,
);
assert_eq!(
    routing.truth_context(),
    ForgeQueryDeclarationBridgeTruthContext::Preview,
);
let _ = routing.continuation_request();
let _ = routing.binding();
let _ = routing.bridge_routing_digest();
let _ = routing.explain();
```

What this example is showing:

- the public lane still lowers through the full declaration proof chain
- the routed artifact tells you which continuation request was selected
- the routed artifact tells you which lower bridge family now owns
  continuation semantics
- later phases can keep using retained proof instead of rediscovering route,
  receipt, or envelope meaning

## How It Relates To Other Features

- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md) are
  the required public input to Phase 13
- [Declaration Relational Truth Routing](./declaration-relational-truth-routing.md)
  still owns the relational slice of mixed-authority route plans
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md) still own
  crossing posture; bridge routing does not replace them
- [Configured Domain Handles](./configured-domain-handles.md) still own the
  public orchestration lane and support snapshot

Use envelopes when you need one public crossing artifact. Use bridge
continuation routing when you need to bind that crossing artifact to one real
bridge authority family.

## Inspection And Debugging

Use these surfaces when inspecting a routed artifact:

- `class()`
- `continuation_request()`
- `truth_context()`
- `continuation_family()`
- `binding()`
- `declaration_family_key()`
- `handle_identity_digest()`
- `operating_context_identity_digest()`
- `declaration_digest()`
- `progression_digest()`
- `route_plan_digest()`
- `receipt_digest()`
- `envelope_digest()`
- `bridge_routing_digest()`
- `envelope()`
- `route_denial_cause()`
- `receipt_denial_cause()`
- `evidence_origin()`
- `explain()`

Use `bridge_continuation_support::<I>()` when you need the family-scoped
readiness row before attempting routing.

That support row is the fastest way to answer:

- whether the family exposes any bridge continuation contract at all
- whether the current admitted world enables the required runtime-bridge
  config
- whether the current admitted world exposes the capability families that the
  selected continuation mode needs

## Anti-Patterns

- trying to start bridge continuation routing from receipts without envelope
  truth
- trying to reconstruct bridge target selection from family labels or host
  choreography
- treating the routed artifact as a live bridge runtime
- treating bridge continuation routing as if it already executed the lower
  bridge operation
- assuming mixed route plans become pure bridge successes in this phase
- using common-lane helpers on non-bridge families

## Current Limits

Declaration bridge continuation routing now binds retained envelope truth to
one real bridge continuation family. It still does not provide:

- signal execution
- signal compatibility classification over retained bridge and envelope posture
- grouped declaration bridge-routing batches
- a later unified inspection surface over routed declaration artifacts
- direct execution of lower bridge semantics inside Query
- bridge-routing entry from earlier declaration artifacts on the public lane

The retained bridge binding tells you which lower bridge surface this
declaration now routes to. It does not claim the lower bridge request already
ran.

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Declaration Relational Truth Routing](./declaration-relational-truth-routing.md)
- [Declaration Signal Compatibility](./declaration-signal-compatibility.md)
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
- [Declaration Route Plans](./declaration-route-plan.md)
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
- [Domain Capabilities](./README.md)
