import type { SignalValue } from "../model.js";
import type {
  ResourceDetailFieldValue,
  ResourceDetailFields,
} from "./resource_detail_fields.js";
import type {
  ResourceDetailRegionValue,
  ResourceDetailRegions,
} from "./resource_detail_regions.js";
import type {
  ResourceDetailJsonPathValue,
  ResourceDetailJsonPaths,
} from "./resource_detail_json_paths.js";
import type {
  InvalidateResourceDelivery,
  PatchResourceDelivery,
  ReplaceResourceDelivery,
  ResourceCollectionShape,
  ResourceItemAspectMap,
  ResourceItemAspectValue,
  ResourcePatchForReconcile,
  ResourceReconcileAspectMap,
  ResourceReconcileSummaryMap,
  ResourceReconcileSummaryPatchScope,
  ResourceValueSummaryValue,
} from "./resource_reconciliation.js";

type ApiFamilyPatchReplaceHelper<TValue> = {
  replace(nextValue: TValue): ResourcePatchForReconcile<TValue, never, undefined>;
};

type ApiFamilyDeliveryBaseOptions = {
  packetId: string;
  basisId?: string | null;
  nextBasisId?: string | null;
};

type ApiFamilyPatchItemHelper<TValue, TItem, TReconcile> = [TReconcile] extends [
  ResourceCollectionShape<any, TItem, any, any>,
]
  ? {
      item(options: {
        itemId: string;
        nextItem: TItem;
      }): ResourcePatchForReconcile<TValue, TItem, TReconcile>;
      delete(options: {
        itemId: string;
      }): ResourcePatchForReconcile<TValue, TItem, TReconcile>;
      insert(options: {
        itemId: string;
        placement: "append" | "prepend";
        nextItem: TItem;
      }): ResourcePatchForReconcile<TValue, TItem, TReconcile>;
    }
  : {};

type ApiFamilyPatchAspectNames<TItem, TReconcile> =
  keyof ResourceReconcileAspectMap<TReconcile> & string;

type ApiFamilyPatchAspectHelper<TValue, TItem, TReconcile> = [ApiFamilyPatchAspectNames<
  TItem,
  TReconcile
>] extends [never]
  ? {}
  : {
      itemAspect<TAspect extends ApiFamilyPatchAspectNames<TItem, TReconcile>>(
        options: {
          itemId: string;
          aspect: TAspect;
          value: ResourceItemAspectValue<
            ResourceReconcileAspectMap<TReconcile>[TAspect]
          >;
        },
      ): ResourcePatchForReconcile<TValue, TItem, TReconcile>;
    };

type ApiFamilyPatchSummaryNames<TValue, TReconcile, TFamilyKind> =
  TFamilyKind extends "paged"
    ? ResourceReconcileSummaryPatchScope<TReconcile> extends "pageWindow"
      ? keyof ResourceReconcileSummaryMap<TReconcile> & string
      : never
    : keyof ResourceReconcileSummaryMap<TReconcile> & string;

type ApiFamilyPatchSummaryHelper<
  TValue,
  TItem,
  TReconcile,
  TFamilyKind extends "collection" | "paged",
> = [ApiFamilyPatchSummaryNames<TValue, TReconcile, TFamilyKind>] extends [never]
  ? {}
  : {
      summary<
        TSummary extends ApiFamilyPatchSummaryNames<TValue, TReconcile, TFamilyKind>,
      >(options: {
        summary: TSummary;
        value: ResourceValueSummaryValue<
          ResourceReconcileSummaryMap<TReconcile>[TSummary]
        >;
      }): ResourcePatchForReconcile<TValue, TItem, TReconcile, TFamilyKind>;
    };

type ApiFamilyPatchFieldNames<TValue, TReconcile> =
  [TReconcile] extends [ResourceDetailFields<TValue, infer TFieldMap>]
    ? keyof TFieldMap & string
    : never;

type ApiFamilyPatchJsonPathNames<TValue, TReconcile> =
  [TReconcile] extends [ResourceDetailJsonPaths<TValue, infer TPathMap>]
    ? keyof TPathMap & string
    : never;

