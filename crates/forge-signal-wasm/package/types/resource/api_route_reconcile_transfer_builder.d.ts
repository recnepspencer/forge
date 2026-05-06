import type {
  ResourceItemAspectMap,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";
import type { ApiRequestParamsShape } from "./api_request_params.js";
import type {
  ApiRouteProcessingKind,
  ApiRouteUploadKind,
} from "./api_route_transfer_kinds.js";
import type { ApiRouteCollectionTransferStep } from "./api_route_collection_transfer_steps.js";
import type { ApiRouteArrayItemsSummaryMode } from "./api_route_array_items_common.js";
import type { ApiRouteReconcileBuilder } from "./api_route_reconcile_builder.js";

export type ApiRouteReconcileTransferBuilderStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
> = ApiRouteCollectionTransferStep<
  TRoute,
  TRequestParams,
  TProcessingKind,
  TUploadKind,
  ApiRouteReconcileBuilder<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    TSummaryMode,
    TProcessingKind,
    "signed"
  >,
  ApiRouteReconcileBuilder<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    TSummaryMode,
    TProcessingKind,
    "multipart"
  >,
  ApiRouteReconcileBuilder<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    TSummaryMode,
    "poll",
    TUploadKind
  >,
  ApiRouteReconcileBuilder<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    TSummaryMode,
    "callback",
    TUploadKind
  >,
  ApiRouteReconcileBuilder<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    TSummaryMode,
    "webhook",
    TUploadKind
  >
>;
