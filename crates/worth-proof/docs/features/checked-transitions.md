# Checked Transitions

## What This Feature Is

Checked transitions are the progression surfaces you use when non-success categories must stay typed and visible instead of collapsing into a simple success-only flow.

## Why You Use It

- resolution or readiness can be denied or deferred
- lowering can require rebinding instead of progressing
- readiness can yield stale or rebind-required categories
- you want one helper to preserve the real topology of progression failure

## Stable Entry Points

- pleasant lane:
  - `use worth_proof::prelude::*;`
  - `.try_resolve(...)`
  - `.try_resolve_ready(...)`
  - `.try_lower(...)`
  - `.try_lower_ready(...)`
  - `.try_admit(...)`
  - `.try_admit_ready(...)`
  - `.try_ready(...)`
  - `.try_ready_now(...)`
  - `.try_execute()`
  - `ProofOutcome`
- raw lane:
  - `use worth_proof::raw::*;`
  - `CheckedResolveRecipeTransition`
  - `CheckedLowerRecipeTransition<C>::new()`
  - `CheckedAdmitRecipeTransition<Auth>::new()`
  - `RecipeResolutionGate<...>`
  - `RecipeLoweringReadiness<...>`
  - `RecipeAdmissionReadiness<...>`
  - `ExecutionReadyAdmissionReadiness<...>`
  - `TransitionOutcome`

## Core Mental Model

Checked transitions preserve real divergence.

They are for situations where the answer is not just:

- progressed
- did not progress

Instead, they preserve distinctions such as:

- denied
- deferred
- stale
- rebind-required
- failed

## Pleasant Lane First

```rust
use worth_proof::prelude::*;

fn checked_ready_execute(
    resolution_authority: worth_proof::AuthorityWitness<ResolutionAuthority>,
    lowering_capability: worth_proof::CapabilityWitness<LoweringCapability>,
    readiness_authority: worth_proof::AuthorityWitness<ReadinessAuthority>,
) {
    let executed = recipe("payload")
        .try_resolve_ready(12_u8, resolution_authority)
        .try_lower_ready(lowering_capability)
        .try_ready_now("runtime admission", readiness_authority)
        .try_execute();

    let _ = executed.kind();
}

struct ResolutionAuthority;
impl worth_proof::AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl worth_proof::CapabilityMarker for LoweringCapability {}

struct ReadinessAuthority;
impl worth_proof::AuthorityMarker for ReadinessAuthority {}
```

## Equivalent Raw Surface

```rust
use worth_proof::raw::*;

type CheckedReadiness = ExecutionReadyAdmissionReadiness<
    &'static str,
    u8,
    &'static str,
    ReadinessAuthority,
    &'static str,
    &'static str,
    &'static str,
>;

fn checked_ready_execute(
    resolution_authority: AuthorityWitness<ResolutionAuthority>,
    lowering_capability: CapabilityWitness<LoweringCapability>,
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

    let executed = checked_admit_ready_and_execute_recipe(
        lowered,
        CheckedReadiness::ready(ExecutionReadinessContext::new(
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

struct ReadinessAuthority;
impl AuthorityMarker for ReadinessAuthority {}
```

Use the raw lane when:

- you need direct gate and readiness types in the same local view
- you are building a domain alias over the checked substrate
- you need exact raw `TransitionOutcome` pattern matching

## How It Relates To Other Features

- Pair this with [Transition Outcomes](./transition-outcomes.md) because those outcome categories are the point of the checked surfaces.
- Pair this with [Preconstruction And Readiness Gates](./preconstruction-and-readiness-gates.md) because checked progression is built on those gate types.
- Pair this with [Runtime Readmission](./runtime-readmission.md) when a boundary-bridged lowered form must regain execution-readiness under checked conditions.

## Inspection And Debugging

- `ProofOutcome::kind()` is the fastest pleasant-lane read
- checked alias types show the exact divergence topology at the type level
- raw `TransitionOutcome` pattern matching is the main way to inspect what happened when you drop lower

## Anti-Patterns

- Do not replace checked progression with `Result<T, E>` when stale or rebind categories matter.
- Do not use checked helpers just to look sophisticated when only the straight success path exists.
- Do not erase checked outcomes immediately if downstream logic still cares about why progression stopped.

## Related Docs

- [Transition Outcomes](./transition-outcomes.md)
- [Preconstruction And Readiness Gates](./preconstruction-and-readiness-gates.md)
- [Runtime Readmission](./runtime-readmission.md)
