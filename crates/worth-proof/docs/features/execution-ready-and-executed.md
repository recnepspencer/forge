# Execution-Ready And Executed

## What This Feature Is

These wrappers represent the two stronger runtime-adjacent recipe states beyond plain staged progression:

- `ExecutionReadyRecipe<T, A>`
- `ExecutedRecipe<T, A>`

## Why You Use It

- a lowered recipe is not yet safe to treat as execution-ready
- you need an explicit type boundary before execution
- you want executed forms to stay distinguishable from merely admitted or lowered forms

## Stable Entry Points

- pleasant lane:
  - `use worth_proof::prelude::*;`
  - `.ready_with(...)`
  - `.ready(...)`
  - `.execute()`
  - `.stage()`
  - `.basis_posture()`
- raw lane:
  - `use worth_proof::raw::*;`
  - `ExecutionReadyRecipe<T, A>`
  - `ExecutedRecipe<T, A>`
  - `ExecutionReadyRecipe::payload()`
  - `ExecutionReadyRecipe::basis()`
  - `ExecutionReadyRecipe::into_parts()`
  - `ExecutedRecipe::payload()`
  - `ExecutedRecipe::basis()`
  - `ExecutedRecipe::into_parts()`

## Core Mental Model

The crate keeps these states distinct:

- lowered
- execution-ready
- executed

That separation matters because a lowered form may still require runtime admission, readmission, or checked readiness handling before execution is legal.

## Pleasant Lane First

```rust
use worth_proof::prelude::*;

fn execute(
    resolution_authority: worth_proof::AuthorityWitness<ResolutionAuthority>,
    lowering_capability: worth_proof::CapabilityWitness<LoweringCapability>,
    readiness_authority: worth_proof::AuthorityWitness<ReadinessAuthority>,
) {
    let executed = recipe("payload")
        .resolve_with(resolution_authority, 17_u8)
        .lower_with(lowering_capability)
        .ready_with(readiness_authority, "runtime admission")
        .execute();

    let _ = executed.stage();
}

struct ResolutionAuthority;
impl worth_proof::AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl worth_proof::CapabilityMarker for LoweringCapability {}

struct ReadinessAuthority;
impl worth_proof::AuthorityMarker for ReadinessAuthority {}
```

## Equivalent Raw Surface

```rust
use worth_proof::raw::*;

fn execute(
    resolution_authority: AuthorityWitness<ResolutionAuthority>,
    lowering_capability: CapabilityWitness<LoweringCapability>,
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
    let ready = AdmitExecutionReadyRecipeTransition.transition(
        lowered,
        ExecutionReadinessContext::new("runtime admission", readiness_authority),
    );
    let executed = ExecuteReadyRecipeTransition.transition(ready.into_value()).into_value();

    let _ = executed.payload();
}

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}

struct ReadinessAuthority;
impl AuthorityMarker for ReadinessAuthority {}
```

Use the raw lane when:

- you need direct control over readiness context construction
- you are building a domain-facing runtime progression helper
- you want the explicit transition types in view

## How It Relates To Other Features

- Pair this with [Basic Transitions](./basic-transitions.md) for straight-line progression.
- Pair this with [Checked Transitions](./checked-transitions.md) when readiness can deny, defer, stale, or rebind.
- Pair this with [Runtime Readmission](./runtime-readmission.md) when the lowered form crossed a trust boundary before readiness.

## Inspection And Debugging

- `payload()` and `basis()` let you inspect the wrapped lowered state without erasing the stronger wrapper
- `strong_basis()` is available on current-validity readiness and executed forms
- `into_parts()` is the honest owned extraction surface when lower-level code needs the payload and basis directly

## Anti-Patterns

- Do not treat `Recipe<Lowered, ...>` as though it were already execution-ready.
- Do not treat `ExecutionReadyRecipe<T, A>` as equivalent to `ExecutedRecipe<T, A>`.
- Do not skip readiness by inventing local aliases that erase the wrapper difference.

## Related Docs

- [Basic Transitions](./basic-transitions.md)
- [Checked Transitions](./checked-transitions.md)
- [Runtime Readmission](./runtime-readmission.md)
