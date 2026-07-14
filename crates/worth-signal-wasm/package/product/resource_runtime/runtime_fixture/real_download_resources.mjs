export function createRealDownloadDetail(mod, signals, options = {}) {
  return signals.resource.detail({
    params: mod.resourceParams(),
    normalizeParams: ({ assetId }) => mod.resourceParamIdentity({ assetId }, assetId),
    ...options,
  });
}

export function createRealDownloadCollection(mod, signals, options = {}) {
  return signals.resource.collection({
    params: mod.resourceParams(),
    normalizeParams: ({ workspaceId }) =>
      mod.resourceParamIdentity({ workspaceId }, workspaceId),
    itemIdentity: (item) => item.id,
    reconcile: mod.resourceCollectionShape({
      items: (value) => value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      aspects: mod.resourceItemAspects({
        title: {
          read: (item) => item.title,
          write: (item, title) => ({ ...item, title: String(title) }),
        },
      }),
    }),
    ...options,
  });
}
