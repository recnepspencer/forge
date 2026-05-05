# Runtime Readmission

## What This Feature Is

Runtime-readmission surfaces handle the specific case where a lowered recipe crossed a trust boundary, must regain a strong basis, and then must regain execution-readiness before execution can continue.

## Why You Use It

- you have a boundary-bridged lowered recipe
- execution is still the goal, but the original strong basis no longer applies
- you need a checked or unchecked path back into ready and executed forms

## Stable Entry Points

- `LoweredReadmissionContext<NextB, ReadmitAuth, Runtime, ReadinessAuth>::new(...)`
- `LoweredReadmissionReadiness<T, PrevB, NextB, ReadmitAuth, Runtime, ReadinessAuth, D, De, F>`
- `ReadmitLoweredForExecutionReadyTransition`
- `CheckedReadmitLoweredForExecutionReadyTransition`
- `readmit_ready_and_execute_recipe(...)`
- `checked_readmit_ready_and_execute_recipe(...)`

## Core Mental Model

Runtime readmission is a two-step recovery path:

1. regain a strong current basis from a boundary-bridged lowered recipe
2. admit the readmitted lowered recipe back into execution-ready state

The crate treats that as one explicit progression family instead of letting each domain rebuild it locally.

## How It Executes

Representative lifecycle:

1. start with `Recipe<Lowered, T, BoundaryBridgedStaleReadableBasis<PrevB>>`
2. provide a `LoweredReadmissionContext`
3. readmit with explicit readmission authority
4. run readiness admission with explicit readiness authority
5. receive either `ExecutionReadyRecipe` or `ExecutedRecipe`, depending on the helper used

The checked version preserves denial, deferment, stale-bridged, and failure categories.

## Small Example

```rust
use forge_proof::LoweredReadmissionContext;

type Context = LoweredReadmissionContext<u16, ReadmissionAuthority, &'static str, ReadinessAuthority>;

struct ReadmissionAuthority;
struct ReadinessAuthority;

let _ = std::any::type_name::<Context>();
```

This is the smallest honest example because the readmission context is the core stable entrypoint for the feature.

## Real Example

```rust
use forge_proof::{
    checked_readmit_ready_and_execute_recipe, AuthorityMarker, AuthorityWitness,
    CapabilityMarker, CapabilityWitness, ContextualTransition, LowerRecipeTransition,
    LoweredReadmissionContext, Recipe, RecipeResolutionContext, ResolveRecipeTransition,
    Transition, Unresolved,
};

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}

struct ReadmissionAuthority;
impl AuthorityMarker for ReadmissionAuthority {}

struct ReadinessAuthority;
impl AuthorityMarker for ReadinessAuthority {}

fn restore_and_execute(
    resolution_authority: AuthorityWitness<ResolutionAuthority>,
    lowering_capability: CapabilityWitness<LoweringCapability>,
    readmission_authority: AuthorityWitness<ReadmissionAuthority>,
    readiness_authority: AuthorityWitness<ReadinessAuthority>,
) {
    let unresolved = Recipe::<Unresolved, _>::new("payload");
    let resolved = ResolveRecipeTransition.transition(
        unresolved,
        RecipeResolutionContext::new(17_u8, resolution_authority),
    );
    let lowered = LowerRecipeTransition::new(lowering_capability)
        .transition(resolved.into_value())
        .into_value();
    let bridged = lowered.bridge_trust_boundary();

    let executed = checked_readmit_ready_and_execute_recipe(
        bridged,
        forge_proof::TransitionReadiness::ready(LoweredReadmissionContext::new(
            19_u16,
            readmission_authority,
            "runtime admission",
            readiness_authority,
        )),
    );

    let _ = executed;
}
```

What this shows:

- trust-boundary weakening is explicit
- readmission authority and readiness authority are distinct
- the new strong basis can differ from the original one
- checked execution preserves richer divergence categories if needed

## How It Relates To Other Features

- Pair this with [Boundary Readmission](./boundary-readmission.md) because runtime readmission starts from a bridged lowered recipe.
- Pair this with [Execution-Ready And Executed](./execution-ready-and-executed.md) because the destination is usually ready or executed.
- Pair this with [Checked Transitions](./checked-transitions.md) when non-success categories must remain visible.

## Inspection And Debugging

- inspect the bridged basis first to confirm the input is really boundary-bridged
- inspect `LoweredReadmissionContext` construction to see which basis and authorities are being supplied
- use the checked variant when you need to see whether the flow denied, deferred, stayed stale, or failed

## Anti-Patterns

- Do not treat a boundary-bridged lowered recipe as though it were still ready for execution.
- Do not rebuild a strong basis manually outside the explicit readmission surfaces.
- Do not conflate readmission authority with readiness authority unless your domain intentionally makes them the same lane.

## Current Limits

- the feature is focused on lowered recipe re-entry, not every possible bridged form
- the crate preserves progression law, not rich runtime diagnostics
- generic context aliases can still be verbose before domain-specific wrappers exist

## Related Docs

- [Boundary Readmission](./boundary-readmission.md)
- [Execution-Ready And Executed](./execution-ready-and-executed.md)
- [Checked Transitions](./checked-transitions.md)
