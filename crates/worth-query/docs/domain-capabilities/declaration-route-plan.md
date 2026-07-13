# Declaration Route Plans

## What This Feature Is

Declaration route planning is the Query-owned boundary that turns admitted
declaration progression plus matching foundational evidence into one explicit
lower-authority route plan.

The important split is:

- declaration progression proves the declaration is admitted strongly enough to
  continue
- foundational evidence describes that retained truth through shared
  provenance, support, and receipt artifacts
- route planning decides which lower-authority route families are honestly in
  play without performing the crossing yet

This is planning, not lowering and not receipt production.
Boundary receipts are the next declaration-side artifact boundary.

## Why You Use It

- turn admitted declaration proof into one inspectable route plan
- preserve zero, one, or many lower-authority routes as explicit public truth
- keep caller route narrowing typed instead of stringly
- preserve deferred, denied, and failed route posture without flattening it
- hand later crossing and receipt surfaces one retained route artifact instead of
  recomputing route meaning

Use [Declaration Boundary Receipts](./declaration-boundary-receipts.md) when
you need the Query-owned public crossing artifact that records what followed
from this route plan.
Use [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md) when
you need the self-describing public crossing story that carries retained
evidence, route truth, and receipt truth together.

## Stable Entry Points

- `WorthQueryDeclarationRouteContract`
- `WorthQueryDeclarationRouteIntent`
- `WorthQueryDeclarationRoutePlanInput`
- `WorthQueryDeclarationRoutePlan`
- `WorthQueryDeclarationRoutePlanChecked`
- `WorthQueryDeclarationRoutePlanDeferred`
- `WorthQueryDeclarationRoutePlanDenied`
- `WorthQueryDeclarationRoutePlanFailed`
- `WorthQueryDeclarationRoutePlanDenialCause`
- `WorthQueryDeclarationRoutePlanExplanation`
- `WorthQueryDeclarationRouteSegment`
- `WorthQueryDeclarationRouteSet`
- `WorthQueryLowerAuthorityRouteFamily`
- `WorthQueryAdmittedConfiguredDomainHandle::plan_routes(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::plan_routes_checked(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::plan_routes_from_progressed(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::plan_routes_from_progressed_with_intent(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::orchestrate_routes_from_progressed(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::orchestrate_routes_from_progressed_with_intent(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::declare_review_progress_describe_and_plan(...)`

Good to know:

- route planning starts from admitted progression plus matching foundational
  evidence, not from legality alone
- the admitted handle stays the entry surface because retained admitted-world
  proof still matters
- route planning exposes an explicit route set from day one
- temporal and async declarations still lower through this same route-plan lane;
  their future-sensitive posture is retained on the plan artifact instead of
  creating a second planner
- `primary_route()` is convenience only; the route set is authoritative

## API Reference

Family marker contract:

- `route_contract() -> WorthQueryDeclarationRouteContract`

Route-contract presets:

- `relational_only() -> WorthQueryDeclarationRouteContract`
- `bridge_only() -> WorthQueryDeclarationRouteContract`
- `signal_only() -> WorthQueryDeclarationRouteContract`
- `relational_and_bridge() -> WorthQueryDeclarationRouteContract`
- `deferred_auto() -> WorthQueryDeclarationRouteContract`
- `required_relational_intent() -> WorthQueryDeclarationRouteContract`
- `relational_intent_forbidden() -> WorthQueryDeclarationRouteContract`
- `unresolved_mixed() -> WorthQueryDeclarationRouteContract`

Route-contract inspection:

- `allowed_route_families() -> &'static [WorthQueryLowerAuthorityRouteFamily]`
- `multiplicity() -> WorthQueryDeclarationRouteMultiplicity`
- `intent_requirement() -> WorthQueryDeclarationRouteIntentRequirement`
- `can_defer() -> bool`
- `signal_routed() -> bool`
- `reason() -> &'static str`

Route-intent variants:

