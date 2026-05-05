# Boundary Readmission

## What This Feature Is

Boundary-readmission surfaces model what happens when a currently trusted form crosses a trust boundary. Instead of pretending nothing changed, `forge-proof` weakens the basis into a boundary-bridged form and requires explicit rebind or readmission before current trust resumes.

## Why You Use It

- you serialize, transport, restore, or otherwise cross a trust boundary
- you need to keep payload and proof shape but weaken basis trust honestly
- you want rebind or readmission to be explicit and authority-backed

## Stable Entry Points

- `BoundaryBridged<A>`
- `BoundaryBridgedStaleReadableBasis<B>`
- `BoundaryBridgedRebindRequiredBasis<B>`
- `BoundaryBridgedAuthorityRevalidationRequiredBasis<B>`
- `bridge_trust_boundary()`
- `readmit_with_authority(...)`
- `rebind_with_authority(...)`

## Core Mental Model

Crossing a trust boundary is itself a progression event.

The crate treats that event as:

- payload still exists
- proof carriage still exists
- prior basis value can still be remembered
- trust in that basis is weakened
- a fresh strong basis requires explicit authority-backed progression

This is how `forge-proof` avoids ambient "round-trip and continue" flows.

## How It Executes

A normal boundary-readmission sequence looks like this:

1. start with a current-validity form
2. call `bridge_trust_boundary()`
3. receive a weaker `BoundaryBridged<...>` basis form
4. call `readmit_with_authority(...)` or `rebind_with_authority(...)`
5. receive a new current-validity form under a new basis

Which weakened form you get depends on the thing you bridged:

- resolved recipes bridge to rebind-required
- lowered recipes bridge to stale-readable
- admitted forms bridge to authority-revalidation-required

## Small Example

```rust
use forge_proof::{BoundaryBridged, StaleReadableBasis};

type Bridged = BoundaryBridged<StaleReadableBasis<u8>>;
let _ = std::any::type_name::<Bridged>();
```

This is the smallest honest example because it shows the explicit boundary wrapper itself. The public API is about preserving weakened trust, not about hiding the transition.

## Real Example

```rust
use forge_proof::{
    Admitted, AssumptionBasis, AuthorityMarker, AuthorityWitness,
    BoundaryBridgedAuthorityRevalidationRequiredBasis, CurrentValidity,
    FreshnessScopedBasis, Recipe,
};

struct ReadmissionAuthority;
impl AuthorityMarker for ReadmissionAuthority {}

type CurrentAdmitted =
    Recipe<Admitted, &'static str, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u8>>>;

type BridgedAdmitted =
    Recipe<Admitted, &'static str, BoundaryBridgedAuthorityRevalidationRequiredBasis<u8>>;

fn readmit(recipe: BridgedAdmitted, authority: AuthorityWitness<ReadmissionAuthority>) {
    let readmitted = recipe.readmit_with_authority(37_u16, authority);
    assert_eq!(readmitted.strong_basis().value(), &37_u16);
}
```

What is happening here:

- the admitted form stayed admitted
- its trust basis was weakened by a boundary crossing
- authority explicitly provided a new strong basis
- the resumed form is current again, but not under the old basis type

## How It Relates To Other Features

- Pair this with [Freshness And Downgrade](./freshness-and-downgrade.md) because boundary bridging is one important source of weakening.
- Pair this with [Witnesses](./witnesses.md) because authority-backed readmission is explicit.
- Pair this with [Recipes And Stages](./recipes-and-stages.md) since most public examples are recipe-stage re-entry flows.

## Inspection And Debugging

- `weakened_basis()` tells you what weakened basis the bridged form still remembers
- the bridged type itself tells you whether the next step is stale readmission, rebinding, or authority revalidation
- readmission APIs require explicit authority in their signature, which makes trust re-entry easy to spot

## Anti-Patterns

- Do not continue using a boundary-bridged form as though it were still current.
- Do not erase the boundary bridge by manually rebuilding a stronger basis outside the readmission API.
- Do not confuse "payload survived transport" with "trust survived transport."

## Current Limits

- boundary-readmission surfaces are static and explicit, not automatic
- the crate models weakened trust and re-entry, but not every possible external transport mechanism
- the meaning of the basis value itself remains domain-owned

## Related Docs

- [Assumption Basis](./assumption-basis.md)
- [Freshness And Downgrade](./freshness-and-downgrade.md)
- [Witnesses](./witnesses.md)
