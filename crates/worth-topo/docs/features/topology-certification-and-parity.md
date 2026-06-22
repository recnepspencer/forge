<!-- worth-doc
crate: worth-topo
kind: feature
id: topology-certification-and-parity
query_integration_required: false
query_proof_required: false
touches_query: false
-->

# Topology Certification And Parity

## What This Feature Is

This feature owns the topology-side proof surfaces that certify legality,
parity, breadth, and hostile closeout over topology truth.

## Why You Use It

Use this when you need the proof story over topology authority rather than only
the authority objects or the Query runtime posture.

## Stable Entry Points

- `worth_topo::certification`
- `worth_topo::facade::NamingAttachmentReport`
- `worth_topo::facade::RejectedMutationScopeReport`

## Common Path

1. Start from topology authority truth.
2. Run the relevant validation or certification lane.
3. Inspect the resulting proof rows, parity surfaces, or rejection-locality
   artifacts.

## Advanced Path

Use the advanced path when you need to inspect:

- hostile closeout and parity lanes
- naming attachment and continuity proof
- changed-scope, fallout, or rejection locality rows
- derived parity versus authoritative topology truth

## Inspection And Debugging

Read this when the topology graph is present but you need to know whether the
topology-side proof layer certified it honestly.

## Anti-Patterns

- treating certification as a second source of truth
- teaching parity only through one hostile suite transcript
- mixing Query runtime support posture with topology certification meaning

## Current Limits

This doc backfills the enduring topology proof substrate that later Milestone 4
surfaces depend on. It is not a substitute for the newer runtime-boundary docs.

## Related Docs

- [Topology Graph Authority](./topology-graph-authority.md)
- [Topo Query Runtime Boundary](../boundaries/topo-query-runtime-boundary.md)
- [Worth To Query](../../../worth-kernel/docs/boundaries/worth-to-query.md)
