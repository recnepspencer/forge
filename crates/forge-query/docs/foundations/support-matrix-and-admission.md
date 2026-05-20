# Support Matrix And Admission

## What This Feature Is

The public support matrix is Forge Query's explicit contract for what the
runtime facade supports now, what is intentionally deferred, and what must
fail closed until a real implementation exists.

Admission is the executable form of that contract. It lets callers ask the
runtime whether a public family is actually available before they build on it.

## Why You Use It

- you need to know whether a public surface is truly supported or only visible
  so later milestones extend the same facade
- you want typed denials instead of silent fallback when a family is deferred
  or unsupported
- you want one machine-checkable way to teach support posture in docs, product
  code, and certification

## Stable Entry Points

- `workspace.public_api_contract()`
- `workspace.public_handle_contract()`
- `workspace.public_support_matrix()`
- `workspace.public_mutation_surface_report()`
- `workspace.admit_public_api_family(...)`
- `ForgeQueryRuntimeFacadeFamily`
- `ForgeQueryRuntimeFamilySupportStatus`

## Core Mental Model

There are three different questions here:

- what names exist in the public vocabulary
- which families are supported for real runtime-backed use
- which families are visible now so future work extends the same facade

The support matrix answers those questions row by row.

For mutation-specific preferred-versus-lower-level posture, pair it with
`workspace.public_mutation_surface_report()`.

Each row tells you:

- the surface or facade family
- whether it is `Supported`, `DeferredDebt`, or `Unsupported`
- the owning roadmap closure or runtime gate
- whether it must fail closed
- whether future work is forbidden from creating a sibling API instead

Good to know:

- method presence is not a support claim
- `Intent` in the public facade means the shared intent vocabulary exists and
  covered intent families can be admitted when the runtime profile supports
  them
- it does not mean every intent-shaped operation is admitted as an ordinary
  production-facing runtime path in every runtime profile

## How It Executes

1. the runtime derives a public API contract from its support profile
2. the public support matrix freezes that into supported, deferred, and
   unsupported rows
3. your code calls `workspace.admit_public_api_family(...)` before depending on
   a family that may be deferred or unsupported
4. supported families return a sealed family contract
5. deferred or unsupported families deny typed and early

## Small Example

```rust
use forge_query::facade::ForgeQueryRuntimeFacadeFamily;

let workspace = runtime.workspace("support").unwrap();

let matrix = workspace.public_support_matrix();
let live = matrix.row_for_family(ForgeQueryRuntimeFacadeFamily::Live).unwrap();

assert_eq!(live.status().as_str(), "supported");
assert!(!live.support_contract_digest().is_empty());

workspace
    .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Live)
    .unwrap();
```

This is the smallest honest example because it shows both sides: inspect the
support matrix, then ask for executable admission.

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
assert!(temporal.admission_fail_closed());

assert_eq!(async_resource.status().as_str(), "deferred-debt");
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

Read that `Intent` row carefully. It means "do not teach blanket facade-family
intent support here." It does not erase the concrete covered intent
families described in [Intent Admission](../execution/intent-admission.md).

What is supported now:

- `Read`
- `Live`
- `Computed`
- `Effect`
- `BranchPreview`
- `Write`
- `Inspect`

What is visible but deferred:

- `Temporal` -> temporal basis and time-aware subscription closure
- `AsyncResource` -> async/resource query closure
- `MixedCauseDelivery` -> mixed truth/time/async delivery closure
- `temporal-async-certification` -> temporal/async certification closure
- `StoreBackedExecution` -> store-backed execution parity
- `DurableArtifacts` -> durable artifact reload and continuation

What is public vocabulary but not blanket facade-family support:

- `Intent`

## How It Relates To Other Features

- Use [Workspace Overview](workspace-overview.md) when you are deciding what
  runtime DX you can rely on today.
- Use [State](state.md) when you need posture before execution exists.
- Use [Intent Admission](../execution/intent-admission.md) when you need the
  concrete covered intent families and their common-path or advanced-path
  usage, not just the facade-family support posture.
- Use [Inspection](../capabilities/inspection.md) when you are building
  tooling, certification, or explainability on top of runtime evidence.

## Inspection And Debugging

The public support matrix gives you row-by-row posture. The public API
contract and handle contract give you the richer details behind it.

Look for:

- `status()`
- `owner_milestone()`
- `extension_rule()`
- `parallel_api_forbidden()`
- `admission_fail_closed()`
- `support_contract_digest()`

`owner_milestone()` is roadmap provenance. Product code should usually branch
on support status, fail-closed posture, extension rule, and the support
contract digest rather than hard-coding milestone names.

For deeper checks:

- `workspace.public_api_contract()` for family-level lane and evidence posture
- `workspace.public_handle_contract()` for handle families and required
  inspection sections
- `workspace.public_mutation_surface_report()` for preferred and lower-level
  mutation posture

## Anti-Patterns

- assuming a public method is supported because it exists
- teaching blanket intent support from the `Intent` facade-family row
- building runtime features against deferred async or temporal families without
  an admission gate
- adding a new public sibling API for future async work instead of extending
  the stabilized facade

## Current Limits

- the support matrix is the source of truth for facade-family posture today
- deferred families are intentionally visible before implementation so
  downstream work can plan honestly
- admission tells you support posture; it does not perform the feature on your
  behalf

## Related Docs

- [Workspace Overview](workspace-overview.md)
- [State](state.md)
- [Intent Admission](../execution/intent-admission.md)
- [Inspection](../capabilities/inspection.md)
- [Writes And Intents](../execution/writes-and-intents.md)
