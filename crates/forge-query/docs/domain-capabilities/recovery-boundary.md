# Recovery Boundary

## What This Feature Is

The recovery boundary is the Query-owned surface for turning a typed stop into
one typed recovery brief.

Use it when a declaration-entry, continuation, signal-orchestration, or
contribution-composed call did not finish and your app needs to answer:

- what stopped
- where it stopped
- who owns the fix
- what the next supported repair step is

This feature does not retry work for you. It explains the stop honestly and
returns one typed recovery request shape that keeps the same context attached.

## Why You Use It

- keep `Deferred`, `Stale`, `RebindRequired`, `WrongWorld`, `WrongHandle`,
  `BasisMismatch`, `ContributionDenied`, and `Refused` distinct
- get one machine-readable next-step surface instead of parsing error strings
- preserve route-plan and receipt denial causes when those are the real stop
- keep ordinary, checked, and proof-visible non-success lanes aligned
- tell your app whether the fix belongs to world selection, input narrowing,
  contribution intent, signal posture, or an explicit handoff boundary

## Stable Entry Points

Core recovery types:

- `ForgeQueryRecoveryBrief`
- `ForgeQueryRecoveryStopFamily`
- `ForgeQueryRecoveryStopKind`
- `ForgeQueryRecoveryAuthoritySurface`
- `ForgeQueryRecoveryAction`
- `ForgeQueryRecoveryExplanation`
- `ForgeQueryRecoveryRequest`
- `ForgeQueryRecoveryRequestKind`

Admitted-handle recovery entry points:

- `recover_from_outcome(...)`
- `recover_from_declaration_entry_checked(...)`
- `recover_from_declaration_entry_proof(...)`
- `recover_from_declaration_route_plan_checked(...)`
- `recover_from_declaration_receipt_checked(...)`
- `recover_from_prepared_continuation_checked(...)`
- `recover_from_prepared_continuation_proof(...)`
- `recover_from_continuation_execution_checked(...)`
- `recover_from_continuation_execution_proof(...)`
- `recover_from_signal_compatibility_checked(...)`
- `recover_from_signal_compatibility_proof(...)`
- `recover_from_contribution_composed_checked(...)`
- `recover_from_contribution_composed_proof(...)`

Good to know:

- `recover_from_outcome(...)` is the compact public lane for ordinary outcomes
- checked and proof recovery entry points preserve stronger stop evidence
- recovery requests are guidance-grade typed requests, not automatic repair
  execution

## Core Mental Model

Think of the recovery boundary as a public translation layer over typed stop
posture that already exists elsewhere in Query.

The recovery layer does not invent a second denial system. It projects the
real stop into one recovery brief that answers four questions:

1. `stop_family()`
   What kind of surface stopped?
2. `stop_kind()`
   What kind of stop was it?
3. `authority_surface()`
   Which boundary owns the fix?
4. `recommended_action()`
   What kind of repair should the caller attempt next?

The important rule is:

- recovery advice must stay narrower than the proof it came from

That means:

- prepared continuation is not explained as execution failure
- readiness is not treated as proof of what happened in one concrete run
- declaration denial and contribution denial stay distinct
- signal `Compatible`, `Prepared`, and `BasisMismatch` are not collapsed into
  one generic "not ready"

## How It Executes

The recovery boundary accepts one of three source shapes:

1. an ordinary outcome
2. a checked result
3. a proof-visible transcript

It then:

1. maps the stop into one recovery stop family and stop kind
2. selects the authority surface that actually owns the fix
3. picks one recommended recovery action
4. builds one `ForgeQueryRecoveryExplanation`
5. attaches that same explanation to one typed `ForgeQueryRecoveryRequest`

`ForgeQueryRecoveryExplanation` is the retained context surface. It can carry:

- checked topology
- orchestration stop stage
- retained digest
- refusal class
- route governing reason
- route denial cause
- receipt governing reason
- receipt denial cause
- contribution digest

That same explanation is available from both:

- `brief.route_sensitive_explanation()`
- `brief.recovery_request().explanation()`

So the "what happened?" and "what should I do next?" lanes stay synchronized.

## Small Example

```rust
let outcome = handle.prepare_continuation_from_target_outcome(
    ForgeQueryResolveContinuationFromTargetRequest::new(
        envelope,
        PreparePreviewForActiveFaceSelection::aspect_contract(),
    ),
);

match outcome {
    ForgeQueryOrdinaryOutcome::Bound(prepared) => {
        let _ = prepared.prepared_digest();
    }
    other => {
        let brief = handle
            .recover_from_outcome(&other)
            .expect("ordinary non-success should yield a recovery brief");

        assert_eq!(
            brief.authority_surface(),
            ForgeQueryRecoveryAuthoritySurface::AdmittedOperatingWorld,
        );
        assert_eq!(
            brief.recommended_action(),
            ForgeQueryRecoveryAction::CorrectWorld,
        );
    }
}
```

