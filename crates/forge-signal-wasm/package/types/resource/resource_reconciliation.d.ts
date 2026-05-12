import type { SignalValue } from "../model.js";
import type {
  ResourceDetailFieldMap,
  ResourceDetailFieldValue,
  ResourceDetailFields,
} from "./resource_detail_fields.js";
import type {
  ResourceDetailRegionMap,
  ResourceDetailRegionValue,
  ResourceDetailRegions,
} from "./resource_detail_regions.js";
import type {
  ResourceDetailJsonPathMap,
  ResourceDetailJsonPathValue,
  ResourceDetailJsonPaths,
} from "./resource_detail_json_paths.js";
import type { ResourceResponseLensProof } from "./resource_response.js";

declare const forgeSignalResourceItemAspectsBrand: unique symbol;
declare const forgeSignalResourceCollectionShapeBrand: unique symbol;
declare const forgeSignalResourcePatchBrand: unique symbol;
declare const forgeSignalResourceValueSummariesBrand: unique symbol;
declare const forgeSignalResourceDeliveryBrand: unique symbol;

export type ResourceSummaryPatchScope = "line" | "pageWindow";
export type ResourceItemAspectLocus = "itemAspect" | "jsonItemAspect";
export type ResourceDetailReconcile<TValue> =
  | ResourceDetailFields<TValue, any>
  | ResourceDetailRegions<TValue, any>
  | ResourceDetailJsonPaths<TValue, any>;

export interface ResourceItemAspect<TItem, TValue = SignalValue> {
  read(item: TItem): TValue;
  write(item: TItem, value: TValue): TItem;
  readonly locus?: ResourceItemAspectLocus;
}

export type ResourceItemAspectMap<TItem> = Readonly<
  Record<string, ResourceItemAspect<TItem, any>>
>;

export type ResourceItemAspectValue<TAspect> =
  TAspect extends ResourceItemAspect<any, infer TValue> ? TValue : never;

export interface ResourceItemAspects<
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = ResourceItemAspectMap<TItem>,
> {
  readonly definitions: TAspectMap;
  readonly [forgeSignalResourceItemAspectsBrand]: "resourceItemAspects";
}

export interface ResourceValueSummary<TValue, TSummary = SignalValue> {
  read(value: TValue): TSummary;
  write(value: TValue, summary: TSummary): TValue;
}

export type ResourceValueSummaryMap<TValue> = Readonly<
  Record<string, ResourceValueSummary<TValue, any>>
>;

export type ResourceValueSummaryValue<TSummary> =
  TSummary extends ResourceValueSummary<any, infer TValue> ? TValue : never;

export interface ResourceValueSummaries<
  TValue,
  TSummaryMap extends ResourceValueSummaryMap<TValue> = ResourceValueSummaryMap<TValue>,
  TPatchScope extends ResourceSummaryPatchScope = "line",
> {
  readonly definitions: TSummaryMap;
  readonly patchScope: TPatchScope;
  readonly [forgeSignalResourceValueSummariesBrand]: "resourceValueSummaries";
}

export interface ResourceCollectionShape<
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
> {
  items(value: TValue): readonly TItem[];
  replaceItems(value: TValue, nextItems: readonly TItem[]): TValue;
  readonly aspects: ResourceItemAspects<TItem, TAspectMap> | null;
  readonly summaries: ResourceValueSummaries<TValue, TSummaryMap, TSummaryPatchScope> | null;
  readonly responseLensProof: ResourceResponseLensProof | null;
  readonly [forgeSignalResourceCollectionShapeBrand]: "resourceCollectionShape";
}

export interface FieldResourcePatch<
  TField extends string = string,
  TValue = SignalValue,
> {
  readonly kind: "field";
  readonly field: TField;
  readonly value: TValue;
  readonly [forgeSignalResourcePatchBrand]: "resourcePatch";
}

export interface JsonPathResourcePatch<
  TPath extends string = string,
  TValue = SignalValue,
> {
  readonly kind: "jsonPath";
  readonly path: TPath;
  readonly value: TValue;
  readonly [forgeSignalResourcePatchBrand]: "resourcePatch";
}

export interface RegionResourcePatch<
  TRegion extends string = string,
  TValue = SignalValue,
> {
  readonly kind: "region";
  readonly region: TRegion;
  readonly value: TValue;
  readonly [forgeSignalResourcePatchBrand]: "resourcePatch";
}

