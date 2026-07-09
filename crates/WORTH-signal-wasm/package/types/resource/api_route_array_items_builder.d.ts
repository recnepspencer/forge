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
import type { ApiRouteReconcileBuilder } from "./api_route_reconcile_builder.js";
import type {
  ApiRouteArrayItemsBuilderBaseNone,
  ApiRouteArrayItemsBuilderBaseProcessing,
  ApiRouteArrayItemsBuilderBaseProcessingUpload,
  ApiRouteArrayItemsBuilderBaseUpload,
} from "./api_route_array_items_builder_base.js";
import type {
  ApiRouteArrayItemsSummaryMode,
  ApiRouteUnusedDefinitionName,
} from "./api_route_array_items_common.js";

type ApiRouteArrayItemsBuilderParamsStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<any>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TMethod extends ResourceRequestMethod | undefined,
> = [TRequestParams] extends [undefined]
  ? ApiRouteParamsConstraint<TRoute> extends { readonly __WORTHInvalidApiRequestParams__: string }
    ? {}
    : {
        params<TNextRequestParams extends ApiRequestParamsShape>(): ApiRouteArrayItemsBuilder<TRoute, TNextRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;
      }
  : {};

type ApiRouteArrayItemsBuilderReconcileStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<any>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TMethod extends ResourceRequestMethod | undefined,
> = [keyof TSummaryMap] extends [never]
  ? TSummaryMode extends "none"
    ? {
        reconcile<TValue>(items: (value: TValue) => readonly TItem[], replaceItems: (value: TValue, nextItems: readonly TItem[]) => TValue): ApiRouteReconcileBuilder<TRoute, TRequestParams, TValue, TItem, TAspectMap, {}, "none", TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;
      }
    : {}
  : {};

type ApiRouteArrayItemsBuilderAspectStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<any>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TMethod extends ResourceRequestMethod | undefined,
> = {
  aspect<TName extends string, TValue>(name: ApiRouteUnusedDefinitionName<TName, TAspectMap>, read: (item: TItem) => TValue, write: (item: TItem, value: TValue) => TItem): ApiRouteArrayItemsBuilder<TRoute, TRequestParams, TItem, TAspectMap & Record<TName, ResourceItemAspect<TItem, TValue>>, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;
};

type ApiRouteArrayItemsBuilderSummaryStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<any>,
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
      summary<TValue extends readonly TItem[], TName extends string, TSummaryValue>(name: ApiRouteUnusedDefinitionName<TName, TSummaryMap>, read: (value: TValue) => TSummaryValue, write: (value: TValue, summary: TSummaryValue) => TValue): ApiRouteArrayItemsBuilder<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap & Record<TName, ResourceValueSummary<TValue, TSummaryValue>>, "line", TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    };

type ApiRouteArrayItemsBuilderPageWindowSummaryStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<any>,
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
      pageWindowSummary<TValue extends readonly TItem[], TName extends string, TSummaryValue>(name: ApiRouteUnusedDefinitionName<TName, TSummaryMap>, read: (value: TValue) => TSummaryValue, write: (value: TValue, summary: TSummaryValue) => TValue): ApiRouteArrayItemsBuilder<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap & Record<TName, ResourceValueSummary<TValue, TSummaryValue>>, "pageWindow", TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    };

type ApiRouteArrayItemsBuilderRequestShapeStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<any>,
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
          readonly TItem[]
        >,
      ): ApiRouteArrayItemsBuilder<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, true, THeadersOwned, TMethod>;
    }) &
  (THeadersOwned extends true ? {} : {
    headers(headers: Record<string, string> | ((params: ApiRouteLineParams<TRoute, TRequestParams, TBody>) => Record<string, string>)): ApiRouteArrayItemsBuilder<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, true, TMethod>;
  }) &
  ([TBody] extends [undefined] ? {
    body<TNextBody>(): ApiRouteArrayItemsBuilder<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TNextBody, TDownloadsOwned, THeadersOwned, TMethod>;
  } : {}) &
  ([TMethod] extends [undefined] ? {
    verb<TNextMethod extends ResourceRequestMethod>(method: TNextMethod): ApiRouteArrayItemsBuilder<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TNextMethod>;
  } : {});

type ApiRouteArrayItemsBuilderCommonSteps<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<any>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TMethod extends ResourceRequestMethod | undefined,
> = ApiRouteArrayItemsBuilderReconcileStep<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>
  & ApiRouteArrayItemsBuilderParamsStep<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>
  & ApiRouteArrayItemsBuilderAspectStep<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>
  & ApiRouteArrayItemsBuilderSummaryStep<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>
  & ApiRouteArrayItemsBuilderPageWindowSummaryStep<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>
  & ApiRouteArrayItemsBuilderRequestShapeStep<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;

type ApiRouteArrayItemsBuilderNone<TRoute extends string, TRequestParams extends ApiRequestParamsShape | undefined, TItem, TAspectMap extends ResourceItemAspectMap<TItem>, TSummaryMap extends ResourceValueSummaryMap<any>, TSummaryMode extends ApiRouteArrayItemsSummaryMode, TBody, TDownloadsOwned extends boolean, THeadersOwned extends boolean, TMethod extends ResourceRequestMethod | undefined> =
  ApiRouteArrayItemsBuilderBaseNone<TRoute, TRequestParams, readonly TItem[], TItem, TAspectMap, TSummaryMap, TSummaryMode, TBody, TDownloadsOwned, THeadersOwned>
  & ApiRouteArrayItemsBuilderCommonSteps<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, "none", "none", TBody, TDownloadsOwned, THeadersOwned, TMethod>
  & {
    signedUpload(options?: SignedUploadTransportOptions): ApiRouteArrayItemsBuilder<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, "none", "signed", TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    multipartUpload(options?: DirectMultipartUploadTransportOptions): ApiRouteArrayItemsBuilder<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, "none", "multipart", TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    processing(kind: "poll"): ApiRouteArrayItemsBuilder<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, "poll", "none", TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    processing(kind: "callback", options: CallbackProcessingJobOptions): ApiRouteArrayItemsBuilder<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, "callback", "none", TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    processing(kind: "webhook", options: WebhookProcessingJobOptions): ApiRouteArrayItemsBuilder<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, "webhook", "none", TBody, TDownloadsOwned, THeadersOwned, TMethod>;
  };

