# Declaration Vs Admitted-Plan Targets

## What This Feature Is

Domain capability admission work has two ordinary public target families:
declaration-bound and admitted-plan-bound. This doc explains how to choose the
right one.

## Why You Use It

- you want your admission artifact to land on the right authority boundary
- you need to keep geometry-kernel declaration review separate from runtime
  execution review
- you want to avoid fake parity between authoring-time and admitted-plan
  semantics

## Stable Entry Points

- `forge_query_domain(...).for_intent(&declaration)`
- `forge_query_domain(...).for_admitted_intent_plan(&plan)`

Common admission verbs then hang off the returned surface:

- declaration lane: `.advises(...)`, `.violates_invariant(...)`
- admitted-plan lane: `.advises(...)`, `.violates(...)`

## Core Mental Model

Use `for_intent(...)` when the domain is speaking about a Query declaration as
authored.

Use `for_admitted_intent_plan(...)` when the domain is speaking about runtime
work that has already crossed the ordinary admission front door and now has an
admitted plan identity.

These are not interchangeable surfaces with cosmetic differences. They encode
different proof and authority facts.

## How It Executes

1. choose the target family first
2. author posture on the returned surface
3. materialize through common, checked, proof, or raw lanes
4. let Query keep the target identity attached through canonicalization

## Small Example

```rust
let declaration_artifact = forge_query_domain("worth.spatial")
    .for_intent(&declaration)
    .advises("topology.requires_manual_review")
    .because("the declaration leaves two valid manifold-preserving edge paths")
    .materialize()?;
```

## Real Example

```rust
let runtime_decision = forge_query_domain("worth.spatial")
    .for_admitted_intent_plan(&plan)
    .violates("topology.authoritative_target_mismatch")
    .because("the admitted plan no longer binds the intended edge chain")
    .materialize()?;
```

The second example is runtime-facing because the domain is now talking about
the admitted plan's binding and authority, not just the original declaration.

## How It Relates To Other Features

- [Advisory And Violation Contributions](./advisory-and-violation-contributions.md)
  shows the actual admission verbs.
- [Admission-Local Support Reports](../support/admission-local-support-reports.md)
  uses the admitted-plan target when support belongs beside a runtime decision.
- [Continuity Contributions And Authoritative Successors](../continuity/continuity-contributions-and-authoritative-successors.md)
  also uses admitted-plan targets because continuity evidence belongs to
  authoritative mutation truth.

## Inspection And Debugging

- if the target is wrong, the resulting artifact category may still compile but
  tell the wrong story
- compile-fail boundaries protect some illegal mixes, but choosing the honest
  target is still part of correct modeling

## Anti-Patterns

- using `for_intent(...)` after you already have an admitted runtime plan
- teaching the declaration and admitted-plan lanes as if they were just two
  spellings for the same thing
- rebuilding admitted-plan identity outside Query

## Current Limits

- this doc only covers admission target choice
- lower-runtime-bound targets live in support and explanation categories, not
  in ordinary admission contributions

## Related Docs

- [Advisory And Violation Contributions](./advisory-and-violation-contributions.md)
- [Admission-Local Support Reports](../support/admission-local-support-reports.md)
- [Intent Admission](../../execution/intent-admission.md)