export interface ReplaceResourcePatch<TValue> {
  readonly kind: "replace";
  readonly nextValue: TValue;
  readonly [forgeSignalResourcePatchBrand]: "resourcePatch";
}

export interface ItemResourcePatch<TItem> {
  readonly kind: "item";
  readonly itemId: string;
  readonly nextItem: TItem;
  readonly [forgeSignalResourcePatchBrand]: "resourcePatch";
}

export interface ItemAspectResourcePatch<
  TAspect extends string = string,
  TValue = SignalValue,
> {
  readonly kind: "itemAspect";
  readonly itemId: string;
  readonly aspect: TAspect;
  readonly value: TValue;
  readonly [forgeSignalResourcePatchBrand]: "resourcePatch";
}

export interface SummaryResourcePatch<
  TSummary extends string = string,
  TValue = SignalValue,
> {
  readonly kind: "summary";
  readonly summary: TSummary;
  readonly value: TValue;
  readonly [forgeSignalResourcePatchBrand]: "resourcePatch";
}

export type AspectResourcePatchUnion<
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
> = {
  [TAspect in keyof TAspectMap & string]: ItemAspectResourcePatch<
    TAspect,
    ResourceItemAspectValue<TAspectMap[TAspect]>
  >;
}[keyof TAspectMap & string];

export type SummaryResourcePatchUnion<
  TValue,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
> = {
  [TSummary in keyof TSummaryMap & string]: SummaryResourcePatch<
    TSummary,
    ResourceValueSummaryValue<TSummaryMap[TSummary]>
  >;
}[keyof TSummaryMap & string];

export type DetailFieldResourcePatchUnion<
  TValue,
  TFieldMap extends ResourceDetailFieldMap<TValue>,
> = {
  [TField in keyof TFieldMap & string]: FieldResourcePatch<
    TField,
    ResourceDetailFieldValue<TFieldMap[TField]>
  >;
}[keyof TFieldMap & string];

export type DetailJsonPathResourcePatchUnion<
  TValue,
  TPathMap extends ResourceDetailJsonPathMap<TValue>,
> = {
  [TPath in keyof TPathMap & string]: JsonPathResourcePatch<
    TPath,
    ResourceDetailJsonPathValue<TPathMap[TPath]>
  >;
}[keyof TPathMap & string];

export type DetailRegionResourcePatchUnion<
  TValue,
  TRegionMap extends ResourceDetailRegionMap<TValue>,
> = {
  [TRegion in keyof TRegionMap & string]: RegionResourcePatch<
    TRegion,
    ResourceDetailRegionValue<TRegionMap[TRegion]>
  >;
}[keyof TRegionMap & string];

export type ResourcePatch<
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
> =
  | ReplaceResourcePatch<TValue>
  | DetailFieldResourcePatchUnion<TValue, {}>
  | DetailRegionResourcePatchUnion<TValue, {}>
  | DetailJsonPathResourcePatchUnion<TValue, {}>
  | ItemResourcePatch<TItem>
  | AspectResourcePatchUnion<TItem, TAspectMap>
  | SummaryResourcePatchUnion<TValue, TSummaryMap>;

export type ResourceReconcileAspectMap<TReconcile> =
  [TReconcile] extends [
    ResourceCollectionShape<any, infer TItem, infer TAspectMap, any>,
  ]
    ? TAspectMap extends ResourceItemAspectMap<TItem>
      ? TAspectMap
      : {}
    : {};

export type ResourceReconcileSummaryMap<TReconcile> =
  [TReconcile] extends [
    ResourceCollectionShape<infer TValue, any, any, infer TSummaryMap, any>,
  ]
    ? TSummaryMap extends ResourceValueSummaryMap<TValue>
      ? TSummaryMap
      : {}
    : {};

export type ResourceReconcileSummaryPatchScope<TReconcile> =
  [TReconcile] extends [
    ResourceCollectionShape<any, any, any, any, infer TSummaryPatchScope>,
  ]
    ? TSummaryPatchScope extends ResourceSummaryPatchScope
      ? TSummaryPatchScope
      : "line"
    : "line";

export type ResourcePatchForReconcile<
  TValue,
  TItem,
  TReconcile,
  TFamilyKind extends "detail" | "collection" | "paged" = "collection",
