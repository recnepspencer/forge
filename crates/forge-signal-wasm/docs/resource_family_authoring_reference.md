# Resource Family Authoring Reference

## What This Feature Is

Resource family authoring is how you define a resource before you load any
specific item from it.

This is where you say:

- what params the resource takes
- how those params become a stable identity
- how it loads
- whether it is a detail, collection, or paged resource

## Why You Use It

- keep API shape and request rules in one place
- make sure the same params always point at the same logical resource member
- define collection- and paging-specific behavior once instead of per call site
- stop request setup from turning into scattered helper code

## Stable Entry Points

Main authoring entry points:

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
import {
  createSignals,
  resourceParamIdentity,
  resourceParams,
} from "forge-signal-wasm";

const signals = createSignals();

const profileDetail = signals.resource.detail({
  params: resourceParams<{ profileId: string }>(),
  normalizeParams: ({ profileId }) =>
    resourceParamIdentity({ profileId }, profileId),
  load: ({ profileId }) => ({
    id: profileId,
    label: `Profile ${profileId}`,
  }),
});
```

This is the smallest honest example because it shows the minimum contract:

- declared params
- canonical identity
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

const signals = createSignals();

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

- [api_resources_overview.md](./api_resources_overview.md)
- [resource_line_reference.md](./resource_line_reference.md)
- [resource_request_and_policy_reference.md](./resource_request_and_policy_reference.md)
- [resource_recipes.md](./resource_recipes.md)
