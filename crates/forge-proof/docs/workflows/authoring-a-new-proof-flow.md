# Authoring A New Proof Flow

## What This Feature Is

This workflow explains how a domain crate should build a new progression surface on top of `forge-proof` without smuggling in forged progression, hidden trust boundaries, or flattened non-success topology.

## Why You Use It

- you are building a new domain-specific progression API
- you want to reuse `forge-proof` instead of inventing another bespoke typestate layer
- you want the resulting domain flow to stay honest under hostile pressure

## Stable Entry Points

- pleasant lane:
  - `use forge_proof::prelude::*;`
  - `recipe(...)`
  - `.resolve_with(...)`
  - `.lower_with(...)`
  - `.admit_with(...)`
  - `.ready_with(...)`
  - `.try_*` checked progression verbs
- raw lane:
  - `use forge_proof::raw::*;`
  - `Artifact`
  - `Recipe`
  - `Proof`
  - `AuthorityWitness`
  - `CapabilityWitness`
  - `Transition`
  - `ContextualTransition`
  - `TransitionOutcome`
  - `PreConstructionGate`
  - `TransitionReadiness`

## Core Mental Model

When authoring a new proof flow, start from the law, not from the helper shape.

Ask:

- what phase or stage distinction is real?
- what facts must be carried as proofs?
- what authority or capability must be explicit?
- what basis must be explicit?
- what divergence categories must stay distinct?
- where does trust weaken?

Then compose the domain flow from the existing substrate instead of hiding those answers inside one custom wrapper.

## Pleasant Lane First

```rust
use forge_proof::prelude::*;

fn domain_flow(
    resolution_authority: forge_proof::AuthorityWitness<DomainResolutionAuthority>,
    lowering_capability: forge_proof::CapabilityWitness<DomainLoweringCapability>,
    admission_authority: forge_proof::AuthorityWitness<DomainAdmissionAuthority>,
) {
    let admitted = recipe("payload")
        .resolve_with(resolution_authority, 7_u8)
        .lower_with(lowering_capability)
        .admit_with(admission_authority);

    let _ = admitted;
}

struct DomainResolutionAuthority;
impl forge_proof::AuthorityMarker for DomainResolutionAuthority {}

struct DomainLoweringCapability;
impl forge_proof::CapabilityMarker for DomainLoweringCapability {}

struct DomainAdmissionAuthority;
impl forge_proof::AuthorityMarker for DomainAdmissionAuthority {}
```

This is the right starting point for most new domain flows:

- it reuses the crate's blessed grammar
- it keeps authority and capability explicit
- it avoids introducing a second local proof language too early

## Equivalent Raw Surface

```rust
use forge_proof::raw::*;

fn domain_flow(
    resolution_authority: AuthorityWitness<DomainResolutionAuthority>,
    lowering_capability: CapabilityWitness<DomainLoweringCapability>,
    admission_authority: AuthorityWitness<DomainAdmissionAuthority>,
) {
    let unresolved = Recipe::<Unresolved, _>::new("payload");
    let resolved = ResolveRecipeTransition.transition(
        unresolved,
        RecipeResolutionContext::new(7_u8, resolution_authority),
    );
    let lowered = LowerRecipeTransition::new(lowering_capability)
        .transition(resolved.into_value())
        .into_value();
    let admitted = AdmitRecipeTransition::new(admission_authority)
        .transition(lowered)
        .into_value();

    let _ = admitted;
}

struct DomainResolutionAuthority;
impl AuthorityMarker for DomainResolutionAuthority {}

struct DomainLoweringCapability;
impl CapabilityMarker for DomainLoweringCapability {}

struct DomainAdmissionAuthority;
impl AuthorityMarker for DomainAdmissionAuthority {}
```

Use the raw lane when:

- the pleasant lane stops being semantically obvious
- you need direct access to checked gates or readiness topology
- you are implementing a domain helper that itself should lower into the substrate

## How It Relates To Other Features

- Use [When To Stay Low-Level](./when-to-stay-low-level.md) when deciding whether to expose the raw substrate directly or wrap it.
- Use [Checked Recipe Progression](./checked-recipe-progression.md) when the new flow needs richer divergence categories.
- Use [Composition-Family Lowering](./composition-family-lowering.md) when the domain flow includes same-family symbolic identity work.

## Inspection And Debugging

- if your new domain API hides authority, basis, or trust-boundary distinctions, it is probably too magical
- if the flow returns a plain `Result` but stale or rebind categories matter, it is probably flattening too much
- if you are inventing new wrappers, verify that each one buys a real invariant rather than just renaming a substrate type

## Anti-Patterns

- Do not create a second typestate substrate inside a domain crate.
- Do not use domain-specific builders to hide trust-boundary or witness law.
- Do not move shared progression law into `forge-foundational`; it belongs in `forge-proof`.

## Related Docs

- [Recipes And Stages](../features/recipes-and-stages.md)
- [Checked Transitions](../features/checked-transitions.md)
- [When To Stay Low-Level](./when-to-stay-low-level.md)
