# Preview Inspection And Mutation Planning

## What This Feature Is

This feature lets a domain contribute preview inspection and preview mutation
planning through ordinary Query entry points and receive canonical workflow
artifacts.

## Why You Use It

- you want preview-bound workflow meaning without building a local preview
  adapter layer
- you need a geometry domain to express read-only preview inspection or
  promotion-eligible preview mutation planning
- you want the same canonical workflow semantics across common, checked, and
  proof lanes

## Stable Entry Points

- `forge_query_domain(...).for_intent(...).inspects_query_preview(...).because(...).materialize()`
- `forge_query_domain(...).for_intent(...).plans_preview_mutation(...).because(...).materialize()`

Checked lane:

- `.try_materialize()`

Lower-lane artifact access:

- `ForgeQueryIntentPreviewMutationPlan::workflow_declaration()`
- `ForgeQueryIntentPreviewMutationPlan::into_workflow_declaration()`

## Core Mental Model

Preview inspection and preview mutation planning are declaration-bound domain
capabilities.

Inspection is preview-only and read-oriented.

Mutation planning is promotion-eligible and produces a Query-owned workflow
declaration, not a lowered mutation execution on its own.

## How It Executes

1. enter the declaration-bound surface with `for_intent(...)`
2. choose inspection or mutation planning
3. attach preview session identity and domain reason
4. materialize the preview foundation artifact or workflow plan

## Small Example

```rust
let artifact = forge_query_domain("worth.spatial")
    .for_intent(&declaration)
    .inspects_query_preview(
        "topology.preview_conflict",
        forge_query::facade::runtime::BridgePreviewSessionIdentity::new("preview-session:42"),
    )
    .because("the edge split must stay read-only while conflict topology is inspected")
    .materialize()?;
```

## Real Example

```rust
let plan = forge_query_domain("worth.spatial")
    .for_intent(&declaration)
    .plans_preview_mutation(
        "topology.preview_mutation",
        forge_query::facade::runtime::BridgePreviewSessionIdentity::new("preview-session:77"),
    )
    .because("a manifold-preserving edge split can be planned for later promotion")
    .materialize()?;

let declaration = plan.workflow_declaration();
let digest = declaration.report().declaration_digest();
```

What is authoritative:
- Query's workflow declaration and preview artifact families

What is derived:
- the domain-authored preview posture and reason

## How It Relates To Other Features

- use [Runtime-Preflight Workflow Contributions](./runtime-preflight-workflow-contributions.md)
  when the domain is binding to real runtime preflight rather than a declaration
  preview session
- use [Workflow Lanes: Common, Checked, Proof, And Raw](./workflow-lanes-common-checked-proof-raw.md)
  when you need to choose the right lane for tooling or certification

## Inspection And Debugging

- use the checked lane when you want denied metadata without throwing
- mutation planning returns a Query-owned wrapper so the common lane does not
  leak raw workflow declarations too early

## Anti-Patterns

- treating preview mutation planning as if it already executed writeback
- using read-only preview inspection when you really need promotion-eligible
  mutation posture
- rebuilding preview binding identity outside Query

## Current Limits

- the common lane covers preview inspection and promotion-eligible planning
- it does not promise that every later lowering or writeback step belongs on
  the same ordinary surface

## Related Docs

- [Runtime-Preflight Workflow Contributions](./runtime-preflight-workflow-contributions.md)
- [Workflow Lanes: Common, Checked, Proof, And Raw](./workflow-lanes-common-checked-proof-raw.md)
- [Branches And Previews](../../foundations/branches-and-previews.md)
