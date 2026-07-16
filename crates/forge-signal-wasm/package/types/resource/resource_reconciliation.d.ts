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
  ResourceDetailJsonPathDefinitionMap,
  ResourceDetailJsonPathDefinitions,
  ResourceDetailJsonPathMap,
  ResourceDetailJsonPathValue,
  ResourceDetailJsonPaths,
} from "./resource_detail_json_paths.js";
import type { ResourceResponseLensProof } from "./resource_response.js";
import type {
  ResourceDeliveryFactory,
  ResourceDeliveryForReconcile,
  ResourceDeliveryResult,
  ResourceExternalDeliveryFactory,
  ResourceLineReconciliation,
  ResourcePatchFactory,
  ResourcePatchExecutionOptions,
  ResourcePatchExecutionResult,
  ResourcePatchResult,
} from "./resource_patch_delivery_surface.js";
export type {
  AppliedResourceDeliveryResult,
  BasisRefreshedResourceDeliveryResult,
  BasisRejectedResourceDeliveryResult,
  DuplicateIgnoredResourceDeliveryResult,
  ExternalBasisRefreshResourceDelivery,
  ExternalInvalidateResourceDelivery,
  ExternalPatchResourceDelivery,
  ExternalReplaceResourceDelivery,
  NarrowedDetailFieldPatchResult,
  NarrowedDetailJsonPathPatchResult,
  NarrowedDetailRegionPatchResult,
  NarrowedItemAspectPatchResult,
  NarrowedItemPatchResult,
  NarrowedSummaryPatchResult,
  ReplacedResourcePatchResult,
  ResourceDeliveryFactory,
  ResourceDeliveryForReconcile,
  ResourceDeliveryResult,
  ResourceExternalDeliveryFactory,
  ResourceLineReconciliation,
  ResourcePatchFactory,
  ResourcePatchExecutionOptions,
  ResourcePatchExecutionResult,
  ResourcePatchResult,
} from "./resource_patch_delivery_surface.js";

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

export interface DeleteResourcePatch {
  readonly kind: "delete";
  readonly itemId: string;
  readonly [forgeSignalResourcePatchBrand]: "resourcePatch";
}

export interface InsertResourcePatch<TItem> {
  readonly kind: "insert";
  readonly itemId: string;
  readonly placement: "append" | "prepend";
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
  | DeleteResourcePatch
  | InsertResourcePatch<TItem>
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
        | DeleteResourcePatch
        | InsertResourcePatch<TItem>
        | AspectResourcePatchUnion<TItem, TAspectMap>
        | (TFamilyKind extends "paged"
            ? TSummaryPatchScope extends "pageWindow"
              ? SummaryResourcePatchUnion<TValue, TSummaryMap>
              : never
            : SummaryResourcePatchUnion<TValue, TSummaryMap>)
      : never);

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
  TPathMap extends ResourceDetailJsonPathDefinitionMap<TValue>,
>(definitions: TPathMap): ResourceDetailJsonPaths<
  TValue,
  ResourceDetailJsonPathDefinitions<TValue, TPathMap>
>;

export const resourcePatch: ResourcePatchFactory;
export const resourceDelivery: ResourceDeliveryFactory;
