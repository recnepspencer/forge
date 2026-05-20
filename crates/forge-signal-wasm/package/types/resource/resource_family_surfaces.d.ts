import type { SignalValue } from "../model.js";
import type { ResourceLine } from "./resource_lifecycle.js";
import type {
  ResourceDetailFieldMap,
  ResourceDetailFields,
} from "./resource_detail_fields.js";
import type {
  ResourceDetailRegionMap,
  ResourceDetailRegions,
} from "./resource_detail_regions.js";
import type {
  ResourceDetailJsonPathMap,
  ResourceDetailJsonPaths,
} from "./resource_detail_json_paths.js";
import type {
  ResourceCollectionShape,
  ResourceDeliveryForReconcile,
  ResourceDeliveryResult,
  ResourceDetailReconcile,
  ResourceItemAspectMap,
  ResourceLineReconciliation,
  ResourcePatchForReconcile,
  ResourcePatchResult,
  ResourceReconcileAspectMap,
  ResourceReconcileSummaryPatchScope,
  ResourceReconcileSummaryMap,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";

type ExactResourceParams<TExpected, TActual extends TExpected> =
  Exclude<keyof TActual, keyof TExpected> extends never ? TActual : never;

type ResourceLineAspectMapFor<
  TItem,
  TReconcile,
> = ResourceReconcileAspectMap<TReconcile> extends ResourceItemAspectMap<TItem>
  ? ResourceReconcileAspectMap<TReconcile>
  : {};

type ResourceLineSummaryMapFor<TReconcile> =
  ResourceReconcileSummaryMap<TReconcile> extends ResourceValueSummaryMap<any>
    ? ResourceReconcileSummaryMap<TReconcile>
    : {};

type ResourceLineFieldMapFor<TValue, TReconcile> =
  [TReconcile] extends [ResourceDetailFields<TValue, infer TFieldMap>]
    ? TFieldMap extends ResourceDetailFieldMap<TValue>
      ? TFieldMap
      : {}
    : {};

type ResourceLineJsonPathMapFor<TValue, TReconcile> =
  [TReconcile] extends [ResourceDetailJsonPaths<TValue, infer TPathMap>]
    ? TPathMap extends ResourceDetailJsonPathMap<TValue>
      ? TPathMap
      : {}
    : {};

type ResourceLineRegionMapFor<TValue, TReconcile> =
  [TReconcile] extends [ResourceDetailRegions<TValue, infer TRegionMap>]
    ? TRegionMap extends ResourceDetailRegionMap<TValue>
      ? TRegionMap
      : {}
    : {};

export interface DetailResourceFamily<
  TParams,
  TValue,
  TReconcile extends ResourceDetailReconcile<TValue> | undefined = undefined,
> {
  invalidate<TActualParams extends TParams>(
    params: ExactResourceParams<TParams, TActualParams>,
  ): boolean;
  invalidateAll(): number;
  line<TActualParams extends TParams>(
    params: ExactResourceParams<TParams, TActualParams>,
  ): ResourceDetailPatchCapableLine<TParams, TValue, TReconcile>;
}

export interface CollectionResourceFamily<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>,
    any
  > | undefined = undefined,
> {
  invalidate<TActualParams extends TParams>(
    params: ExactResourceParams<TParams, TActualParams>,
  ): boolean;
  invalidateAll(): number;
  line<TActualParams extends TParams>(
    params: ExactResourceParams<TParams, TActualParams>,
  ): ResourcePatchCapableLine<TParams, TValue, TItem, TReconcile, "collection">;
}

export interface PagedResourceFamily<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>,
    any
  > | undefined = undefined,
> {
  invalidate<TActualParams extends TParams>(
    params: ExactResourceParams<TParams, TActualParams>,
  ): boolean;
  invalidateAll(): number;
  line<TActualParams extends TParams>(
    params: ExactResourceParams<TParams, TActualParams>,
  ): ResourcePatchCapableLine<TParams, TValue, TItem, TReconcile, "paged">;
}

export interface ResourcePatchCapableLine<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>,
    any
  > | undefined = undefined,
  TFamilyKind extends "collection" | "paged" = "collection",
> extends ResourceLine<TParams, TValue | null> {
  patch(
    patch: ResourcePatchForReconcile<TValue, TItem, TReconcile, TFamilyKind>,
  ): ResourcePatchResult;
  deliver(
    packet: ResourceDeliveryForReconcile<TValue, TItem, TReconcile, TFamilyKind>,
  ): ResourceDeliveryResult;
  reconciliation(): ResourceLineReconciliation<
    TItem,
    ResourceLineAspectMapFor<TItem, TReconcile>,
    ResourceLineSummaryMapFor<TReconcile>,
    {},
    {},
    {},
    [TReconcile] extends [ResourceCollectionShape<any, any, any, any>] ? true : false,
    false,
    TFamilyKind extends "paged"
      ? ResourceReconcileSummaryPatchScope<TReconcile> extends "pageWindow"
        ? [TReconcile] extends [ResourceCollectionShape<any, any, any, any, any>] ? true : false
        : false
      : [TReconcile] extends [ResourceCollectionShape<any, any, any, any, any>] ? true : false
  >;
}

export interface ResourceDetailPatchCapableLine<
  TParams,
  TValue,
  TReconcile extends ResourceDetailReconcile<TValue> | undefined = undefined,
> extends ResourceLine<TParams, TValue | null> {
  patch(
    patch: ResourcePatchForReconcile<TValue, never, TReconcile, "detail">,
  ): ResourcePatchResult;
  deliver(
    packet: ResourceDeliveryForReconcile<TValue, never, TReconcile, "detail">,
  ): ResourceDeliveryResult;
  reconciliation(): ResourceLineReconciliation<
    never,
    {},
    {},
    ResourceLineFieldMapFor<TValue, TReconcile>,
    ResourceLineRegionMapFor<TValue, TReconcile>,
    ResourceLineJsonPathMapFor<TValue, TReconcile>,
    false,
    [TReconcile] extends [ResourceDetailFields<TValue, any>] ? true : false,
    [TReconcile] extends [ResourceDetailRegions<TValue, any>] ? true : false,
    [TReconcile] extends [ResourceDetailJsonPaths<TValue, any>] ? true : false,
    false
  >;
}
