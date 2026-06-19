<!-- worth-doc
crate: worth-kernel
kind: feature
id: construction-results-and-diagnostics
query_integration_required: true
query_proof_required: false
touches_query: false
-->

# Construction Results And Diagnostics

## What This Feature Is

This feature is the kernel-owned result surface for admitted construction
workflows: one canonical place to inspect success, rejection locality, replay
posture, and the lower-layer proofs that were attached.

## Why You Use It

Use this when you need to understand what happened after a construction run
without spelunking each lower crate independently.

## Stable Entry Points

- kernel artifact family returned by admitted construction flows
- kernel Query adoption reports in `worth_kernel::query_adoption`

## Common Path

The result surface is where the workflow becomes inspectable. It is not a dump
of unrelated receipts. It is the canonical artifact family for the run.

The kernel collects the relevant proof-bearing outputs from:

- Query runtime posture
- spatial birth truth
- topology certification
- replay or branch-local parity surfaces when relevant

and presents them as one inspection surface.

## Small Example

Use this when you need to answer "why did this run reject?" or "which lower
boundary changed the outcome?"

## Advanced Path

Serious diagnostic questions often involve both semantics and runtime posture:

- did the workflow consume an admitted Query family?
- did spatial birth truth fail?
- did topology certification reject?
- did replay parity drift?

This feature is the common entrypoint for those answers.

## Query Integration

Diagnostics stay honest only when Query-owned runtime proof remains Query-owned.
Kernel result docs should name that boundary instead of implying the kernel can
reconstruct Query proof from local payload archaeology.

## How It Relates To Other Features

- [Primitive Construction](./primitive-construction.md)
- [Construction Replay](./construction-replay.md)
- [Worth To Query](../boundaries/worth-to-query.md)

## Inspection And Debugging

Start with the canonical artifact family. Then inspect:

- Query adoption and support posture
- spatial boundary docs
- topology runtime support or domain-read docs

## Anti-Patterns

- rebuilding diagnostics from row archaeology
- flattening rejection locality into generic error text
- proving Query posture with local folklore after Query 9.8

## Current Limits

This doc only covers the shipped Milestone 4 diagnostic surface.

## Related Docs

- [Worth To Query](../boundaries/worth-to-query.md)
- [Kernel To Spatial](../boundaries/kernel-to-spatial.md)
