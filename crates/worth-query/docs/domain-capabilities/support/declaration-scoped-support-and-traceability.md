# Declaration-Scoped Support And Traceability

## What This Feature Is

Declaration-scoped support and traceability let a domain attach typed support
or traceability meaning to a Query declaration and receive a canonical
declaration-bound support artifact.

## Why You Use It

- you want to record why a geometry declaration is supported, not just whether
  it succeeded
- you need typed traceability artifacts for certification and debugging
- you want declaration-local support without mutating global support matrices

## Stable Entry Points

- `worth_query_domain(...).for_intent(...).supports_capability(...).because(...).materialize()`
- `worth_query_domain(...).for_intent(...).supports_traceability(...).because(...).materialize()`

Checked lane:

- `.try_materialize()`

Proof lane:

- `WorthQuerySupportContributionAuthoring::declaration_support(...)`
- `WorthQuerySupportContributionAuthoring::declaration_traceability(...)`

## Core Mental Model

Support says a declaration is backed by some capability fact.

Traceability says the declaration should retain a support artifact that helps a
later engineer understand why the domain considered it meaningful.

Both are declaration-scoped. They do not mutate a global runtime registry.

## How It Executes

1. choose `for_intent(...)`
2. choose capability support or traceability support
3. attach the domain reason with `.because(...)`
4. materialize a declaration-bound support artifact

## Small Example

```rust
let artifact = worth_query_domain("worth.spatial")
    .for_intent(&declaration)
    .supports_capability("graph.face_inner_loop_insertion")
    .because("the topology kernel supports inserting one bounded inner loop")
    .materialize()?;
```

## Real Example

```rust
let traceability = worth_query_domain("worth.spatial")
    .for_intent(&declaration)
    .supports_traceability("traceability.edge_split")
    .because("the split preserves edge provenance and successor naming")
    .materialize()?;

let digest = traceability.materialization_digest();
let intent_name = traceability.intent_name();
```

What is authoritative:
- the canonical Query support artifact

What is derived:
- the domain-authored reason and semantic code

What gets retained:
- declaration identity and support digest

## How It Relates To Other Features

- use [Admission-Local Support Reports](./admission-local-support-reports.md) when
  support belongs beside an admitted runtime decision instead of a declaration
- use [Lower-Runtime Support And Boundary Traceability](./lower-runtime-support-and-boundary-traceability.md)
  when the support story belongs to a lower-runtime boundary envelope
- pair this with [Advisory And Violation Contributions](../admission/advisory-and-violation-contributions.md)
  when support should accompany an admission posture

## Inspection And Debugging

- the artifact digest is the stable cross-lane identity
- use `.try_materialize()` when tooling needs denied metadata
- the checked lane keeps the category, posture, and target kind visible

## Anti-Patterns

- using declaration support as a substitute for runtime admission support
- flattening capability meaning into free-form strings outside the typed lane
- treating declaration support as global support inventory mutation

## Current Limits

- this feature is declaration-bound by design
- support and traceability share the artifact family but preserve distinct
  posture
- richer certification bundles live in the certification category, not here

## Related Docs

- [Admission-Local Support Reports](./admission-local-support-reports.md)
- [Lower-Runtime Support And Boundary Traceability](./lower-runtime-support-and-boundary-traceability.md)
- [Advisory And Violation Contributions](../admission/advisory-and-violation-contributions.md)
