export function titlePatch(runtime, index, title) {
  return runtime.mod.resourcePatch.itemAspect({
    itemId: `item:${index}`,
    aspect: "title",
    value: title,
  });
}

export function createEffectLine(runtime) {
  const { mod, resource } = runtime;
  const family = resource.collection({
    params: mod.resourceParams(),
    normalizeParams: ({ workspaceId }) =>
      mod.resourceParamIdentity({ workspaceId }, workspaceId),
    requestContext: mod.resourceRequestContext({ basisId: "basis-1" }),
    effects: mod.resourceEffects.branchNative(),
    itemIdentity: (item) => item.id,
    reconcile: mod.resourceCollectionShape({
      items: (value) => value.items,
      replaceItems: (value, items) => ({ ...value, items: [...items] }),
      aspects: mod.resourceItemAspects({
        title: {
          read: (item) => item.title,
          write: (item, title) => ({ ...item, title: String(title) }),
        },
      }),
    }),
    load: () => ({
      items: Array.from({ length: 10 }, (_, index) => ({
        id: `item:${index}`,
        title: `loaded-${index}`,
      })),
    }),
  });
  return family.line({ workspaceId: "demo" });
}
