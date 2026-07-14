# Checked Recipe Progression

## What This Feature Is

This workflow shows how to progress a recipe while preserving denied, deferred, stale, rebind-required, and failed categories instead of flattening them away.

## Why You Use It

- progression may legitimately stop for more than one reason
- downstream logic cares why progression stopped
- you want checked readiness and admission to stay explicit

## Stable Entry Points

- pleasant lane:
  - `use worth_proof::prelude::*;`
  - `.try_resolve(...)`
  - `.try_resolve_ready(...)`
  - `.try_lower(...)`
  - `.try_lower_ready(...)`
  - `.try_admit(...)`
  - `.try_admit_ready(...)`
  - `.try_ready(...)`
  - `.try_ready_now(...)`
  - `.try_execute()`
  - `ProofOutcome`
- raw lane:
  - `use worth_proof::raw::*;`
  - `PreConstructionGate`
  - `TransitionReadiness`
  - `TransitionOutcome`
  - `resolve_lower_and_admit_recipe(...)`
  - `resolve_checked_lower_and_admit_recipe(...)`
  - `checked_admit_ready_and_execute_recipe(...)`

## Core Mental Model

This is the honest workflow when the answer is not just "it progressed" or "it failed."

Checked progression preserves distinctions such as:

- denied
- deferred
- stale
- rebind-required
- failed

That keeps the non-success topology usable by later code instead of forcing every lane through one generic error path.

## Pleasant Lane First

```rust
use worth_proof::prelude::*;

fn checked_progression(
    resolution_authority: worth_proof::AuthorityWitness<ResolutionAuthority>,
    lowering_capability: worth_proof::CapabilityWitness<LoweringCapability>,
    readiness_authority: worth_proof::AuthorityWitness<ReadinessAuthority>,
) {
    let outcome = recipe("payload")
        .try_resolve_ready(7_u8, resolution_authority)
        .try_lower_ready(lowering_capability)
        .try_ready_now("runtime admission", readiness_authority);

    match outcome.kind() {
        worth_proof::ProofOutcomeKind::Success => {}
        worth_proof::ProofOutcomeKind::Denied => {}
        worth_proof::ProofOutcomeKind::Deferred => {}
        worth_proof::ProofOutcomeKind::Stale => {}
        worth_proof::ProofOutcomeKind::RebindRequired => {}
        worth_proof::ProofOutcomeKind::Failed => {}
    }
}

struct ResolutionAuthority;
impl worth_proof::AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl worth_proof::CapabilityMarker for LoweringCapability {}

struct ReadinessAuthority;
impl worth_proof::AuthorityMarker for ReadinessAuthority {}
```

What this keeps visible:

- the checked lane stays in one grammar
- non-success categories are still first-class
- the DX surface lowers to the same checked substrate instead of inventing `Result`

## Equivalent Raw Surface

```rust
use worth_proof::raw::*;

fn checked_progression(
    resolution_authority: AuthorityWitness<ResolutionAuthority>,
    lowering_capability: CapabilityWitness<LoweringCapability>,
    readiness_authority: AuthorityWitness<ReadinessAuthority>,
) {
    let unresolved = Recipe::<Unresolved, _>::new("payload");
    let resolved = CheckedResolveRecipeTransition.transition(
        unresolved,
        PreConstructionGate::ready(RecipeResolutionContext::new(
            7_u8,
            resolution_authority,
        )),
    );
    let lowered = match resolved {
        TransitionOutcome::Success(resolved) => CheckedLowerRecipeTransition::<LoweringCapability>::new()
            .transition(resolved, TransitionReadiness::ready(lowering_capability)),
        TransitionOutcome::Denied(reason) => TransitionOutcome::denied(reason),
        TransitionOutcome::Deferred(reason) => TransitionOutcome::deferred(reason),
        TransitionOutcome::Stale(impossible) => match impossible {},
        TransitionOutcome::RebindRequired(impossible) => match impossible {},
        TransitionOutcome::Failed(impossible) => match impossible {},
    };
    let ready = match lowered {
        TransitionOutcome::Success(lowered) => CheckedAdmitExecutionReadyRecipeTransition
            .transition(
                lowered,
                TransitionReadiness::ready(ExecutionReadinessContext::new(
                    "runtime admission",
                    readiness_authority,
                )),
            ),
        TransitionOutcome::Denied(reason) => TransitionOutcome::denied(reason),
        TransitionOutcome::Deferred(reason) => TransitionOutcome::deferred(reason),
        TransitionOutcome::Stale(impossible) => match impossible {},
        TransitionOutcome::RebindRequired(recipe) => TransitionOutcome::rebind_required(recipe),
        TransitionOutcome::Failed(reason) => TransitionOutcome::failed(reason),
    };

    let _ = ready;
}

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}

struct ReadinessAuthority;
impl AuthorityMarker for ReadinessAuthority {}
```

Use the raw lane when:

- you need explicit gate and readiness construction in the same local view
- you are building a new domain-facing checked helper
- the pleasant chain stops being semantically obvious

## How It Relates To Other Features

- Use [Happy-Path Recipe Progression](./happy-path-recipe-progression.md) when only the straight-line success lane matters.
- Use [Staleness And Rebind](./staleness-and-rebind.md) when the important non-success categories are freshness-specific.
- Use [Runtime Readmission](./runtime-readmission.md) when checked progression must resume from a boundary-bridged lowered form.

## Inspection And Debugging

- inspect `ProofOutcome::kind()` first in the pleasant lane
- pattern match on raw `TransitionOutcome` when you need the full substrate directly
- inspect the gate construction site when a flow unexpectedly denies or defers
- if a flow should preserve stale or rebind-required but does not, check whether you accidentally used the success-only surfaces

## Anti-Patterns

- Do not flatten checked progression into `Result<T, E>` just because it is shorter.
- Do not use checked progression only to immediately discard its category information.
- Do not construct fake ready gates to bypass explicit denial or deferment handling.

## Related Docs

- [Checked Transitions](../features/checked-transitions.md)
- [Transition Outcomes](../features/transition-outcomes.md)
- [Preconstruction And Readiness Gates](../features/preconstruction-and-readiness-gates.md)
