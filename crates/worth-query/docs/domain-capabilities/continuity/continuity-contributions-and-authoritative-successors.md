# Continuity Contributions And Authoritative Successors

## What This Feature Is

Continuity contributions let a domain say how authoritative identity continues
across an admitted runtime plan and materialize Query-owned continuity evidence.

## Why You Use It

- you need to prove which edge, face, or region continued as which successor
- you want continuity truth attached to admitted runtime work rather than left
  as caller-side bookkeeping
- you need canonical continuity evidence for certification and inspection

## Stable Entry Points

- `worth_query_domain(...).for_admitted_intent_plan(...).preserves_continuity(...).because(...).materialize()`

Checked lane:

- `.try_materialize()`

Proof lane:

- `WorthQueryContinuityContributionAuthoring::preserved_rebind(...)`

## Core Mental Model

Continuity is authoritative mutation evidence, not an annotation on a
declaration.

That is why the ordinary lane is admitted-plan-bound. You are saying how truth
continued through an admitted runtime path.

## How It Executes

1. start with an admitted intent plan
2. name the prior and successor authoritative identities
3. attach the domain reason
4. materialize Query continuity evidence

## Small Example

```rust
let evidence = worth_query_domain("worth.spatial")
    .for_admitted_intent_plan(&plan)
    .preserves_continuity("identity.edge_split", "edge:before", "edge:after")
    .because("the split replaces one edge with one canonical successor")
    .materialize()?;
```

## Real Example

```rust
let evidence = worth_query_domain("worth.spatial")
    .for_admitted_intent_plan(&plan)
    .preserves_continuity("identity.face_rebind", "face:before", "face:after")
    .because("topology normalization rebinds the authoritative face without losing identity")
    .materialize()?;

let digest = evidence.continuity_resolution_digest();
```

## How It Relates To Other Features

- use [Continuity Vs Correspondence](./continuity-vs-correspondence.md) when
  you are deciding whether you need authoritative continuity or weaker
  correspondence
- pair this with [Advisory And Violation Contributions](../admission/advisory-and-violation-contributions.md)
  when an admitted operation also carries admission posture
- pair it with [Capability Gaps And Invariant Denials](../invariants/capability-gaps-and-invariant-denials.md)
  when a continuity story interacts with graph invariant truth

## Inspection And Debugging

- continuity evidence has its own digest family
- use the checked lane when tooling needs denial details
- continuity belongs to runtime truth; if the target is wrong, the evidence
  story is wrong even if the code compiles

## Anti-Patterns

- using declaration-bound surfaces for authoritative continuity
- using correspondence-only logic when the runtime truly knows predecessor and
  successor truth
- rebuilding successor evidence outside the admitted plan

## Current Limits

- the ordinary common lane currently centers on preserved continuity through
  authoritative successor rebinding
- correspondence-only and split-heavy variants are sharper surfaces, not the
  main common path

## Related Docs

- [Continuity Vs Correspondence](./continuity-vs-correspondence.md)
- [Advisory And Violation Contributions](../admission/advisory-and-violation-contributions.md)
- [Capability Gaps And Invariant Denials](../invariants/capability-gaps-and-invariant-denials.md)
