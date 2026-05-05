# Execution-Ready And Executed

## What This Feature Is

These wrappers represent the two stronger runtime-adjacent recipe states beyond plain staged progression:

- `ExecutionReadyRecipe<T, A>`
- `ExecutedRecipe<T, A>`

They make execution-admitted and post-execution progression explicit without turning `forge-proof` into a runtime engine.

## Why You Use It

- a lowered recipe is not yet safe to treat as execution-ready
- you need an explicit type boundary before execution
- you want executed forms to stay distinguishable from merely admitted or lowered forms

## Stable Entry Points

- `ExecutionReadyRecipe<T, A>`
- `ExecutionReadyRecipe::payload()`
- `ExecutionReadyRecipe::basis()`
- `ExecutionReadyRecipe::into_parts()`
- `ExecutionReadyRecipe::strong_basis()`
- `ExecutedRecipe<T, A>`
- `ExecutedRecipe::payload()`
- `ExecutedRecipe::basis()`
- `ExecutedRecipe::into_parts()`
- `ExecutedRecipe::strong_basis()`

Related transitions live in:

- [Basic Transitions](./basic-transitions.md)
- [Checked Transitions](./checked-transitions.md)
- [Runtime Readmission](./runtime-readmission.md)

## Core Mental Model

Execution readiness is not just another label on `Recipe<Lowered, ...>`.

The crate keeps these states distinct:

- lowered
- execution-ready
- executed

That separation matters because a lowered form may still require runtime admission, readmission, or checked readiness handling before execution is legal.

## How It Executes

Typical progression:

1. begin with `Recipe<Lowered, T, A>`
2. admit it for execution into `ExecutionReadyRecipe<T, A>`
3. execute it into `ExecutedRecipe<T, A>`

The wrappers preserve the same payload and basis shape, but they tell the compiler and the reader that an additional progression boundary was crossed.

## Small Example

```rust
use forge_proof::ExecutionReadyRecipe;

type ReadyPayload = ExecutionReadyRecipe<&'static str, u8>;
let _ = std::any::type_name::<ReadyPayload>();
```

This is the smallest honest example because public callers generally receive these wrappers from transitions rather than constructing them directly.

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

    assert_eq!(executed.payload(), &"payload");
    assert_eq!(executed.strong_basis().value(), &17_u8);
}
```

What this shows:

- readiness is a separate progression step
- execution consumes a ready wrapper, not a raw lowered recipe
- the executed form still exposes its strong basis honestly

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

## Current Limits

- public construction is sealed
- execution here means proof-bearing progression, not a generic runtime engine
- these wrappers preserve shape honesty but do not add diagnostics or descriptive reporting

## Related Docs

- [Basic Transitions](./basic-transitions.md)
- [Checked Transitions](./checked-transitions.md)
- [Runtime Readmission](./runtime-readmission.md)
