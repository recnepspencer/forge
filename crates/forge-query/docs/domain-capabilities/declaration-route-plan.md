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
- hand later crossing and receipt phases one retained route artifact instead of
  recomputing route meaning

Use [Declaration Boundary Receipts](./declaration-boundary-receipts.md) when
you need the Query-owned public crossing artifact that records what followed
from this route plan.
Use [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md) when
you need the self-describing public crossing story that carries retained
evidence, route truth, and receipt truth together.

## Stable Entry Points

- `ForgeQueryDeclarationRouteContract`
- `ForgeQueryDeclarationRouteIntent`
- `ForgeQueryDeclarationRoutePlanInput`
- `ForgeQueryDeclarationRoutePlan`
- `ForgeQueryDeclarationRoutePlanChecked`
- `ForgeQueryDeclarationRoutePlanDeferred`
- `ForgeQueryDeclarationRoutePlanDenied`
- `ForgeQueryDeclarationRoutePlanFailed`
- `ForgeQueryDeclarationRoutePlanDenialCause`
- `ForgeQueryDeclarationRoutePlanExplanation`
- `ForgeQueryDeclarationRouteSegment`
- `ForgeQueryDeclarationRouteSet`
- `ForgeQueryLowerAuthorityRouteFamily`
- `ForgeQueryAdmittedConfiguredDomainHandle::plan_routes(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::plan_routes_checked(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::plan_routes_from_progressed(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::plan_routes_from_progressed_with_intent(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::declare_review_progress_describe_and_plan(...)`

Good to know:

- route planning starts from admitted progression plus matching foundational
  evidence, not from legality alone
- the admitted handle stays the entry surface because retained admitted-world
  proof still matters
- route planning exposes an explicit route set from day one
- `primary_route()` is convenience only; the route set is authoritative

## API Reference

Family marker contract:

- `route_contract() -> ForgeQueryDeclarationRouteContract`

Route-contract presets:

- `relational_only() -> ForgeQueryDeclarationRouteContract`
- `bridge_only() -> ForgeQueryDeclarationRouteContract`
- `signal_only() -> ForgeQueryDeclarationRouteContract`
- `relational_and_bridge() -> ForgeQueryDeclarationRouteContract`
- `deferred_auto() -> ForgeQueryDeclarationRouteContract`
- `required_relational_intent() -> ForgeQueryDeclarationRouteContract`
- `relational_intent_forbidden() -> ForgeQueryDeclarationRouteContract`
- `unresolved_mixed() -> ForgeQueryDeclarationRouteContract`

Route-contract inspection:

- `allowed_route_families() -> &'static [ForgeQueryLowerAuthorityRouteFamily]`
- `multiplicity() -> ForgeQueryDeclarationRouteMultiplicity`
- `intent_requirement() -> ForgeQueryDeclarationRouteIntentRequirement`
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

- `ForgeQueryDeclarationRoutePlanInput::admitted(progressed, evidence)`
- `ForgeQueryDeclarationRoutePlanInput::with_intent(progressed, evidence, intent)`

Admitted-handle route-planning entry points:

- `plan_routes(subject) -> Result<ForgeQueryDeclarationRoutePlan<D, I>, ForgeQueryDeclarationRoutePlanTerminalError<D, I>>`
- `plan_routes_checked(subject) -> ForgeQueryDeclarationRoutePlanChecked<D, I>`
- `plan_routes_from_progressed(progressed) -> Result<ForgeQueryDeclarationRoutePlan<D, I>, ForgeQueryDeclarationRoutePlanTerminalError<D, I>>`
- `plan_routes_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationRoutePlan<D, I>, ForgeQueryDeclarationRoutePlanTerminalError<D, I>>`
- `declare_review_progress_describe_and_plan(input) -> Result<ForgeQueryDeclarationRoutePlan<D, I>, ForgeQueryDeclarationEntryRoutePlanError<D, I>>`

Checked route-plan outcomes:

