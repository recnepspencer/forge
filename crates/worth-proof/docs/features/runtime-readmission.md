# Runtime Readmission

## What This Feature Is

Runtime-readmission surfaces handle the specific case where a lowered recipe crossed a trust boundary, must regain a strong basis, and then must regain execution-readiness before execution can continue.

## Why You Use It

- you have a boundary-bridged lowered recipe
- execution is still the goal, but the original strong basis no longer applies
- you need a checked or unchecked path back into ready and executed forms

## Stable Entry Points

- pleasant lane:
  - `use worth_proof::prelude::*;`
  - `.bridge_trust_boundary()`
  - `.readmit_with(authority, basis)`
  - `.ready_with(authority, runtime)`
  - `.execute()`
- raw lane:
  - `use worth_proof::raw::*;`
  - `LoweredReadmissionContext<...>::new(...)`
  - `LoweredReadmissionReadiness<...>`
  - `ReadmitLoweredForExecutionReadyTransition`
  - `CheckedReadmitLoweredForExecutionReadyTransition`
  - `readmit_ready_and_execute_recipe(...)`
  - `checked_readmit_ready_and_execute_recipe(...)`

## Core Mental Model

Runtime readmission is a two-step recovery path:

1. regain a strong current basis from a boundary-bridged lowered recipe
2. admit the readmitted lowered recipe back into execution-ready state

## Pleasant Lane First

```rust
use worth_proof::prelude::*;

fn restore_and_execute(
    resolution_authority: worth_proof::AuthorityWitness<ResolutionAuthority>,
    lowering_capability: worth_proof::CapabilityWitness<LoweringCapability>,
    readmission_authority: worth_proof::AuthorityWitness<ReadmissionAuthority>,
    readiness_authority: worth_proof::AuthorityWitness<ReadinessAuthority>,
) {
    let executed = recipe("payload")
        .resolve_with(resolution_authority, 17_u8)
        .lower_with(lowering_capability)
        .bridge_trust_boundary()
        .readmit_with(readmission_authority, 19_u16)
        .ready_with(readiness_authority, "runtime admission")
        .execute();

    let _ = executed;
}

struct ResolutionAuthority;
impl worth_proof::AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl worth_proof::CapabilityMarker for LoweringCapability {}

struct ReadmissionAuthority;
impl worth_proof::AuthorityMarker for ReadmissionAuthority {}

struct ReadinessAuthority;
impl worth_proof::AuthorityMarker for ReadinessAuthority {}
```

## Equivalent Raw Surface

```rust
use worth_proof::raw::*;

fn restore_and_execute(
    resolution_authority: AuthorityWitness<ResolutionAuthority>,
    lowering_capability: CapabilityWitness<LoweringCapability>,
    readmission_authority: AuthorityWitness<ReadmissionAuthority>,
    readiness_authority: AuthorityWitness<ReadinessAuthority>,
) {
    let unresolved = Recipe::<Unresolved, _>::new("payload");
    let resolved = ResolveRecipeTransition.transition(
        unresolved,
        RecipeResolutionContext::new(17_u8, resolution_authority),
    );
    let lowered = LowerRecipeTransition::new(lowering_capability)
        .transition(resolved.into_value())
        .into_value();
    let bridged = lowered.bridge_trust_boundary();

    let executed = checked_readmit_ready_and_execute_recipe(
        bridged,
        TransitionReadiness::ready(LoweredReadmissionContext::new(
            19_u16,
            readmission_authority,
            "runtime admission",
            readiness_authority,
        )),
    );

    let _ = executed;
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

## How It Relates To Other Features

- Pair this with [Boundary Readmission](./boundary-readmission.md) because runtime readmission starts from a bridged lowered recipe.
- Pair this with [Execution-Ready And Executed](./execution-ready-and-executed.md) because the destination is usually ready or executed.
- Pair this with [Checked Transitions](./checked-transitions.md) when non-success categories must remain visible.

## Inspection And Debugging

- inspect the bridged basis first to confirm the input is really boundary-bridged
- inspect `LoweredReadmissionContext` construction to see which basis and authorities are being supplied
- use the checked variant when you need to see whether the flow denied, deferred, stayed stale, or failed

## Anti-Patterns

- Do not treat a boundary-bridged lowered recipe as though it were still ready for execution.
- Do not rebuild a strong basis manually outside the explicit readmission surfaces.
- Do not conflate readmission authority with readiness authority unless your domain intentionally makes them the same lane.

## Related Docs

- [Boundary Readmission](./boundary-readmission.md)
- [Execution-Ready And Executed](./execution-ready-and-executed.md)
- [Checked Transitions](./checked-transitions.md)
