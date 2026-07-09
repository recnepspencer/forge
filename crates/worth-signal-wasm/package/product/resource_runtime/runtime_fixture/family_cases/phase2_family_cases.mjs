function createPhase2FamilyCases(resource, mod) {
  return [
    {
      kind: "detail",
      build({ load, policy }) {
        return resource.detail({
          params: mod.resourceParams(),
          policy,
          normalizeParams: ({ productId }) =>
            mod.resourceParamIdentity({ productId }, productId),
          load,
        });
      },
      value(productId, version) {
        return {
          id: productId,
          version,
          kind: "detail",
        };
      },
      changedValue(productId, version) {
        return {
          id: productId,
          version,
          kind: "detail",
        };
      },
    },
    {
      kind: "collection",
      build({ load, policy }) {
        return resource.collection({
          params: mod.resourceParams(),
          policy,
          normalizeParams: ({ productId }) =>
            mod.resourceParamIdentity({ productId }, productId),
          itemIdentity: (item) => item.id,
          load,
        });
      },
      value(productId, version) {
        return [{
          id: productId,
          version,
          kind: "collection",
        }];
      },
      changedValue(productId, version) {
        return [{
          id: productId,
          version,
          kind: "collection",
        }];
      },
    },
    {
      kind: "paged",
      build({ load, policy }) {
        return resource.paged({
          params: mod.resourceParams(),
          policy,
          normalizeParams: ({ productId }) =>
            mod.resourceParamIdentity({ productId }, productId),
          itemIdentity: (item) => item.id,
          accumulatePage: (existing, next) => [...existing, ...next],
          load,
        });
      },
      value(productId, version) {
        return [{
          id: productId,
          version,
          kind: "paged",
        }];
      },
      changedValue(productId, version) {
        return [{
          id: productId,
          version,
          kind: "paged",
        }];
      },
    },
  ];
}

export { createPhase2FamilyCases };
