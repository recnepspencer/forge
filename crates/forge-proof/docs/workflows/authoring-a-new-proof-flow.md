# Authoring A New Proof Flow

## What This Feature Is

This workflow explains how a domain crate should build a new progression surface on top of `forge-proof` without smuggling in forged progression, hidden trust boundaries, or flattened non-success topology.

## Why You Use It

- you are building a new domain-specific progression API
- you want to reuse `forge-proof` instead of inventing another bespoke typestate layer
- you want the resulting domain flow to stay honest under hostile pressure

## Stable Entry Points

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

## How It Executes

1. choose whether the flow is artifact-based, recipe-based, or both
2. decide what proofs, basis wrappers, and witnesses are real
3. decide whether the workflow is success-only or checked
4. build the smallest explicit public domain wrapper or helper around those surfaces
5. add compile-fail or hostile tests for the invariants you care about

## Small Example

```rust
use forge_proof::{Recipe, Unresolved};

type DomainDraft = Recipe<Unresolved, &'static str>;

let _ = std::any::type_name::<DomainDraft>();
```

This is the smallest honest example because most domain flows start by deciding which base carrier and initial stage actually fit.

## Real Example

```rust
use forge_proof::{
    AdmitRecipeTransition, AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness,
    LowerRecipeTransition, Recipe, RecipeResolutionContext, ResolveRecipeTransition, Transition,
    Unresolved,
};

struct DomainResolutionAuthority;
impl AuthorityMarker for DomainResolutionAuthority {}

struct DomainLoweringCapability;
impl CapabilityMarker for DomainLoweringCapability {}

struct DomainAdmissionAuthority;
impl AuthorityMarker for DomainAdmissionAuthority {}

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
```

What this demonstrates:

- the domain flow reuses the substrate instead of replacing it
- domain-specific authority and capability names remain domain-owned
- the proof-bearing progression law stays in `forge-proof`

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

## Current Limits

- this workflow is guidance, not a macro generator
- the crate still expects domain crates to choose their own marker types and payload semantics
- AI-friendly higher-level facades are still a future DX refinement, not the current stable public story

## Related Docs

- [Recipes And Stages](../features/recipes-and-stages.md)
- [Checked Transitions](../features/checked-transitions.md)
- [When To Stay Low-Level](./when-to-stay-low-level.md)
