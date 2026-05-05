# Preconstruction And Readiness Gates

## What This Feature Is

Preconstruction and readiness gates are the typed "not yet" surfaces that sit in front of actual progression. They let code say whether a transition is ready, denied, deferred, stale, rebind-required, or failed before pretending progression can run.

## Why You Use It

- you need to gate construction or progression explicitly
- you need denial or deferment before a stronger form is even attempted
- you need checked readiness to preserve stale or rebind-required inputs

## Stable Entry Points

- `PreConstructionGate<C, D, De>`
- `PreConstructionGate::ready(...)`
- `PreConstructionGate::denied(...)`
- `PreConstructionGate::deferred(...)`
- `PreConstructionGate::map_ready(...)`
- `TransitionReadiness<C, D, De, St, R, F>`
- `TransitionReadiness::ready(...)`
- `TransitionReadiness::denied(...)`
- `TransitionReadiness::deferred(...)`
- `TransitionReadiness::stale(...)`
- `TransitionReadiness::rebind_required(...)`
- `TransitionReadiness::failed(...)`
- `TransitionReadiness::map_ready(...)`

## Core Mental Model

These gate types are not just "another result."

They are pre-progression truth:

- `PreConstructionGate` is for "can we even form the next trusted context?"
- `TransitionReadiness` is for "is this transition currently ready, or which exact non-ready category applies?"

That distinction matters because checked progression is built from these gates.

## How It Executes

Typical usage:

1. compute or inspect some contextual conditions
2. package the result as a preconstruction gate or readiness gate
3. pass that gate into a checked progression surface
4. let the checked transition preserve the category without inventing fake success

## Small Example

```rust
use forge_proof::PreConstructionGate;

let ready = PreConstructionGate::<u64, &'static str, &'static str>::ready(7);
let mapped = ready.map_ready(|value| value + 1);

assert!(matches!(mapped, PreConstructionGate::Ready(8)));
```

This is the smallest honest example because it shows the "map only the ready lane" behavior that defines the gate model.

## Real Example

```rust
use forge_proof::{
    ExecutionReadinessContext, TransitionReadiness,
};

type Readiness = TransitionReadiness<
    ExecutionReadinessContext<&'static str, ReadinessAuthority>,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
>;

struct ReadinessAuthority;

let stale = Readiness::stale("stale");
let rebind = Readiness::rebind_required("rebind");
let failed = Readiness::failed("failed");

assert!(matches!(stale, TransitionReadiness::Stale("stale")));
assert!(matches!(rebind, TransitionReadiness::RebindRequired("rebind")));
assert!(matches!(failed, TransitionReadiness::Failed("failed")));
```

What this shows:

- readiness is richer than yes/no
- non-ready categories are explicit and stable
- downstream checked transitions can preserve those categories directly

## How It Relates To Other Features

- Pair this with [Checked Transitions](./checked-transitions.md) because the checked APIs consume these gate types.
- Pair this with [Transition Outcomes](./transition-outcomes.md) because the gate categories usually become outcome categories.
- Pair this with [Runtime Readmission](./runtime-readmission.md) when a bridged lowered form must be checked before execution-readiness resumes.

## Inspection And Debugging

- look at the gate type aliases first; they usually show the intended divergence categories more clearly than a long function body
- `map_ready(...)` is the clean way to transform only the ready lane
- when a flow is mysteriously blocked, inspect the gate construction site before the checked transition itself

## Anti-Patterns

- Do not use a gate when the transition is already known to be unconditionally ready.
- Do not flatten stale or rebind-required into generic failure at the gate layer.
- Do not construct fake ready contexts just to avoid handling denial or deferment honestly.

## Current Limits

- raw generic forms can be verbose
- the gate types preserve category shape but do not choose domain-specific policy for you
- today they are explicit enums, not a fluent policy DSL

## Related Docs

- [Checked Transitions](./checked-transitions.md)
- [Transition Outcomes](./transition-outcomes.md)
- [Runtime Readmission](./runtime-readmission.md)
