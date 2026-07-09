# Assumption Basis

## What This Feature Is

Assumption-basis types make the basis of a stronger form explicit. Instead of carrying "some context" implicitly, `worth-proof` wraps that dependency in `AssumptionBasis<B>` or `FreshnessScopedBasis<F, B>` so the type tells you what the form currently depends on.

## Why You Use It

- you need a stronger form to remember what it was validated or resolved against
- you want basis-dependent progression to stay explicit
- you want stale, rebind, and revalidation states to wrap the same underlying basis honestly

## Stable Entry Points

- `NoAssumptionBasis`
- `AssumptionBasis<B>::new(value)`
- `AssumptionBasis::value()`
- `FreshnessScopedBasis<F, B>::basis()`
- `FreshnessScopedBasis<F, B>::into_basis()`

Adjacent surfaces:

- [Freshness And Downgrade](./freshness-and-downgrade.md) defines the freshness classes and downgrade aliases
- [Boundary Readmission](./boundary-readmission.md) defines boundary-bridged weakened basis wrappers

## DX Posture

This is mostly substrate/reference material.

- there is no separate pleasant-lane basis constructor family beyond the progression helpers that carry basis implicitly
- when you work directly with basis wrappers, prefer `use worth_proof::raw::*;`
- the pleasant lane teaches basis through progression calls such as `.resolve_with(...)`, `.rebind_with(...)`, and `.readmit_with(...)`

## Core Mental Model

An assumption basis answers the question:

- "What basis is this stronger form currently trusted under?"

That is different from:

- proof facts, which answer "what has been proven?"
- witnesses, which answer "what trusted lane can authorize progression?"

If a form has no meaningful basis, it should use `NoAssumptionBasis`. If it has a basis, use `AssumptionBasis<B>` instead of leaving that dependency implicit.

## How It Executes

Basis carriage normally shows up in this order:

1. begin with `NoAssumptionBasis` when there is no stronger dependency yet
2. transition into `AssumptionBasis<B>` or `FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>`
3. degrade or bridge that basis explicitly when confidence weakens
4. rebind or revalidate into a new strong basis when trusted progression resumes

## Small Example

```rust
use worth_proof::AssumptionBasis;

let basis = AssumptionBasis::new(7_u8);

assert_eq!(basis.value(), &7_u8);
```

This is the smallest honest example because it shows the public basis wrapper directly, without mixing in freshness or transitions.

## Real Example

```rust
use worth_proof::{
    AssumptionBasis, CurrentValidity, FreshnessScopedBasis, Recipe, Resolved,
};

type CurrentResolvedRecipe =
    Recipe<Resolved, &'static str, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u8>>>;

fn inspect_basis(recipe: &CurrentResolvedRecipe) -> &u8 {
    recipe.basis().basis().value()
}
```

What is happening here:

- the recipe is already in a stronger resolved stage
- its basis is explicit and current
- the basis is not mixed into the payload or hidden behind ambient state

## How It Relates To Other Features

- Pair this with [Recipes And Stages](./recipes-and-stages.md) when progression should carry explicit basis state.
- Pair this with [Freshness And Downgrade](./freshness-and-downgrade.md) when the basis can lose validity over time.
- Pair this with [Boundary Readmission](./boundary-readmission.md) when trust-boundary crossings must weaken the basis explicitly.

## Inspection And Debugging

- `AssumptionBasis::value()` tells you the underlying basis value
- `FreshnessScopedBasis::basis()` tells you the wrapped basis at the current freshness layer
- `into_basis()` is the owned extraction point when a transition needs to rebuild a stronger wrapper

## Anti-Patterns

- Do not hide basis-dependent correctness inside payload fields if the basis is part of progression law.
- Do not use `NoAssumptionBasis` just because it is shorter when the form truly depends on something.
- Do not confuse basis carriage with proof facts or witness authority.

## Current Limits

- `worth-proof` owns basis carriage and weakening law, not basis semantics for every domain
- public code can inspect basis wrappers, but stronger state minting still happens through progression surfaces
- basis wrappers remain intentionally simple and static

## Related Docs

- [Freshness And Downgrade](./freshness-and-downgrade.md)
- [Boundary Readmission](./boundary-readmission.md)
- [Recipes And Stages](./recipes-and-stages.md)
