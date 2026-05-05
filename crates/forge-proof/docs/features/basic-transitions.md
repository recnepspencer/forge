# Basic Transitions

## What This Feature Is

Basic transitions are the straight-line progression surfaces for moving recipes through their canonical stages when you already have the required authority or capability and do not need checked denial or deferment topology.

## Why You Use It

- you want the direct resolve -> lower -> admit flow
- you already know the required witness or context is available
- you want explicit progression types without checked-readiness overhead

## Stable Entry Points

- pleasant lane:
  - `use forge_proof::prelude::*;`
  - `.resolve_with(...)`
  - `.lower_with(...)`
  - `.admit_with(...)`
  - `.ready_with(...)`
  - `.execute()`
- raw lane:
  - `use forge_proof::raw::*;`
  - `Transition<Input>`
  - `ContextualTransition<Input, Context>`
  - `ResolveRecipeTransition`
  - `RecipeResolutionContext<B, Auth>::new(...)`
  - `LowerRecipeTransition<C>::new(...)`
  - `AdmitRecipeTransition<Auth>::new(...)`
  - `AdmitExecutionReadyRecipeTransition`
  - `ExecutionReadinessContext<R, Auth>::new(...)`
  - `ExecuteReadyRecipeTransition`

## Core Mental Model

These are the direct progression operators.

They do not pretend every transition is always available. Instead, they require the trusted context in their signatures.

## Pleasant Lane First

```rust
use forge_proof::prelude::*;

fn progress(
    resolution_authority: forge_proof::AuthorityWitness<ResolutionAuthority>,
    lowering_capability: forge_proof::CapabilityWitness<LoweringCapability>,
    admission_authority: forge_proof::AuthorityWitness<AdmissionAuthority>,
) {
    let admitted = recipe("payload")
        .resolve_with(resolution_authority, 7_u8)
        .lower_with(lowering_capability)
        .admit_with(admission_authority);

    let _ = admitted.strong_basis();
}

struct ResolutionAuthority;
impl forge_proof::AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl forge_proof::CapabilityMarker for LoweringCapability {}

struct AdmissionAuthority;
impl forge_proof::AuthorityMarker for AdmissionAuthority {}
```

## Equivalent Raw Surface

```rust
use forge_proof::raw::*;

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

    let _ = admitted.strong_basis();
}

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}

struct AdmissionAuthority;
impl AuthorityMarker for AdmissionAuthority {}
```

Use the raw lane when:

- the explicit transition types are themselves the thing you are teaching or building
- you need direct control over context object construction
- you are authoring a new domain-facing helper that should lower into the substrate

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

## Related Docs

- [Recipes And Stages](./recipes-and-stages.md)
- [Checked Transitions](./checked-transitions.md)
- [Execution-Ready And Executed](./execution-ready-and-executed.md)
