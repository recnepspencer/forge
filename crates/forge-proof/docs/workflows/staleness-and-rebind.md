# Staleness And Rebind

## What This Feature Is

This workflow shows how to weaken a strong basis honestly when freshness is lost, and how to distinguish stale readability from rebind-required or authority-revalidation-required states.

## Why You Use It

- a once-strong form is no longer current
- you need to keep the payload but weaken the trust state honestly
- you want downstream code to know whether it can still read, must rebind, or must revalidate

## Stable Entry Points

- `downgrade_to_stale_readable()`
- `downgrade_to_rebind_required()`
- `downgrade_to_authority_revalidation_required()`
- `StaleReadableBasis<B>`
- `RebindRequiredBasis<B>`
- `AuthorityRevalidationRequiredBasis<B>`

## Core Mental Model

This workflow is about losing confidence without losing the value entirely.

The important distinction is:

- stale-readable still permits reading
- rebind-required means semantic rebinding is required
- authority-revalidation-required means authority must explicitly restore trust

Those are different states, and the workflow should preserve that.

## How It Executes

1. begin with a current-validity form
2. choose the correct weakening path
3. continue only through surfaces that accept the weaker state honestly

## Small Example

```rust
use forge_proof::{CurrentValidity, FreshnessScopedBasis, RebindRequiredBasis};

type Current = FreshnessScopedBasis<CurrentValidity, u8>;
type Rebind = RebindRequiredBasis<u8>;

let _ = std::any::type_name::<Current>();
let _ = std::any::type_name::<Rebind>();
```

This is the smallest honest example because the workflow begins with understanding the weakened basis shapes.

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

- the recipe was once current and strong
- the weakening preserves payload and basis value
- the type now says rebinding is required before trusted progression can continue

## How It Relates To Other Features

- Use [Boundary Readmission](./runtime-readmission.md) when a trust-boundary crossing is what caused the weakening.
- Use [Checked Recipe Progression](./checked-recipe-progression.md) when stale or rebind categories must be preserved inside one checked flow.
- Use [Happy-Path Recipe Progression](./happy-path-recipe-progression.md) only when the form is still current and strong.

## Inspection And Debugging

- `strong_basis()` is only valid on current-validity forms
- inspect the basis type first; it tells you whether the next legal step is read, rebind, or revalidation
- if downstream code still assumes current validity, the weakening probably needs to happen earlier

## Anti-Patterns

- Do not keep treating a downgraded form as though it still had a strong basis.
- Do not collapse stale, rebind, and authority revalidation into one generic "expired" state.
- Do not silently replace the basis instead of using an explicit rebind or readmission surface.

## Current Limits

- this workflow explains weakening, not full recovery
- the crate models freshness law, not every domain-specific cause of freshness loss
- more complex recovery belongs in the readmission workflows

## Related Docs

- [Freshness And Downgrade](../features/freshness-and-downgrade.md)
- [Boundary Readmission](../features/boundary-readmission.md)
- [Checked Recipe Progression](./checked-recipe-progression.md)