- `Auto`
- `RelationalOnly`
- `BridgeOnly`
- `SignalOnly`
- `RelationalAndBridge`
- `DeferredRouting`

Route-plan input constructors:

- `WorthQueryDeclarationRoutePlanInput::admitted(progressed, evidence)`
- `WorthQueryDeclarationRoutePlanInput::with_intent(progressed, evidence, intent)`

Admitted-handle route-planning entry points:

- `plan_routes(subject) -> Result<WorthQueryDeclarationRoutePlan<D, I>, WorthQueryDeclarationRoutePlanTerminalError<D, I>>`
- `plan_routes_checked(subject) -> WorthQueryDeclarationRoutePlanChecked<D, I>`
- `plan_routes_from_progressed(progressed) -> Result<WorthQueryDeclarationRoutePlan<D, I>, WorthQueryDeclarationRoutePlanTerminalError<D, I>>`
- `plan_routes_from_progressed_with_intent(progressed, intent) -> Result<WorthQueryDeclarationRoutePlan<D, I>, WorthQueryDeclarationRoutePlanTerminalError<D, I>>`
- `orchestrate_routes_from_progressed(progressed) -> Result<WorthQueryDeclarationRoutePlan<D, I>, WorthQueryDeclarationRoutePlanTerminalError<D, I>>`
- `orchestrate_routes_from_progressed_with_intent(progressed, intent) -> Result<WorthQueryDeclarationRoutePlan<D, I>, WorthQueryDeclarationRoutePlanTerminalError<D, I>>`
- `bind_receipt_from_target(request) -> WorthQueryBindingOutcome<WorthQueryDeclarationReceiptInput<D, I>>`
- `bind_receipt_from_target_checked(request) -> WorthQueryBindingChecked<WorthQueryDeclarationReceiptInput<D, I>>`
- `bind_receipt_from_target_proof(request) -> WorthQueryBindingTranscript<WorthQueryDeclarationReceiptInput<D, I>>`
- `declare_review_progress_describe_and_plan(input) -> Result<WorthQueryDeclarationRoutePlan<D, I>, WorthQueryDeclarationEntryRoutePlanError<D, I>>`

Checked route-plan outcomes:

- `WorthQueryDeclarationRoutePlanChecked::Planned(WorthQueryDeclarationRoutePlan<D, I>)`
- `WorthQueryDeclarationRoutePlanChecked::Deferred(WorthQueryDeclarationRoutePlanDeferred<D, I>)`
- `WorthQueryDeclarationRoutePlanChecked::Denied(WorthQueryDeclarationRoutePlanDenied<D, I>)`
- `WorthQueryDeclarationRoutePlanChecked::Failed(WorthQueryDeclarationRoutePlanFailed<D, I>)`

Terminal errors:

- `WorthQueryDeclarationRoutePlanTerminalError::Deferred(...)`
- `WorthQueryDeclarationRoutePlanTerminalError::Denied(...)`
- `WorthQueryDeclarationRoutePlanTerminalError::Failed(...)`

Combined common-lane error:

- `WorthQueryDeclarationEntryRoutePlanError::Entry(WorthQueryDeclarationEntryProgressionError<D, I>)`
- `WorthQueryDeclarationEntryRoutePlanError::RoutePlan(WorthQueryDeclarationRoutePlanTerminalError<D, I>)`

Route-plan inspection:

