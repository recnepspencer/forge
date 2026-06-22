<!-- worth-doc
crate: worth-kernel
kind: boundary
id: kernel-to-spatial
query_integration_required: false
query_proof_required: false
touches_query: false
-->

# Kernel To Spatial

## Boundary

This boundary explains where kernel orchestration stops and spatial birth truth
or spatial impossibility begins.

## Allowed Upstream Inputs

- kernel-owned construction or workload declarations
- runtime-backed artifacts that have reached the spatial handoff point

## Required Downstream Outputs

- spatial birth truth
- spatial impossibility or incompleteness truth
- spatial binding semantics that remain inspectable at the returned artifact
  boundary

## Stable Entry Points

- upstream orchestration: `worth_kernel::workload_composition`
- downstream semantics: `worth_spatial::facade`

## Query Usage

If this boundary depends on Query posture, Query still owns the runtime lane.
The kernel does not pass a second runtime into spatial code.

## Forbidden Bypasses

- inventing kernel-local birth semantics
- flattening spatial impossibility into generic kernel rejection
- reconstructing spatial proof by reading topology-only artifacts

## Binding Artifacts Or Receipts

The caller should expect the kernel artifact family to retain the spatial truth
needed to explain:

- what birth semantics were applied
- why a birth path was impossible or incomplete
- how the spatial result relates to the topology result

## Inspection And Debugging

If the workflow failed after runtime admission but before topology completion,
this is the first boundary to inspect.

## Related Docs

- [Primitive Construction](../features/primitive-construction.md)
- [worth-spatial Construction-Time Birth Bindings](../../../worth-spatial/docs/features/construction-time-birth-bindings.md)
