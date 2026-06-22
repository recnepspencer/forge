# Execution Context And Artifact Policy

## What This Feature Is

This doc explains how `worth-kernel` chooses the execution context for a
workflow and why the output is a canonical artifact family instead of a loose
bag of receipts.

## Why You Use It

Use this when you need to understand why a kernel workflow returns a particular
artifact, what layer owns each piece of truth, and which proof surfaces are
safe to inspect after execution.

## Stable Entry Points

- `worth_kernel::workload_composition`
- `worth_kernel::query_adoption`

## Core Mental Model

Execution context answers: which shipped workflow path is being used?

Artifact policy answers: which inspectable result family is retained and why?

The kernel keeps those decisions explicit so callers do not have to rediscover
them from individual lower-layer receipts.

## How It Executes

1. The kernel prepares a workflow or workload declaration.
2. Query owns runtime entry, support posture, and lower-level proof posture.
3. `worth-spatial` and `worth-topo` attach their owned truth or certification.
4. The kernel exposes the resulting artifact family as the common inspection
   surface.

## Small Example

If two workflows share the same lower topology authority but differ in replay
or branch-local posture, the execution context changes while topology authority
does not.

## Real Example

For Milestone 4 construction flows, the returned artifact family may need to
preserve:

- Query runtime authoring posture
- replay or branch-local basis posture
- spatial birth truth
- topology certification
- rejection locality

The kernel artifact policy keeps that proof chain together.

## How It Relates To Other Features

- [Primitive Construction](../features/primitive-construction.md)
- [Construction Results And Diagnostics](../features/construction-results-and-diagnostics.md)
- [Worth To Query](../boundaries/worth-to-query.md)

## Inspection And Debugging

Inspect the artifact family first. Drop to lower-layer reports only when you
need to know which owned boundary changed the outcome.

## Anti-Patterns

- treating one receipt as the whole workflow truth
- rebuilding artifact meaning from payload archaeology
- splitting Query proof, spatial truth, and topology truth into local caller
  folklore

## Current Limits

The artifact policy described here is the Milestone 4 shipped shape. Newer
workload families may extend it, but they should not replace the existing
authority boundaries.

## Related Docs

- [Construction Results And Diagnostics](../features/construction-results-and-diagnostics.md)
- [Worth To Query](../boundaries/worth-to-query.md)
