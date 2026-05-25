# Advisory And Violation Contributions

## What This Feature Is

Admission contributions let a domain say that a Query declaration or admitted
plan is advisory or violating for domain-specific reasons while still
materializing a canonical Query-owned admission artifact.

## Why You Use It

- you need geometry-specific admission posture without opening canonical
  admission constructors
- you want one typed place to express "this can proceed with warnings" versus
  "this must stop"
- you want admission artifacts that still participate in Query inspection,
  traceability, and certification

## Stable Entry Points

- `forge_query_domain(...).for_intent(...).advises(...).because(...).materialize()`
- `forge_query_domain(...).for_intent(...).violates_invariant(...).because(...).materialize()`
- `forge_query_domain(...).for_admitted_intent_plan(...).advises(...).because(...).materialize()`
- `forge_query_domain(...).for_admitted_intent_plan(...).violates(...).because(...).materialize()`

Checked lane:

- `.try_materialize()`

Proof lane:

- `ForgeQueryAdmissionContributionAuthoring::advisory(...)`
- `ForgeQueryAdmissionContributionAuthoring::violation(...)`

## Core Mental Model

The domain owns the semantic reason. Query owns the admission artifact.

You are not constructing a raw `ForgeQueryIntentAdmissionDecision` yourself.
You are contributing posture to one of two valid target families:

- declaration-bound intent work
- admitted-plan-bound runtime work

That distinction matters because declaration-bound posture shapes intent-facing
artifacts, while admitted-plan posture shapes runtime-facing admission
decisions.

## How It Executes

1. choose the correct target family with `for_intent(...)` or
   `for_admitted_intent_plan(...)`
2. choose advisory or violation posture
3. add the domain reason with `.because(...)`
4. materialize through the common or checked lane
5. let Query produce the canonical admission artifact or decision

## Small Example

```rust
let artifact = forge_query_domain("worth.spatial")
    .for_intent(&declaration)
    .violates_invariant("topology.non_manifold_edge_split")
    .because("splitting this edge would create a non-manifold vertex fan")
    .materialize()?;
```

This is the smallest honest example because it shows the ordinary declaration
lane without dropping into proof types.

## Real Example

```rust
let decision = forge_query_domain("worth.spatial")
    .for_admitted_intent_plan(&plan)
    .advises("topology.merge_requires_review")
    .because("the merge preserves authority but leaves two plausible face pairings")
    .materialize()?;

let digest = match decision {
    ForgeQueryIntentAdmissionDecision::Advisory(advisory) => advisory.decision_digest(),
    ForgeQueryIntentAdmissionDecision::Admitted(plan) => plan.decision_digest(),
    ForgeQueryIntentAdmissionDecision::Violation(violation) => violation.decision_digest(),
};
```

What is authoritative:
- Query's admission artifact family

What is derived:
- the domain-authored posture and reason

What gets retained:
- decision digest and trace identity

What gets inspected:
- the canonical admission result, not a domain-local wrapper

## How It Relates To Other Features

- Pair this with [Declaration Vs Admitted-Plan Targets](./declaration-vs-admitted-plan-targets.md)
  when you need to choose the right admission target.
- Pair it with [Declaration-Scoped Support And Traceability](../support/declaration-scoped-support-and-traceability.md)
  when you want supporting rows in addition to the admission outcome.
- Pair it with [Registering Domain Invariants Through Query](../invariants/registering-domain-invariants-through-query.md)
  when an advisory or violation should align with explicit invariant policy.

## Inspection And Debugging

- use `.try_materialize()` when you need denied metadata instead of an error
- inspect the resulting admission digest and trace through the returned Query
  artifact
- use the checked lane when tooling or certification code needs the exact
  transition outcome

## Anti-Patterns

- treating advisory as a soft string note instead of typed posture
- using the declaration lane when you already hold an admitted runtime plan
- building a local domain admission enum above the Query artifact family

## Current Limits

- admission contributions only cover advisory and violation posture here
- admission-local support is documented separately because it materializes a
  support report, not an admission decision
- this feature does not make canonical admission constructors public

## Related Docs

- [Declaration Vs Admitted-Plan Targets](./declaration-vs-admitted-plan-targets.md)
- [Declaration-Scoped Support And Traceability](../support/declaration-scoped-support-and-traceability.md)
- [Admission-Local Support Reports](../support/admission-local-support-reports.md)
- [Intent Admission](../../execution/intent-admission.md)
