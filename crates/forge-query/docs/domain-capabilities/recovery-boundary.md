# Recovery Boundary

## What This Feature Is

The recovery boundary is the Query-owned surface for turning a typed stop into
one typed next-step answer.

Use it when a declaration-entry, continuation, signal, contribution-composed,
or grouped run did not finish and your app needs to know:

- what stopped
- which feature family it came from
- whether the stop is stale, mismatched, unsupported, denied, or conflicted
- whether the next move is refresh, rebind, declaration repair, contribution
  review, or manual inspection

This feature does not retry work for you. It gives you one recovery brief and
one typed recovery request that stay attached to the same retained explanation.

## Why You Use It

- keep declaration, continuation, signal, contribution, and grouped stops
  distinct
- keep aspect-native failures visible instead of flattening them into generic
  denial
- distinguish stale basis, basis mismatch, wrong world, wrong handle, and
  authority mismatch
- preserve proof strength so your app knows whether the answer came from the
  ordinary lane, a checked artifact, or a proof-visible transcript
- reuse foundational support and diagnostics vocabulary without making your app
  read lower-crate artifacts directly

## Stable Entry Points

Most callers only need two starting points:

- `recover_from_outcome(...)` when you are already on the ordinary lane
- the matching `recover_from_..._checked(...)` or `recover_from_..._proof(...)`
  method when you need stronger retained context

The main recovery types are:

- `ForgeQueryRecoveryBrief`
- `ForgeQueryRecoveryExplanation`
- `ForgeQueryRecoveryGroupedMemberContext`
- `ForgeQueryRecoveryRequest`
- `ForgeQueryRecoveryRequestKind`
- `ForgeQueryRecoveryStopFamily`
- `ForgeQueryRecoveryStopKind`
- `ForgeQueryRecoveryAuthoritySurface`
- `ForgeQueryRecoveryAction`
- `ForgeQueryRecoverySourceFamily`
- `ForgeQueryRecoveryAspectPosture`
- `ForgeQueryRecoveryBasisPosture`
- `ForgeQueryRecoveryEvidenceStrength`
- `ForgeQueryRecoveryConflictPosture`
- `ForgeQueryRecoveryMaterialization`
- `ForgeQueryRecoveryFoundationalSupportContext`
- `ForgeQueryRecoveryFoundationalDiagnosticContext`

Common recovery entry points:

- `forge_query_recovery_brief_from_ordinary_outcome(...)`
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
- `recover_from_grouped_orchestration_checked(...)`
- `recover_from_grouped_orchestration_proof(...)`

The recovery boundary also has matching checked/proof entry points for:

- declaration entry
- route plans
- receipts
- prepared continuations
- continuation execution
- signal compatibility
- contribution-composed orchestration
- grouped orchestration

Grouped routes, receipts, envelopes, and grouped contributions are currently
best treated as rich inspection surfaces. When you need one typed next-step
repair answer with retained member-local grouped context, the grouped
orchestration checked/proof recovery lane is the strongest grouped recovery
surface today.

## Core Mental Model

Recovery is a projection layer over proof you already have. It does not invent a
second denial system.

Every recovery brief answers four questions first:

1. `stop_family()`: which public feature stopped?
2. `stop_kind()`: what kind of stop was it?
3. `authority_surface()`: who owns the fix?
4. `recommended_action()`: what should the caller do next?

The explanation behind that brief then answers the follow-up questions:

- `source_family()`: was this declaration, continuation, signal,
  contribution-composed, or grouped?
- `aspect_posture()`: was aspect truth irrelevant, required, retained, or
  readmission-sensitive?
- `basis_posture()`: is the problem stale basis, basis mismatch, reduced basis,
  or unknown?
- `evidence_strength()`: did this answer come from the ordinary lane, a checked
  artifact, or proof-visible retained context?

## How It Executes

The recovery boundary accepts one of three source shapes:

1. an ordinary outcome
2. a checked result
3. a proof-visible transcript

It then:

1. maps the stop into one `ForgeQueryRecoveryStopFamily`
2. maps the stop into one `ForgeQueryRecoveryStopKind`
3. chooses the authority surface that owns the repair
4. chooses one recommended action
5. builds one `ForgeQueryRecoveryExplanation`
6. attaches that same explanation to one `ForgeQueryRecoveryRequest`

The checked and proof entry points keep stronger source-family data when it
exists. For example:

- continuation recovery keeps aspect-sensitive readmission posture
- signal recovery keeps required-aspect posture
- contribution-composed recovery can keep retained declaration aspect truth and
  one contribution intent descriptor
- grouped recovery can keep the stopped member index, role, and member-local
  aspect record when the grouped lane fails on one member

## Small Example

