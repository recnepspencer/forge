# Resource Inspection And History Reference

If your question is "how do I check restore availability, replay availability,
or verification packages?", start with
[History And Restore](./history-and-restore.md) before using
this lower-level reference page.

## What This Feature Is

This is the part of the resource surface you use when you need to understand
what a line is doing and why.

It covers:

- detailed diagnostics
- condensed diagnostics summaries
- lifecycle history
- basis history
- replay and restore availability
- exact replay and exact restore action surfaces
- verification packages for deeper comparison

## Why You Use It

- explain why a line is visible, stale, pending, rejected, or timed out
- inspect request, upload, processing, download, patch, and delivery state from
  one place
- understand whether replay or restore is actually supported on the current
  runtime
- restore a line through an explicit typed action when the runtime supports it
- inspect exact-replay availability through the same typed history surface
- compare resource state through one stable verification artifact

## Stable Entry Points

All of the inspection surfaces live on a line:

```ts
const line = family.line(params);
```

Stable entry points:

- `line.diagnostics()`
- `line.diagnosticsSummary()`
- `line.history()`

Stable history sub-surfaces:

- `line.history().availability`
- `line.history().lifecycle`
- `line.history().basis`
- `line.history().branch`
- `line.history().replay`
- `line.history().lineage`
- `line.history().replayExact()`
- `line.history().restoreExact()`
- `line.history().verificationPackage()`

## Core Mental Model

There are three inspection layers:

1. current detailed state
2. current quick summary
3. historical and exact replay/restore state

Use them like this:

- `diagnostics()` when you want the full current picture
- `diagnosticsSummary()` when you want the short version
- `history()` when you want to explain how the line got here or whether exact
  replay or exact restore are available on the current runtime

## How It Executes

As the line changes, the runtime keeps these surfaces aligned:

1. value, status, freshness, request, transfer, patch, and delivery state
2. detailed diagnostics counters and latest-operation markers
3. condensed summaries of the current line
4. lifecycle history entries
5. basis history entries
6. replay and restore availability

If the runtime cannot provide exact replay or exact restore, the line reports
typed unavailability instead of pretending those actions always exist.

Current shipped posture:

- `restoreExact()` is a real supported same-runtime branch-restore path when
  the runtime exposes the required branch snapshot APIs
- `replayExact()` is a typed action surface, but the shipped wasm Signals
  runtime does not currently expose the signal-exact replay capability needed
  to make it available

## Small Example

```ts
import { createSignals } from "forge-signal-wasm";

const signals = await createSignals();

const productDetail = signals.api({
  baseUrl: "/api",
}).url("/products/:productId").detail({
  load: ({ productId }) => ({ id: productId }),
});

const line = productDetail.line({ productId: "p1" });

console.log(line.diagnosticsSummary());
console.log(line.history().availability);
```

This is the quickest useful inspection pattern:

- summary for the current state
- availability for replay/restore capability

## Real Example

```ts
import {
  createSignals,
  resourcePatch,
} from "forge-signal-wasm";

const signals = await createSignals();

const tasks = signals.api({}).url("/workspaces/:workspaceId/tasks")
  .items((item: { id: string; title: string }) => item.id)
  .aspect(
    "title",
    (item) => item.title,
    (item, title: string) => ({ ...item, title }),
  )
  .list({
    load: ({ workspaceId }) => [{
      id: `${workspaceId}:1`,
      title: "First",
    }],
  }),
});

const line = tasks.line({ workspaceId: "demo" });

line.patch(
  resourcePatch.itemAspect({
    itemId: "demo:1",
    aspect: "title",
    value: "Updated",
  }),
);

console.log(line.diagnostics());
console.log(line.diagnosticsSummary());
console.log(line.history().basis);
console.log(line.history().lifecycle.at(-1));
console.log(line.history().verificationPackage());
```

Use this pattern when you need to answer:

- what changed most recently
- whether a patch or delivery affected the line
- how the basis moved over time
- whether two runs can be compared through a stable verification package

## How It Relates To Other Features

- Pair this with the line reference when you need to interpret the inspected
  values operationally.
- Pair it with reconciliation and delivery when patch or pushed-update evidence
  matters.
- Pair it with later router or graph work when a resource line needs to explain
  route-local transitions or published feature behavior.

## Inspection And Debugging

Use `diagnostics()` when you need:

- full request, basis, upload, processing, and download state
- detailed counts
- latest patch, delivery, timeout, or invalidation evidence

Use `diagnosticsSummary()` when you need:

- grouped current state
- grouped counts
- a fast UI-friendly explanation

Start with `line.summary()` when you want the smallest app-facing read before
dropping into diagnostics and history detail.

Use `history()` when you need:

- lifecycle events over time
- basis advancement history
- replay and lineage artifacts
- exact replay availability and exact restore actions
- proof-grade verification packages

## Anti-Patterns

- using `diagnosticsSummary()` as if it were a full historical record
- assuming exact replay or exact restore are always available
- reading only `status()` when the real question is about request, basis, patch,
  or delivery state
- treating verification packages like normal app payloads

## Current Limits

- exact replay and exact restore depend on runtime support
- exact replay currently reports typed unavailability on the shipped wasm
  Signals runtime because signal-exact replay execution is not yet exposed at
  the history boundary
- retained-history and same-runtime-exact support are different capability
  levels and should not be treated as interchangeable

## Related Docs

- [Resource Line Reference](../api-reference/resource-line.md)
- [Delivery And Compatibility Contract](./delivery-and-compatibility.md)
- [Diagnostics And History](../app-surface/diagnostics-and-history.md)
