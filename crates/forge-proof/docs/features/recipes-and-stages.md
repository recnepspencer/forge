# Recipes And Stages

## What This Feature Is

Recipes are the crate's main staged progression carrier. `Recipe<S, T, A>` lets you represent the same logical payload as it moves through explicitly typed stages such as unresolved, resolved, lowered, and admitted.

## Why You Use It

- you need a staged progression model rather than a generic phase carrier
- you want each step of a flow to be visible in the type system
- you want later execution-readiness and checked-transition surfaces to build on a stable core carrier

## Stable Entry Points

- `Recipe<S, T, A>`
- `Recipe::<Unresolved, T>::new(payload)`
- `Recipe::payload()`
- `Recipe::basis()`
- `Recipe::into_parts()`
- stage markers:
  - `Unresolved`
  - `Resolved`
  - `Lowered`
  - `Admitted`
- wrappers built on top of recipes:
  - `ExecutionReadyRecipe<T, A>`
  - `ExecutedRecipe<T, A>`

## Core Mental Model

A recipe is a payload in a specific progression stage.

The main stages mean:

- `Unresolved`
  - initial intent exists, but authority-backed resolution has not happened
- `Resolved`
  - authority-backed resolution happened, but lowering has not
- `Lowered`
  - the resolved form has been lowered into the next execution-facing shape
- `Admitted`
  - the lowered form passed admission law

This is the main substrate that later readiness, execution, and checked-transition APIs build on.

## How It Executes

Canonical progression usually looks like:

1. start with `Recipe<Unresolved, T>`
2. resolve into `Recipe<Resolved, T, ...>`
3. lower into `Recipe<Lowered, T, ...>`
4. admit into `Recipe<Admitted, T, ...>`
5. optionally progress into execution-ready and executed wrappers

Not every workflow must pass through every stage, but the stage distinctions are real and enforced.

## Small Example

```rust
use forge_proof::{Recipe, Unresolved};

let unresolved = Recipe::<Unresolved, _>::new("payload");

assert_eq!(unresolved.payload(), &"payload");
```

This is the smallest honest example because only unresolved recipes have a public direct constructor.

## Real Example

```rust
use forge_proof::{
    AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness, ContextualTransition,
    LowerRecipeTransition, Recipe, RecipeResolutionContext, ResolveRecipeTransition, Transition,
    Unresolved,
};

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}

fn progress(
    authority: AuthorityWitness<ResolutionAuthority>,
    capability: CapabilityWitness<LoweringCapability>,
) {
    let unresolved = Recipe::<Unresolved, _>::new("payload");
    let resolved = ResolveRecipeTransition.transition(
        unresolved,
        RecipeResolutionContext::new(12_u8, authority),
    );
    let lowered = LowerRecipeTransition::new(capability)
        .transition(resolved.into_value())
        .into_value();

    let _payload = lowered.payload();
}
```

What this shows:

- the recipe starts unresolved
- authority-backed resolution is explicit
- capability-backed lowering is explicit
- the stronger lowered form is not manually constructed by public code

## How It Relates To Other Features

- Pair this with [Witnesses](./witnesses.md) because stage progression often requires authority or capability witnesses.
- Pair this with [Assumption Basis](./assumption-basis.md) because stronger stages usually carry basis state.
- Pair this with [Freshness And Downgrade](./freshness-and-downgrade.md) and [Boundary Readmission](./boundary-readmission.md) when stage-carried bases weaken over time or across boundaries.

## Inspection And Debugging

- `payload()` tells you what the recipe currently carries
- `basis()` tells you the basis wrapper for the current stage
- the stage type itself is often the most important debugging clue
- `into_parts()` is the honest owned extraction boundary

## Anti-Patterns

- Do not flatten all recipe states into one payload type plus comments.
- Do not try to skip directly to stronger stages by reconstructing internals manually.
- Do not use generic artifacts when the real problem is staged progression law.

## Current Limits

- only `Unresolved` has a public direct constructor
- stronger-stage minting is sealed
- recipes model staged progression, not rich diagnostics or descriptive reporting

## Related Docs

- [Witnesses](./witnesses.md)
- [Assumption Basis](./assumption-basis.md)
- [Freshness And Downgrade](./freshness-and-downgrade.md)
