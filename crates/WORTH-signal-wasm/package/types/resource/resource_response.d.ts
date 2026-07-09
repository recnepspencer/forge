import type { SignalValue } from "../model.js";
import type {
  ResourceDetailFieldMap,
  ResourceDetailFields,
  ResourceDetailObjectFieldDefinitions,
  ResourceDetailObjectFieldMap,
} from "./resource_detail_fields.js";
import type {
  ResourceDetailRegionMap,
  ResourceDetailRegions,
} from "./resource_detail_regions.js";
import type {
  ResourceDetailJsonPathDeclaration,
  ResourceDetailJsonPathDeclarationMap,
  ResourceDetailJsonPathDefinitions,
  ResourceDetailJsonPathMap,
  ResourceDetailJsonPaths,
} from "./resource_detail_json_paths.js";
import type {
  ResourceItemAspect,
  ResourceItemAspectMap,
  ResourceItemAspects,
  ResourceValueSummaries,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";
import type { ResourceResponseFactory } from "./resource_response_factory.js";
import type { ResourceResponseLensProof } from "./resource_response_lens_proof.js";
export type { ResourceResponseFactory } from "./resource_response_factory.js";
export type {
  ResourceResponseLensCapabilityRow,
  ResourceResponseLensDenialProof,
  ResourceResponseLensLocus,
  ResourceResponseLensProof,
  ResourceResponseLensTopology,
} from "./resource_response_lens_proof.js";

declare const WORTHSignalResourceCollectionResponseBrand: unique symbol;
declare const WORTHSignalResourceDetailResponseBrand: unique symbol;
declare const WORTHSignalResourceSummaryResponseBrand: unique symbol;

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

export type ResourceJsonPathSegments<TValue> =
  TValue extends readonly (infer TItem)[]
    ? readonly [number] | readonly [number, ...ResourceJsonPathSegments<TItem>]
    : TValue extends object
      ? {
          readonly [TField in keyof TValue & string]:
            | readonly [TField]
            | readonly [TField, ...ResourceJsonPathSegments<TValue[TField]>];
        }[keyof TValue & string]
      : never;

export type ResourceJsonPathAspectDeclaration<TItem> = {
  readonly [TField in keyof TItem & string]: {
    readonly field: TField;
    readonly path: ResourceJsonPathSegments<TItem[TField]>;
    readonly presence?: "required" | "optional";
  };
}[keyof TItem & string];

export type ResourceJsonPathAspectDeclarationMap<TItem> = Readonly<
  Record<string, ResourceJsonPathAspectDeclaration<TItem>>
>;

export type ResourceJsonPathAspectDefinitions<
  TItem,
  TValueMap extends Readonly<Record<string, SignalValue>>,
> = {
  readonly [TAspect in keyof TValueMap & string]: ResourceItemAspect<
    TItem,
    TValueMap[TAspect]
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

export interface ResourceEntityStoreResponse<
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
> extends ResourceCollectionResponse<
  TValue,
  TItem,
  TAspectMap,
  TSummaryMap
> {
}

export interface ResourceMapCollectionResponse<
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
> extends ResourceCollectionResponse<
  TValue,
  TItem,
  TAspectMap,
  TSummaryMap
> {
}

export interface ResourceConnectionResponse<
  TValue,
  TEdge,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
> extends ResourceCollectionResponse<TValue, TItem, TAspectMap, TSummaryMap> {}

export interface ResourceDiscriminatedTupleResponse<
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
> extends ResourceCollectionResponse<TValue, TItem, TAspectMap, TSummaryMap> {}

export interface ResourceGroupedCollectionResponse<
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
> extends ResourceCollectionResponse<TValue, TItem, TAspectMap, TSummaryMap> {}

export interface ResourceSparsePageResponse<
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
> extends ResourceCollectionResponse<TValue, TItem, TAspectMap, TSummaryMap> {}

export interface ResourceNamedCollectionResponse<
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
> extends ResourceCollectionResponse<TValue, TItem, TAspectMap, TSummaryMap> {}

export interface ResourceTreeResponse<
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
> extends ResourceCollectionResponse<TValue, TItem, TAspectMap, TSummaryMap> {}

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
  readonly [WORTHSignalResourceCollectionResponseBrand]: "resourceCollectionResponse";
}

export interface ResourceDetailResponse<
  TValue,
  TFieldMap extends ResourceDetailFieldMap<TValue> = {},
  TRegionMap extends ResourceDetailRegionMap<TValue> = {},
  TJsonPathMap extends ResourceDetailJsonPathMap<TValue> = {},
> {
  readonly fields?: ResourceDetailFields<TValue, TFieldMap> | null;
  readonly regions?: ResourceDetailRegions<TValue, TRegionMap> | null;
  readonly jsonPaths?: ResourceDetailJsonPaths<TValue, TJsonPathMap> | null;
  readonly kind: "detail";
  readonly lensProof: ResourceResponseLensProof;
  readonly [WORTHSignalResourceDetailResponseBrand]: "resourceDetailResponse";
}

export interface ResourceSummaryResponse<TValue> {
  readonly kind: "summary";
  readonly lensProof: ResourceResponseLensProof;
  readonly [WORTHSignalResourceSummaryResponseBrand]: "resourceSummaryResponse";
}

export type ResourceAnyResponse =
  | ResourceCollectionResponse<any, any, any, any>
  | ResourceDetailResponse<any, any, any>
  | ResourceSummaryResponse<any>;

export type ResourceResponseValue<TResponse> =
  TResponse extends ResourceCollectionResponse<infer TValue, any, any>
    ? TValue
    : TResponse extends ResourceDetailResponse<infer TValue, any, any>
      ? TValue
    : TResponse extends ResourceSummaryResponse<infer TValue>
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

export type ResourceResponseDetailFieldMap<TResponse> =
  TResponse extends ResourceDetailResponse<any, infer TFieldMap, any, any>
    ? TFieldMap
    : {};

export type ResourceResponseDetailRegionMap<TResponse> =
  TResponse extends ResourceDetailResponse<any, any, infer TRegionMap, any>
    ? TRegionMap
    : {};

export type ResourceResponseDetailJsonPathMap<TResponse> =
  TResponse extends ResourceDetailResponse<any, any, any, infer TJsonPathMap>
    ? TJsonPathMap
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

export const resourceResponse: ResourceResponseFactory;

export type {
  ResourceDetailFieldMap,
  ResourceDetailFields,
  ResourceDetailRegionMap,
  ResourceDetailRegions,
  ResourceDetailJsonPathDeclaration,
  ResourceDetailJsonPathDeclarationMap,
  ResourceDetailJsonPathDefinitions,
  ResourceDetailJsonPathMap,
  ResourceDetailJsonPaths,
  ResourceDetailObjectFieldDefinitions,
  ResourceDetailObjectFieldMap,
};
