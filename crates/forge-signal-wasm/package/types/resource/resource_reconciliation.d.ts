import type { SignalValue } from "../model.js";
import type { ResourceResponseLensProof } from "./resource_response.js";

declare const forgeSignalResourceItemAspectsBrand: unique symbol;
declare const forgeSignalResourceCollectionShapeBrand: unique symbol;
declare const forgeSignalResourcePatchBrand: unique symbol;
declare const forgeSignalResourceValueSummariesBrand: unique symbol;
declare const forgeSignalResourceDeliveryBrand: unique symbol;

export type ResourceSummaryPatchScope = "line" | "pageWindow";

export interface ResourceItemAspect<TItem, TValue = SignalValue> {
  read(item: TItem): TValue;
  write(item: TItem, value: TValue): TItem;
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

export type ResourcePatch<
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
> =
  | ReplaceResourcePatch<TValue>
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
  TFamilyKind extends "collection" | "paged" = "collection",
> = ReplaceResourcePatch<TValue>
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
  | NarrowedItemPatchResult
  | NarrowedItemAspectPatchResult
  | NarrowedSummaryPatchResult;

export interface ResourceLineReconciliation<
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<any> = {},
  TNarrowItem extends boolean = boolean,
  TNarrowSummary extends boolean = boolean,
> {
  readonly broadReplace: true;
  readonly narrowItem: TNarrowItem;
  readonly narrowSummary: TNarrowSummary;
  readonly aspectNames: readonly (keyof TAspectMap & string)[];
  readonly summaryNames: readonly (keyof TSummaryMap & string)[];
}

export interface ResourcePatchFactory {
  replace<TValue>(nextValue: TValue): ReplaceResourcePatch<TValue>;
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
  TFamilyKind extends "collection" | "paged" = "collection",
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
  TFamilyKind extends "collection" | "paged" = "collection",
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
  TFamilyKind extends "collection" | "paged" = "collection",
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
  readonly scope: "line" | "item" | "aspect" | "summary" | "invalidate";
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
    TFamilyKind extends "collection" | "paged" = "collection",
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
    TFamilyKind extends "collection" | "paged" = "collection",
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

export const resourcePatch: ResourcePatchFactory;
export const resourceDelivery: ResourceDeliveryFactory;