- `ForgeQueryDeclarationRoutePlanChecked::Planned(ForgeQueryDeclarationRoutePlan<D, I>)`
- `ForgeQueryDeclarationRoutePlanChecked::Deferred(ForgeQueryDeclarationRoutePlanDeferred<D, I>)`
- `ForgeQueryDeclarationRoutePlanChecked::Denied(ForgeQueryDeclarationRoutePlanDenied<D, I>)`
- `ForgeQueryDeclarationRoutePlanChecked::Failed(ForgeQueryDeclarationRoutePlanFailed<D, I>)`

Terminal errors:

- `ForgeQueryDeclarationRoutePlanTerminalError::Deferred(...)`
- `ForgeQueryDeclarationRoutePlanTerminalError::Denied(...)`
- `ForgeQueryDeclarationRoutePlanTerminalError::Failed(...)`

Combined common-lane error:

- `ForgeQueryDeclarationEntryRoutePlanError::Entry(ForgeQueryDeclarationEntryProgressionError<D, I>)`
- `ForgeQueryDeclarationEntryRoutePlanError::RoutePlan(ForgeQueryDeclarationRoutePlanTerminalError<D, I>)`

Route-plan inspection:

- `class() -> ForgeQueryDeclarationRoutePlanClass`
- `route_set() -> &ForgeQueryDeclarationRouteSet`
- `primary_route() -> Option<&ForgeQueryDeclarationRouteSegment>`
- `route_count() -> usize`
- `route_families() -> &[ForgeQueryLowerAuthorityRouteFamily]`
- `route_intent() -> Option<ForgeQueryDeclarationRouteIntent>`
- `declaration_family_key() -> &'static str`
- `handle_identity_digest() -> &str`
- `operating_context_identity_digest() -> &str`
- `declaration_digest() -> &str`
- `progression_digest() -> &str`
- `route_plan_digest() -> &str`
- `foundational_evidence() -> &ForgeQueryDeclarationFoundationalEvidence<D, I>`
- `progressed_declaration() -> &ForgeQueryAdmittedDeclarationProgression<D, I>`
- `explain() -> &ForgeQueryDeclarationRoutePlanExplanation`

Route-set and segment inspection:

- `route_count() -> usize`
- `routes() -> &[ForgeQueryDeclarationRouteSegment]`
- `route_families() -> &[ForgeQueryLowerAuthorityRouteFamily]`
- `family() -> ForgeQueryLowerAuthorityRouteFamily`
- `reason() -> &str`

Explanation inspection:

- `route_contract_reason() -> &'static str`
- `retained_facts() -> &[String]`
- `route_segment_reasons() -> &[String]`
- `intent_reason() -> Option<&str>`

Denied route-plan inspection:

- `cause() -> ForgeQueryDeclarationRoutePlanDenialCause`
- `reason() -> &'static str`
- `route_intent() -> Option<ForgeQueryDeclarationRouteIntent>`
- `route_contract() -> ForgeQueryDeclarationRouteContract`
- `declaration_family_key() -> &'static str`
- `progressed_declaration() -> &ForgeQueryAdmittedDeclarationProgression<D, I>`
- `foundational_evidence() -> &ForgeQueryDeclarationFoundationalEvidence<D, I>`

Denied route-plan causes:

- `WrongAdmittedWorld`
- `EvidenceMismatch`
- `IntentRequired`
- `IntentForbidden`
- `IntentConflictsWithRouteContract`
- `NoAllowedRoutes`
- `ForbiddenRouteCombination`

Deferred and failed inspection:

- `reason() -> &'static str`
- `route_intent() -> Option<ForgeQueryDeclarationRouteIntent>`
- `route_contract() -> ForgeQueryDeclarationRouteContract`
- `declaration_family_key() -> &'static str`
- `progressed_declaration() -> &ForgeQueryAdmittedDeclarationProgression<D, I>`
- `foundational_evidence() -> &ForgeQueryDeclarationFoundationalEvidence<D, I>`

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