Use this when you already chose the ordinary lane and want one compact recovery
answer without switching back to checked or proof manually.

## Real Example

```rust
let progression = handle.declare_review_and_progress(
    geometry_session.publish_boundary_change_for_active_face_selection()?,
)?;
let foundational = handle.describe_foundational(progression.clone())?;
let route_checked = handle.plan_routes_checked(
    ForgeQueryDeclarationRoutePlanInput::admitted(progression, foundational),
);

if let Some(brief) = handle.recover_from_declaration_route_plan_checked(route_checked) {
    assert_eq!(
        brief.stop_family(),
        ForgeQueryRecoveryStopFamily::DeclarationRoutePlan,
    );
    assert_eq!(
        brief.recommended_action(),
        ForgeQueryRecoveryAction::UseExplicitHandoff,
    );

    let explanation = brief.route_sensitive_explanation();
    let _ = explanation.route_denial_cause();
    let _ = explanation.route_governing_reason();

    let request = brief.recovery_request();
    let _ = request.kind();
    let _ = request.explanation().route_denial_cause();
}
```

What this example is showing:

- the stop came from a route-plan checked lane, not a generic error bucket
- the recovery brief preserved the typed route denial cause
- the recovery request carries the same explanation context forward
- Query tells the caller to use an explicit handoff instead of pretending it
  can silently repair the route intent itself

## How It Relates To Other Features

- [Ordinary Outcomes](./ordinary-outcomes.md) are the compact public stop lane
  that `recover_from_outcome(...)` projects from.
- [Continuation Pipeline](./continuation-pipeline.md) owns prepared/executed
  continuation truth; recovery only explains how to respond when those lanes
  stop.
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
  owns signal-facing `Compatible`, `Prepared`, and typed non-success posture;
  recovery explains who owns the fix when that lane stops.
- [Contribution-Composed Orchestration](./contribution-composed-orchestration.md)
  owns the distinction between declaration-side and contribution-side stop
  posture; recovery preserves that distinction.
- [Declaration Entry Readiness](./declaration-entry-readiness.md) is family-level
  support posture, not concrete run recovery. Use readiness before work when
  you want seam posture. Use recovery after a real stop when you need the next
  repair step.
- [Orchestration Inventory](./orchestration-inventory.md) is the anti-drift
  registry that keeps public recovery lanes aligned with the shipped
  orchestration surface.

## Inspection And Debugging

Use `ForgeQueryRecoveryBrief` for the compact decision surface:

- `stop_family()`
- `stop_kind()`
- `authority_surface()`
- `recommended_action()`
- `reason()`
- `route_sensitive_explanation()`
- `recovery_request()`

Use `ForgeQueryRecoveryExplanation` when you need the retained context:

- `checked_topology()`
- `stop_stage()`
- `retained_digest()`
- `refusal_class()`
- `route_governing_reason()`
- `route_denial_cause()`
- `receipt_governing_reason()`
- `receipt_denial_cause()`
- `contribution_digest()`

Use `ForgeQueryRecoveryRequest` when the next layer in your app wants one typed
repair intent instead of a prose explanation:

- `kind()`
- `explanation()`

## Anti-Patterns

- flattening `Deferred`, `Denied`, `Stale`, and `RebindRequired` into one retry
  path
- treating prepared-but-not-executed continuation as execution failure
- using family-level readiness as if it explained one concrete failed run
- teaching recovery requests as if Query already executes the repair
- parsing `reason()` when `stop_kind()`, `authority_surface()`, and
  `recommended_action()` already carry the machine decision
- merging declaration-side denial and contribution-side denial into the same
  app behavior

## Current Limits

- recovery currently ships as an explanation and typed request surface; it does
  not apply repairs automatically
- the ordinary lane is the generic public front door, but checked and proof
  recovery lanes still carry stronger evidence
- recovery currently covers ordinary, declaration-entry, route-plan, receipt,
  continuation, signal-orchestration, and contribution-composed stop surfaces
- readiness and support reports may inform the next step, but they are not
  treated as proof of one concrete failure run

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Ordinary Outcomes](./ordinary-outcomes.md)
- [Declaration Entry Readiness](./declaration-entry-readiness.md)
- [Continuation Pipeline](./continuation-pipeline.md)
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
- [Contribution-Composed Orchestration](./contribution-composed-orchestration.md)
- [Orchestration Inventory](./orchestration-inventory.md)
- [Domain Capabilities](./README.md)
