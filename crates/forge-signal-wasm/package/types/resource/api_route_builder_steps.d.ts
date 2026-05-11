import type {
  DirectMultipartUploadTransportOptions,
  CallbackProcessingJobOptions,
  ResourceRequestMethod,
  SignedUploadTransportOptions,
  WebhookProcessingJobOptions,
} from "./resource_postures.js";
import type { ResourceEffectProfile } from "./resource_effect_profiles.js";
import type {
  ApiRequestParamsShape,
  ApiRouteLineParams,
  ApiRouteParamsConstraint,
} from "./api_request_params.js";
import type { ApiRouteDownloadsDeclaration } from "./api_route_downloads.js";
import type { ApiRouteArrayItemsBuilder } from "./api_route_array_items_builder.js";
import type { ApiRouteBuilder } from "./api_route_builder.js";
import type { ApiRouteReconcileBuilder } from "./api_route_reconcile_builder.js";
import type {
  ResourceCollectionResponse,
  ResourceDetailResponse,
  ResourceResponseAspectMap,
  ResourceResponseItem,
  ResourceResponseSummaryMap,
  ResourceResponseValue,
} from "./resource_response.js";
import type {
  ApiRouteProcessingKind,
  ApiRouteUploadKind,
} from "./api_route_transfer_kinds.js";

export type ApiRouteBuilderParamsStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadValue,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TEffectsOwned extends boolean,
  TMethod extends ResourceRequestMethod | undefined,
> = [TRequestParams] extends [undefined]
  ? ApiRouteParamsConstraint<TRoute> extends {
      readonly __forgeInvalidApiRequestParams__: string;
    }
    ? {}
    : {
        params<TNextRequestParams extends ApiRequestParamsShape>(): ApiRouteBuilder<
          TRoute,
          TNextRequestParams,
          TProcessingKind,
          TUploadKind,
          TBody,
          TDownloadValue,
          TDownloadsOwned,
          THeadersOwned,
          TEffectsOwned,
          TMethod
        >;
      }
  : {};

export type ApiRouteBuilderTransferStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadValue,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TEffectsOwned extends boolean,
  TMethod extends ResourceRequestMethod | undefined,
> = (TUploadKind extends "none"
  ? {
      signedUpload(options?: SignedUploadTransportOptions): ApiRouteBuilder<
        TRoute,
        TRequestParams,
        TProcessingKind,
        "signed",
        TBody,
        TDownloadValue,
        TDownloadsOwned,
        THeadersOwned,
        TEffectsOwned,
        TMethod
      >;
      multipartUpload(options?: DirectMultipartUploadTransportOptions): ApiRouteBuilder<
        TRoute,
        TRequestParams,
        TProcessingKind,
        "multipart",
        TBody,
        TDownloadValue,
        TDownloadsOwned,
        THeadersOwned,
        TEffectsOwned,
        TMethod
      >;
    }
  : {}) &
  (TProcessingKind extends "none"
    ? {
        processing(kind: "poll"): ApiRouteBuilder<
          TRoute,
          TRequestParams,
          "poll",
          TUploadKind,
          TBody,
          TDownloadValue,
          TDownloadsOwned,
          THeadersOwned,
          TEffectsOwned,
          TMethod
        >;
        processing(
          kind: "callback",
          options: CallbackProcessingJobOptions,
        ): ApiRouteBuilder<
          TRoute,
          TRequestParams,
          "callback",
          TUploadKind,
          TBody,
          TDownloadValue,
          TDownloadsOwned,
          THeadersOwned,
          TEffectsOwned,
          TMethod
        >;
        processing(
          kind: "webhook",
          options: WebhookProcessingJobOptions,
        ): ApiRouteBuilder<
          TRoute,
          TRequestParams,
          "webhook",
          TUploadKind,
          TBody,
          TDownloadValue,
          TDownloadsOwned,
          THeadersOwned,
          TEffectsOwned,
          TMethod
        >;
      }
    : {});

export type ApiRouteBuilderRequestShapeStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadValue,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TEffectsOwned extends boolean,
  TMethod extends ResourceRequestMethod | undefined,
> = (TDownloadsOwned extends true
  ? {}
  : {
      downloads<TNextDownloadValue>(
        declaration: ApiRouteDownloadsDeclaration<
          ApiRouteLineParams<TRoute, TRequestParams, TBody>,
          TNextDownloadValue
        >,
      ): ApiRouteBuilder<
        TRoute,
        TRequestParams,
        TProcessingKind,
        TUploadKind,
        TBody,
        TNextDownloadValue,
        true,
        THeadersOwned,
        TEffectsOwned,
        TMethod
      >;
    }) &
  (THeadersOwned extends true
    ? {}
    : {
        headers(
          headers:
            | Record<string, string>
            | ((params: ApiRouteLineParams<TRoute, TRequestParams, TBody>) => Record<string, string>),
        ): ApiRouteBuilder<
          TRoute,
          TRequestParams,
          TProcessingKind,
          TUploadKind,
          TBody,
          TDownloadValue,
          TDownloadsOwned,
          true,
          TEffectsOwned,
          TMethod
        >;
      }) &
  (TEffectsOwned extends true
    ? {}
    : {
        effects(
          effects:
            | ResourceEffectProfile
            | ((params: ApiRouteLineParams<TRoute, TRequestParams, TBody>) => ResourceEffectProfile),
        ): ApiRouteBuilder<
          TRoute,
          TRequestParams,
          TProcessingKind,
          TUploadKind,
          TBody,
          TDownloadValue,
          TDownloadsOwned,
          THeadersOwned,
          true,
          TMethod
        >;
      }) &
  ([TBody] extends [undefined]
    ? {
        body<TNextBody>(): ApiRouteBuilder<
          TRoute,
          TRequestParams,
          TProcessingKind,
          TUploadKind,
          TNextBody,
          TDownloadValue,
          TDownloadsOwned,
          THeadersOwned,
          TEffectsOwned,
          TMethod
        >;
      }
    : {}) &
  ([TMethod] extends [undefined]
    ? {
        verb<TNextMethod extends ResourceRequestMethod>(
          method: TNextMethod,
        ): ApiRouteBuilder<
          TRoute,
          TRequestParams,
          TProcessingKind,
          TUploadKind,
          TBody,
          TDownloadValue,
          TDownloadsOwned,
          THeadersOwned,
          TEffectsOwned,
          TNextMethod
        >;
      }
    : {});

export type ApiRouteBuilderItemsStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadValue,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TEffectsOwned extends boolean,
  TMethod extends ResourceRequestMethod | undefined,
> = {
  items<TItem>(itemIdentity: (item: TItem) => string): ApiRouteArrayItemsBuilder<
    TRoute,
    TRequestParams,
    TItem,
    {},
    {},
    "none",
    TProcessingKind,
    TUploadKind,
    TBody,
    TDownloadsOwned,
    THeadersOwned,
    TMethod
  >;
  response<
    TResponse extends
      | ResourceCollectionResponse<any, any, any, any>
      | ResourceDetailResponse<any>,
  >(
    response: TResponse,
  ): TResponse extends ResourceDetailResponse<any>
    ? Pick<
        ApiRouteBuilder<
          TRoute,
          TRequestParams,
          TProcessingKind,
          TUploadKind,
          TBody,
          TDownloadValue,
          TDownloadsOwned,
          THeadersOwned,
          TEffectsOwned,
          TMethod
        >,
        "detail"
      >
    : Omit<ApiRouteReconcileBuilder<
        TRoute,
        TRequestParams,
        ResourceResponseValue<TResponse>,
        ResourceResponseItem<TResponse>,
        ResourceResponseAspectMap<TResponse>,
        ResourceResponseSummaryMap<TResponse>,
        "none",
        TProcessingKind,
        TUploadKind,
        TBody,
        TDownloadsOwned,
        THeadersOwned,
        TMethod
      >, "aspect" | "reconcile" | "summary" | "pageWindowSummary">;
};