```rust
let outcome = handle.orchestrate_declaration_with_contributions_outcome(input);

if let Some(recovery) = forge_query_recovery_brief_from_ordinary_outcome(&outcome) {
    assert_eq!(
        recovery.source_family(),
        ForgeQueryRecoverySourceFamily::ContributionComposed,
    );
    assert_eq!(
        recovery.recommended_action(),
        ForgeQueryRecoveryAction::ReviewContributionIntent,
    );
}
```

Use this when you are already on the compact ordinary lane and want the next
repair step without switching back to checked/proof manually.

## Real Example

```rust
let proof = handle.orchestrate_signal_compatibility_proof(input);

let recovery = handle
    .recover_from_signal_compatibility_proof(proof)
    .expect("non-success should yield a recovery brief");

assert_eq!(
    recovery.source_family(),
    ForgeQueryRecoverySourceFamily::SignalCompatibility,
);
assert_eq!(
    recovery.aspect_posture(),
    ForgeQueryRecoveryAspectPosture::RequiredContract,
);

let explanation = recovery.explanation();
let _ = (
    explanation.evidence_strength(),
    explanation.basis_posture(),
    explanation.diagnostic_outcome_kind(),
    explanation.diagnostic_denial_class(),
);
```

What this is showing:

- recovery keeps the source-family identity instead of flattening everything
  into generic denial
- signal aspect failures stay aspect-native
- proof-visible recovery can carry stronger evidence posture than ordinary
  recovery
- grouped checked/proof recovery can also carry member-local grouped context
  that the ordinary lane does not keep

## How It Relates To Other Features

- [Ordinary Outcomes](./ordinary-outcomes.md) are the compact public stop lane.
- [Continuation Pipeline](./continuation-pipeline.md) owns prepared/executed
  continuation truth; recovery explains what to do when that truth stops.
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
  owns signal-facing compatibility and preparation truth; recovery tells you who
  owns the fix.
- [Contribution-Composed Orchestration](./contribution-composed-orchestration.md)
  owns declaration-plus-contribution proof; recovery keeps declaration-owned and
  contribution-owned repair paths distinct.
- [Orchestration Inventory](./orchestration-inventory.md) is the semantic
  registry that tells you which public surfaces participate in this recovery
  model.
- [Recovery Docs](./recovery/README.md) go deeper on aspect posture, support
  truth, and request meanings.

## Inspection And Debugging

Use `ForgeQueryRecoveryBrief` for the compact decision surface:

- `stop_family()`
- `stop_kind()`
- `authority_surface()`
- `recommended_action()`
- `source_family()`
- `basis_posture()`
- `aspect_posture()`
- `evidence_strength()`
- `conflict_posture()`

Use `ForgeQueryRecoveryExplanation` when you need retained context:

- `checked_topology()`
- `stop_stage()`
- `retained_digest()`
- `route_governing_reason()`
- `route_denial_cause()`
- `receipt_governing_reason()`
- `receipt_denial_cause()`
- `contribution_digest()`
- `contribution_intent_descriptor()`
- `grouped_member_context()`
- `support_truth_kind()`
- `basis_disclosure()`
- `degraded_recovery_posture()`
- `diagnostic_outcome_kind()`
- `diagnostic_denial_class()`
- `materialization()`

Use `ForgeQueryRecoveryRequest` when the next layer in your app wants one typed
repair intent instead of re-deriving actions from prose.

When the stop came from grouped orchestration, `grouped_member_context()` is
the shortest path to the retained member-local witness. It tells you which
group member stopped, whether it was the seed or a later member, and which
member-local aspect record the grouped lane had already retained.

## Anti-Patterns

- treating aspect-native failures as generic denial
- retrying every stop the same way
- treating stale basis and basis mismatch as the same repair
- using support-grade recovery truth as if it were fresh proof-bearing
  readmission truth
- expecting recovery for a bound partial contribution artifact; inspect the
  bound artifact directly instead
- parsing `reason()` when the typed stop family, stop kind, and recommended
  action already exist

## Current Limits

- recovery is an explanation and next-step surface, not an automatic repair
  engine
- collaborative merge resolution is outside this surface; recovery preserves the
  distinctions a dedicated conflict or merge workflow would need
- route-plan and receipt recovery still carry thinner aspect context than the
  continuation, signal, and contribution-composed proof lanes
- recovery currently materializes a lean explanation surface rather than a
  heavy operator bundle by default

## Related Docs

- [Recovery Overview](./recovery/README.md)
- [Aspect-Native Recovery](./recovery/aspect-native-recovery.md)
- [Foundational Support And Evidence Strength](./recovery/foundational-support-and-evidence-strength.md)
- [Recovery Requests And Next-Step Actions](./recovery/recovery-requests-and-next-step-actions.md)
- [Ordinary Outcomes](./ordinary-outcomes.md)
- [Continuation Pipeline](./continuation-pipeline.md)
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
- [Contribution-Composed Orchestration](./contribution-composed-orchestration.md)
- [Orchestration Inventory](./orchestration-inventory.md)
