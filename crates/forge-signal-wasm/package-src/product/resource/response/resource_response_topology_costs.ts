const RESOURCE_RESPONSE_TOPOLOGY_COSTS = Object.freeze({
  connection: Object.freeze({
    itemLocus: "connection",
    item: ["connection-edge-item-id", 1, "single-connection-edge", "replaceNode"],
    itemDelete: ["connection-edge-item-id", 1, "whole-connection-edges", "replaceNodes"],
    itemInsert: ["connection-edge-item-id", 1, "whole-connection-edges", "replaceNodes"],
    broad: ["whole-connection-edges", 0, "whole-response", "replaceNodes"],
  }),
  detail: Object.freeze({
    itemLocus: "detailResponse",
    item: ["detail-response", 0, "whole-response", "replaceDetailResponse"],
    broad: ["detail-response", 0, "whole-response", "replaceDetailResponse"],
    field: ["detail-field", 1, "single-top-level-field", "replaceDetailField"],
    region: ["detail-region", 1, "declared-region", "replaceDetailRegion"],
    jsonPath: ["detail-json-path", 1, "json-path-segments", "replaceDetailJsonPath"],
  }),
  discriminatedTuple: Object.freeze({
    itemLocus: "discriminatedTuple",
    item: ["tuple-discriminator-item-id", 1, "active-variant-items", "replaceVariantItems"],
    itemInsert: ["tuple-discriminator-item-id", 1, "active-variant-items", "replaceVariantItems"],
    broad: ["whole-tuple-envelope", 0, "whole-response", "replaceVariantItems"],
  }),
  entityStore: Object.freeze({
    itemLocus: "entityStore",
    item: ["entity-id", 1, "single-entity-record", "replaceEntity"],
    itemDelete: ["entity-id", 1, "whole-entity-record", "replaceEntities"],
    itemInsert: ["entity-id", 1, "whole-entity-record", "replaceEntities"],
    broad: ["whole-entity-record", 0, "whole-response", "replaceEntities"],
  }),
  groupedCollection: Object.freeze({
    itemLocus: "groupedCollection",
    item: ["group-key-item-id", 1, "single-group", "replaceGroupItem"],
    itemDelete: ["group-key-item-id", 1, "single-group", "replaceGroups"],
    itemInsert: ["group-key-item-id", 1, "single-group", "replaceGroups"],
    broad: ["whole-group-record", 0, "whole-response", "replaceGroups"],
  }),
  mapCollection: Object.freeze({
    itemLocus: "mapCollection",
    item: ["map-key", 1, "single-map-entry", "replaceEntry"],
    itemDelete: ["map-key", 1, "whole-map", "replaceEntries"],
    itemInsert: ["map-key", 1, "whole-map", "replaceEntries"],
    broad: ["whole-map", 0, "whole-response", "replaceEntries"],
  }),
  namedCollection: Object.freeze({
    itemLocus: "namedCollection",
    item: ["collection-key-item-id", 1, "single-named-collection", "replaceCollectionItem"],
    itemDelete: ["collection-key-item-id", 1, "single-named-collection", "replaceCollections"],
    itemInsert: ["collection-key-item-id", 1, "single-named-collection", "replaceCollections"],
    broad: ["whole-named-collection-record", 0, "whole-response", "replaceCollections"],
  }),
  recursiveTree: Object.freeze({
    itemLocus: "recursiveTree",
    item: ["tree-descendant-path", 1, "single-descendant-path", "replaceNode"],
    itemDelete: ["tree-descendant-path", 1, "single-descendant-path", "replaceChildrenOrRoots"],
    itemInsert: ["tree-descendant-path", 1, "single-descendant-path", "replaceChildrenOrRoots"],
    broad: ["whole-tree-roots", 0, "whole-response", "replaceRoots"],
  }),
  sparsePage: Object.freeze({
    itemLocus: "sparsePage",
    item: ["sparse-page-item-id", 1, "loaded-page-chunk", "replacePageItem"],
    itemDelete: ["sparse-page-item-id", 1, "loaded-page-chunk", "replacePages"],
    itemInsert: ["sparse-page-item-id", 1, "loaded-page-chunk", "replacePages"],
    broad: ["whole-sparse-pages", 0, "whole-response", "replacePages"],
  }),
  summary: Object.freeze({
    itemLocus: "summaryResponse",
    item: ["summary-response", 0, "whole-response", "replaceSummaryResponse"],
  }),
});

export { RESOURCE_RESPONSE_TOPOLOGY_COSTS };
