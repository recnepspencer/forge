# Freshness And Downgrade

## What This Feature Is

Freshness and downgrade surfaces let `forge-proof` represent that a form was once strong but is no longer fully current. Instead of hiding that loss of confidence, the crate exposes explicit freshness classes and downgrade helpers.

## Why You Use It

- you need to keep using a value after its original basis is no longer fully current
- you need to distinguish stale readability from rebind requirements or authority revalidation loss
- you want the type system to preserve why a form weakened

## Stable Entry Points

- `FreshnessClass`
- `CurrentValidity`
- `StaleReadable`
- `RebindRequired`
- `AuthorityRevalidationRequired`
- `FreshnessScopedBasis<F, B>`
- `StaleReadableBasis<B>`
- `RebindRequiredBasis<B>`
- `AuthorityRevalidationRequiredBasis<B>`
- `strong_basis()` on current-validity carriers that support it
- downgrade helpers such as:
  - `downgrade_to_stale_readable()`
  - `downgrade_to_rebind_required()`
  - `downgrade_to_authority_revalidation_required()`

## Core Mental Model

Freshness is not a runtime log detail. It is part of semantic truth.

The important distinction is:

- `CurrentValidity` means the basis is still strong
- `StaleReadable` means the form is still readable but not current
- `RebindRequired` means trusted use requires rebinding
- `AuthorityRevalidationRequired` means authority must explicitly revalidate

Those states are not interchangeable.

## How It Executes

Typical lifecycle:

1. a form is created under `CurrentValidity`
2. time, branching, runtime drift, or boundary movement weakens trust
3. the form downgrades into one of the weaker freshness classes
4. later progression may rebind or revalidate into a new current-validity form

## Small Example

```rust
use forge_proof::{CurrentValidity, FreshnessScopedBasis, RebindRequiredBasis};

type Current = FreshnessScopedBasis<CurrentValidity, u8>;
type Rebind = RebindRequiredBasis<u8>;

let _ = std::any::type_name::<Current>();
let _ = std::any::type_name::<Rebind>();
```

This is the smallest honest example because downgrade is really about the basis wrapper shape. Public callers do not directly construct stronger recipe stages just to demonstrate freshness.

## Real Example

```rust
use forge_proof::{
    AssumptionBasis, CurrentValidity, FreshnessScopedBasis, Recipe, Resolved,
    RebindRequiredBasis,
};

type CurrentResolved =
    Recipe<Resolved, &'static str, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u8>>>;

fn weaken(recipe: CurrentResolved) -> Recipe<Resolved, &'static str, RebindRequiredBasis<u8>> {
    recipe.downgrade_to_rebind_required()
}
```

What is happening here:

- the resolved recipe started under a current strong basis
- the weakening is explicit and typed
- the resulting form still carries the underlying basis value
- the type now says rebinding is required before trusted use resumes

## How It Relates To Other Features

- Pair this with [Assumption Basis](./assumption-basis.md) because freshness always wraps a basis story.
- Pair this with [Boundary Readmission](./boundary-readmission.md) when trust-boundary crossings are the cause of weakening.
- Pair this with [Recipes And Stages](./recipes-and-stages.md) because most real downgrade flows happen on staged recipes.

## Inspection And Debugging

- `strong_basis()` is available only on forms that still have `CurrentValidity`
- weaker forms still expose their wrapped basis through `basis().basis().value()` patterns
- downgrade helpers preserve payload and proof carriage while changing the basis type

## Anti-Patterns

- Do not collapse stale, rebind-required, and authority-revalidation-required into one generic "invalid" state.
- Do not keep calling `strong_basis()` semantics on downgraded forms.
- Do not silently replace downgrade with a fresh current basis unless a real readmission or rebind happened.

## Current Limits

- freshness classes are intentionally small and static
- the crate distinguishes weakening states, but it does not define every domain-specific reason for those states
- readmission and rebinding are handled by adjacent progression surfaces rather than by downgrade alone

## Related Docs

- [Assumption Basis](./assumption-basis.md)
- [Boundary Readmission](./boundary-readmission.md)
- [Recipes And Stages](./recipes-and-stages.md)
