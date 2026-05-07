# Runtime Readmission

## What This Feature Is

This workflow shows how to take a lowered recipe that crossed a trust boundary, restore it under a strong basis, regain execution-readiness, and optionally execute it.

## Why You Use It

- a lowered form was serialized, transported, restored, or otherwise trust-bridged
- the original strong basis is no longer valid as-is
- you still need to continue toward execution honestly

## Stable Entry Points

- pleasant lane:
  - `use forge_proof::prelude::*;`
  - `.bridge_trust_boundary()`
  - `.readmit_with(authority, basis)`
  - `.ready_with(authority, runtime)`
  - `.execute()`
- raw lane:
  - `use forge_proof::raw::*;`
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

## Pleasant Lane First

```rust
use forge_proof::prelude::*;

fn readmit(
    resolution_authority: forge_proof::AuthorityWitness<ResolutionAuthority>,
    lowering_capability: forge_proof::CapabilityWitness<LoweringCapability>,
    readmission_authority: forge_proof::AuthorityWitness<ReadmissionAuthority>,
    readiness_authority: forge_proof::AuthorityWitness<ReadinessAuthority>,
) {
    let executed = recipe("payload")
        .resolve_with(resolution_authority, 12_u8)
        .lower_with(lowering_capability)
        .bridge_trust_boundary()
        .readmit_with(readmission_authority, 19_u16)
        .ready_with(readiness_authority, "runtime admission")
        .execute();

    let _ = executed;
}

struct ResolutionAuthority;
impl forge_proof::AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl forge_proof::CapabilityMarker for LoweringCapability {}

struct ReadmissionAuthority;
impl forge_proof::AuthorityMarker for ReadmissionAuthority {}

struct ReadinessAuthority;
impl forge_proof::AuthorityMarker for ReadinessAuthority {}
```

What stays explicit:

- trust-boundary weakening happens before readmission
- readmission authority is distinct from readiness authority
- a new strong basis is visibly supplied in the readmission call

## Equivalent Raw Surface

```rust
use forge_proof::raw::*;

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

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}

struct ReadmissionAuthority;
impl AuthorityMarker for ReadmissionAuthority {}

struct ReadinessAuthority;
impl AuthorityMarker for ReadinessAuthority {}
```

Use the raw lane when:

- you need checked readmission categories in the same local view
- you are authoring a domain-specific readmission helper
- you need direct access to `LoweredReadmissionContext` or `LoweredReadmissionReadiness`

## How It Relates To Other Features

- Use [Staleness And Rebind](./staleness-and-rebind.md) when you only need weakening, not recovery through execution-readiness.
- Use [Happy-Path Recipe Progression](./happy-path-recipe-progression.md) when the form never crossed a trust boundary.
- Use [Fixed-Arity Join](./fixed-arity-join.md) when the readmitted ready form participates in static multi-input composition afterward.

## Inspection And Debugging

- inspect whether the input is really boundary-bridged before debugging later readiness
- inspect the readmission authority and replacement basis first
- if a flow is surprisingly denied or deferred, drop to the raw checked variant and inspect its exact category

## Anti-Patterns

- Do not resume from a bridged lowered form as though nothing happened.
- Do not conflate readmission authority with readiness authority unless the domain explicitly does so.
- Do not rebuild a strong basis manually instead of using the readmission surfaces.

## Related Docs

- [Runtime Readmission](../features/runtime-readmission.md)
- [Boundary Readmission](../features/boundary-readmission.md)
- [Execution-Ready And Executed](../features/execution-ready-and-executed.md)
