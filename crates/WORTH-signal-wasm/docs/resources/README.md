# Resources Docs Home

This is the navigation root for the rewritten resource docs.

The goal of this tree is simple: start from what you want to do, not from the
internal subsystem name you happen to remember.

## Start Here

- [Overview](./index.md)
  The shortest path into the shipped resource surface.
- [Glossary](./glossary.md)
  Common resource terms in plain language.
- [Recipes](./recipes.md)
  Copyable task-first examples.
- [Milestone Crosswalk](./milestone-crosswalk.md)
  Where the older flat docs map into the new tree.

## First Sections

- [Start Here](./start-here/README.md)
  Picking the normal lane, choosing a shape, and getting your first line.
- [Fetching Data](./fetching/README.md)
  Single records, collections, paged lists, auth/context, and request policy.
- [Updating Data](./updating/README.md)
  Standard writes, local patches, effect profiles, and what a write does next.
- [Handling Server Responses](./responses/README.md)
  Mutation responses, partial canonical truth, fallback reconciliation, and
  response-owned read updates.
- [Working With Lists](./lists/README.md)
  List identity, visible selection, narrow item updates, and derived list
  views.
- [Caching And Refresh](./caching/README.md)
  Line reuse, stale/fresh state, invalidation, refresh, and derived truth.
- [Partial Updates And Derived Views](./partial-updates/README.md)
  Narrow updates, automatic derived views, explicit aspect declarations, and
  partial-update continuity.
- [Uploads And Transfers](./transfers/README.md)
  Upload preparation, deferred processing, transfer result reads, and posture
  compatibility.
- [Downloads And Binary Data](./downloads/README.md)
  Builder-owned downloads, binary descriptors, multipart handoff, and download
  availability.
- [Inspecting And Debugging Resources](./debugging/README.md)
  Grouped line reads, retained history, delivery/compatibility inspection, and
  exact recovery paths.
- [Effects And Recovery](./effects/README.md)
  Branch-native effects, envelopes, merge/rebase, and rollback posture.
- [Using Resources In Forms](./forms/README.md)
  Resource-line form sources, settlement, drift, merge, mutation-response
  readback, and replay/restore.
- [Using Resources In Routes](./router/README.md)
  Route-resource declarations, prefetch/warmup, and projected/admitted route
  capability reads.
- [Advanced Resource Modeling](./advanced/README.md)
  Raw family declarations, canonical identity, detail fields/regions/json
  paths, aspects, summaries, and raw lines.
- [Verification And Proof](./verification/README.md)
  Verification packages, topology proof, mutation-response support, and
  delivery/compatibility digests.

## Still In The Older Flat Slice

These pages are still real and supported as deeper companion docs:

- [Collections And Delivery](./collections-and-delivery.md)
- [Mutation Response Reconciliation](./mutation-response-reconciliation.md)
- [Transfers](./transfers.md)
- [Downloads](./downloads.md)
- [Line Inspection](./line-inspection.md)
- [Branch-Native Resource Effects](./branch-native-effects.md)
- [Effect Merge And Rebase](./merge-and-rebase.md)
- [JSON Path Effects](./json-effects.md)
- [External Delivery And Compatibility](./external-delivery-and-compatibility.md)
- [Raw Escape Hatch](./raw-escape-hatch.md)

## Reading Order

1. [Overview](./index.md)
2. [Start Here](./start-here/README.md)
3. [Fetching Data](./fetching/README.md) or [Updating Data](./updating/README.md)
4. [Handling Server Responses](./responses/README.md),
   [Working With Lists](./lists/README.md), or
   [Caching And Refresh](./caching/README.md) when that is the actual job
5. [Partial Updates And Derived Views](./partial-updates/README.md),
   [Uploads And Transfers](./transfers/README.md), or
   [Downloads And Binary Data](./downloads/README.md) when that is the actual job
6. [Inspecting And Debugging Resources](./debugging/README.md),
   [Effects And Recovery](./effects/README.md), or
   [Using Resources In Forms](./forms/README.md) when the resource already
   exists and the question is now operational
7. [Using Resources In Routes](./router/README.md),
   [Advanced Resource Modeling](./advanced/README.md), or
   [Verification And Proof](./verification/README.md) when the job has moved
   into route ownership, raw modeling, or proof
8. the one focused page that matches your task
9. one of the older flat docs when you need the deeper milestone-era treatment
   of the same feature area
