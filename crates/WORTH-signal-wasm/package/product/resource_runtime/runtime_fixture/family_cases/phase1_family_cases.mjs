function createPhase1FamilyCases(resource, mod) {
  return [
    {
      kind: "detail",
      build(load) {
        return resource.detail({
          params: mod.resourceParams(),
          normalizeParams: ({ productId }) =>
            mod.resourceParamIdentity({ productId }, productId),
          load,
        });
      },
      directLoad: ({ productId }) => ({
        id: productId,
        label: `product:${productId}`,
      }),
      helperLoad: ({ productId }) => ({
        id: productId,
        label: `product:${productId}`,
      }),
      expectedValue(productId) {
        return {
          id: productId,
          label: `product:${productId}`,
        };
      },
      view(line) {
        return line.view((value) => value?.label ?? null);
      },
      expectedViewValue(productId) {
        return `product:${productId}`;
      },
    },
    {
      kind: "collection",
      build(load) {
        return resource.collection({
          params: mod.resourceParams(),
          normalizeParams: ({ productId }) =>
            mod.resourceParamIdentity({ productId }, productId),
          itemIdentity: (item) => item.id,
          load,
        });
      },
      directLoad: ({ productId }) => [
        { id: productId, label: `product:${productId}` },
      ],
      helperLoad: ({ productId }) => [
        { id: productId, label: `product:${productId}` },
      ],
      expectedValue(productId) {
        return [{ id: productId, label: `product:${productId}` }];
      },
      view(line) {
        return line.view((value) => value?.[0]?.label ?? null);
      },
      expectedViewValue(productId) {
        return `product:${productId}`;
      },
    },
    {
      kind: "paged",
      build(load) {
        return resource.paged({
          params: mod.resourceParams(),
          normalizeParams: ({ productId }) =>
            mod.resourceParamIdentity({ productId }, productId),
          itemIdentity: (item) => item.id,
          accumulatePage: (existing, next) => [...existing, ...next],
          load,
        });
      },
      directLoad: ({ productId }) => [
        { id: productId, label: `product:${productId}` },
      ],
      helperLoad: ({ productId }) => [
        { id: productId, label: `product:${productId}` },
      ],
      expectedValue(productId) {
        return [{ id: productId, label: `product:${productId}` }];
      },
      view(line) {
        return line.view((value) => value?.[0]?.label ?? null);
      },
      expectedViewValue(productId) {
        return `product:${productId}`;
      },
    },
  ];
}

export { createPhase1FamilyCases };
