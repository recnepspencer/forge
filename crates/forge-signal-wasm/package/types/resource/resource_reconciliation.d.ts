import type { SignalValue } from "../model.js";

declare const forgeSignalResourceItemAspectsBrand: unique symbol;
declare const forgeSignalResourceCollectionShapeBrand: unique symbol;
declare const forgeSignalResourcePatchBrand: unique symbol;
declare const forgeSignalResourceValueSummariesBrand: unique symbol;

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
> {
  readonly definitions: TSummaryMap;
  readonly [forgeSignalResourceValueSummariesBrand]: "resourceValueSummaries";
}

export interface ResourceCollectionShape<
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
> {
  items(value: TValue): readonly TItem[];
  replaceItems(value: TValue, nextItems: readonly TItem[]): TValue;
  readonly aspects: ResourceItemAspects<TItem, TAspectMap> | null;
  readonly summaries: ResourceValueSummaries<TValue, TSummaryMap> | null;
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
    ResourceCollectionShape<infer TValue, any, any, infer TSummaryMap>,
  ]
    ? TSummaryMap extends ResourceValueSummaryMap<TValue>
      ? TSummaryMap
      : {}
    : {};

export type ResourcePatchForReconcile<
  TValue,
  TItem,
  TReconcile,
> = ReplaceResourcePatch<TValue>
  | ([TReconcile] extends [
      ResourceCollectionShape<any, TItem, infer TAspectMap, infer TSummaryMap>,
    ]
      ? | ItemResourcePatch<TItem>
        | AspectResourcePatchUnion<TItem, TAspectMap>
        | SummaryResourcePatchUnion<TValue, TSummaryMap>
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

export function resourceItemAspects<
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
>(definitions: TAspectMap): ResourceItemAspects<TItem, TAspectMap>;

export function resourceValueSummaries<
  TValue,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
>(definitions: TSummaryMap): ResourceValueSummaries<TValue, TSummaryMap>;

export function resourceCollectionShape<
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
>(options: {
  items(value: TValue): readonly TItem[];
  replaceItems(value: TValue, nextItems: readonly TItem[]): TValue;
  aspects?: ResourceItemAspects<TItem, TAspectMap>;
  summaries?: ResourceValueSummaries<TValue, TSummaryMap>;
}): ResourceCollectionShape<TValue, TItem, TAspectMap, TSummaryMap>;

export const resourcePatch: ResourcePatchFactory;
