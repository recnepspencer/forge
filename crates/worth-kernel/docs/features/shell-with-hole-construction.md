<!-- worth-doc
crate: worth-kernel
kind: feature
id: shell-with-hole-construction
query_integration_required: true
query_proof_required: false
touches_query: false
-->

# Shell-With-Hole Construction

## What This Feature Is

Shell-with-hole construction is the admitted kernel workflow for primitive
construction cases that require shell-level topology and hole-aware spatial
truth rather than a simple closed primitive surface.

## Why You Use It

Use this when the constructed result needs shell semantics with interior voids
while still staying on the same Query-native runtime and artifact path as the
rest of Milestone 4.

## Stable Entry Points

- `worth_kernel::workload_composition`

## Common Path

This is not a side runtime. It is a stricter workflow shape over the same
kernel, Query, spatial, and topology boundaries.

The flow is the same as primitive construction, but the downstream spatial and
topology authorities must account for shell-with-hole semantics explicitly.

## Small Example

Reach for this when the presence of a hole is part of the intended shell truth,
not an incidental post-process.

## Advanced Path

Use the same artifact family and replay surfaces as primitive construction so
that shell-with-hole cases do not drift into a parallel proof story.

## Query Integration

Query owns the admitted runtime lane for this construction family. The kernel
doc must keep that front door explicit so later work does not regress to local
runtime folklore.

## How It Relates To Other Features

- [Primitive Construction](./primitive-construction.md)
- [Kernel To Spatial](../boundaries/kernel-to-spatial.md)

## Inspection And Debugging

Inspect the same kernel artifact family first, then drop to the spatial and
topology boundaries when hole semantics or shell certification are the issue.

## Anti-Patterns

- handling hole semantics as caller-owned topology surgery
- documenting this as a separate runtime path

## Current Limits

This page documents the admitted shell-with-hole Milestone 4 surface only.

## Related Docs

- [Primitive Construction](./primitive-construction.md)