type ApiRouteArrayItemsBuilderUpload<TRoute extends string, TRequestParams extends ApiRequestParamsShape | undefined, TItem, TAspectMap extends ResourceItemAspectMap<TItem>, TSummaryMap extends ResourceValueSummaryMap<any>, TSummaryMode extends ApiRouteArrayItemsSummaryMode, TUploadKind extends "signed" | "multipart", TBody, TDownloadsOwned extends boolean, THeadersOwned extends boolean, TMethod extends ResourceRequestMethod | undefined> =
  ApiRouteArrayItemsBuilderBaseUpload<TRoute, TRequestParams, readonly TItem[], TItem, TAspectMap, TSummaryMap, TSummaryMode, TUploadKind, TBody, TDownloadsOwned, THeadersOwned>
  & ApiRouteArrayItemsBuilderCommonSteps<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, "none", TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>
  & {
    processing(kind: "poll"): ApiRouteArrayItemsBuilder<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, "poll", TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    processing(kind: "callback", options: CallbackProcessingJobOptions): ApiRouteArrayItemsBuilder<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, "callback", TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    processing(kind: "webhook", options: WebhookProcessingJobOptions): ApiRouteArrayItemsBuilder<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, "webhook", TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;
  };

type ApiRouteArrayItemsBuilderProcessing<TRoute extends string, TRequestParams extends ApiRequestParamsShape | undefined, TItem, TAspectMap extends ResourceItemAspectMap<TItem>, TSummaryMap extends ResourceValueSummaryMap<any>, TSummaryMode extends ApiRouteArrayItemsSummaryMode, TProcessingKind extends "poll" | "callback" | "webhook", TBody, TDownloadsOwned extends boolean, THeadersOwned extends boolean, TMethod extends ResourceRequestMethod | undefined> =
  ApiRouteArrayItemsBuilderBaseProcessing<TRoute, TRequestParams, readonly TItem[], TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TBody, TDownloadsOwned, THeadersOwned>
  & ApiRouteArrayItemsBuilderCommonSteps<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, "none", TBody, TDownloadsOwned, THeadersOwned, TMethod>
  & {
    signedUpload(options?: SignedUploadTransportOptions): ApiRouteArrayItemsBuilder<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, "signed", TBody, TDownloadsOwned, THeadersOwned, TMethod>;
    multipartUpload(options?: DirectMultipartUploadTransportOptions): ApiRouteArrayItemsBuilder<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, "multipart", TBody, TDownloadsOwned, THeadersOwned, TMethod>;
  };

type ApiRouteArrayItemsBuilderProcessingUpload<TRoute extends string, TRequestParams extends ApiRequestParamsShape | undefined, TItem, TAspectMap extends ResourceItemAspectMap<TItem>, TSummaryMap extends ResourceValueSummaryMap<any>, TSummaryMode extends ApiRouteArrayItemsSummaryMode, TProcessingKind extends "poll" | "callback" | "webhook", TUploadKind extends "signed" | "multipart", TBody, TDownloadsOwned extends boolean, THeadersOwned extends boolean, TMethod extends ResourceRequestMethod | undefined> =
  ApiRouteArrayItemsBuilderBaseProcessingUpload<TRoute, TRequestParams, readonly TItem[], TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned>
  & ApiRouteArrayItemsBuilderCommonSteps<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, TProcessingKind, TUploadKind, TBody, TDownloadsOwned, THeadersOwned, TMethod>;

export type ApiRouteArrayItemsBuilder<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<any> = {},
  TSummaryMode extends ApiRouteArrayItemsSummaryMode = "none",
  TProcessingKind extends ApiRouteProcessingKind = "none",
  TUploadKind extends ApiRouteUploadKind = "none",
  TBody = undefined,
  TDownloadsOwned extends boolean = false,
  THeadersOwned extends boolean = false,
  TMethod extends ResourceRequestMethod | undefined = undefined,
> = TProcessingKind extends "none"
  ? TUploadKind extends "none"
    ? ApiRouteArrayItemsBuilderNone<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, TBody, TDownloadsOwned, THeadersOwned, TMethod>
    : ApiRouteArrayItemsBuilderUpload<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, Extract<TUploadKind, "signed" | "multipart">, TBody, TDownloadsOwned, THeadersOwned, TMethod>
  : TUploadKind extends "none"
    ? ApiRouteArrayItemsBuilderProcessing<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, Extract<TProcessingKind, "poll" | "callback" | "webhook">, TBody, TDownloadsOwned, THeadersOwned, TMethod>
    : ApiRouteArrayItemsBuilderProcessingUpload<TRoute, TRequestParams, TItem, TAspectMap, TSummaryMap, TSummaryMode, Extract<TProcessingKind, "poll" | "callback" | "webhook">, Extract<TUploadKind, "signed" | "multipart">, TBody, TDownloadsOwned, THeadersOwned, TMethod>;
