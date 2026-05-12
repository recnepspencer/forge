const RESOURCE_RESPONSE_TOPOLOGY_COSTS = Object.freeze({
  connection: Object.freeze({
    itemLocus: "connection",
    item: ["connection-edge-item-id", 1, "single-connection-edge", "replaceNode"],
    broad: ["whole-connection-edges", 0, "whole-response", "replaceNodes"],
  }),
  detail: Object.freeze({
    itemLocus: "detailResponse",
    item: ["detail-response", 0, "whole-response", "replaceDetailResponse"],
  }),
  discriminatedTuple: Object.freeze({
    itemLocus: "discriminatedTuple",
    item: ["tuple-discriminator-item-id", 1, "active-variant-items", "replaceVariantItems"],
    broad: ["whole-tuple-envelope", 0, "whole-response", "replaceVariantItems"],
  }),
  entityStore: Object.freeze({
    itemLocus: "entityStore",
    item: ["entity-id", 1, "single-entity-record", "replaceEntity"],
    broad: ["whole-entity-record", 0, "whole-response", "replaceEntities"],
  }),
  groupedCollection: Object.freeze({
    itemLocus: "groupedCollection",
    item: ["group-key-item-id", 1, "single-group", "replaceGroupItem"],
    broad: ["whole-group-record", 0, "whole-response", "replaceGroups"],
  }),
  mapCollection: Object.freeze({
    itemLocus: "mapCollection",
    item: ["map-key", 1, "single-map-entry", "replaceEntry"],
    broad: ["whole-map", 0, "whole-response", "replaceEntries"],
  }),
  namedCollection: Object.freeze({
    itemLocus: "namedCollection",
    item: ["collection-key-item-id", 1, "single-named-collection", "replaceCollectionItem"],
    broad: ["whole-named-collection-record", 0, "whole-response", "replaceCollections"],
  }),
  recursiveTree: Object.freeze({
    itemLocus: "recursiveTree",
    item: ["tree-descendant-path", 1, "single-descendant-path", "replaceNode"],
    broad: ["whole-tree-roots", 0, "whole-response", "replaceRoots"],
  }),
  sparsePage: Object.freeze({
    itemLocus: "sparsePage",
    item: ["sparse-page-item-id", 1, "loaded-page-chunk", "replacePageItem"],
    broad: ["whole-sparse-pages", 0, "whole-response", "replacePages"],
  }),
  summary: Object.freeze({
    itemLocus: "summaryResponse",
    item: ["summary-response", 0, "whole-response", "replaceSummaryResponse"],
  }),
});

export { RESOURCE_RESPONSE_TOPOLOGY_COSTS };
