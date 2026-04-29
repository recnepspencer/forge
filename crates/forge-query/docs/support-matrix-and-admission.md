# Support Matrix And Admission

## What This Feature Is

The public support matrix is Forge Query's explicit contract for what the
runtime facade supports now, what is deferred to later milestones, and what
must fail closed until a real runtime implementation exists. Admission is the
typed gate that lets callers ask the runtime whether a public family is
actually available before they build on it.

## Why You Use It

- you need to know whether a public surface is truly stable or only vocabulary
- you want to build runtime features now without accidentally taking a
  dependency on future async work
- you need typed denials instead of silent fallback when a family is deferred
  or unsupported
- you want a machine-checkable compatibility posture rather than a prose promise

## Stable Entry Points

- `workspace.public_api_contract()`
- `workspace.public_handle_contract()`
- `workspace.public_support_matrix()`
- `workspace.public_mutation_api_compatibility_report()`
- `workspace.admit_public_api_family(...)`
- `ForgeQueryRuntimeFacadeFamily`
- `ForgeQueryRuntimeFamilySupportStatus`

These are part of the stabilized public facade. Method presence alone is not a
support claim.

## Core Mental Model

There are three different questions here:

- what names exist in the public vocabulary
- which families are stable for real runtime-backed use
- which families are visible only so future work extends the same facade

The support matrix answers those questions row by row.
For mutation-specific preferred-versus-compatibility posture, pair it with
`workspace.public_mutation_api_compatibility_report()`.

Each row tells you:

- the surface or facade family
- whether it is `Supported`, `DeferredDebt`, or `Unsupported`
- the owning milestone
- whether it must fail closed
- whether future work is forbidden from creating a sibling API instead

Admission is the executable version of that contract. If a family is not
admitted, downstream code should not pretend it is safe just because a method
or type exists.

## How It Executes

1. The runtime derives a public API contract from its support profile.
2. The public support matrix freezes that into stable, deferred, and
   unsupported rows.
3. Your code calls `workspace.admit_public_api_family(...)` before depending on
   a family that might be deferred or unsupported.
4. Supported families return a sealed family contract.
5. Deferred or unsupported families deny typed and early.

## Small Example

```rust
use forge_query::facade::ForgeQueryRuntimeFacadeFamily;

let workspace = runtime.workspace("support").unwrap();

let matrix = workspace.public_support_matrix();
let live = matrix.row_for_family(ForgeQueryRuntimeFacadeFamily::Live).unwrap();

assert_eq!(live.status().as_str(), "supported");
assert_eq!(live.owner_milestone(), "Milestone 9.3");

workspace
    .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Live)
    .unwrap();
```

This is the smallest honest example because it shows both sides: inspection of
the matrix and executable admission.

## Real Example

```rust
use forge_query::facade::{ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily};

let workspace = runtime.workspace("future-gates").unwrap();
let matrix = workspace.public_support_matrix();

let temporal = matrix.row("temporal").unwrap();
let async_resource = matrix.row("async-resource").unwrap();
let intent = matrix
    .row_for_family(ForgeQueryRuntimeFacadeFamily::Intent)
    .unwrap();

assert_eq!(temporal.status().as_str(), "deferred-debt");
assert_eq!(temporal.owner_milestone(), "Milestone 9.4");
assert!(temporal.admission_fail_closed());

assert_eq!(async_resource.status().as_str(), "deferred-debt");
assert_eq!(async_resource.owner_milestone(), "Milestone 9.5");
assert!(async_resource.parallel_api_forbidden());

assert_eq!(intent.status().as_str(), "unsupported");
assert!(intent.admission_fail_closed());

for family in [
    ForgeQueryRuntimeFacadeFamily::Temporal,
    ForgeQueryRuntimeFacadeFamily::AsyncResource,
    ForgeQueryRuntimeFacadeFamily::Intent,
] {
    let error = workspace
        .admit_public_api_family(family)
        .expect_err("deferred or unsupported families must fail closed");

    match error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(denial.family(), family);
            assert!(!denial.reason().is_empty());
        }
        other => panic!("expected typed support denial, got {other:?}"),
    }
}
```

What is stable now:

- `Read`
- `Live`
- `Computed`
- `Effect`
- `BranchPreview`
- `Write`
- `Inspect`

What is visible but deferred:

- `Temporal` -> Milestone 9.4
- `AsyncResource` -> Milestone 9.5
- `MixedCauseDelivery` -> Milestone 9.6
- `temporal-async-certification` -> Milestone 9.7
- `StoreBackedExecution` -> Milestone 10
- `DurableArtifacts` -> Milestone 11

What is vocabulary but not stable compatibility support:

- `Intent`

## How It Relates To Other Features

- Use this with [Workspace Overview](./workspace-overview.md) when you are
  deciding what public runtime DX you can rely on today.
- Use it with [State](./state.md) when you need a family to expose posture even
  before execution exists.
- Use it with [Inspection](./inspection.md) and handle contracts when you are
  building tooling, certification, or explainability on top of the runtime.

## Inspection And Debugging

The public support matrix gives you row-by-row posture. The public API contract
and handle contract give you the richer details behind it.

Look for:

- `status()`
- `owner_milestone()`
- `extension_rule()`
- `parallel_api_forbidden()`
- `admission_fail_closed()`
- `support_contract_digest()`

For deeper checks:

- `workspace.public_api_contract()` for family-level lane and evidence posture
- `workspace.public_handle_contract()` for handle families and required
  inspection sections

## Anti-Patterns

- Assuming a public method is stable because it exists.
- Building runtime features against deferred async or temporal families without
  an admission gate.
- Treating unsupported `Intent` support as equivalent to stable `write(...)`
  support.
- Adding a new public sibling API for future async work instead of extending
  the stabilized facade.

## Current Limits

- The support matrix is the source of truth for public support posture today.
- Deferred families are intentionally visible before implementation so
  downstream work can plan around them.
- Admission tells you support posture. It does not perform the feature on your
  behalf.

## Related Docs

- [Workspace Overview](./workspace-overview.md)
- [State](./state.md)
- [Inspection](./inspection.md)
- [Writes And Intents](./writes-and-intents.md)
