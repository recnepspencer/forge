# Checked Transitions

## What This Feature Is

Checked transitions are the progression surfaces you use when non-success categories must stay typed and visible instead of collapsing into a simple success-only flow.

## Why You Use It

- resolution or readiness can be denied or deferred
- lowering can require rebinding instead of progressing
- readiness can yield stale or rebind-required categories
- you want one helper to preserve the real topology of progression failure

## Stable Entry Points

- `CheckedResolveRecipeTransition`
- `CheckedLowerRecipeTransition<C>::new()`
- `CheckedAdmitRecipeTransition<Auth>::new()`
- `RecipeResolutionGate<B, Auth, D, De>`
- `RecipeLoweringReadiness<T, B, C, D, De, F>`
- `RecipeAdmissionReadiness<T, B, Auth, D, De, F>`
- `resolve_lower_and_admit_recipe(...)`
- `resolve_checked_lower_and_admit_recipe(...)`
- `CheckedAdmitExecutionReadyRecipeTransition`
- `ExecutionReadyAdmissionReadiness<T, B, R, Auth, D, De, F>`
- `checked_admit_ready_and_execute_recipe(...)`

## Core Mental Model

Checked transitions preserve real divergence.

They are for situations where the answer is not just:

- progressed
- did not progress

Instead, they preserve distinctions such as:

- denied
- deferred
- stale
- rebind-required
- failed

That makes them the honest surface for adversarial or runtime-sensitive flows.

## How It Executes

Representative checked flow:

1. start with unresolved input
2. gate resolution with a `PreConstructionGate`
3. evaluate lowering readiness with `TransitionReadiness`
4. evaluate admission or execution-readiness with `TransitionReadiness`
5. receive a `TransitionOutcome<...>` that keeps the exact non-success category

## Small Example

```rust
use forge_proof::{
    PreConstructionGate, RecipeResolutionContext, RecipeResolutionGate,
};

type Gate = RecipeResolutionGate<u8, ResolutionAuthority, &'static str, &'static str>;

struct ResolutionAuthority;

let _ = std::any::type_name::<Gate>();
let _denied = PreConstructionGate::<RecipeResolutionContext<u8, ResolutionAuthority>, _, _>::denied(
    "denied",
);
```

This is the smallest honest example because checked progression starts with explicit gating, not just with the transition type names.

## Real Example

```rust
use forge_proof::{
    AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness,
    CheckedAdmitExecutionReadyRecipeTransition, ContextualTransition,
    ExecutionReadinessContext, ExecutionReadyAdmissionReadiness, LowerRecipeTransition, Recipe,
    RecipeResolutionContext, ResolveRecipeTransition, Transition, Unresolved,
    checked_admit_ready_and_execute_recipe,
};

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}

struct ReadinessAuthority;
impl AuthorityMarker for ReadinessAuthority {}

type CheckedReadiness = ExecutionReadyAdmissionReadiness<
    &'static str,
    u8,
    &'static str,
    ReadinessAuthority,
    &'static str,
    &'static str,
    &'static str,
>;

fn checked_ready_execute(
    resolution_authority: AuthorityWitness<ResolutionAuthority>,
    lowering_capability: CapabilityWitness<LoweringCapability>,
    readiness_authority: AuthorityWitness<ReadinessAuthority>,
) {
    let unresolved = Recipe::<Unresolved, _>::new("payload");
    let resolved = ResolveRecipeTransition.transition(
        unresolved,
        RecipeResolutionContext::new(12_u8, resolution_authority),
    );
    let lowered = LowerRecipeTransition::new(lowering_capability)
        .transition(resolved.into_value())
        .into_value();

    let executed = checked_admit_ready_and_execute_recipe(
        lowered,
        CheckedReadiness::ready(ExecutionReadinessContext::new(
            "runtime admission",
            readiness_authority,
        )),
    );

    let _ = executed;
}
```

What this shows:

- checked readiness preserves more than simple success
- the readiness type names all possible divergence categories
- the result remains a `TransitionOutcome`, not a flattened boolean

## How It Relates To Other Features

- Pair this with [Transition Outcomes](./transition-outcomes.md) because those outcome categories are the point of the checked surfaces.
- Pair this with [Preconstruction And Readiness Gates](./preconstruction-and-readiness-gates.md) because checked progression is built on those gate types.
- Pair this with [Runtime Readmission](./runtime-readmission.md) when a boundary-bridged lowered form must regain execution-readiness under checked conditions.

## Inspection And Debugging

- checked alias types show the exact divergence topology at the type level
- `TransitionOutcome` pattern matching is the main way to inspect what happened
- the helper functions are usually the clearest place to look when a flow appears to "stop early"

## Anti-Patterns

- Do not replace checked progression with `Result<T, E>` when stale or rebind categories matter.
- Do not use checked helpers just to look sophisticated when only the straight success path exists.
- Do not erase checked outcomes immediately if downstream logic still cares about why progression stopped.

## Current Limits

- checked types can be verbose, especially before domain-specific aliases are added
- the crate preserves the topology, but domain crates still choose the concrete denial and failure payloads
- checked progression remains explicit rather than builder-driven today

## Related Docs

- [Transition Outcomes](./transition-outcomes.md)
- [Preconstruction And Readiness Gates](./preconstruction-and-readiness-gates.md)
- [Runtime Readmission](./runtime-readmission.md)
