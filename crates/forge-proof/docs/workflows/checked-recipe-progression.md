# Checked Recipe Progression

## What This Feature Is

This workflow shows how to progress a recipe while preserving denial, deferment, stale, rebind-required, and failure categories instead of flattening them away.

## Why You Use It

- progression may legitimately stop for more than one reason
- downstream logic cares why progression stopped
- you want checked readiness and admission to stay explicit

## Stable Entry Points

- `PreConstructionGate`
- `resolve_lower_and_admit_recipe(...)`
- `resolve_checked_lower_and_admit_recipe(...)`
- `CheckedAdmitExecutionReadyRecipeTransition`
- `ExecutionReadyAdmissionReadiness`
- `checked_admit_ready_and_execute_recipe(...)`
- `TransitionOutcome`

## Core Mental Model

This is the honest workflow when the answer is not just "it progressed" or "it failed."

Checked progression preserves distinctions such as:

- denied
- deferred
- stale
- rebind-required
- failed

That keeps the non-success topology usable by later code instead of forcing every lane through one generic error path.

## How It Executes

1. build a `PreConstructionGate` for resolution
2. optionally build `TransitionReadiness` for later stages
3. run the checked progression helper
4. inspect the resulting `TransitionOutcome`

## Small Example

```rust
use forge_proof::{PreConstructionGate, TransitionOutcome};

let denied = PreConstructionGate::<u64, _, &'static str>::denied("denied");
let _ = denied;

let outcome: TransitionOutcome<u64, &'static str> = TransitionOutcome::denied("denied");
assert!(matches!(outcome, TransitionOutcome::Denied("denied")));
```

This is the smallest honest example because checked flows begin by preserving non-success categories explicitly.

## Real Example

```rust
use forge_proof::{
    resolve_lower_and_admit_recipe, AdmitRecipeTransition, AuthorityMarker, AuthorityWitness,
    CapabilityMarker, CapabilityWitness, LowerRecipeTransition, PreConstructionGate, Recipe,
    RecipeResolutionContext, TransitionOutcome, Unresolved,
};

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}

struct AdmissionAuthority;
impl AuthorityMarker for AdmissionAuthority {}

fn checked_progression(
    resolution_authority: AuthorityWitness<ResolutionAuthority>,
    lowering_capability: CapabilityWitness<LoweringCapability>,
    admission_authority: AuthorityWitness<AdmissionAuthority>,
) {
    let unresolved = Recipe::<Unresolved, _>::new("payload");
    let denied =
        PreConstructionGate::<RecipeResolutionContext<u8, ResolutionAuthority>, _, &'static str>::denied(
            "denied",
        );
    let deferred =
        PreConstructionGate::<RecipeResolutionContext<u8, ResolutionAuthority>, &'static str, _>::deferred(
            "deferred",
        );
    let ready: PreConstructionGate<
        RecipeResolutionContext<u8, ResolutionAuthority>,
        &'static str,
        &'static str,
    > = PreConstructionGate::ready(RecipeResolutionContext::new(7_u8, resolution_authority));
    let lower = LowerRecipeTransition::new(lowering_capability);
    let admit = AdmitRecipeTransition::new(admission_authority);

    let denied_outcome = resolve_lower_and_admit_recipe(
        Recipe::<Unresolved, _>::new("payload"),
        denied,
        &lower,
        &admit,
    );
    let deferred_outcome = resolve_lower_and_admit_recipe(
        Recipe::<Unresolved, _>::new("payload"),
        deferred,
        &lower,
        &admit,
    );
    let ready_outcome = resolve_lower_and_admit_recipe(unresolved, ready, &lower, &admit);

    assert!(matches!(denied_outcome, TransitionOutcome::Denied("denied")));
    assert!(matches!(deferred_outcome, TransitionOutcome::Deferred("deferred")));
    assert!(matches!(ready_outcome, TransitionOutcome::Success(_)));
}
```

What gets retained:

- the reason the flow denied
- the reason the flow deferred
- the success lane when progression was actually ready

## How It Relates To Other Features

- Use [Happy-Path Recipe Progression](./happy-path-recipe-progression.md) when only the straight-line success lane matters.
- Use [Staleness And Rebind](./staleness-and-rebind.md) when the important non-success categories are freshness-specific.
- Use [Runtime Readmission](./runtime-readmission.md) when checked progression must resume from a boundary-bridged lowered form.

## Inspection And Debugging

- pattern match on `TransitionOutcome` instead of converting early
- inspect the gate construction site when a flow unexpectedly denies or defers
- if a flow should preserve stale or rebind-required but does not, check whether you accidentally used the success-only surfaces

## Anti-Patterns

- Do not flatten `TransitionOutcome` into `Result<T, E>` just because it is shorter.
- Do not use checked progression only to immediately discard its category information.
- Do not construct fake ready gates to bypass explicit denial or deferment handling.

## Current Limits

- the generic signatures are still verbose
- the crate preserves topology but does not choose domain-specific denial payloads for you
- these helpers are explicit rather than fully fluent today

## Related Docs

- [Checked Transitions](../features/checked-transitions.md)
- [Transition Outcomes](../features/transition-outcomes.md)
- [Preconstruction And Readiness Gates](../features/preconstruction-and-readiness-gates.md)
