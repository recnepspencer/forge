# External Delivery And Compatibility

Use this page when another system is pushing resource packets or external
resource definitions into your app.

## What This Covers

- `signals.resource.compatibility.*(...)`
- `signals.resource.compatibility.delivery.*(...)`
- `basisRefresh(...)`
- external packet delivery through `line.deliver(...)`
- basis-aware inspection after external delivery

## Happy Path

```ts
import {
  createSignals,
  resourceCollectionShape,
  resourceItemAspects,
  resourceParamIdentity,
  resourceParams,
  resourceRequestContext,
} from "forge-signal-wasm";

const signals = await createSignals();

const externalTasks = signals.resource.compatibility.collection({
  version: "forge-resource-external-v1",
  family: "collection",
  definitionId: "external-tasks",
  requestContract: "native-v1",
  reconciliationContract: "collection-v1",
  declaration: {
    params: resourceParams(),
    requestContext: resourceRequestContext({ basisId: "basis-1" }),
    normalizeParams: ({ workspaceId }) =>
      resourceParamIdentity({ workspaceId }, workspaceId),
    itemIdentity: (item: { id: string; title: string }) => item.id,
    reconcile: resourceCollectionShape({
      items: (value: { items: Array<{ id: string; title: string }> }) =>
        value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      aspects: resourceItemAspects({
        title: {
          read: (item) => item.title,
          write: (item, title: string) => ({ ...item, title }),
        },
      }),
    }),
    load: (_params, request) => ({
      items: [{ id: "t1", title: `Basis:${request.context.basisId}` }],
    }),
  },
});

const line = externalTasks.line({ workspaceId: "demo" });

line.deliver(
  signals.resource.compatibility.delivery.basisRefresh({
    packetId: "pkt-refresh",
    basisId: "basis-1",
    nextBasisId: "basis-2",
  }),
);

console.log(line.value());
console.log(line.request().context.basisId);
```

## When To Reach For This Feature

- websocket or push packets update a line
- an external system owns the resource declaration grammar
- stale packets must be rejected instead of silently applied
- basis repair has to happen explicitly before later packets can converge

## What To Inspect

- the `ResourceDeliveryResult` returned by `line.deliver(...)`
- `line.request().context.basisId`
- `line.diagnostics().lastDeliveryKind`
- `line.history().basis`
- `line.history().lifecycle`

## Where To Go Next

- collection patch and delivery helpers:
  [Collections And Delivery](./collections-and-delivery.md)
- line reads and history:
  [Line Inspection](./line-inspection.md)
- lower-level delivery reference:
  [Delivery And Compatibility Contract](../resource-contracts/delivery-and-compatibility.md)
