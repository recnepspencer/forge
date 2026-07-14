# Staleness And Rebind

## What This Feature Is

This workflow shows how to weaken a strong basis honestly when freshness is lost, and how to distinguish stale readability from rebind-required or authority-revalidation-required states.

## Why You Use It

- a once-strong form is no longer current
- you need to keep the payload but weaken the trust state honestly
- you want downstream code to know whether it can still read, must rebind, or must revalidate

## Stable Entry Points

- pleasant lane:
  - `use worth_proof::prelude::*;`
  - `.downgrade_to_stale_readable()`
  - `.downgrade_to_rebind_required()`
  - `.downgrade_to_authority_revalidation_required()`
  - `.rebind_with(authority, basis)`
- raw lane:
  - `use worth_proof::raw::*;`
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

## Pleasant Lane First

```rust
use worth_proof::prelude::*;

fn weaken_and_rebind(
    resolution_authority: worth_proof::AuthorityWitness<ResolutionAuthority>,
    readmission_authority: worth_proof::AuthorityWitness<ReadmissionAuthority>,
) {
    let rebound = recipe("payload")
        .resolve_with(resolution_authority, 7_u8)
        .downgrade_to_rebind_required()
        .rebind_with(readmission_authority, 9_u16);

    let _ = rebound.strong_basis();
}

struct ResolutionAuthority;
impl worth_proof::AuthorityMarker for ResolutionAuthority {}

struct ReadmissionAuthority;
impl worth_proof::AuthorityMarker for ReadmissionAuthority {}
```

What this keeps visible:

- weakening is explicit
- rebind is a different operation from readmission-after-bridge
- basis replacement stays visible at the call site

## Equivalent Raw Surface

```rust
use worth_proof::raw::*;

type CurrentResolved =
    Recipe<Resolved, &'static str, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u8>>>;

fn weaken(recipe: CurrentResolved) -> Recipe<Resolved, &'static str, RebindRequiredBasis<u8>> {
    recipe.downgrade_to_rebind_required()
}
```

Use the raw lane when:

- you are reasoning directly about basis wrapper types
- you need a substrate-local type alias for the weakened state
- you are building a domain-facing freshness helper

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

## Related Docs

- [Freshness And Downgrade](../features/freshness-and-downgrade.md)
- [Boundary Readmission](../features/boundary-readmission.md)
- [Checked Recipe Progression](./checked-recipe-progression.md)