> = ReplaceResourcePatch<TValue>
  | ([TReconcile] extends [
      ResourceDetailFields<TValue, infer TFieldMap>,
    ]
      ? DetailFieldResourcePatchUnion<TValue, TFieldMap>
      : never)
  | ([TReconcile] extends [
      ResourceDetailRegions<TValue, infer TRegionMap>,
    ]
      ? DetailRegionResourcePatchUnion<TValue, TRegionMap>
      : never)
  | ([TReconcile] extends [
      ResourceDetailJsonPaths<TValue, infer TPathMap>,
    ]
      ? DetailJsonPathResourcePatchUnion<TValue, TPathMap>
      : never)
  | ([TReconcile] extends [
      ResourceCollectionShape<
        any,
        TItem,
        infer TAspectMap,
        infer TSummaryMap,
        infer TSummaryPatchScope
      >,
    ]
      ? | ItemResourcePatch<TItem>
        | AspectResourcePatchUnion<TItem, TAspectMap>
        | (TFamilyKind extends "paged"
            ? TSummaryPatchScope extends "pageWindow"
              ? SummaryResourcePatchUnion<TValue, TSummaryMap>
              : never
            : SummaryResourcePatchUnion<TValue, TSummaryMap>)
      : never);

export interface ReplacedResourcePatchResult {
  readonly kind: "replaced";
  readonly scope: "line";
  readonly itemId: null;
  readonly aspect: null;
}

export interface NarrowedItemPatchResult {
  readonly kind: "narrowed";
  readonly scope: "item";
  readonly itemId: string;
  readonly aspect: null;
}

export interface NarrowedDetailFieldPatchResult {
  readonly kind: "narrowed";
  readonly scope: "field";
  readonly itemId: null;
  readonly aspect: null;
  readonly field: string;
}

export interface NarrowedDetailRegionPatchResult {
  readonly kind: "narrowed";
  readonly scope: "region";
  readonly itemId: null;
  readonly aspect: null;
  readonly region: string;
}

export interface NarrowedDetailJsonPathPatchResult {
  readonly kind: "narrowed";
  readonly scope: "jsonPath";
  readonly itemId: null;
  readonly aspect: null;
  readonly path: string;
}

export interface NarrowedItemAspectPatchResult {
  readonly kind: "narrowed";
  readonly scope: "aspect";
  readonly itemId: string;
  readonly aspect: string;
}

export interface NarrowedSummaryPatchResult {
  readonly kind: "narrowed";
  readonly scope: "summary";
  readonly itemId: null;
  readonly aspect: null;
  readonly summary: string;
}

export type ResourcePatchResult =
  | ReplacedResourcePatchResult
  | NarrowedDetailFieldPatchResult
  | NarrowedDetailRegionPatchResult
  | NarrowedDetailJsonPathPatchResult
  | NarrowedItemPatchResult
  | NarrowedItemAspectPatchResult
  | NarrowedSummaryPatchResult;

export interface ResourceLineReconciliation<
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<any> = {},
  TFieldMap extends ResourceDetailFieldMap<any> = {},
  TRegionMap extends ResourceDetailRegionMap<any> = {},
  TJsonPathMap extends ResourceDetailJsonPathMap<any> = {},
  TNarrowItem extends boolean = boolean,
  TNarrowField extends boolean = boolean,
  TNarrowRegion extends boolean = boolean,
  TNarrowJsonPath extends boolean = boolean,
  TNarrowSummary extends boolean = boolean,
> {
  readonly broadReplace: true;
  readonly narrowItem: TNarrowItem;
  readonly narrowField: TNarrowField;
  readonly narrowRegion: TNarrowRegion;
  readonly narrowJsonPath: TNarrowJsonPath;
  readonly narrowSummary: TNarrowSummary;
  readonly fieldNames: readonly (keyof TFieldMap & string)[];
  readonly regionNames: readonly (keyof TRegionMap & string)[];
  readonly jsonPathNames: readonly (keyof TJsonPathMap & string)[];
  readonly aspectNames: readonly (keyof TAspectMap & string)[];
  readonly summaryNames: readonly (keyof TSummaryMap & string)[];
}