If two declarations retain the same proof and the same route intent, they
should converge to the same route-plan digest. If admitted world, route
contract, or route intent differ, the plan should diverge honestly.

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
use forge_query::facade::{
    ForgeQueryDeclarationRoutePlanChecked, ForgeQueryDeclarationRoutePlanInput,
};

let progressed = handle.declare_review_and_progress(
    SplitEdgeAtMidpoint { edge_ref: "edge:42" },
)?;
let evidence = handle.describe_foundational(
    ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
        progressed.clone(),
    ),
)?;

match handle.plan_routes_checked(
    ForgeQueryDeclarationRoutePlanInput::admitted(progressed, evidence),
) {
    ForgeQueryDeclarationRoutePlanChecked::Planned(plan) => {
        assert!(plan.route_count() >= 1);
    }
    other => panic!("unexpected route-plan outcome: {:?}", std::mem::discriminant(&other)),
}
```

## Real Example

```rust
use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationFoundationalEvidenceInput,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDeclarationRouteIntent,
    ForgeQueryDeclarationRoutePlanChecked, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str { "worth.geometry" }
    fn display_name(&self) -> &'static str { "Worth Geometry" }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CollaborativeWorld;

impl ForgeQueryDomainOperatingContext<GeometryDomain> for CollaborativeWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        "geometry.collaborative".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SplitEdge;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdge {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_and_bridge()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SplitEdgeAtMidpoint {
    edge_ref: &'static str,
}

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeAtMidpoint {
    type Family = SplitEdge;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
    }
}

let query = ForgeQueryApplicationFacade::runtime_backed_default();
let handle = query
    .domain(GeometryDomain)
    .with_operating_context(CollaborativeWorld)
    .validate()?
    .admit()?;

let progressed = handle.declare_review_and_progress(SplitEdgeAtMidpoint {
    edge_ref: "edge:42",
})?;
let evidence = handle.describe_foundational(
    ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
        progressed.clone(),
    ),
)?;

match handle.plan_routes_checked(
    ForgeQueryDeclarationRoutePlanInput::with_intent(
        progressed,
        evidence,
        ForgeQueryDeclarationRouteIntent::Auto,
    ),
) {
    ForgeQueryDeclarationRoutePlanChecked::Planned(plan) => {
        assert_eq!(plan.declaration_family_key(), "split-edge");
        assert_eq!(plan.route_count(), 2);
        assert_eq!(plan.route_families().len(), 2);
    }
    ForgeQueryDeclarationRoutePlanChecked::Denied(denial) => {
        let _ = denial.cause();
    }
    ForgeQueryDeclarationRoutePlanChecked::Deferred(plan) => {
        let _ = plan.reason();
    }
    ForgeQueryDeclarationRoutePlanChecked::Failed(plan) => {
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

## How It Relates To Other Features

- [Declaration Progression](./declaration-progression.md) produces the admitted
  declaration proof route planning consumes
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
  describes the retained declaration truth route planning requires
- [Configured Domain Handles](./configured-domain-handles.md) retain the
  admitted-world identity route planning must not rediscover

## Inspection And Debugging

Use these surfaces when reviewing route plans:

- `plan_routes_checked(...)`
- `plan_routes_from_progressed(...)`
- `declare_review_progress_describe_and_plan(...)`
- `plan.route_set()`
- `plan.route_families()`
- `plan.route_plan_digest()`
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

## Anti-Patterns

- attempting route planning from legality evidence alone
- attempting route planning from canonical declarations alone
- rebuilding route meaning from family labels or payload folklore
- treating `primary_route()` as more authoritative than the route set
- using raw strings instead of `ForgeQueryDeclarationRouteIntent`

## Current Limits

Declaration route planning now produces one explicit Query-owned route plan
over retained declaration proof. It still does not perform:

- lower-authority boundary crossing
- public Query boundary-receipt materialization
- public Query boundary-envelope materialization
- grouped declaration lowering

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Declaration Progression](./declaration-progression.md)
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Declaration Legality](./declaration-legality.md)
- [Domain Capabilities](./README.md)