- `class() -> WorthQueryDeclarationRoutePlanClass`
- `route_set() -> &WorthQueryDeclarationRouteSet`
- `primary_route() -> Option<&WorthQueryDeclarationRouteSegment>`
- `route_count() -> usize`
- `route_families() -> &[WorthQueryLowerAuthorityRouteFamily]`
- `route_intent() -> Option<WorthQueryDeclarationRouteIntent>`
- `declaration_family_key() -> &'static str`
- `handle_identity_digest() -> &str`
- `operating_context_identity_digest() -> &str`
- `declaration_digest() -> &str`
- `progression_digest() -> &str`
- `route_plan_digest() -> &str`
- `future_projection() -> &WorthQueryDeclarationFutureProjection`
- `binding_target() -> WorthQueryDeclarationRoutePlanBindingTarget`
- `foundational_evidence() -> &WorthQueryDeclarationFoundationalEvidence<D, I>`
- `progressed_declaration() -> &WorthQueryAdmittedDeclarationProgression<D, I>`
- `aspect_contract() -> &WorthQueryDeclarationAspectContract`
- `aspect_fit() -> WorthQueryDeclarationAspectFit`
- `aspect_publication() -> &WorthQueryDeclarationAspectPublication`
- `explain() -> &WorthQueryDeclarationRoutePlanExplanation`

Route-set and segment inspection:

- `route_count() -> usize`
- `routes() -> &[WorthQueryDeclarationRouteSegment]`
- `route_families() -> &[WorthQueryLowerAuthorityRouteFamily]`
- `family() -> WorthQueryLowerAuthorityRouteFamily`
- `reason() -> &str`

Explanation inspection:

- `route_contract_reason() -> &'static str`
- `retained_facts() -> &[String]`
- `route_segment_reasons() -> &[String]`
- `intent_reason() -> Option<&str>`

Denied route-plan inspection:

- `cause() -> WorthQueryDeclarationRoutePlanDenialCause`
- `reason() -> &'static str`
- `route_intent() -> Option<WorthQueryDeclarationRouteIntent>`
- `route_contract() -> WorthQueryDeclarationRouteContract`
- `declaration_family_key() -> &'static str`
- `progressed_declaration() -> &WorthQueryAdmittedDeclarationProgression<D, I>`
- `foundational_evidence() -> &WorthQueryDeclarationFoundationalEvidence<D, I>`

Denied route-plan causes:

- `WrongAdmittedWorld`
- `EvidenceMismatch`
- `MissingRequiredAspect`
- `AspectConflict`
- `IntentRequired`
- `IntentForbidden`
- `IntentConflictsWithRouteContract`
- `NoAllowedRoutes`
- `ForbiddenRouteCombination`

Deferred and failed inspection:

- `reason() -> &'static str`
- `route_intent() -> Option<WorthQueryDeclarationRouteIntent>`
- `route_contract() -> WorthQueryDeclarationRouteContract`
- `declaration_family_key() -> &'static str`
- `progressed_declaration() -> &WorthQueryAdmittedDeclarationProgression<D, I>`
- `foundational_evidence() -> &WorthQueryDeclarationFoundationalEvidence<D, I>`

Route families:

- `Relational`
- `Bridge`
- `Signal`
- `Mixed`
- `Deferred`
- `Forbidden`

Route-plan classes:

- `RelationalOnly`
- `BridgeOnly`
- `SignalOnly`
- `Mixed`

## Core Mental Model

Think of route planning as the first lower-authority decision boundary over
retained declaration proof:

1. the admitted handle proves the operating world
2. admitted progression proves the declaration can continue
3. foundational evidence publishes matching retained declaration truth
4. the family route contract says which lower-authority families are allowed
5. Query materializes one explicit route set or one typed deferred/denied/failed
   outcome

Temporal and async declaration meaning does not create new public
lower-authority route families here. Query still routes through the ordinary
`Relational`, `Bridge`, `Signal`, and `Mixed` family vocabulary. The
future-sensitive difference is carried in `future_projection()` and in the
route-plan digest/explanation, so later bridge and continuation surfaces can
consume retained proof instead of rediscovering declaration posture.

If two declarations retain the same proof and the same route intent, they
should converge to the same route-plan digest. If admitted world, route
contract, or route intent differ, the plan should diverge honestly.

The route plan is also now one shared retained binding target. Receipt,
envelope, and later continuation surfaces should bind from this retained
artifact seam instead of reconstructing route meaning or inventing
route-local binding helpers.

