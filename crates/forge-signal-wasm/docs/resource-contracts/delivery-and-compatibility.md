# Resource Delivery And Compatibility Reference

If your question is "how do I handle pushed packets, external definitions, or
basis refresh?", start with
[External Delivery And Compatibility](../resources/external-delivery-and-compatibility.md)
before using this lower-level reference page.

## What This Feature Is

This is the resource surface for server-push updates and external resource
definitions.

Use it when:

- a websocket or push event updates a resource line
- another system sends patches or invalidations into your app
- you need to describe a resource definition that came from outside the local
  app code

If you are only doing ordinary request/refresh work, you can ignore this page
for now.

## Why You Use It

- apply pushed updates through the same line rules as local updates
- reject stale pushed patches instead of silently corrupting local state
- admit external resource definitions without building a second client engine
- repair stale external packets explicitly when the server version moved ahead
- inspect whether a pushed update was applied, ignored, or rejected

## Stable Entry Points

Local delivery helpers:

- `resourceDelivery.replace(...)`
- `resourceDelivery.patch(...)`
- `resourceDelivery.invalidate(...)`
- `line.deliver(...)`

External compatibility entry points:

- `signals.resource.compatibility.detail(...)`
- `signals.resource.compatibility.collection(...)`
- `signals.resource.compatibility.paged(...)`
- `signals.resource.compatibility.delivery.replace(...)`
- `signals.resource.compatibility.delivery.patch(...)`
- `signals.resource.compatibility.delivery.invalidate(...)`
- `signals.resource.compatibility.delivery.basisRefresh(...)`

## Core Mental Model

Delivery is not a second cache.

It is a way to feed pushed updates into an existing line.

That means every delivered update is judged against the same line that already
owns:

- the current value
- request state
- reconciliation rules
- diagnostics
- history

You will also see the term `basis` here. In plain English, that is the version
or snapshot the incoming patch was built against.

If the line is on a different basis, the patch can be rejected until you repair
or refresh it.

## How It Executes

For local delivery:

1. a line receives a delivered update
2. the runtime checks whether the update matches the line's current basis and
   reconciliation rules
3. the runtime applies it, rejects it, or ignores it

For external compatibility:

1. you admit an external definition through
   `signals.resource.compatibility.*(...)`
2. that external definition still creates ordinary local lines
3. external packets are delivered through the compatibility delivery helpers
4. if an external packet is stale, `basisRefresh(...)` repairs the line's basis
   before later packets apply

You do not need the compatibility lane unless another system is generating
resource definitions or delivery packets for you.

## Small Example

```ts
import {
  createSignals,
  resourceCollectionShape,
  resourceDelivery,
  resourceParamIdentity,
  resourceParams,
  resourcePatch,
} from "forge-signal-wasm";

const signals = await createSignals();

const tasks = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (item: { id: string; title: string }) => item.id,
  reconcile: resourceCollectionShape({
    items: (value: { items: Array<{ id: string; title: string }> }) =>
      value.items,
    replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
  }),
  load: () => ({
    items: [{ id: "t1", title: "First" }],
  }),
});

const line = tasks.line({ workspaceId: "demo" });

line.deliver(
  resourceDelivery.patch({
    packetId: "pkt-1",
    basisId: null,
    patch: resourcePatch.replace({
      items: [{ id: "t1", title: "Delivered" }],
    }),
  }),
);
```

This is the basic case: a pushed update changes the existing line through
`line.deliver(...)`.

## Real Example

```ts
import {
  createSignals,
  resourceCollectionShape,
  resourceItemAspects,
  resourceParamIdentity,
  resourceParams,
  resourcePatch,
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
    params: resourceParams<{ workspaceId: string }>(),
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
          read: (item: { id: string; title: string }) => item.title,
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

line.deliver(
  signals.resource.compatibility.delivery.patch({
    packetId: "pkt-patch",
    basisId: "basis-2",
    nextBasisId: "basis-3",
    patch: resourcePatch.itemAspect({
      itemId: "t1",
      aspect: "title",
      value: "Delivered After Refresh",
    }),
  }),
);

console.log(line.value());
console.log(line.request().context.basisId);
console.log(line.diagnosticsSummary().latest);
```

Use this pattern when another system is sending you patches and you need a safe
way to reject stale ones, repair the basis, and keep using the same local line.

## How It Relates To Other Features

- Pair delivery with reconciliation when pushed updates should target one item,
  one field, or one summary value.
- Pair compatibility with family authoring when you want to understand the
  declaration that external systems are feeding into.
- Pair it with line history and diagnostics when you need to explain packet
  provenance or basis changes.

## Inspection And Debugging

Start with:

- the `ResourceDeliveryResult` returned by `line.deliver(...)`
- `line.request().context.basisId`
- `line.diagnostics().lastDeliveryKind`
- `line.diagnostics().basis`
- `line.history().basis`
- `line.history().lifecycle`

These answer:

- whether the pushed update applied
- whether it was duplicated or rejected as stale
- which basis the line currently trusts
- whether a basis refresh actually advanced the line

## Anti-Patterns

- treating delivery as a second resource engine
- applying external packets without basis checks
- assuming external definitions get different lifecycle semantics than native
  ones
- using delivery to smuggle in general mutation workflow intent

## Current Limits

- compatibility is explicit and typed; it does not accept arbitrary foreign
  packet grammars
- basis refresh is the supported way to repair stale external delivery

## Related Docs

- [Reconciliation Contract](./reconciliation.md)
- [Resource Family Authoring Reference](../api-reference/resource-family-authoring.md)
- [Resource Line Reference](../api-reference/resource-line.md)