type ApiFamilyPatchRegionNames<TValue, TReconcile> =
  [TReconcile] extends [ResourceDetailRegions<TValue, infer TRegionMap>]
    ? keyof TRegionMap & string
    : never;

type ApiFamilyPatchFieldHelper<TValue, TReconcile> = [ApiFamilyPatchFieldNames<
  TValue,
  TReconcile
>] extends [never]
  ? {}
  : {
      field<TField extends ApiFamilyPatchFieldNames<TValue, TReconcile>>(options: {
        field: TField;
        value: ResourceDetailFieldValue<
          Extract<TReconcile, ResourceDetailFields<TValue, any>>["definitions"][TField]
        >;
      }): ResourcePatchForReconcile<TValue, never, TReconcile, "detail">;
    };

type ApiFamilyPatchRegionHelper<TValue, TReconcile> = [ApiFamilyPatchRegionNames<
  TValue,
  TReconcile
>] extends [never]
  ? {}
  : {
      region<TRegion extends ApiFamilyPatchRegionNames<TValue, TReconcile>>(options: {
        region: TRegion;
        value: ResourceDetailRegionValue<
          Extract<TReconcile, ResourceDetailRegions<TValue, any>>["definitions"][TRegion]
        >;
      }): ResourcePatchForReconcile<TValue, never, TReconcile, "detail">;
    };

type ApiFamilyPatchJsonPathHelper<TValue, TReconcile> = [ApiFamilyPatchJsonPathNames<
  TValue,
  TReconcile
>] extends [never]
  ? {}
  : {
      jsonPath<TPath extends ApiFamilyPatchJsonPathNames<TValue, TReconcile>>(options: {
        path: TPath;
        value: ResourceDetailJsonPathValue<
          Extract<TReconcile, ResourceDetailJsonPaths<TValue, any>>["definitions"][TPath]
        >;
      }): ResourcePatchForReconcile<TValue, never, TReconcile, "detail">;
    };

export type ApiFamilyPatchHelpers<
  TValue,
  TItem,
  TReconcile,
  TFamilyKind extends "detail" | "collection" | "paged",
> =
  & ApiFamilyPatchReplaceHelper<TValue>
  & ApiFamilyPatchFieldHelper<TValue, TReconcile>
  & ApiFamilyPatchRegionHelper<TValue, TReconcile>
  & ApiFamilyPatchJsonPathHelper<TValue, TReconcile>
  & ApiFamilyPatchItemHelper<TValue, TItem, TReconcile>
  & ApiFamilyPatchAspectHelper<TValue, TItem, TReconcile>
  & ApiFamilyPatchSummaryHelper<TValue, TItem, TReconcile, TFamilyKind>;

type ApiFamilyDeliveryReplaceHelper<
  TValue,
  TItem,
  TReconcile,
  TFamilyKind extends "detail" | "collection" | "paged",
> = {
  replace(
    options: ApiFamilyDeliveryBaseOptions & {
      nextValue: TValue;
    },
  ): ReplaceResourceDelivery<TValue>;
  patch(
    options: ApiFamilyDeliveryBaseOptions & {
      patch: ResourcePatchForReconcile<TValue, TItem, TReconcile, TFamilyKind>;
    },
  ): PatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>;
  invalidate(options: ApiFamilyDeliveryBaseOptions): InvalidateResourceDelivery;
};

type ApiFamilyDeliveryItemHelper<
  TValue,
  TItem,
  TReconcile,
  TFamilyKind extends "collection" | "paged",
> = [TReconcile] extends [
  ResourceCollectionShape<any, TItem, any, any>,
]
  ? {
      item(
        options: ApiFamilyDeliveryBaseOptions & {
          itemId: string;
          nextItem: TItem;
        },
      ): PatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>;
      delete(
        options: ApiFamilyDeliveryBaseOptions & {
          itemId: string;
        },
      ): PatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>;
      insert(
        options: ApiFamilyDeliveryBaseOptions & {
          itemId: string;
          placement: "append" | "prepend";
          nextItem: TItem;
        },
      ): PatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>;
    }
  : {};

type ApiFamilyDeliveryAspectHelper<
  TValue,
  TItem,
  TReconcile,
  TFamilyKind extends "collection" | "paged",
