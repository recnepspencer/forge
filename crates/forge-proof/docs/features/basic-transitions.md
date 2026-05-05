# Basic Transitions

## What This Feature Is

Basic transitions are the straight-line progression surfaces for moving recipes through their canonical stages when you already have the required authority or capability and do not need checked denial or deferment topology.

## Why You Use It

- you want the direct resolve -> lower -> admit flow
- you already know the required witness or context is available
- you want explicit progression types without checked-readiness overhead

## Stable Entry Points

- `Transition<Input>`
- `ContextualTransition<Input, Context>`
- `apply_transition(...)`
- `apply_contextual_transition(...)`
- `ResolveRecipeTransition`
- `RecipeResolutionContext<B, Auth>::new(basis, authority)`
- `LowerRecipeTransition<C>::new(capability)`
- `AdmitRecipeTransition<Auth>::new(authority)`
- `AdmitExecutionReadyRecipeTransition`
- `ExecutionReadinessContext<R, Auth>::new(runtime, authority)`
- `ExecuteReadyRecipeTransition`
- `admit_ready_and_execute_recipe(...)`

## Core Mental Model

These are the direct progression operators.

They do not pretend every transition is always available. Instead, they require the trusted context in their signatures:

- resolution requires a resolution context
- lowering requires a capability-backed transition value
- admission requires an authority-backed transition value
- readiness requires a readiness context

If you need denial, deferment, stale, or rebind categories to remain first-class, move to the checked surfaces instead of overloading the basic ones.

## How It Executes

Representative lifecycle:

1. start with `Recipe<Unresolved, T>`
2. resolve through `ResolveRecipeTransition`
3. lower through `LowerRecipeTransition`
4. admit through `AdmitRecipeTransition`
5. optionally move into execution-ready and executed states

The output of these transitions is success-only progression wrappers or direct ready/executed composition helpers.

## Small Example

```rust
use forge_proof::{apply_transition, Transition};

struct Increment;

impl Transition<u64> for Increment {
    type Output = u64;

    fn transition(&self, input: u64) -> Self::Output {
        input + 1
    }
}

assert_eq!(apply_transition(&Increment, 7), 8);
```

This is the smallest honest example because it shows the stable transition contract directly, without forcing recipe-specific context into the first example.

## Real Example

```rust
use forge_proof::{
    AdmitRecipeTransition, AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness,
    ContextualTransition, LowerRecipeTransition, Recipe, RecipeResolutionContext,
    ResolveRecipeTransition, Transition, Unresolved,
};

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}

struct AdmissionAuthority;
impl AuthorityMarker for AdmissionAuthority {}

fn progress(
    resolution_authority: AuthorityWitness<ResolutionAuthority>,
    lowering_capability: CapabilityWitness<LoweringCapability>,
    admission_authority: AuthorityWitness<AdmissionAuthority>,
) {
    let unresolved = Recipe::<Unresolved, _>::new("payload");
    let resolved = ResolveRecipeTransition.transition(
        unresolved,
        RecipeResolutionContext::new(7_u8, resolution_authority),
    );
    let lowered = LowerRecipeTransition::new(lowering_capability)
        .transition(resolved.into_value())
        .into_value();
    let admitted = AdmitRecipeTransition::new(admission_authority)
        .transition(lowered)
        .into_value();

    assert_eq!(admitted.strong_basis().value(), &7_u8);
}
```

What this shows:

- the transition chain is explicit and ordered
- each step names the authority or capability lane it depends on
- the stronger form is returned by the transition rather than by local reconstruction

## How It Relates To Other Features

- Pair this with [Recipes And Stages](./recipes-and-stages.md) because recipes are the main inputs and outputs.
- Pair this with [Execution-Ready And Executed](./execution-ready-and-executed.md) for runtime-adjacent progression.
- Use [Checked Transitions](./checked-transitions.md) when you need richer outcome topology.

## Inspection And Debugging

- transition types make progression lanes easy to find in code review
- context types such as `RecipeResolutionContext` and `ExecutionReadinessContext` show what trusted inputs were required
- success wrappers can be inspected with `.value()` or consumed with `.into_value()`

## Anti-Patterns

- Do not use the basic transitions when denial, deferment, stale, or rebind categories need to remain visible.
- Do not hide witness-bearing progression behind generic helper closures when the explicit transition type is the real law.
- Do not bypass the transition contracts by reconstructing stronger forms manually.

## Current Limits

- basic transitions are success-path surfaces, not full readiness topology surfaces
- they remain explicit and static rather than offering a fluent runtime engine
- richer checked combinations live in adjacent helpers rather than here

## Related Docs

- [Recipes And Stages](./recipes-and-stages.md)
- [Checked Transitions](./checked-transitions.md)
- [Execution-Ready And Executed](./execution-ready-and-executed.md)
