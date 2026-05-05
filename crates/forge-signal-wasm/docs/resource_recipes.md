# Resource Recipes

## What This Feature Is

This page is the practical companion to the resource reference docs.

If the reference pages tell you what each surface means, this page shows how
they fit together in app-shaped code.

## Why You Use It

- start from working patterns instead of only reading type signatures
- see which helper belongs where in a real resource declaration
- copy a smaller pattern into controllers, graphs, and later route-local code
- understand the normal shape before you start combining advanced features

## Stable Entry Points

The recipes in this guide use the already-documented stable surfaces:

- `signals.resource.detail(...)`
- `signals.resource.collection(...)`
- `signals.resource.paged(...)`
- `line.*`
- request and policy helpers
- reconciliation helpers
- transfer helpers
- binary/download helpers
- compatibility and delivery helpers

## Core Mental Model

Almost every resource feature follows the same shape:

1. declare the family
2. materialize a line
3. read or operate on the line
4. inspect the line if something goes wrong

That is the main habit to build. Resources are family-first and line-centered.

## How It Executes

Each recipe here uses the same underlying flow:

- the declaration shapes request and lifecycle behavior
- the line materializes one canonical resource member
- operations update that line through runtime-owned rules
- inspection stays on the line

The examples differ in which helper surfaces they add on top.

## Small Example

```ts
import {
  createSignals,
  resourceParamIdentity,
  resourceParams,
} from "forge-signal-wasm";

const signals = createSignals();

const userDetail = signals.resource.detail({
  params: resourceParams<{ userId: string }>(),
  normalizeParams: ({ userId }) =>
    resourceParamIdentity({ userId }, userId),
  load: ({ userId }) => ({ id: userId, name: `User ${userId}` }),
});

const line = userDetail.line({ userId: "u1" });
console.log(line.value());
```

This is the base pattern:

- declare a family
- materialize a line
- read from the line

## Real Example

```ts
import {
  createSignals,
  resourceAuth,
  resourceBinaryDescriptor,
  resourceBinaryValue,
  resourceCollectionShape,
  resourceDownload,
  resourceItemAspects,
  resourceParamIdentity,
  resourceParams,
  resourcePatch,
  resourcePolicyProfiles,
  resourceProcessingJob,
  resourceRequestContext,
  resourceUploadTransport,
  resourceValueSummaries,
} from "forge-signal-wasm";

const signals = createSignals();

const reports = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  policy: resourcePolicyProfiles.retryOnce(),
  auth: resourceAuth.workspace(),
  requestContext: ({ workspaceId }) =>
    resourceRequestContext({
      headers: { "x-workspace-id": workspaceId },
      correlationId: `reports:${workspaceId}`,
    }),
  processingJob: resourceProcessingJob.poll(),
  uploadTransport: resourceUploadTransport.signed({
    method: "POST",
    finalizeRequired: true,
  }),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (item: { id: string; title: string }) => item.id,
  reconcile: resourceCollectionShape({
    items: (
      value: {
        items: Array<{ id: string; title: string }>;
        total: number;
      },
    ) => value.items,
    replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
    aspects: resourceItemAspects({
      title: {
        read: (item: { id: string; title: string }) => item.title,
        write: (item, title: string) => ({ ...item, title }),
      },
    }),
    summaries: resourceValueSummaries({
      total: {
        read: (value) => value.total,
        write: (value, total: number) => ({ ...value, total }),
      },
    }),
  }),
  load: ({ workspaceId }, request) =>
    resourceBinaryValue({
      value: {
        items: [
          {
            id: `${workspaceId}:1`,
            title: `${request.auth.kind}:${workspaceId}`,
          },
        ],
        total: 1,
      },
      descriptors: [
        resourceBinaryDescriptor.export({
          id: "reports-export",
          fileName: `${workspaceId}.zip`,
          mediaType: "application/zip",
          download: resourceDownload.ready({
            url: `https://downloads.example/${workspaceId}.zip`,
            method: "GET",
          }),
        }),
      ],
    }),
});

const line = reports.line({ workspaceId: "demo" });

line.patch(
  resourcePatch.itemAspect({
    itemId: "demo:1",
    aspect: "title",
    value: "Updated Title",
  }),
);

