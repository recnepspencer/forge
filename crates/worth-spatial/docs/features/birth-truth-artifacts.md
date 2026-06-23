<!-- worth-doc
crate: worth-spatial
kind: feature
id: birth-truth-artifacts
query_integration_required: true
query_proof_required: false
touches_query: false
-->

# Birth Truth Artifacts

## What This Feature Is

Birth truth artifacts are the retained spatial proof surfaces that let later
inspection, replay, and closeout work see what spatial meaning was actually
attached.

## Why You Use It

Use this when you need to inspect, replay, or certify the spatial side of a
constructed result without reconstructing the semantics from raw rows or local
helpers.

## Stable Entry Points

- `worth_spatial::facade::inspection`
- `worth_spatial::facade::projection`
- `worth_spatial::facade::projection_workload`

## Common Path

Spatial birth truth is an inspectable artifact family, not just an ephemeral
step in a larger workflow.

The artifact is produced on the Query-backed runtime path, but the semantic
truth it exposes belongs to `worth-spatial`.

## Small Example

Reach for these artifacts when a result "looked wrong" and you need the spatial
facts themselves, not just the final topology classification.

## Advanced Path

Birth truth artifacts are one of the places where replay and inspection stay
honest across crate boundaries: kernel owns orchestration, spatial owns birth
meaning, topo owns topology truth.

## Query Integration

These artifacts are produced on the Query-backed runtime lane, but the birth
truth they expose remains spatial-owned. That split should stay explicit in the
docs and in the proof story.

## How It Relates To Other Features

- [Birth Completeness And Impossibility](./birth-completeness-and-impossibility.md)
- [Spatial Query Proof Posture](../boundaries/spatial-query-proof-posture.md)

## Inspection And Debugging

Inspect birth truth artifacts before you fall back to source archaeology.

## Anti-Patterns

- rebuilding birth meaning from final topology state alone
- treating projection rows as if they were the same thing as spatial truth

## Current Limits

Only the admitted Milestone 4 birth-artifact surfaces are documented here.

## Related Docs

- [Spatial To Topo](../boundaries/spatial-to-topo.md)
