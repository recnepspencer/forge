import type { SignalValue } from "../model.js";
import type {
  ResourceItemAspectMap,
  ResourceItemAspects,
  ResourceValueSummaries,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";
import type {
  ResourceArrayResponse,
  ResourceCollectionResponse,
  ResourceConnectionResponse,
  ResourceDiscriminatedTupleResponse,
  ResourceEntityStoreResponse,
  ResourceGroupedCollectionResponse,
  ResourceJsonPathAspectDeclaration,
  ResourceJsonPathAspectDeclarationMap,
  ResourceJsonPathAspectDefinitions,
  ResourceMapCollectionResponse,
  ResourceNamedCollectionResponse,
  ResourceObjectArrayFieldItem,
  ResourceObjectArrayFieldName,
  ResourceObjectAspectDefinitions,
  ResourceObjectAspectFieldMap,
  ResourceSparsePageResponse,
  ResourceDetailResponse,
  ResourceSummaryResponse,
  ResourceTreeResponse,
} from "./resource_response.js";
import type {
  ResourceDetailObjectFieldDefinitions,
  ResourceDetailObjectFieldMap,
} from "./resource_detail_fields.js";
import type { ResourceDetailRegionMap } from "./resource_detail_regions.js";
import type {
  ResourceDetailJsonPathDeclaration,
  ResourceDetailJsonPathDeclarationMap,
  ResourceDetailJsonPathDefinitions,
} from "./resource_detail_json_paths.js";

export interface ResourceResponseFactory {
  objectAspects<TItem>(): <TFields extends ResourceObjectAspectFieldMap<TItem>>(
    fields: TFields,
  ) => ResourceItemAspects<TItem, ResourceObjectAspectDefinitions<TItem, TFields>>;
  jsonObjectAspects<TItem>(): <TFields extends ResourceObjectAspectFieldMap<TItem>>(
    fields: TFields,
  ) => ResourceItemAspects<TItem, ResourceObjectAspectDefinitions<TItem, TFields>>;
  jsonPathAspects<TItem>(): <
    TValueMap extends Readonly<Record<string, SignalValue>> = Readonly<Record<string, SignalValue>>,
    TDefinitions extends ResourceJsonPathAspectDeclarationMap<TItem> = {
      readonly [TAspect in keyof TValueMap & string]: ResourceJsonPathAspectDeclaration<TItem>;
    },
  >(
    definitions: TDefinitions,
  ) => ResourceItemAspects<TItem, ResourceJsonPathAspectDefinitions<TItem, TValueMap>>;
  array<
    TItem,
    TAspectMap extends ResourceItemAspectMap<TItem> = {},
    TSummaryMap extends ResourceValueSummaryMap<readonly TItem[]> = {},
  >(options: {
    itemId(item: TItem): string;
    aspects?: ResourceItemAspects<TItem, TAspectMap>;
    summaries?: ResourceValueSummaries<readonly TItem[], TSummaryMap, any>;
  }): ResourceArrayResponse<TItem, TAspectMap, TSummaryMap>;
  collection<
    TValue,
    TItem,
    TAspectMap extends ResourceItemAspectMap<TItem> = {},
    TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  >(options: {
    itemId(item: TItem): string;
    items(value: TValue): readonly TItem[];
    replaceItems(value: TValue, nextItems: readonly TItem[]): TValue;
    aspects?: ResourceItemAspects<TItem, TAspectMap>;
    summaries?: ResourceValueSummaries<TValue, TSummaryMap, any>;
  }): ResourceCollectionResponse<TValue, TItem, TAspectMap, TSummaryMap>;
  collection<TValue>(): <
    TItem,
    TAspectMap extends ResourceItemAspectMap<TItem> = {},
    TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  >(options: {
    itemId(item: TItem): string;
    items(value: TValue): readonly TItem[];
    replaceItems(value: TValue, nextItems: readonly TItem[]): TValue;
    aspects?: ResourceItemAspects<TItem, TAspectMap>;
    summaries?: ResourceValueSummaries<TValue, TSummaryMap, any>;
  }) => ResourceCollectionResponse<TValue, TItem, TAspectMap, TSummaryMap>;
  connection<TValue>(): <
    TEdge,
    TItem,
    TAspectMap extends ResourceItemAspectMap<TItem> = {},
    TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  >(options: {
    itemId(item: TItem): string;
    edges(value: TValue): readonly TEdge[];
    node(edge: TEdge): TItem;
    edgeIndexForItem(value: TValue, itemId: string): number | null | undefined;
    replaceNodes(value: TValue, nextNodes: readonly TItem[]): TValue;
    replaceNode(value: TValue, itemId: string, nextNode: TItem): TValue;
    aspects?: ResourceItemAspects<TItem, TAspectMap>;
    summaries?: ResourceValueSummaries<TValue, TSummaryMap, any>;
  }) => ResourceConnectionResponse<TValue, TEdge, TItem, TAspectMap, TSummaryMap>;
  discriminated<TValue>(): <
    TItem,
    TAspectMap extends ResourceItemAspectMap<TItem> = {},
    TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  >(options: {
    itemId(item: TItem): string;
    discriminator(value: TValue): string;
    variants: Readonly<Record<string, {
      items(value: TValue): readonly TItem[];
      replaceItems(value: TValue, nextItems: readonly TItem[]): TValue;
    }>>;
    aspects?: ResourceItemAspects<TItem, TAspectMap>;
    summaries?: ResourceValueSummaries<TValue, TSummaryMap, any>;
  }) => ResourceDiscriminatedTupleResponse<TValue, TItem, TAspectMap, TSummaryMap>;
  objectItems<TValue>(): <
    TField extends ResourceObjectArrayFieldName<TValue>,
    TItem extends ResourceObjectArrayFieldItem<TValue, TField>,
    TAspectMap extends ResourceItemAspectMap<TItem> = {},
    TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  >(options: {
    field: TField;
    itemId(item: TItem): string;
    aspects?: ResourceItemAspects<TItem, TAspectMap>;
    summaries?: ResourceValueSummaries<TValue, TSummaryMap, any>;
  }) => ResourceCollectionResponse<TValue, TItem, TAspectMap, TSummaryMap>;
  entityStore<TValue>(): <
    TItem,
    TAspectMap extends ResourceItemAspectMap<TItem> = {},
    TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  >(options: {
    itemId(item: TItem): string;
    entities(value: TValue): Readonly<Record<string, TItem>>;
    replaceEntities(value: TValue, nextEntities: Readonly<Record<string, TItem>>): TValue;
    replaceEntity(value: TValue, itemId: string, nextItem: TItem): TValue;
    aspects?: ResourceItemAspects<TItem, TAspectMap>;
    summaries?: ResourceValueSummaries<TValue, TSummaryMap, any>;
  }) => ResourceEntityStoreResponse<TValue, TItem, TAspectMap, TSummaryMap>;
  map<TValue>(): <
    TItem,
    TAspectMap extends ResourceItemAspectMap<TItem> = {},
    TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  >(options: {
    itemId(item: TItem): string;
    entries(value: TValue): ReadonlyMap<string, TItem>;
    replaceEntries(value: TValue, nextEntries: ReadonlyMap<string, TItem>): TValue;
    replaceEntry(value: TValue, itemId: string, nextItem: TItem): TValue;
    aspects?: ResourceItemAspects<TItem, TAspectMap>;
    summaries?: ResourceValueSummaries<TValue, TSummaryMap, any>;
  }) => ResourceMapCollectionResponse<TValue, TItem, TAspectMap, TSummaryMap>;
  grouped<TValue>(): <
    TItem,
    TAspectMap extends ResourceItemAspectMap<TItem> = {},
    TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  >(options: {
    itemId(item: TItem): string;
    groupId(item: TItem): string;
    groupForItem(itemId: string): string;
    groups(value: TValue): Readonly<Record<string, readonly TItem[]>>;
    replaceGroups(value: TValue, nextGroups: Readonly<Record<string, readonly TItem[]>>): TValue;
    replaceGroupItem(value: TValue, groupId: string, itemId: string, nextItem: TItem): TValue;
    aspects?: ResourceItemAspects<TItem, TAspectMap>;
    summaries?: ResourceValueSummaries<TValue, TSummaryMap, any>;
  }) => ResourceGroupedCollectionResponse<TValue, TItem, TAspectMap, TSummaryMap>;
  sparse<TValue>(): <
    TItem,
    TAspectMap extends ResourceItemAspectMap<TItem> = {},
    TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  >(options: {
    itemId(item: TItem): string;
    pageId(item: TItem): string;
    pageForItem(itemId: string): string;
    pages(value: TValue): Readonly<Record<string, readonly TItem[]>>;
    replacePages(value: TValue, nextPages: Readonly<Record<string, readonly TItem[]>>): TValue;
    replacePageItem(value: TValue, pageId: string, itemId: string, nextItem: TItem): TValue;
    aspects?: ResourceItemAspects<TItem, TAspectMap>;
    summaries?: ResourceValueSummaries<TValue, TSummaryMap, any>;
  }) => ResourceSparsePageResponse<TValue, TItem, TAspectMap, TSummaryMap>;
  named<TValue>(): <
    TItem,
    TAspectMap extends ResourceItemAspectMap<TItem> = {},
    TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  >(options: {
    itemId(item: TItem): string;
    collectionId(item: TItem): string;
    collectionForItem(itemId: string): string;
    collections(value: TValue): Readonly<Record<string, readonly TItem[]>>;
    replaceCollections(value: TValue, nextCollections: Readonly<Record<string, readonly TItem[]>>): TValue;
    replaceCollectionItem(value: TValue, collectionId: string, itemId: string, nextItem: TItem): TValue;
    aspects?: ResourceItemAspects<TItem, TAspectMap>;
    summaries?: ResourceValueSummaries<TValue, TSummaryMap, any>;
  }) => ResourceNamedCollectionResponse<TValue, TItem, TAspectMap, TSummaryMap>;
  multiple<TValue>(): <
    TItem,
    TAspectMap extends ResourceItemAspectMap<TItem> = {},
    TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  >(options: {
    itemId(item: TItem): string;
    collectionId(item: TItem): string;
    collectionForItem(itemId: string): string;
    collections(value: TValue): Readonly<Record<string, readonly TItem[]>>;
    replaceCollections(value: TValue, nextCollections: Readonly<Record<string, readonly TItem[]>>): TValue;
    replaceCollectionItem(value: TValue, collectionId: string, itemId: string, nextItem: TItem): TValue;
    aspects?: ResourceItemAspects<TItem, TAspectMap>;
    summaries?: ResourceValueSummaries<TValue, TSummaryMap, any>;
  }) => ResourceNamedCollectionResponse<TValue, TItem, TAspectMap, TSummaryMap>;
  tree<TValue>(): <
    TItem,
    TAspectMap extends ResourceItemAspectMap<TItem> = {},
    TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  >(options: {
    itemId(item: TItem): string;
    roots(value: TValue): readonly TItem[];
    children(item: TItem): readonly TItem[];
    replaceRoots(value: TValue, nextRoots: readonly TItem[]): TValue;
    nodeForItem(itemId: string): readonly string[];
    replaceNode(value: TValue, path: readonly string[], itemId: string, nextNode: TItem): TValue;
    aspects?: ResourceItemAspects<TItem, TAspectMap>;
    summaries?: ResourceValueSummaries<TValue, TSummaryMap, any>;
  }) => ResourceTreeResponse<TValue, TItem, TAspectMap, TSummaryMap>;
  detail<TValue>(): <TFields extends ResourceDetailObjectFieldMap<TValue> = {}>(
    fields?: TFields,
  ) => ResourceDetailResponse<
    TValue,
    ResourceDetailObjectFieldDefinitions<TValue, TFields>,
    {},
    {}
  >;
  detailRegions<TValue>(): <
    TRegionMap extends ResourceDetailRegionMap<TValue> = {},
  >(
    definitions: TRegionMap,
  ) => ResourceDetailResponse<
    TValue,
    {},
    TRegionMap,
    {}
  >;
  detailJsonPaths<TValue>(): <
    TValueMap extends Readonly<Record<string, SignalValue>> = Readonly<Record<string, SignalValue>>,
    TDefinitions extends ResourceDetailJsonPathDeclarationMap<TValue> = {
      readonly [TPath in keyof TValueMap & string]: ResourceDetailJsonPathDeclaration<TValue>;
    },
  >(
    definitions: TDefinitions,
  ) => ResourceDetailResponse<TValue, {}, {}, ResourceDetailJsonPathDefinitions<TValue, TValueMap>>;
  summary<TValue>(): ResourceSummaryResponse<TValue>;
}