That binding seam is explicitly aspect-aware:

- the route plan carries a route-scoped `aspect_contract()` derived from the
  admitted declaration contract, not from family labels alone
- `aspect_fit()` records whether foundational evidence actually satisfied that
  route-scoped semantic contract
- `aspect_publication()` records what later receipts and envelopes are allowed
  to publish from the route-backed slice without widening into unrelated
  declaration semantics

## How It Executes

1. define `route_contract()` on the family marker when the default deferred
   route posture is not enough
2. produce admitted progression through declaration progression
3. produce matching foundational evidence from that admitted progression
4. optionally choose one typed route intent
5. call one of the route-planning entry points on the admitted handle
6. Query verifies:
   - handle identity match
   - operating-context identity match
   - declaration digest match
   - progression digest match
   - admitted progression evidence class
7. Query applies:
   - the family route contract
   - the caller route intent
   - the retained declaration proof
   - the foundational evidence aspect coverage against the route-scoped aspect
     contract
8. Query returns one planned, deferred, denied, or failed route artifact

The convenience lane `declare_review_progress_describe_and_plan(...)` preserves
the same structure. It still performs:

1. family admission
2. canonicalization
3. legality review
4. progression
5. foundational evidence materialization
6. route planning

## Small Example

```rust
use worth_query::facade::foundation::{
    WorthQueryDeclarationRoutePlanChecked, WorthQueryDeclarationRoutePlanInput,
};

let progressed = handle.declare_review_and_progress(
    geometry_session.attach_face_material_for_active_selection()?,
)?;
let evidence = handle.describe_foundational(
    WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
        progressed.clone(),
    ),
)?;

match handle.plan_routes_checked(
    WorthQueryDeclarationRoutePlanInput::admitted(progressed, evidence),
) {
    WorthQueryDeclarationRoutePlanChecked::Planned(plan) => {
        assert!(plan.route_count() >= 1);
    }
    other => panic!("unexpected route-plan outcome: {:?}", std::mem::discriminant(&other)),
}
```

## Real Example

```rust
use worth_query::facade::foundation::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily,
    WorthQueryConfigSectionFamily, WorthQueryDeclarationCanonicalEntry,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationFoundationalEvidenceInput,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract,
    WorthQueryDeclarationRouteContract, WorthQueryDeclarationRouteIntent,
    WorthQueryDeclarationRoutePlanChecked, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext, WorthQueryNeighborhoodCapableGrouping,
    WorthQueryRelationalTruthAuthority, WorthQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str { "worth.geometry" }
    fn display_name(&self) -> &'static str { "Worth Geometry" }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CollaborativeWorld;

impl WorthQueryDomainOperatingContext<GeometryDomain> for CollaborativeWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        "geometry.collaborative".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AttachFaceMaterial;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AttachFaceMaterial {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "attach-face-material"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_and_bridge()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AttachFaceMaterialAssignment {
    face_ref: &'static str,
    material_profile_ref: &'static str,
}

impl WorthQueryDeclarationInput<GeometryDomain> for AttachFaceMaterialAssignment {
    type Family = AttachFaceMaterial;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            WorthQueryDeclarationCanonicalEntry::text("face_ref", self.face_ref),
            WorthQueryDeclarationCanonicalEntry::text(
                "material_profile_ref",
                self.material_profile_ref,
            ),
        ]
    }
}

let query = WorthQueryApplicationFacade::runtime_backed_default();
let handle = query
    .domain(GeometryDomain)
    .with_operating_context(CollaborativeWorld)
    .validate()?
    .admit()?;

let progressed = handle.declare_review_and_progress(AttachFaceMaterialAssignment {
    face_ref: "face:loading-bay-west",
    material_profile_ref: "material-profile:fire-rated-primer",
})?;
let evidence = handle.describe_foundational(
    WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
        progressed.clone(),
    ),
)?;

match handle.plan_routes_checked(
    WorthQueryDeclarationRoutePlanInput::with_intent(
        progressed,
        evidence,
        WorthQueryDeclarationRouteIntent::Auto,
    ),
) {
    WorthQueryDeclarationRoutePlanChecked::Planned(plan) => {
        assert_eq!(plan.declaration_family_key(), "attach-face-material");
        assert_eq!(plan.route_count(), 2);
        assert_eq!(plan.route_families().len(), 2);
    }
    WorthQueryDeclarationRoutePlanChecked::Denied(denial) => {
        let _ = denial.cause();
    }
    WorthQueryDeclarationRoutePlanChecked::Deferred(plan) => {
        let _ = plan.reason();
    }
    WorthQueryDeclarationRoutePlanChecked::Failed(plan) => {
        let _ = plan.reason();
    }
}
```

