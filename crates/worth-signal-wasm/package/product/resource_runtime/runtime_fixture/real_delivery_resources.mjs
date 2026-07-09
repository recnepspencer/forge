import { createRealResourceNamespace } from "./real_resource_signals.mjs";

export function createRealDeliveryCollectionLine(
  resourceMod,
  signals,
  load,
) {
  return signals.resource.collection({
    params: resourceMod.resourceParams(),
    normalizeParams: ({ workspaceId }) =>
      resourceMod.resourceParamIdentity({ workspaceId }, workspaceId),
    requestContext: resourceMod.resourceRequestContext({ basisId: "basis-1" }),
    itemIdentity: (item) => item.id,
    reconcile: resourceMod.resourceCollectionShape({
      items: (value) => value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      aspects: resourceMod.resourceItemAspects({
        title: {
          read: (item) => item.title,
          write: (item, title) => ({ ...item, title: String(title) }),
        },
      }),
    }),
    load,
  }).line({ workspaceId: "demo" });
}

export function createRealCompatibilityDelivery(
  resourceMod,
  signals,
  overrides = null,
) {
  return createRealResourceNamespace(
    resourceMod,
    signals,
    overrides,
  ).compatibility.delivery;
}

export function createRealCompatibilityDeliveryLine(
  resourceMod,
  signals,
  load,
  overrides = null,
) {
  return createRealResourceNamespace(resourceMod, signals, overrides).collection({
    params: resourceMod.resourceParams(),
    normalizeParams: ({ workspaceId }) =>
      resourceMod.resourceParamIdentity({ workspaceId }, workspaceId),
    requestContext: resourceMod.resourceRequestContext({ basisId: "basis-1" }),
    itemIdentity: (item) => item.id,
    reconcile: resourceMod.resourceCollectionShape({
      items: (value) => value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      aspects: resourceMod.resourceItemAspects({
        title: {
          read: (item) => item.title,
          write: (item, title) => ({ ...item, title: String(title) }),
        },
      }),
    }),
    load,
  }).line({ workspaceId: "demo" });
}