> = [ApiFamilyPatchAspectNames<TItem, TReconcile>] extends [never]
  ? {}
  : {
      itemAspect<TAspect extends ApiFamilyPatchAspectNames<TItem, TReconcile>>(
        options: ApiFamilyDeliveryBaseOptions & {
          itemId: string;
          aspect: TAspect;
          value: ResourceItemAspectValue<
            ResourceReconcileAspectMap<TReconcile>[TAspect]
          >;
        },
      ): PatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>;
    };

type ApiFamilyDeliverySummaryHelper<
  TValue,
  TItem,
  TReconcile,
  TFamilyKind extends "collection" | "paged",
> = [ApiFamilyPatchSummaryNames<TValue, TReconcile, TFamilyKind>] extends [never]
  ? {}
  : {
      summary<
        TSummary extends ApiFamilyPatchSummaryNames<TValue, TReconcile, TFamilyKind>,
      >(
        options: ApiFamilyDeliveryBaseOptions & {
          summary: TSummary;
          value: ResourceValueSummaryValue<
            ResourceReconcileSummaryMap<TReconcile>[TSummary]
          >;
        },
      ): PatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>;
    };

type ApiFamilyDeliveryFieldHelper<
  TValue,
  TReconcile,
  TFamilyKind extends "detail" | "collection" | "paged",
> = [ApiFamilyPatchFieldNames<TValue, TReconcile>] extends [never]
  ? {}
  : {
      field<TField extends ApiFamilyPatchFieldNames<TValue, TReconcile>>(
        options: ApiFamilyDeliveryBaseOptions & {
          field: TField;
          value: ResourceDetailFieldValue<
            Extract<TReconcile, ResourceDetailFields<TValue, any>>["definitions"][TField]
          >;
        },
      ): PatchResourceDelivery<TValue, never, TReconcile, TFamilyKind>;
    };

type ApiFamilyDeliveryJsonPathHelper<
  TValue,
  TReconcile,
  TFamilyKind extends "detail" | "collection" | "paged",
> = [ApiFamilyPatchJsonPathNames<TValue, TReconcile>] extends [never]
  ? {}
  : {
      jsonPath<TPath extends ApiFamilyPatchJsonPathNames<TValue, TReconcile>>(
        options: ApiFamilyDeliveryBaseOptions & {
          path: TPath;
          value: ResourceDetailJsonPathValue<
            Extract<TReconcile, ResourceDetailJsonPaths<TValue, any>>["definitions"][TPath]
          >;
        },
      ): PatchResourceDelivery<TValue, never, TReconcile, TFamilyKind>;
    };

type ApiFamilyDeliveryRegionHelper<
  TValue,
  TReconcile,
  TFamilyKind extends "detail" | "collection" | "paged",
> = [ApiFamilyPatchRegionNames<TValue, TReconcile>] extends [never]
  ? {}
  : {
      region<TRegion extends ApiFamilyPatchRegionNames<TValue, TReconcile>>(
        options: ApiFamilyDeliveryBaseOptions & {
          region: TRegion;
          value: ResourceDetailRegionValue<
            Extract<TReconcile, ResourceDetailRegions<TValue, any>>["definitions"][TRegion]
          >;
        },
      ): PatchResourceDelivery<TValue, never, TReconcile, TFamilyKind>;
    };

export type ApiFamilyDeliveryHelpers<
  TValue,
  TItem = SignalValue,
  TReconcile = undefined,
  TFamilyKind extends "detail" | "collection" | "paged" = "collection",
> =
  & ApiFamilyDeliveryReplaceHelper<TValue, TItem, TReconcile, TFamilyKind>
  & ApiFamilyDeliveryFieldHelper<TValue, TReconcile, TFamilyKind>
  & ApiFamilyDeliveryRegionHelper<TValue, TReconcile, TFamilyKind>
  & ApiFamilyDeliveryJsonPathHelper<TValue, TReconcile, TFamilyKind>
  & ApiFamilyDeliveryItemHelper<TValue, TItem, TReconcile, TFamilyKind>
  & ApiFamilyDeliveryAspectHelper<TValue, TItem, TReconcile, TFamilyKind>
  & ApiFamilyDeliverySummaryHelper<TValue, TItem, TReconcile, TFamilyKind>;
