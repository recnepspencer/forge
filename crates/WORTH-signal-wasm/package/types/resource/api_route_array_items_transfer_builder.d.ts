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
import type { ApiRouteArrayItemsBuilder } from "./api_route_array_items_builder.js";

export type ApiRouteArrayItemsTransferBuilderStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<any>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
> = ApiRouteCollectionTransferStep<
  TRoute,
  TRequestParams,
  TProcessingKind,
  TUploadKind,
  ApiRouteArrayItemsBuilder<
    TRoute,
    TRequestParams,
    TItem,
    TAspectMap,
    TSummaryMap,
    TSummaryMode,
    TProcessingKind,
    "signed"
  >,
  ApiRouteArrayItemsBuilder<
    TRoute,
    TRequestParams,
    TItem,
    TAspectMap,
    TSummaryMap,
    TSummaryMode,
    TProcessingKind,
    "multipart"
  >,
  ApiRouteArrayItemsBuilder<
    TRoute,
    TRequestParams,
    TItem,
    TAspectMap,
    TSummaryMap,
    TSummaryMode,
    "poll",
    TUploadKind
  >,
  ApiRouteArrayItemsBuilder<
    TRoute,
    TRequestParams,
    TItem,
    TAspectMap,
    TSummaryMap,
    TSummaryMode,
    "callback",
    TUploadKind
  >,
  ApiRouteArrayItemsBuilder<
    TRoute,
    TRequestParams,
    TItem,
    TAspectMap,
    TSummaryMap,
    TSummaryMode,
    "webhook",
    TUploadKind
  >
>;
