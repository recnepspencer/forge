# Resource Line Reference

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
- `line.descriptor()`
- `line.request()`
- `line.status()`
- `line.freshness()`
- `line.refresh()`
- `line.revalidate()`
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
`deliver(...)`, `replayExact()`, and `restoreExact()` all update that same line
state.

## Small Example

```ts
import {
  createSignals,
  resourceParamIdentity,
  resourceParams,
} from "forge-signal-wasm";

const signals = createSignals();

const productDetail = signals.resource.detail({
  params: resourceParams<{ productId: string }>(),
  normalizeParams: ({ productId }) =>
    resourceParamIdentity({ productId }, productId),
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

## Real Example

```ts
import {
  createSignals,
  resourceAuth,
  resourceParamIdentity,
  resourceParams,
  resourceRequestContext,
} from "forge-signal-wasm";

const signals = createSignals();

const accountDetail = signals.resource.detail({
  params: resourceParams<{ workspaceId: string; accountId: string }>(),
  auth: resourceAuth.workspace(),
  requestContext: ({ workspaceId }) =>
    resourceRequestContext({
      headers: { "x-workspace-id": workspaceId },
      correlationId: `account:${workspaceId}`,
    }),
  normalizeParams: ({ workspaceId, accountId }) =>
    resourceParamIdentity(
      { workspaceId, accountId },
      `${workspaceId}:${accountId}`,
    ),
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
- Inspection/history docs explain the debugging and replay/restore side.

## Inspection And Debugging

When a line is not behaving the way you expect, start with:

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

## Current Limits

- exact replay and exact restore depend on what the runtime supports
- patching and delivery only exist on patch-capable collection and paged lines

## Related Docs

- [api_resources_overview.md](./api_resources_overview.md)
- [resource_family_authoring_reference.md](./resource_family_authoring_reference.md)
- [resource_request_and_policy_reference.md](./resource_request_and_policy_reference.md)
- [resource_inspection_and_history_reference.md](./resource_inspection_and_history_reference.md)
