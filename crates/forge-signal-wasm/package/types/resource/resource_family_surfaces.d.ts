import type { SignalValue } from "../model.js";
import type { ResourceLine } from "./resource_lifecycle.js";
import type {
  ResourceCollectionShape,
  ResourceItemAspectMap,
  ResourceLineReconciliation,
  ResourcePatchForReconcile,
  ResourcePatchResult,
  ResourceReconcileAspectMap,
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
    ResourceValueSummaryMap<TValue>
  > | undefined = undefined,
> {
  invalidate(params: TParams): boolean;
  invalidateAll(): number;
  line(
    params: TParams,
  ): ResourcePatchCapableLine<TParams, TValue, TItem, TReconcile>;
}

export interface PagedResourceFamily<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>
  > | undefined = undefined,
> {
  invalidate(params: TParams): boolean;
  invalidateAll(): number;
  line(
    params: TParams,
  ): ResourcePatchCapableLine<TParams, TValue, TItem, TReconcile>;
}

export interface ResourcePatchCapableLine<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>
  > | undefined = undefined,
> extends ResourceLine<TParams, TValue | null> {
  patch(
    patch: ResourcePatchForReconcile<TValue, TItem, TReconcile>,
  ): ResourcePatchResult;
  reconciliation(): ResourceLineReconciliation<
    TItem,
    ResourceReconcileAspectMap<TReconcile>,
    ResourceReconcileSummaryMap<TReconcile>,
    [TReconcile] extends [ResourceCollectionShape<any, any, any, any>] ? true : false,
    [TReconcile] extends [ResourceCollectionShape<any, any, any, any>] ? true : false
  >;
}