export interface ResourcePatchFactory {
  replace<TValue>(nextValue: TValue): ReplaceResourcePatch<TValue>;
  field<TField extends string, TValue>(options: {
    field: TField;
    value: TValue;
  }): FieldResourcePatch<TField, TValue>;
  region<TRegion extends string, TValue>(options: {
    region: TRegion;
    value: TValue;
  }): RegionResourcePatch<TRegion, TValue>;
  jsonPath<TPath extends string, TValue>(options: {
    path: TPath;
    value: TValue;
  }): JsonPathResourcePatch<TPath, TValue>;
  item<TItem>(options: {
    itemId: string;
    nextItem: TItem;
  }): ItemResourcePatch<TItem>;
  itemAspect<TAspect extends string, TValue>(options: {
    itemId: string;
    aspect: TAspect;
    value: TValue;
  }): ItemAspectResourcePatch<TAspect, TValue>;
  summary<TSummary extends string, TValue>(options: {
    summary: TSummary;
    value: TValue;
  }): SummaryResourcePatch<TSummary, TValue>;
}

export interface ReplaceResourceDelivery<TValue> {
  readonly kind: "replace"; readonly packetId: string;
  readonly basisId: string | null; readonly nextBasisId: string | null | undefined;
  readonly nextValue: TValue;
  readonly [forgeSignalResourceDeliveryBrand]: "resourceDelivery";
}

export interface PatchResourceDelivery<
  TValue, TItem, TReconcile,
  TFamilyKind extends "detail" | "collection" | "paged" = "collection",
> {
  readonly kind: "patch"; readonly packetId: string;
  readonly basisId: string | null; readonly nextBasisId: string | null | undefined;
  readonly patch: ResourcePatchForReconcile<TValue, TItem, TReconcile, TFamilyKind>;
  readonly [forgeSignalResourceDeliveryBrand]: "resourceDelivery";
}

export interface InvalidateResourceDelivery {
  readonly kind: "invalidate"; readonly packetId: string;
  readonly basisId: string | null; readonly nextBasisId: string | null | undefined;
  readonly [forgeSignalResourceDeliveryBrand]: "resourceDelivery";
}

export interface ExternalReplaceResourceDelivery<TValue> {
  readonly kind: "replace"; readonly packetId: string;
  readonly basisId: string | null; readonly nextBasisId: string | null | undefined;
  readonly nextValue: TValue; readonly version: "forge-resource-external-delivery-v1";
  readonly contract: "basis-compat-v1";
}

export interface ExternalPatchResourceDelivery<
  TValue, TItem, TReconcile,
  TFamilyKind extends "detail" | "collection" | "paged" = "collection",
> {
  readonly kind: "patch"; readonly packetId: string;
  readonly basisId: string | null; readonly nextBasisId: string | null | undefined;
  readonly patch: ResourcePatchForReconcile<TValue, TItem, TReconcile, TFamilyKind>;
  readonly version: "forge-resource-external-delivery-v1";
  readonly contract: "basis-compat-v1";
}

export interface ExternalInvalidateResourceDelivery {
  readonly kind: "invalidate"; readonly packetId: string;
  readonly basisId: string | null; readonly nextBasisId: string | null | undefined;
  readonly version: "forge-resource-external-delivery-v1";
  readonly contract: "basis-compat-v1";
}

export interface ExternalBasisRefreshResourceDelivery {
  readonly kind: "basisRefresh"; readonly packetId: string;
  readonly basisId: string | null; readonly nextBasisId: string;
  readonly version: "forge-resource-external-delivery-v1";
  readonly contract: "basis-compat-v1";
}

export type ResourceDeliveryForReconcile<
  TValue, TItem, TReconcile,
  TFamilyKind extends "detail" | "collection" | "paged" = "collection",
> =
  | ReplaceResourceDelivery<TValue>
  | PatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>
  | InvalidateResourceDelivery
  | ExternalReplaceResourceDelivery<TValue>
  | ExternalPatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>
  | ExternalInvalidateResourceDelivery
  | ExternalBasisRefreshResourceDelivery;

export interface AppliedResourceDeliveryResult {
  readonly kind: "applied"; readonly deliveryKind: "replace" | "patch" | "invalidate";
  readonly scope: "line" | "field" | "region" | "jsonPath" | "item" | "aspect" | "summary" | "invalidate";
  readonly path?: string | null;
  readonly packetId: string; readonly basisId: string | null;
  readonly nextBasisId: string | null;
  readonly supersededOperation: "initialLoad" | "refresh" | "revalidate" | null;
}

export interface DuplicateIgnoredResourceDeliveryResult {
  readonly kind: "duplicateIgnored"; readonly packetId: string;
  readonly deliveryKind: "replace" | "patch" | "invalidate" | "basisRefresh";
}

