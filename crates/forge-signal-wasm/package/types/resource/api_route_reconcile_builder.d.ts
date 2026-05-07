import type {
  ResourceItemAspect,
  ResourceItemAspectMap,
  ResourceValueSummary,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";
import type {
  ApiRequestParamsShape,
  ApiRouteLineParams,
  ApiRouteParamsConstraint,
} from "./api_request_params.js";
import type {
  ApiRouteProcessingKind,
  ApiRouteUploadKind,
} from "./api_route_transfer_kinds.js";
import type {
  CallbackProcessingJobOptions,
  DirectMultipartUploadTransportOptions,
  ResourceRequestMethod,
  SignedUploadTransportOptions,
  WebhookProcessingJobOptions,
} from "./resource_postures.js";
import type { ApiRouteDownloadsDeclaration } from "./api_route_downloads.js";
import type {
  ApiRouteArrayItemsSummaryMode,
  ApiRouteUnusedDefinitionName,
} from "./api_route_array_items_common.js";
import type {
  ApiRouteReconcileBuilderBaseNone,
  ApiRouteReconcileBuilderBaseProcessing,
  ApiRouteReconcileBuilderBaseProcessingUpload,
  ApiRouteReconcileBuilderBaseUpload,
} from "./api_route_reconcile_builder_base.js";

type ApiRouteReconcileBuilderParamsStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TMethod extends ResourceRequestMethod | undefined,
> = [TRequestParams] extends [undefined]
  ? ApiRouteParamsConstraint<TRoute> extends { readonly __forgeInvalidApiRequestParams__: string }
    ? {}
    : {
        params<TNextRequestParams extends ApiRequestParamsShape>(): ApiRouteReconcileBuilder<TRoute, TNextRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;
      }
  : {};

type ApiRouteReconcileBuilderAspectStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TMethod extends ResourceRequestMethod | undefined,
> = {
  aspect<TName extends string, TAspectValue>(name: ApiRouteUnusedDefinitionName<TName, TAspectMap>, read: (item: TItem) => TAspectValue, write: (item: TItem, value: TAspectValue) => TItem): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap & Record<TName, ResourceItemAspect<TItem, TAspectValue>>, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;
};

type ApiRouteReconcileBuilderSummaryStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TMethod extends ResourceRequestMethod | undefined,
> = TSummaryMode extends "pageWindow"
  ? {}
  : {
      summary<TName extends string, TSummaryValue>(name: ApiRouteUnusedDefinitionName<TName, TSummaryMap>, read: (value: TValue) => TSummaryValue, write: (value: TValue, summary: TSummaryValue) => TValue): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap & Record<TName, ResourceValueSummary<TValue, TSummaryValue>>, "line", TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    };

type ApiRouteReconcileBuilderPageWindowSummaryStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TMethod extends ResourceRequestMethod | undefined,
> = TSummaryMode extends "line"
  ? {}
  : {
      pageWindowSummary<TName extends string, TSummaryValue>(name: ApiRouteUnusedDefinitionName<TName, TSummaryMap>, read: (value: TValue) => TSummaryValue, write: (value: TValue, summary: TSummaryValue) => TValue): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap & Record<TName, ResourceValueSummary<TValue, TSummaryValue>>, "pageWindow", TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    };

type ApiRouteReconcileBuilderRequestShapeStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TMethod extends ResourceRequestMethod | undefined,
> = (TDownloadsOwned extends true
  ? {}
  : {
      downloads(
        declaration: ApiRouteDownloadsDeclaration<
          ApiRouteLineParams<TRoute, TRequestParams, TBody>,
          TValue
        >,
      ): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, true, THeadersOwned, TMethod>;
    }) &
  (THeadersOwned extends true ? {} : {
    headers(headers: Record<string, string> | ((params: ApiRouteLineParams<TRoute, TRequestParams, TBody>) => Record<string, string>)): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, true, TMethod>;
  }) &
  ([TBody] extends [undefined] ? {
    body<TNextBody>(): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TNextBody, TDownloadsOwned, THeadersOwned, TMethod>;
  } : {}) &
  ([TMethod] extends [undefined] ? {
    verb<TNextMethod extends ResourceRequestMethod>(method: TNextMethod): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TNextMethod>;
  } : {});

type ApiRouteReconcileBuilderCommonSteps<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TMethod extends ResourceRequestMethod | undefined,
> = ApiRouteReconcileBuilderParamsStep<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>
  & ApiRouteReconcileBuilderAspectStep<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>
  & ApiRouteReconcileBuilderSummaryStep<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>
  & ApiRouteReconcileBuilderPageWindowSummaryStep<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>
  & ApiRouteReconcileBuilderRequestShapeStep<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;

type ApiRouteReconcileBuilderNone<TRoute extends string, TRequestParams extends ApiRequestParamsShape | undefined, TValue, TItem, TAspectMap extends ResourceItemAspectMap<TItem>, TSummaryMap extends ResourceValueSummaryMap<TValue>, TSummaryMode extends ApiRouteArrayItemsSummaryMode, TBody, TDownloadsOwned extends boolean, THeadersOwned extends boolean, TMethod extends ResourceRequestMethod | undefined> =
  ApiRouteReconcileBuilderBaseNone<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TBody, TDownloadsOwned, THeadersOwned>
  & ApiRouteReconcileBuilderCommonSteps<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, "none", "none", TBody, TDownloadsOwned, THeadersOwned, TMethod>
  & {
    signedUpload(options?: SignedUploadTransportOptions): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, "none", "signed", TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    multipartUpload(options?: DirectMultipartUploadTransportOptions): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, "none", "multipart", TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    processing(kind: "poll"): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, "poll", "none", TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    processing(kind: "callback", options: CallbackProcessingJobOptions): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, "callback", "none", TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    processing(kind: "webhook", options: WebhookProcessingJobOptions): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, "webhook", "none", TBody, TDownloadsOwned, THeadersOwned, TMethod>;
  };

type ApiRouteReconcileBuilderUpload<TRoute extends string, TRequestParams extends ApiRequestParamsShape | undefined, TValue, TItem, TAspectMap extends ResourceItemAspectMap<TItem>, TSummaryMap extends ResourceValueSummaryMap<TValue>, TSummaryMode extends ApiRouteArrayItemsSummaryMode, TUploadKind extends "signed" | "multipart", TBody, TDownloadsOwned extends boolean, THeadersOwned extends boolean, TMethod extends ResourceRequestMethod | undefined> =
  ApiRouteReconcileBuilderBaseUpload<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TUploadKind, TBody, TDownloadsOwned, THeadersOwned>
  & ApiRouteReconcileBuilderCommonSteps<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, "none", TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>
  & {
    processing(kind: "poll"): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, "poll", TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    processing(kind: "callback", options: CallbackProcessingJobOptions): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, "callback", TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    processing(kind: "webhook", options: WebhookProcessingJobOptions): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, "webhook", TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;
  };

type ApiRouteReconcileBuilderProcessing<TRoute extends string, TRequestParams extends ApiRequestParamsShape | undefined, TValue, TItem, TAspectMap extends ResourceItemAspectMap<TItem>, TSummaryMap extends ResourceValueSummaryMap<TValue>, TSummaryMode extends ApiRouteArrayItemsSummaryMode, TProcessingKind extends "poll" | "callback" | "webhook", TBody, TDownloadsOwned extends boolean, THeadersOwned extends boolean, TMethod extends ResourceRequestMethod | undefined> =
  ApiRouteReconcileBuilderBaseProcessing<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TBody, TDownloadsOwned, THeadersOwned>
  & ApiRouteReconcileBuilderCommonSteps<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, "none", TBody, TDownloadsOwned, THeadersOwned, TMethod>
  & {
    signedUpload(options?: SignedUploadTransportOptions): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, "signed", TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    multipartUpload(options?: DirectMultipartUploadTransportOptions): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, "multipart", TBody, TDownloadsOwned, THeadersOwned, TMethod>;
  };

type ApiRouteReconcileBuilderProcessingUpload<TRoute extends string, TRequestParams extends ApiRequestParamsShape | undefined, TValue, TItem, TAspectMap extends ResourceItemAspectMap<TItem>, TSummaryMap extends ResourceValueSummaryMap<TValue>, TSummaryMode extends ApiRouteArrayItemsSummaryMode, TProcessingKind extends "poll" | "callback" | "webhook", TUploadKind extends "signed" | "multipart", TBody, TDownloadsOwned extends boolean, THeadersOwned extends boolean, TMethod extends ResourceRequestMethod | undefined> =
  ApiRouteReconcileBuilderBaseProcessingUpload<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned>
  & ApiRouteReconcileBuilderCommonSteps<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;

export type ApiRouteReconcileBuilder<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryMode extends ApiRouteArrayItemsSummaryMode = "none",
  TProcessingKind extends ApiRouteProcessingKind = "none",
  TUploadKind extends ApiRouteUploadKind = "none",
  TBody = undefined,
  TDownloadsOwned extends boolean = false,
  THeadersOwned extends boolean = false,
  TMethod extends ResourceRequestMethod | undefined = undefined,
> = TProcessingKind extends "none"
  ? TUploadKind extends "none"
    ? ApiRouteReconcileBuilderNone<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, TBody, TDownloadsOwned, THeadersOwned, TMethod>
    : ApiRouteReconcileBuilderUpload<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, Extract<TUploadKind, "signed" | "multipart">, TBody, TDownloadsOwned, THeadersOwned, TMethod>
  : TUploadKind extends "none"
    ? ApiRouteReconcileBuilderProcessing<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, Extract<TProcessingKind, "poll" | "callback" | "webhook">, TBody, TDownloadsOwned, THeadersOwned, TMethod>
    : ApiRouteReconcileBuilderProcessingUpload<TRoute, TRequestParams, TValue, TItem, TAspectMap, TSummaryMap, TSummaryMode, Extract<TProcessingKind, "poll" | "callback" | "webhook">, Extract<TUploadKind, "signed" | "multipart">, TBody, TDownloadsOwned, THeadersOwned, TMethod>;