What this example is showing:

- route planning starts from retained admitted progression plus matching
  foundational evidence
- caller route intent stays typed
- explicit plural routes are public truth, not an internal detail
- explanation and denial surfaces stay at the route-plan semantic level
- orchestration may later choose lean or richer publication, but the route plan
  truth itself stays the same

## How It Relates To Other Features

- [Declaration Progression](./declaration-progression.md) produces the admitted
  declaration proof route planning consumes
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
  describes the retained declaration truth route planning requires
- [Configured Domain Handles](./configured-domain-handles.md) retain the
  admitted-world identity route planning must not rediscover
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md) bind
  retained crossing posture from this route artifact instead of reopening route
  planning
- [Typed Binding Pipeline](./typed-binding-pipeline.md) lets you turn a
  retained route-plan target into the next receipt input without reconstructing
  the route-plan input family yourself

## Aspect Semantics

Route plans expose which semantic slices are required for
route admission, which are preserved into route explanation, and which are
incompatible with the requested route intent. Later receipts, envelopes, and
orchestration must bind from that route-scoped semantic contract rather than
from broad route artifact shape. The route surface exposes
`aspect_contract()`, `aspect_fit()`, and `aspect_publication()` for exactly
that reason.

## Inspection And Debugging

Use these surfaces when reviewing route plans:

- `plan_routes_checked(...)`
- `plan_routes_from_progressed(...)`
- `declare_review_progress_describe_and_plan(...)`
- `plan.route_set()`
- `plan.route_families()`
- `plan.route_plan_digest()`
- `plan.binding_target()`
- `plan.explain()`
- `denial.cause()`
- `denial.reason()`

Use them to answer:

- whether the declaration planned one, many, or no lower-authority routes
- whether a denial came from wrong admitted world, evidence mismatch, or route
  intent posture
- whether two equivalent retained-proof paths converged to the same route-plan
  digest
- whether route divergence came from world identity, route contract, or caller
  route intent
- which retained route identity later receipt/envelope/continuation consumers
  should bind to directly

## Anti-Patterns

- attempting route planning from legality evidence alone
- attempting route planning from canonical declarations alone
- rebuilding route meaning from family labels or payload folklore
- treating `primary_route()` as more authoritative than the route set
- using raw strings instead of `WorthQueryDeclarationRouteIntent`

## Current Limits

Declaration route planning now produces one explicit Query-owned route plan
over retained declaration proof. It still does not perform:

- lower-authority boundary crossing
- public Query boundary-receipt materialization
- public Query boundary-envelope materialization
- public Query relational truth routing
- public Query bridge continuation routing
- public Query signal compatibility classification
- grouped declaration lowering

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Typed Binding Pipeline](./typed-binding-pipeline.md)
- [Declaration Progression](./declaration-progression.md)
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Declaration Relational Truth Routing](./declaration-relational-truth-routing.md)
- [Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
- [Declaration Signal Compatibility](./declaration-signal-compatibility.md)
- [Declaration Legality](./declaration-legality.md)
- [Domain Capabilities](./README.md)
