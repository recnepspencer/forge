# Resource Line Reference

If you want the shortest path to line reads, start with
[Line Inspection](../resources/line-inspection.md) before using this
full line reference.

## What This Feature Is

A resource line is the live handle you get back for one specific resource
member.

If the family is the reusable definition, the line is the thing you actually
work with in app code.

Use the line to:

- read the current value
- check whether it is loading, fresh, stale, rejected, or timed out
- inspect request info
- refresh or invalidate the resource
- inspect diagnostics and history

## Why You Use It

- keep value, loading state, request state, and debug state on one object
- refresh or revalidate without building your own cache layer
- inspect what actually happened on the same object you already use in the UI
- work with uploads, downloads, patching, and delivery from the same surface

## Stable Entry Points

You always get a line from a family:

```ts
const line = family.line(params);
```

Stable line methods:

- `line.value()`
- `line.signal()`
- `line.summarySignal()`
- `line.descriptor()`
- `line.request()`
- `line.summary()`
- `line.status()`
- `line.freshness()`
- `line.refresh()`
- `line.revalidate()`
- `line.awaitSettlement(options?)`
- `line.execute(options?)`
- `line.invalidate()`
- `line.view(...)`
- `line.processing()`
- `line.upload()`
- `line.download()`
- `line.diagnostics()`
- `line.diagnosticsSummary()`
- `line.history()`
- `line.free()`

Collection and paged lines can also expose:

- `line.patch(...)`
- `line.deliver(...)`
- `line.reconciliation()`

Families also expose final-form convenience lanes:

- `family.optionalLine(selection)`
- `family.execute(params, options?)`

## Core Mental Model

Do not think of the line as "just the loaded data".

The line is the full local state for that resource member:

- current visible value
- lifecycle state
- request state
- upload and processing state
- download state
- diagnostics and history

That is why it is usually better to pass a line around than to peel off only
`line.value()`.

## How It Executes

Once a line exists, the runtime keeps these pieces aligned:

1. request posture
2. visible value
3. status and freshness
4. diagnostics
5. history

Operations such as `refresh()`, `revalidate()`, `invalidate()`, `patch(...)`,
`deliver(...)`, and `restoreExact()` all update that same line state.

When a line is pending and the caller needs the next settled truth, use:

- `await line.awaitSettlement()`
- or `await line.execute().settled()`

That lane waits on the line's own runtime lifecycle instead of polling
`line.status()` in app code.

The history surface also exposes `replayExact()`, but on the shipped wasm
Signals runtime that action currently reports typed unavailability rather than
executing exact replay.

## Small Example

```ts
import { createSignals } from "forge-signal-wasm";

const signals = await createSignals();

const productDetail = signals.api({
  baseUrl: "/api",
}).url("/products/:productId").detail({
  load: ({ productId }) => ({
    id: productId,
    title: `Product ${productId}`,
  }),
});

const line = productDetail.line({ productId: "p1" });

console.log(line.value());
console.log(line.status());
console.log(line.freshness());
```

This is the smallest useful line example because it shows the three things you
usually need first:

- value
- status
- freshness

If you want the first product-shaped read instead of stitching a few calls
together, start with:

- `line.summary()`
- `line.summarySignal()` when React or another subscription layer needs the
  grouped line truth as a live signal handle

That grouped read keeps the common lane on one object:

- current status and freshness
- request posture
- upload, processing, and download state
- diagnostics summary plus explainability availability

## Real Example

```ts
import {
  createSignals,
  resourceAuth,
  resourceRequestContext,
} from "forge-signal-wasm";

const signals = await createSignals();

const workspaceApi = signals.api({
  auth: resourceAuth.workspace(),
}).scope({
  requestContext: ({ workspaceId }) =>
    resourceRequestContext({
      headers: { "x-workspace-id": workspaceId },
      correlationId: `account:${workspaceId}`,
    }),
});

const accountDetail = workspaceApi.url(
  "/workspaces/:workspaceId/accounts/:accountId",
).detail({
  load: ({ accountId }) => ({
    id: accountId,
    label: `Account ${accountId}`,
    balance: 42,
  }),
});

const line = accountDetail.line({
  workspaceId: "acme",
  accountId: "acct-7",
});

const balanceView = line.view((account) => account?.balance ?? 0);

console.log(line.descriptor());
console.log(line.request());
console.log(line.diagnosticsSummary());
console.log(line.history().availability);
console.log(balanceView());
```

Use this pattern when:

- the UI needs both data and loading/debug state
- request posture matters
- you want a lightweight derived view with `line.view(...)`

## How It Relates To Other Features

- Family docs explain how the line was declared.
- Request/policy docs explain where auth, headers, retry, and continuation come
  from.
- Inspection/history docs explain the debugging side, exact restore behavior,
  and current exact-replay availability limits.

## Inspection And Debugging

When a line is not behaving the way you expect, start with:

- `line.summary()`
- `line.status()`
- `line.freshness()`
- `line.request()`
- `line.diagnosticsSummary()`
- `line.history().availability`

If that is not enough, move to:

- `line.diagnostics()`
- `line.history().lifecycle`
- `line.history().basis`
- `line.history().verificationPackage()`

## Anti-Patterns

- storing only `line.value()` when you still care about loading or debugging
- treating `refresh()` and `revalidate()` like they mean the same thing
- assuming `view(...)` creates a second resource line
- building new UI helper objects first when `line.summary()` already gives the
  common grouped read

## Current Limits

- exact replay and exact restore depend on what the runtime supports
- exact restore is the currently supported same-runtime exact action on the
  shipped wasm Signals runtime
- exact replay is present as a typed history action surface, but currently
  reports unavailability because the shipped runtime does not yet expose
  signal-exact replay execution
- patching and delivery only exist on patch-capable collection and paged lines

## Related Docs

- [Resource Overview](../resources/overview.md)
- [Resource Family Authoring Reference](./resource-family-authoring.md)
- [Resource Request And Policy Reference](./resource-request-and-policy.md)
- [Inspection And History Contract](../resource-contracts/inspection-and-history.md)
