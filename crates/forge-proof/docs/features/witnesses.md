# Witnesses

## What This Feature Is

Witnesses are sealed, zero-sized authority and capability carriers used to authorize progression steps. They let a transition say "this trusted lane is present" without turning authority into ambient state.

## Why You Use It

- a transition must require explicit authority
- a transition must require a specific capability lane
- you want trusted progression to appear in signatures, not in comments or globals

## Stable Entry Points

- `AuthorityMarker`
- `CapabilityMarker`
- `AuthorityWitness<A>`
- `CapabilityWitness<C>`

Public usage pattern:

- define your authority or capability marker type
- implement the marker trait
- keep authority and capability marker construction inside the domain boundary
  that owns the trust decision
- turn that marker into a witness with
  `AuthorityWitness::from_authority_marker(...)` or
  `CapabilityWitness::from_capability_marker(...)`
- consume the witness in progression APIs that require it

Important boundary:

- public code does not call the sealed `mint` constructors directly
- marker visibility is the domain-owned guardrail: if callers should not obtain
  an authority, do not expose a constructible marker for that authority

## DX Posture

This is mostly substrate/reference material.

- the pleasant lane consumes witnesses through verbs such as `.resolve_with(...)`, `.lower_with(...)`, `.ready_with(...)`, `.readmit_with(...)`, and `proof_flow()`
- witness definition and witness-bearing signatures remain raw-substrate truth
- when you work directly with witness types and marker traits, prefer `use forge_proof::raw::*;`

## Core Mental Model

A witness is not the same thing as a proof.

A proof means:

- a fact has already been established

A witness means:

- a trusted authority or capability lane is available to authorize a transition

That distinction is one of the most important laws in this crate.

## How It Executes

Typical witness flow:

1. some trusted crate-local or test-only surface owns a constructible marker
   for the authority or capability
2. that trusted surface converts the marker into a witness
3. the witness is passed into a transition context or transition constructor
4. the transition consumes it to progress a weaker form into a stronger one
5. the resulting stronger form carries semantic truth, not the witness itself

## Small Example

```rust
use forge_proof::{AuthorityMarker, AuthorityWitness};

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

fn resolution_authority() -> AuthorityWitness<ResolutionAuthority> {
    AuthorityWitness::from_authority_marker(ResolutionAuthority)
}
```

This is the smallest honest public example. If the authority must not be
available outside the owning module or crate, keep `ResolutionAuthority` private
or make its fields private.

## Real Example

```rust
use forge_proof::{
    AuthorityMarker, AuthorityWitness, Recipe, RecipeResolutionContext,
    ResolveRecipeTransition, Transition, Unresolved,
};

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

fn resolve(
    unresolved: Recipe<Unresolved, &'static str>,
    authority: AuthorityWitness<ResolutionAuthority>,
) {
    let _resolved = ResolveRecipeTransition.transition(
        unresolved,
        RecipeResolutionContext::new(8_u8, authority),
    );
}
```

What this shows:

- the API signature makes authority explicit
- the witness authorizes resolution
- the stronger resolved form is the thing that carries semantic progression afterward

## How It Relates To Other Features

- Pair this with [Recipes And Stages](./recipes-and-stages.md) because most witness-bearing flows are staged recipe transitions.
- Pair this with [Boundary Readmission](./boundary-readmission.md) when authority is needed for readmission or rebinding.
- Pair this with [Proof Markers And Sets](./proof-markers-and-sets.md) to keep the witness/proof distinction clear.

## Inspection And Debugging

- if a transition requires trust, look for a witness in the constructor or context type
- if a resulting form should carry a semantic fact, look for proof-bearing state on the result instead of on the witness
- witnesses are zero-sized and explicit by design, so they should not become mysterious runtime payload

## Anti-Patterns

- Do not use a witness as though it were itself a proof fact.
- Do not hide authority requirements in ambient globals or side channels when the transition should expose them.
- Do not invent public witness minting shortcuts that bypass the crate’s sealing rules.

## Current Limits

- sealed witness minting remains unavailable; public witness construction goes
  through domain-owned marker values
- witnesses authorize progression, but they do not describe rich policy or diagnostics
- the meaning of a specific authority or capability marker remains domain-owned

## Related Docs

- [Proof Markers And Sets](./proof-markers-and-sets.md)
- [Boundary Readmission](./boundary-readmission.md)
- [Recipes And Stages](./recipes-and-stages.md)
