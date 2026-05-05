# Boundary Readmission

## What This Feature Is

Boundary-readmission surfaces model what happens when a currently trusted form crosses a trust boundary. Instead of pretending nothing changed, `forge-proof` weakens the basis into a boundary-bridged form and requires explicit rebind or readmission before current trust resumes.

## Why You Use It

- you serialize, transport, restore, or otherwise cross a trust boundary
- you need to keep payload and proof shape but weaken basis trust honestly
- you want rebind or readmission to be explicit and authority-backed

## Stable Entry Points

- pleasant lane:
  - `use forge_proof::prelude::*;`
  - `.bridge_trust_boundary()`
  - `.rebind_with(authority, basis)`
  - `.readmit_with(authority, basis)`
- raw lane:
  - `use forge_proof::raw::*;`
  - `BoundaryBridged<A>`
  - `BoundaryBridgedStaleReadableBasis<B>`
  - `BoundaryBridgedRebindRequiredBasis<B>`
  - `BoundaryBridgedAuthorityRevalidationRequiredBasis<B>`
  - `.readmit_with_authority(...)`
  - `.rebind_with_authority(...)`

## Core Mental Model

Crossing a trust boundary is itself a progression event.

The crate treats that event as:

- payload still exists
- proof carriage still exists
- prior basis value can still be remembered
- trust in that basis is weakened
- a fresh strong basis requires explicit authority-backed progression

## Pleasant Lane First

```rust
use forge_proof::prelude::*;

type CurrentAdmitted =
    forge_proof::Recipe<
        forge_proof::Admitted,
        &'static str,
        forge_proof::FreshnessScopedBasis<
            forge_proof::CurrentValidity,
            forge_proof::AssumptionBasis<u8>,
        >,
    >;

fn readmit(
    recipe: CurrentAdmitted,
    authority: forge_proof::AuthorityWitness<ReadmissionAuthority>,
) {
    let bridged = recipe.bridge_trust_boundary();
    let readmitted = bridged.readmit_with(authority, 37_u16);
    assert_eq!(readmitted.strong_basis().value(), &37_u16);
}

struct ReadmissionAuthority;
impl forge_proof::AuthorityMarker for ReadmissionAuthority {}
```

## Equivalent Raw Surface

```rust
use forge_proof::raw::*;

type BridgedAdmitted =
    Recipe<Admitted, &'static str, BoundaryBridgedAuthorityRevalidationRequiredBasis<u8>>;

fn readmit(recipe: BridgedAdmitted, authority: AuthorityWitness<ReadmissionAuthority>) {
    let readmitted = recipe.readmit_with_authority(37_u16, authority);
    assert_eq!(readmitted.strong_basis().value(), &37_u16);
}

struct ReadmissionAuthority;
impl AuthorityMarker for ReadmissionAuthority {}
```

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

## Related Docs

- [Assumption Basis](./assumption-basis.md)
- [Freshness And Downgrade](./freshness-and-downgrade.md)
- [Witnesses](./witnesses.md)
