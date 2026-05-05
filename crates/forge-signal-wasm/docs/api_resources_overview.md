# API Resources Overview

## What This Feature Is

API resources are the part of `forge-signal-wasm` you use for server-backed
state.

If you have something that is loaded from an API, needs refresh/retry/stale
behavior, and should still feel like normal signal state once it is in memory,
this is the surface you reach for.

## Why You Use It

- load server data without inventing your own fetch/cache/retry layer
- keep request setup, stale behavior, and inspection on one surface
- treat API-backed state like graph-native state once it is materialized
- get line-level status, diagnostics, history, replay, and restore without
  bolting on a second tool

## Stable Entry Points

Most app code starts here:

- `signals.resource.detail(...)`
- `signals.resource.collection(...)`
- `signals.resource.paged(...)`

Helpers you will use right away:

- `resourceParams(...)`
- `resourceParamIdentity(...)`
- `resourcePolicyProfiles.*()`
- `resourceAuth.*()`
- `resourceRequestContext(...)`

## Core Mental Model

Think in two steps:

1. declare a resource family
2. get a line from that family

The family is the reusable definition.
The line is the live resource member for one specific set of params.

You will also see the term `canonical identity` in some adjacent docs. In plain
English, that just means the stable key for "this exact resource member".

Example:

- family: "product detail"
- line: "product detail for `productId = p1`"

Once you have a line, that line owns:

- the current visible value
- current status
- stale/fresh state
- request info
- diagnostics and history

That means you do not need one object for "data", another for "loading", and a
third for "debugging". The line is the whole thing.

## How It Executes

This is the normal flow:

1. you declare a family with params, identity, and `load(...)`
2. you call `family.line(params)`
3. the runtime normalizes those params into one stable identity key
4. the line runs `load(...)`
5. the line exposes value, status, request, and diagnostics
6. later refreshes, retries, patches, deliveries, replay, or restore all
   happen on that same line

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

This is the best starting example because it shows the full shape:

- declare the family
- materialize the line
- read value and lifecycle state from the line

## Real Example

```ts
import {
  createSignals,
  resourceAuth,
  resourceParamIdentity,
  resourceParams,
  resourcePolicyProfiles,
  resourceRequestContext,
} from "forge-signal-wasm";

const signals = createSignals();

const invoiceDetail = signals.resource.detail({
  params: resourceParams<{ workspaceId: string; invoiceId: string }>(),
  policy: resourcePolicyProfiles.retryOnce(),
  auth: resourceAuth.workspace(),
  requestContext: ({ workspaceId }) =>
    resourceRequestContext({
      headers: { "x-workspace-id": workspaceId },
      correlationId: `invoice:${workspaceId}`,
    }),
  normalizeParams: ({ workspaceId, invoiceId }) =>
    resourceParamIdentity(
      { workspaceId, invoiceId },
      `${workspaceId}:${invoiceId}`,
    ),
  load: ({ invoiceId }, request) => ({
    id: invoiceId,
    authKind: request.auth.kind,
  }),
});

const line = invoiceDetail.line({
  workspaceId: "acme",
  invoiceId: "inv-7",
});

console.log(line.value());
console.log(line.request());
console.log(line.diagnosticsSummary());
```

In this example:

- the family declaration decides params, identity, and request posture
- the line owns the current state for that resource member

## How It Relates To Other Features

- Use controllers and graphs when a resource belongs inside a larger feature.
- Use ordinary `input(...)` and `computed(...)` for local state that does not
  come from an API.
- Later router work should consume resources, not replace them.

## Inspection And Debugging

When something feels off, start with:

- `line.status()`
- `line.freshness()`
- `line.request()`
- `line.diagnosticsSummary()`
- `line.history().availability`

Those usually answer:

- did this load succeed
- is it stale
- what request posture did it use
- can this line replay or restore on this runtime

## Anti-Patterns

- using resources for local UI state that is not really API-backed
- rebuilding request or stale logic outside the family declaration
- treating the line like it is only a value holder and ignoring its status and
  diagnostics surfaces

## Current Limits

- this surface is for resource state, not mutation workflow orchestration
- the planned router should consume this resource lane instead of replacing it

## Related Docs

- [resource_family_authoring_reference.md](./resource_family_authoring_reference.md)
- [resource_line_reference.md](./resource_line_reference.md)
- [resource_request_and_policy_reference.md](./resource_request_and_policy_reference.md)
- [resource_recipes.md](./resource_recipes.md)
