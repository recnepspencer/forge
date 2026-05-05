import type { SignalValue } from "../model.js";
import type { ResourceLine } from "./resource_lifecycle.js";
import type {
  ResourceCollectionShape,
  ResourceDeliveryForReconcile,
  ResourceDeliveryResult,
  ResourceItemAspectMap,
  ResourceLineReconciliation,
  ResourcePatchForReconcile,
  ResourcePatchResult,
  ResourceReconcileAspectMap,
  ResourceReconcileSummaryPatchScope,
  ResourceReconcileSummaryMap,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";

export interface DetailResourceFamily<TParams, TValue> {
  invalidate(params: TParams): boolean;
  invalidateAll(): number;
  line(params: TParams): ResourceLine<TParams, TValue | null>;
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
  invalidate(params: TParams): boolean;
  invalidateAll(): number;
  line(
    params: TParams,
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
  invalidate(params: TParams): boolean;
  invalidateAll(): number;
  line(
    params: TParams,
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
    ResourceReconcileAspectMap<TReconcile>,
    ResourceReconcileSummaryMap<TReconcile>,
    [TReconcile] extends [ResourceCollectionShape<any, any, any, any>] ? true : false,
    TFamilyKind extends "paged"
      ? ResourceReconcileSummaryPatchScope<TReconcile> extends "pageWindow"
        ? [TReconcile] extends [ResourceCollectionShape<any, any, any, any, any>] ? true : false
        : false
      : [TReconcile] extends [ResourceCollectionShape<any, any, any, any, any>] ? true : false
  >;
}
