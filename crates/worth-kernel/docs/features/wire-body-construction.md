<!-- worth-doc
crate: worth-kernel
kind: feature
id: wire-body-construction
query_integration_required: true
query_proof_required: false
touches_query: false
-->

# Wire-Body Construction

## What This Feature Is

Wire-body construction is the admitted kernel workflow for construction results
that stay in the wire-domain rather than closing into shell or body truth.

## Why You Use It

Use this when the intended result is an admitted wire-body outcome and you want
the same runtime, artifact, replay, and diagnostics posture as the rest of the
Milestone 4 construction surface.

## Stable Entry Points

- `worth_kernel::workload_composition`

## Common Path

Wire-body construction reuses the same runtime and proof stack. The semantic
difference is the intended topology class, not a new authority model.

Kernel orchestration lowers the request through Query, then the downstream
spatial and topology authorities certify wire-body truth on the same artifact
lane.

## Small Example

Use this when a closed solid is not the intended truth and an admitted wire
result is the real target.

## Advanced Path

A wire-body workflow should still preserve replay, rejection locality, and
diagnostic posture rather than becoming a "lighter" undocumented side path.

## Query Integration

Wire-body construction still uses the Query-owned runtime lane. Query remains
the ordinary execution substrate while the kernel owns workflow orchestration.

## How It Relates To Other Features

- [Primitive Construction](./primitive-construction.md)
- [Construction Results And Diagnostics](./construction-results-and-diagnostics.md)

## Inspection And Debugging

Use the common artifact surfaces first. Only drop to lower layers when the
diagnostic question is specifically about wire topology or spatial semantics.

## Anti-Patterns

- treating wire-body construction as incomplete shell construction
- using local caller classification instead of the shipped artifact truth

## Current Limits

Only the admitted Milestone 4 wire-body workflow class is covered here.

## Related Docs

- [Primitive Construction](./primitive-construction.md)
