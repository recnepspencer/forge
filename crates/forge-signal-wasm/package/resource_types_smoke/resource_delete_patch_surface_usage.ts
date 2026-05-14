import {
  createSignals,
  resourceCollectionShape,
  resourceParamIdentity,
  resourceParams,
} from "../index.js";

const signals = createSignals();

const collection = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (item: { id: string; title: string }) => item.id,
  reconcile: resourceCollectionShape({
    items: (value: { items: Array<{ id: string; title: string }> }) => value.items,
    replaceItems: (
      value: { items: Array<{ id: string; title: string }> },
      nextItems: readonly { id: string; title: string }[],
    ) => ({ ...value, items: [...nextItems] }),
  }),
  load: () => ({
    items: [
      { id: "workspace:demo", title: "Task" },
      { id: "workspace:demo:2", title: "Second" },
    ],
  }),
});

const line = collection.line({ workspaceId: "demo" });
const deletePatch = collection.patch.delete({
  itemId: "workspace:demo",
});
const deleteDelivery = collection.delivery.delete({
  packetId: "pkt-delete",
  basisId: null,
  nextBasisId: "basis-1",
  itemId: "workspace:demo:2",
});

void line.patch(deletePatch);
void line.deliver(deleteDelivery);
