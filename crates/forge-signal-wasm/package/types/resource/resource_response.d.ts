import type {
  ResourceItemAspect,
  ResourceItemAspectMap,
  ResourceItemAspects,
} from "./resource_reconciliation.js";

declare const forgeSignalResourceCollectionResponseBrand: unique symbol;

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
> extends ResourceCollectionResponse<readonly TItem[], TItem, TAspectMap> {
}

export interface ResourceCollectionResponse<
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
> {
  readonly kind: "collection";
  itemIdentity(item: TItem): string;
  items(value: TValue): readonly TItem[];
  replaceItems(value: TValue, nextItems: readonly TItem[]): TValue;
  readonly aspects: ResourceItemAspects<TItem, TAspectMap> | null;
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
  TResponse extends ResourceCollectionResponse<any, infer TItem, infer TAspectMap>
    ? TAspectMap extends ResourceItemAspectMap<TItem>
      ? TAspectMap
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
  >(options: {
    itemId(item: TItem): string;
    aspects?: ResourceItemAspects<TItem, TAspectMap>;
  }): ResourceArrayResponse<TItem, TAspectMap>;
  collection<
    TValue,
    TItem,
    TAspectMap extends ResourceItemAspectMap<TItem> = {},
  >(options: {
    itemId(item: TItem): string;
    items(value: TValue): readonly TItem[];
    replaceItems(value: TValue, nextItems: readonly TItem[]): TValue;
    aspects?: ResourceItemAspects<TItem, TAspectMap>;
  }): ResourceCollectionResponse<TValue, TItem, TAspectMap>;
  collection<TValue>(): <
    TItem,
    TAspectMap extends ResourceItemAspectMap<TItem> = {},
  >(options: {
    itemId(item: TItem): string;
    items(value: TValue): readonly TItem[];
    replaceItems(value: TValue, nextItems: readonly TItem[]): TValue;
    aspects?: ResourceItemAspects<TItem, TAspectMap>;
  }) => ResourceCollectionResponse<TValue, TItem, TAspectMap>;
  objectItems<TValue>(): <
    TField extends ResourceObjectArrayFieldName<TValue>,
    TItem extends ResourceObjectArrayFieldItem<TValue, TField>,
    TAspectMap extends ResourceItemAspectMap<TItem> = {},
  >(options: {
    field: TField;
    itemId(item: TItem): string;
    aspects?: ResourceItemAspects<TItem, TAspectMap>;
  }) => ResourceCollectionResponse<TValue, TItem, TAspectMap>;
}

export const resourceResponse: ResourceResponseFactory;
