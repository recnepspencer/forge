# Recipes And Stages

## What This Feature Is

Recipes are the crate's main staged progression carrier. `Recipe<S, T, A>` lets you represent the same logical payload as it moves through explicitly typed stages such as unresolved, resolved, lowered, and admitted.

## Why You Use It

- you need a staged progression model rather than a generic phase carrier
- you want each step of a flow to be visible in the type system
- you want later execution-readiness and checked-transition surfaces to build on a stable core carrier

## Stable Entry Points

- pleasant lane:
  - `use worth_proof::prelude::*;`
  - `recipe(payload)`
  - `.resolve_with(...)`
  - `.lower_with(...)`
  - `.admit_with(...)`
- raw lane:
  - `use worth_proof::raw::*;`
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

## Pleasant Lane First

```rust
use worth_proof::prelude::*;

fn progress(
    authority: worth_proof::AuthorityWitness<ResolutionAuthority>,
    capability: worth_proof::CapabilityWitness<LoweringCapability>,
) {
    let lowered = recipe("payload")
        .resolve_with(authority, 12_u8)
        .lower_with(capability);

    let _ = lowered.payload();
}

struct ResolutionAuthority;
impl worth_proof::AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl worth_proof::CapabilityMarker for LoweringCapability {}
```

## Equivalent Raw Surface

```rust
use worth_proof::raw::*;

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

    let _ = lowered.payload();
}

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}
```

Use the raw lane when:

- you are authoring a domain-specific progression helper
- you need direct access to transition and context types
- the pleasant chain stops being semantically obvious

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

## Related Docs

- [Witnesses](./witnesses.md)
- [Assumption Basis](./assumption-basis.md)
- [Freshness And Downgrade](./freshness-and-downgrade.md)
