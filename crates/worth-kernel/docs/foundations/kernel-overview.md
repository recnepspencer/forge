# Kernel Overview

## What This Feature Is

`worth-kernel` is the workflow layer that turns admitted Worth construction
intent into one canonical artifact family instead of leaving callers to braid
Query receipts, spatial birth truth, and topology certification themselves.

## Why You Use It

Use the kernel when you want the shipped construction path, replay path, or
workload path for Worth rather than assembling lower layers directly.

## Stable Entry Points

- crate root: `worth_kernel`
- workload surface: `worth_kernel::workload_composition`
- Query proof / adoption surface: `worth_kernel::query_adoption`

## Core Mental Model

The kernel is not a second runtime. It is an orchestration layer over:

- Query-owned runtime lifecycle and proof posture
- `worth-spatial` birth and binding semantics
- `worth-topo` topology authority and certification

The kernel owns the workflow and the canonical artifact policy. It does not own
truth authority for topology or geometry.

## How It Executes

1. A workflow is prepared in kernel-owned vocabulary.
2. The workflow lowers onto Query-owned runtime surfaces.
3. Spatial and topology authorities attach their owned proof or truth.
4. The caller receives one inspectable artifact family instead of local
   fragments.

## Small Example

Use the kernel when you want a composed workload artifact:

- declare the workload through `worth_kernel::workload_composition`
- inspect Query-consumption posture through `worth_kernel::query_adoption`
- follow the kernel feature doc for the specific workflow class

## Real Example

For planar boolean or primitive-construction style workloads, the kernel owns:

- workload declaration vocabulary
- workload stage requirements
- operator harness and outcome shaping
- the final artifact policy that keeps Query, spatial, and topology facts in
  one place

## How It Relates To Other Features

- [Execution Context And Artifact Policy](./execution-context-and-artifact-policy.md)
- [Primitive Construction](../features/primitive-construction.md)
- [Worth To Query](../boundaries/worth-to-query.md)
- [Kernel To Spatial](../boundaries/kernel-to-spatial.md)

## Inspection And Debugging

If you are unsure whether a problem belongs to the kernel or a lower layer,
inspect:

- kernel Query adoption reports
- the feature-specific artifact or replay report
- the next boundary doc in the chain

## Anti-Patterns

- treating `worth-kernel` as a public geometry runtime
- teaching a local runtime story that bypasses Query
- letting feature docs imply the kernel owns topology or spatial authority

## Current Limits

The kernel only documents and certifies the admitted Milestone 4 feature
surfaces. Unsupported or deferred neighbors must fail closed rather than
inherit implied support from names alone.

## Related Docs

- [Execution Context And Artifact Policy](./execution-context-and-artifact-policy.md)
- [Worth To Query](../boundaries/worth-to-query.md)
- [Kernel To Spatial](../boundaries/kernel-to-spatial.md)
