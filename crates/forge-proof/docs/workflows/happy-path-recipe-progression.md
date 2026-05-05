# Happy-Path Recipe Progression

## What This Feature Is

This workflow shows the canonical straight-line recipe progression:

- unresolved
- resolved
- lowered
- execution-ready
- executed

Use it when the trusted authority and capability lanes are already available and you do not need checked denial, deferment, stale, or rebind handling.

## Why You Use It

- you want the shortest honest path through the crate
- your current lane is success-only
- you want a concrete starting point before moving to checked or boundary-bridged flows

## Pleasant Lane First

```rust
use forge_proof::prelude::*;

let executed = recipe("payload")
    .resolve_with(resolution_authority, 8_u8)
    .lower_with(lowering_capability)
    .ready_with(readiness_authority, "runtime admission")
    .execute();
```

## Stable Entry Points

- `Recipe::<Unresolved, T>::new(...)`
- `ResolveRecipeTransition`
- `RecipeResolutionContext::new(...)`
- `LowerRecipeTransition::new(...)`
- `AdmitExecutionReadyRecipeTransition`
- `ExecutionReadinessContext::new(...)`
- `ExecuteReadyRecipeTransition`

## Core Mental Model

This is the "nothing adversarial happened yet" progression.

It still keeps the hard boundaries visible:

- authority is explicit at resolution
- capability is explicit at lowering
- runtime admission authority is explicit at readiness
- executed is distinct from ready

What it does not do is preserve richer non-success topology. If you need that, use the checked workflow instead.

## How It Executes

1. construct `Recipe<Unresolved, T>`
2. resolve it with `ResolveRecipeTransition`
3. lower it with `LowerRecipeTransition`
4. admit it for execution with `AdmitExecutionReadyRecipeTransition`
5. execute it with `ExecuteReadyRecipeTransition`

## Small Example

```rust
use forge_proof::{Recipe, Unresolved};

let unresolved = Recipe::<Unresolved, _>::new("payload");
assert_eq!(unresolved.payload(), &"payload");
```

This is the smallest honest starting point because only unresolved recipes have a public direct constructor.

## Real Example

```rust
use forge_proof::{
    AdmitExecutionReadyRecipeTransition, AuthorityMarker, AuthorityWitness, CapabilityMarker,
    CapabilityWitness, ContextualTransition, ExecuteReadyRecipeTransition,
    ExecutionReadinessContext, LowerRecipeTransition, Recipe, RecipeResolutionContext,
    ResolveRecipeTransition, Transition, Unresolved,
};

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}

struct ReadinessAuthority;
impl AuthorityMarker for ReadinessAuthority {}

fn progress(
    resolution_authority: AuthorityWitness<ResolutionAuthority>,
    lowering_capability: CapabilityWitness<LoweringCapability>,
    readiness_authority: AuthorityWitness<ReadinessAuthority>,
) {
    let unresolved = Recipe::<Unresolved, _>::new("payload");
    let resolved = ResolveRecipeTransition.transition(
        unresolved,
        RecipeResolutionContext::new(8_u8, resolution_authority),
    );
    let lowered = LowerRecipeTransition::new(lowering_capability)
        .transition(resolved.into_value())
        .into_value();
    let ready = AdmitExecutionReadyRecipeTransition.transition(
        lowered,
        ExecutionReadinessContext::new("runtime admission", readiness_authority),
    );
    let executed = ExecuteReadyRecipeTransition.transition(ready.into_value()).into_value();

    assert_eq!(executed.payload(), &"payload");
    assert_eq!(executed.strong_basis().value(), &8_u8);
}
```

What is authoritative here:

- resolution authority authorizes the initial strong basis
- lowering capability authorizes the lowering lane
- readiness authority authorizes execution admission

What is derived here:

- resolved, lowered, ready, and executed forms are all stronger derived forms built from the previous stage

## Equivalent Raw Surface

```rust
use forge_proof::raw::*;

let resolved = ResolveRecipeTransition.transition(
    Recipe::<Unresolved, _>::new("payload"),
    RecipeResolutionContext::new(8_u8, resolution_authority),
);
let lowered = LowerRecipeTransition::new(lowering_capability)
    .transition(resolved.into_value())
    .into_value();
let ready = AdmitExecutionReadyRecipeTransition.transition(
    lowered,
    ExecutionReadinessContext::new("runtime admission", readiness_authority),
);
let executed = ExecuteReadyRecipeTransition.transition(ready.into_value()).into_value();
```

This is the same proof-bearing law surface as the pleasant lane. Stay pleasant
by default; drop to raw when the extra explicitness helps more than the
compression does.

## How It Relates To Other Features

- Use [Checked Recipe Progression](./checked-recipe-progression.md) when the flow can deny, defer, stale, or require rebinding.
- Use [Runtime Readmission](./runtime-readmission.md) when the lowered form crossed a trust boundary before execution.
- Use [Staleness And Rebind](./staleness-and-rebind.md) when you need to weaken a strong basis instead of continuing straight through.

## Inspection And Debugging

- inspect the type at each stage first; that usually tells you where the flow currently is
- inspect `strong_basis()` on the stronger current-validity forms when basis drift is suspected
- if the code is getting noisy, do not erase the boundaries; extract helpers around the explicit transitions instead

## Anti-Patterns

- Do not skip from unresolved directly to lowered or ready by reconstructing stronger forms manually.
- Do not treat this workflow as the general answer when stale or rebind pressure is possible.
- Do not hide authority and capability lanes behind ambient globals.

## Current Limits

- this workflow is intentionally success-path only
- it does not certify non-success divergence
- use the raw escape hatch when the compressed lane would hide too much

## Related Docs

- [Recipes And Stages](../features/recipes-and-stages.md)
- [Basic Transitions](../features/basic-transitions.md)
- [Execution-Ready And Executed](../features/execution-ready-and-executed.md)
