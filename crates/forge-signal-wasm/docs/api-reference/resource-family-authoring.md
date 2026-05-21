# Resource Family Authoring Reference

If you are deciding whether to use the raw lane at all, start with
[Raw Escape Hatch](../resources/raw-escape-hatch.md). This page is the
lower-level family reference once you already know you need it.

## What This Feature Is

Resource family authoring is how you define a resource before you load any
specific item from it.

This is where you say:

- what params the resource takes
- how those params become a stable identity
- how it loads
- whether it is a detail, collection, or paged resource

For most app code, you should do that through the API route lane first:

- `signals.api(...).url(...).detail(...)`
- `signals.api(...).url(...).list(...)`
- `signals.api(...).url(...).paged(...)`

This page also covers the raw family declaration surface, because that still
matters as the explicit escape hatch.

## Why You Use It

- keep API shape and request rules in one place
- make sure the same params always point at the same logical resource member
- define collection- and paging-specific behavior once instead of per call site
- stop request setup from turning into scattered helper code

## Stable Entry Points

Recommended default lane:

- `signals.api(...)`
- `api.scope(...)`
- `api.url(...)`

Raw family entry points:

- `signals.resource.detail(...)`
- `signals.resource.collection(...)`
- `signals.resource.paged(...)`

Core helpers:

- `resourceParams<TParams>()`
- `resourceParamIdentity(params, canonicalKey)`

Common declaration helpers:

- `resourcePolicyProfiles.*()`
- `resourceAuth.*()`
- `resourceRequestContext(...)`
- `resourceContinuation.*()`
- `resourceProcessingJob.*()`
- `resourceUploadTransport.*()`

## Core Mental Model

A family is the recipe.
A line is one live instance of that recipe.

The main authoring choice is:

- use the route lane when the endpoint is ordinary app-facing API work
- use the raw family lane when you intentionally need full manual control over
  params, canonical identity, or compatibility-oriented declaration shape

Use:

- `detail` for one item
- `collection` for a list where items have stable identity
- `paged` for a list where new pages accumulate over time

The most important thing the family owns is canonical identity.

If two param objects mean "the same resource", they should normalize to the
same canonical key. That is what keeps rematerialization, refresh, history, and
delivery honest later.

## How It Executes

When you call `signals.resource.detail(...)` or its collection/paged variants,
the runtime stores a declaration.

When you later call `family.line(params)`, the runtime:

1. checks the family shape
2. runs `normalizeParams(...)`
3. builds the request posture
4. materializes the line
5. calls `load(...)`

The family shape rules are strict on purpose:

- `detail` must not declare `itemIdentity`
- `collection` must declare `itemIdentity`
- `paged` must declare `itemIdentity` and `accumulatePage`

## Small Example

```ts
import { createSignals } from "forge-signal-wasm";

const signals = await createSignals();

const profileDetail = signals.api({
  baseUrl: "/api",
}).url("/profiles/:profileId").detail({
  load: ({ profileId }) => ({
    id: profileId,
    label: `Profile ${profileId}`,
  }),
});
```

This is the smallest honest example because it shows the minimum contract:

- declared route params
- route-derived canonical identity
- `load(...)`

## Real Example

```ts
import {
  createSignals,
  resourceAuth,
  resourceContinuation,
  resourceParamIdentity,
  resourceParams,
  resourcePolicyProfiles,
  resourceProcessingJob,
  resourceRequestContext,
  resourceUploadTransport,
} from "forge-signal-wasm";

const signals = await createSignals();

const invoicePages = signals.resource.paged({
  params: resourceParams<{
    workspaceId: string;
    customerId: string;
    page: number;
  }>(),
  policy: resourcePolicyProfiles.retryOnce(),
  auth: ({ workspaceId }) =>
    workspaceId === "demo"
      ? resourceAuth.workspace()
      : resourceAuth.authenticated(),
  requestContext: ({ workspaceId, page }) =>
    resourceRequestContext({
      headers: {
        "x-workspace-id": workspaceId,
        "x-page": String(page),
      },
      correlationId: `invoice-pages:${workspaceId}:${page}`,
    }),
  continuation: resourceContinuation.callback({
    callbackId: "invoice-pages-loaded",
    returnTo: "/invoices",
  }),
  processingJob: resourceProcessingJob.poll(),
  uploadTransport: resourceUploadTransport.none(),
  normalizeParams: ({ workspaceId, customerId, page }) =>
    resourceParamIdentity(
      { workspaceId, customerId, page },
      `${workspaceId}:${customerId}:${page}`,
    ),
  itemIdentity: (item: { id: string }) => item.id,
  accumulatePage: (existing, next) => ({
    items: [...existing.items, ...next.items],
    cursor: next.cursor,
  }),
  load: ({ customerId, page }, request) => ({
    items: [
      {
        id: `${customerId}:${page}:1`,
        title: `${request.auth.kind}:${customerId}:${page}`,
      },
    ],
    cursor: null,
  }),
});
```

What is authoritative:

- `normalizeParams(...)` decides stable identity
- `itemIdentity(...)` decides item identity inside list-shaped values
- `accumulatePage(...)` decides how later pages are merged

What is derived later:

- line status and freshness
- diagnostics and history
- upload, processing, and download views

## How It Relates To Other Features

- Move to the line reference once you have called `family.line(...)`.
- Move to request/policy docs when you are deciding auth, headers, or retry
  behavior.
- Move to reconciliation docs when collection or paged values need narrow
  patching.
- Stay on this page when you intentionally need the raw `signals.resource.*(...)`
  escape hatch.

## Inspection And Debugging

The family itself is mostly a construction boundary, so the best early checks
are:

- is `normalizeParams(...)` stable and predictable
- did you pick the right family kind
- for lists, does `itemIdentity(...)` really identify one logical item

If those are wrong, everything later gets harder.

## Anti-Patterns

- using `detail` for list-shaped values because it is simpler to start with
- building canonical keys from display text or unstable formatting
- hiding request posture inside `load(...)` instead of declaring it on the
  family

## Current Limits

- this page covers family declaration, not line operations
- external definition envelopes are a separate compatibility lane

## Related Docs

- [Resource Overview](../resources/overview.md)
- [Route Authoring Reference](./route-authoring.md)
- [Resource Line Reference](./resource-line.md)
- [Resource Request And Policy Reference](./resource-request-and-policy.md)
- [Resource Recipes](../learn/recipes.md)