console.log(line.value());
console.log(line.download());
console.log(line.diagnosticsSummary());
```

This example is intentionally busy. It shows how the family can own:

- request posture
- reconciliation rules
- upload/processing posture
- download descriptors

while the line stays the single place you read and inspect.

## How It Relates To Other Features

- Use these recipes to get started, then move to the focused reference pages
  when you need exact API details.
- Pair them with controllers and graphs when the resource belongs inside a
  larger feature module.
- Pair them with diagnostics/history when debugging matters more than authoring.

## Inspection And Debugging

The fastest recipe-level inspection loop is:

- `line.value()`
- `line.status()`
- `line.freshness()`
- `line.request()`
- `line.diagnosticsSummary()`
- `line.history().availability`

## Recipe Set

### Detail resource with auth and request context

Use this when one entity needs explicit auth and request headers.

```ts
const invoiceDetail = signals.resource.detail({
  params: resourceParams<{ workspaceId: string; invoiceId: string }>(),
  auth: resourceAuth.workspace(),
  requestContext: ({ workspaceId }) =>
    resourceRequestContext({
      headers: { "x-workspace-id": workspaceId },
    }),
  normalizeParams: ({ workspaceId, invoiceId }) =>
    resourceParamIdentity(
      { workspaceId, invoiceId },
      `${workspaceId}:${invoiceId}`,
    ),
  load: ({ invoiceId }) => ({ id: invoiceId }),
});
```

### Collection resource with narrow patching

Use this when visible items can be updated by item or field.

```ts
const line = tasks.line({ workspaceId: "demo" });
line.patch(
  resourcePatch.item({
    itemId: "task-1",
    nextItem: { id: "task-1", title: "Updated" },
  }),
);
```

### Paged resource with page-window summaries

Use this when a paged line exposes a visible-window summary such as
`visibleCount`.

```ts
const line = feed.line({ workspaceId: "demo" });
line.patch(
  resourcePatch.summary({
    summary: "visibleCount",
    value: 2,
  }),
);
```

### Upload-first resource

Use this when the first visible state is upload preparation rather than final
business data.

```ts
const line = receiptUpload.line({ receiptId: "r1" });
console.log(line.upload());
```

### Binary/download resource

Use this when the line needs both structured data and downloadable files.

```ts
const line = manualDetail.line({ assetId: "asset-1" });
console.log(line.value());
console.log(line.download());
```

### External basis refresh recovery

Use this when an external pushed patch is stale and the line needs a basis
refresh before later packets can apply.

```ts
line.deliver(
  signals.resource.compatibility.delivery.basisRefresh({
    packetId: "pkt-refresh",
    basisId: "basis-1",
    nextBasisId: "basis-2",
  }),
);
```

### Graph publication around a resource line

Use this when a resource-backed feature should expose a stable graph output.

```ts
const workspaceGraph = signals.graph("workspaceGraph", {
  outputs: {
    report: reports.line({ workspaceId: "demo" }).signal(),
  },
});
```

## Anti-Patterns

- copying only the `load(...)` shape and skipping canonical params
- mixing transfers, reconciliation, and delivery into one giant declaration
  when you do not need them
- treating recipes as a substitute for the focused reference docs when the
  boundary question really matters

## Current Limits

- this guide is for stable resource patterns, not full mutation or router
  stories
- later router work should consume these resource patterns instead of redefining
  them

## Related Docs

- [api_resources_overview.md](./api_resources_overview.md)
- [resource_family_authoring_reference.md](./resource_family_authoring_reference.md)
- [resource_line_reference.md](./resource_line_reference.md)
- [resource_request_and_policy_reference.md](./resource_request_and_policy_reference.md)
- [resource_reconciliation_reference.md](./resource_reconciliation_reference.md)
- [resource_transfers_reference.md](./resource_transfers_reference.md)
- [resource_binary_and_download_reference.md](./resource_binary_and_download_reference.md)
- [resource_delivery_and_compatibility_reference.md](./resource_delivery_and_compatibility_reference.md)
- [resource_inspection_and_history_reference.md](./resource_inspection_and_history_reference.md)