export interface BasisRejectedResourceDeliveryResult {
  readonly kind: "basisRejected"; readonly packetId: string;
  readonly expectedBasisId: string | null; readonly actualBasisId: string;
}

export interface BasisRefreshedResourceDeliveryResult {
  readonly kind: "basisRefreshed"; readonly packetId: string;
  readonly basisId: string | null; readonly nextBasisId: string | null;
  readonly reloadStatus:
    | import("./resource_lifecycle.js").ResourceLinePendingStatus
    | import("./resource_lifecycle.js").ResourceLineFulfilledStatus
    | import("./resource_lifecycle.js").ResourceLineTimedOutStatus
    | import("./resource_lifecycle.js").ResourceLineRejectedStatus;
}

export type ResourceDeliveryResult =
  | AppliedResourceDeliveryResult
  | DuplicateIgnoredResourceDeliveryResult
  | BasisRejectedResourceDeliveryResult
  | BasisRefreshedResourceDeliveryResult;

export interface ResourceDeliveryFactory {
  replace<TValue>(options: {
    packetId: string; basisId?: string | null;
    nextBasisId?: string | null; nextValue: TValue;
  }): ReplaceResourceDelivery<TValue>;
  patch<
    TValue, TItem, TReconcile,
    TFamilyKind extends "detail" | "collection" | "paged" = "collection",
  >(options: {
    packetId: string; basisId?: string | null; nextBasisId?: string | null;
    patch: ResourcePatchForReconcile<TValue, TItem, TReconcile, TFamilyKind>;
  }): PatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>;
  invalidate(options: {
    packetId: string; basisId?: string | null; nextBasisId?: string | null;
  }): InvalidateResourceDelivery;
}

export interface ResourceExternalDeliveryFactory {
  replace<TValue>(options: {
    packetId: string; basisId?: string | null;
    nextBasisId?: string | null; nextValue: TValue;
  }): ExternalReplaceResourceDelivery<TValue>;
  patch<
    TValue, TItem, TReconcile,
    TFamilyKind extends "detail" | "collection" | "paged" = "collection",
  >(options: {
    packetId: string; basisId?: string | null; nextBasisId?: string | null;
    patch: ResourcePatchForReconcile<TValue, TItem, TReconcile, TFamilyKind>;
  }): ExternalPatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>;
  invalidate(options: {
    packetId: string; basisId?: string | null; nextBasisId?: string | null;
  }): ExternalInvalidateResourceDelivery;
  basisRefresh(options: {
    packetId: string; basisId?: string | null; nextBasisId: string;
  }): ExternalBasisRefreshResourceDelivery;
}

export function resourceItemAspects<
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
>(definitions: TAspectMap): ResourceItemAspects<TItem, TAspectMap>;

export function resourceValueSummaries<
  TValue,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
>(definitions: TSummaryMap): ResourceValueSummaries<TValue, TSummaryMap, "line">;

export namespace resourceValueSummaries {
  function pageWindow<
    TValue,
    TSummaryMap extends ResourceValueSummaryMap<TValue>,
  >(definitions: TSummaryMap): ResourceValueSummaries<TValue, TSummaryMap, "pageWindow">;
}

export function resourceCollectionShape<
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
>(options: {
  items(value: TValue): readonly TItem[];
  replaceItems(value: TValue, nextItems: readonly TItem[]): TValue;
  aspects?: ResourceItemAspects<TItem, TAspectMap>;
  summaries?: ResourceValueSummaries<TValue, TSummaryMap, TSummaryPatchScope>;
}): ResourceCollectionShape<TValue, TItem, TAspectMap, TSummaryMap, TSummaryPatchScope>;

export function resourceDetailFields<
  TValue,
  TFieldMap extends ResourceDetailFieldMap<TValue>,
>(definitions: TFieldMap): ResourceDetailFields<TValue, TFieldMap>;

export function resourceDetailRegions<
  TValue,
  TRegionMap extends ResourceDetailRegionMap<TValue>,
>(definitions: TRegionMap): ResourceDetailRegions<TValue, TRegionMap>;

export function resourceDetailJsonPaths<
  TValue,
  TPathMap extends ResourceDetailJsonPathMap<TValue>,
>(definitions: TPathMap): ResourceDetailJsonPaths<TValue, TPathMap>;

export const resourcePatch: ResourcePatchFactory;
export const resourceDelivery: ResourceDeliveryFactory;
