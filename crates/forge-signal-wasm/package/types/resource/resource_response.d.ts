import type {
  ResourceItemAspect,
  ResourceItemAspectMap,
  ResourceItemAspects,
  ResourceValueSummaries,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";

declare const forgeSignalResourceCollectionResponseBrand: unique symbol;
declare const forgeSignalResourceResponseLensProofBrand: unique symbol;

export interface ResourceResponseLensCapabilityRow {
  readonly locus: "broadResponse" | "membership" | "itemAspect" | "summary";
  readonly patchScope: "line" | "item" | "aspect" | "summary";
  readonly admitted: boolean;
  readonly summaryPatchScope: "line" | "pageWindow" | null;
}

export interface ResourceResponseLensProof {
  readonly version: "resource-response-lens-proof-v1";
  readonly source: string;
  readonly topology: "directArray" | "objectItems" | "customCollection";
  readonly itemField: string | null;
  readonly capabilityRows: readonly ResourceResponseLensCapabilityRow[];
  readonly aspectNames: readonly string[];
  readonly summaryNames: readonly string[];
  readonly summaryPatchScope: "line" | "pageWindow" | null;
  readonly [forgeSignalResourceResponseLensProofBrand]: "resourceResponseLensProof";
}

export type ResourceObjectAspectFieldMap<TItem> = Readonly<
  Record<string, keyof TItem & string>
>;

export type ResourceObjectAspectDefinitions<
  TItem,
  TFields extends ResourceObjectAspectFieldMap<TItem>,
> = {
  readonly [TAspect in keyof TFields & string]: ResourceItemAspect<
    TItem,
    TItem[TFields[TAspect]]
  >;
};

export interface ResourceArrayResponse<
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<readonly TItem[]> = {},
> extends ResourceCollectionResponse<
  readonly TItem[],
  TItem,
  TAspectMap,
  TSummaryMap
> {
}

export interface ResourceCollectionResponse<
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
> {
  readonly kind: "collection";
  itemIdentity(item: TItem): string;
  items(value: TValue): readonly TItem[];
  replaceItems(value: TValue, nextItems: readonly TItem[]): TValue;
  readonly aspects: ResourceItemAspects<TItem, TAspectMap> | null;
  readonly summaries: ResourceValueSummaries<TValue, TSummaryMap, any> | null;
  readonly lensProof: ResourceResponseLensProof;
  readonly [forgeSignalResourceCollectionResponseBrand]: "resourceCollectionResponse";
}

export type ResourceResponseValue<TResponse> =
  TResponse extends ResourceCollectionResponse<infer TValue, any, any>
    ? TValue
    : never;

export type ResourceResponseItem<TResponse> =
  TResponse extends ResourceCollectionResponse<any, infer TItem, any>
    ? TItem
    : never;

export type ResourceResponseAspectMap<TResponse> =
  TResponse extends ResourceCollectionResponse<any, infer TItem, infer TAspectMap, any>
    ? TAspectMap extends ResourceItemAspectMap<TItem>
      ? TAspectMap
      : {}
    : {};

export type ResourceResponseSummaryMap<TResponse> =
  TResponse extends ResourceCollectionResponse<infer TValue, any, any, infer TSummaryMap>
    ? TSummaryMap extends ResourceValueSummaryMap<TValue>
      ? TSummaryMap
      : {}
    : {};

export type ResourceArrayResponseItem<TResponse> =
  ResourceResponseItem<TResponse>;

export type ResourceArrayResponseAspectMap<TResponse> =
  ResourceResponseAspectMap<TResponse>;

export type ResourceObjectArrayFieldName<TValue> = {
  [TField in keyof TValue & string]: TValue[TField] extends readonly unknown[]
    ? TField
    : never;
}[keyof TValue & string];

export type ResourceObjectArrayFieldItem<
  TValue,
  TField extends keyof TValue & string,
> = TValue[TField] extends readonly (infer TItem)[] ? TItem : never;

export interface ResourceResponseFactory {
  objectAspects<TItem>(): <
    TFields extends ResourceObjectAspectFieldMap<TItem>,
  >(
    fields: TFields,
  ) => ResourceItemAspects<
    TItem,
    ResourceObjectAspectDefinitions<TItem, TFields>
  >;
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
}

export const resourceResponse: ResourceResponseFactory;
