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

export interface DetailResourceFamily<TParams, TValue> {
  invalidate<TActualParams extends TParams>(
    params: ExactResourceParams<TParams, TActualParams>,
  ): boolean;
  invalidateAll(): number;
  line<TActualParams extends TParams>(
    params: ExactResourceParams<TParams, TActualParams>,
  ): ResourceLine<TParams, TValue | null>;
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
    [TReconcile] extends [ResourceCollectionShape<any, any, any, any>] ? true : false,
    TFamilyKind extends "paged"
      ? ResourceReconcileSummaryPatchScope<TReconcile> extends "pageWindow"
        ? [TReconcile] extends [ResourceCollectionShape<any, any, any, any, any>] ? true : false
        : false
      : [TReconcile] extends [ResourceCollectionShape<any, any, any, any, any>] ? true : false
  >;
}
