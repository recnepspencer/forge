# Runtime Readmission

## What This Feature Is

This workflow shows how to take a lowered recipe that crossed a trust boundary, restore it under a strong basis, regain execution-readiness, and optionally execute it.

## Why You Use It

- a lowered form was serialized, transported, restored, or otherwise trust-bridged
- the original strong basis is no longer valid as-is
- you still need to continue toward execution honestly

## Stable Entry Points

- `bridge_trust_boundary()`
- `LoweredReadmissionContext::new(...)`
- `LoweredReadmissionReadiness`
- `CheckedReadmitLoweredForExecutionReadyTransition`
- `checked_readmit_ready_and_execute_recipe(...)`
- `readmit_ready_and_execute_recipe(...)`

## Core Mental Model

This is not just "resume execution."

It is:

1. explicit basis weakening at the trust boundary
2. explicit authority-backed readmission
3. explicit runtime readiness admission
4. optional execution

If any of those are erased, the architecture stops being honest.

## How It Executes

1. start with a current lowered recipe
2. call `bridge_trust_boundary()`
3. build a `LoweredReadmissionContext`
4. pass it through the checked or unchecked readmission surface
5. continue as ready or executed

## Small Example

```rust
use forge_proof::LoweredReadmissionContext;

type Context = LoweredReadmissionContext<u8, ReadmissionAuthority, &'static str, ReadinessAuthority>;

struct ReadmissionAuthority;
struct ReadinessAuthority;

let _ = std::any::type_name::<Context>();
```

This is the smallest honest example because the readmission context is the stable center of the workflow.

## Real Example

```rust
use forge_proof::{
    AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness,
    CheckedReadmitLoweredForExecutionReadyTransition, ContextualTransition,
    LowerRecipeTransition, LoweredReadmissionContext, LoweredReadmissionReadiness, Recipe,
    RecipeResolutionContext, ResolveRecipeTransition, Transition, Unresolved,
};

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}

struct ReadmissionAuthority;
impl AuthorityMarker for ReadmissionAuthority {}

struct ReadinessAuthority;
impl AuthorityMarker for ReadinessAuthority {}

type SameBasisReadmissionReadiness = LoweredReadmissionReadiness<
    &'static str,
    u8,
    u8,
    ReadmissionAuthority,
    &'static str,
    ReadinessAuthority,
    &'static str,
    &'static str,
    &'static str,
>;

fn readmit(
    resolution_authority: AuthorityWitness<ResolutionAuthority>,
    lowering_capability: CapabilityWitness<LoweringCapability>,
    readmission_authority: AuthorityWitness<ReadmissionAuthority>,
    readiness_authority: AuthorityWitness<ReadinessAuthority>,
) {
    let unresolved = Recipe::<Unresolved, _>::new("payload");
    let resolved = ResolveRecipeTransition.transition(
        unresolved,
        RecipeResolutionContext::new(12_u8, resolution_authority),
    );
    let lowered = LowerRecipeTransition::new(lowering_capability)
        .transition(resolved.into_value())
        .into_value();

    let ready = CheckedReadmitLoweredForExecutionReadyTransition.transition(
        lowered.bridge_trust_boundary(),
        SameBasisReadmissionReadiness::ready(LoweredReadmissionContext::new(
            12_u8,
            readmission_authority,
            "runtime admission",
            readiness_authority,
        )),
    );

    let _ = ready;
}
```

What is happening here:

- the lowered recipe does not skip the trust-boundary weakening step
- readmission and readiness authority remain separate and explicit
- the recovered form is ready again only because both steps occurred

## How It Relates To Other Features

- Use [Staleness And Rebind](./staleness-and-rebind.md) when you only need weakening, not recovery through execution-readiness.
- Use [Happy-Path Recipe Progression](./happy-path-recipe-progression.md) when the form never crossed a trust boundary.
- Use [Fixed-Arity Join](./fixed-arity-join.md) when the readmitted ready form participates in static multi-input composition afterward.

## Inspection And Debugging

- inspect whether the input is really boundary-bridged before debugging later readiness
- inspect the readmission context to see which basis and authorities are being supplied
- if a flow is surprisingly denied or deferred, use the checked variant and inspect its exact category

## Anti-Patterns

- Do not resume from a bridged lowered form as though nothing happened.
- Do not conflate readmission authority with readiness authority unless the domain explicitly does so.
- Do not rebuild a strong basis manually instead of using the readmission surfaces.

## Current Limits

- this workflow is specifically about lowered-form re-entry
- it preserves progression law, not descriptive diagnostics or provenance
- the generic aliases are still a bit verbose today

## Related Docs

- [Runtime Readmission](../features/runtime-readmission.md)
- [Boundary Readmission](../features/boundary-readmission.md)
- [Execution-Ready And Executed](../features/execution-ready-and-executed.md)
