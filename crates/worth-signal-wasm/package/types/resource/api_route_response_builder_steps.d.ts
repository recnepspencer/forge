import type { ResourceRequestMethod } from "./resource_postures.js";
import type {
  ApiRequestParamsShape,
  ApiDetailResourceFamily,
} from "./api_request_params.js";
import type { ApiRouteBuilder } from "./api_route_builder.js";
import type { ApiRouteReconcileBuilder } from "./api_route_reconcile_builder.js";
import type {
  ResourceCollectionResponse,
  ResourceDetailResponse,
  ResourceSummaryResponse,
  ResourceResponseAspectMap,
  ResourceResponseItem,
  ResourceResponseSummaryMap,
  ResourceResponseValue,
} from "./resource_response.js";
import type {
  ApiRouteProcessingKind,
  ApiRouteUploadKind,
} from "./api_route_transfer_kinds.js";
import type {
  ApiRouteCommandSemantics,
  ApiRouteOwnedEffectsDeclaration,
  ApiRouteOwnedHeadersDeclaration,
  ApiRouteResponseCommandMutationDeclarationForState,
  ApiRouteResolvedDownloadValue,
  ApiRouteSemanticMutationDeclarationForState,
  ApiRouteMutationSemantics,
  ApiRouteResponseCreateMutationDeclarationForState,
  ApiRouteResponseRemoveMutationDeclarationForState,
  ApiRouteResponseUpdateMutationDeclarationForState,
  ApiRouteSettledTransferValue,
} from "./api_route_builder_declarations.js";

type ApiRouteResponseWriteFinalizers<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadValue,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TEffectsOwned extends boolean,
> = {
  create<TValue, TWriteBody>(
    declaration: ApiRouteOwnedEffectsDeclaration<ApiRouteOwnedHeadersDeclaration<
      ApiRouteResponseCreateMutationDeclarationForState<
        TRoute,
        ApiRouteResolvedDownloadValue<TValue, TDownloadValue, TDownloadsOwned>,
        TWriteBody,
        TRequestParams,
        TProcessingKind,
        TUploadKind,
        TDownloadsOwned
      >,
      THeadersOwned
    >, TEffectsOwned>,
  ): ApiDetailResourceFamily<
    TRoute,
    TRequestParams,
    ApiRouteSettledTransferValue<
      ApiRouteResolvedDownloadValue<TValue, TDownloadValue, TDownloadsOwned>,
      TProcessingKind,
      TUploadKind
    >,
    TWriteBody
  >;
  update<TValue, TWriteBody>(
    declaration: ApiRouteOwnedEffectsDeclaration<ApiRouteOwnedHeadersDeclaration<
      ApiRouteResponseUpdateMutationDeclarationForState<
        TRoute,
        ApiRouteResolvedDownloadValue<TValue, TDownloadValue, TDownloadsOwned>,
        TWriteBody,
        TRequestParams,
        TProcessingKind,
        TUploadKind,
        TDownloadsOwned
      >,
      THeadersOwned
    >, TEffectsOwned>,
  ): ApiDetailResourceFamily<
    TRoute,
    TRequestParams,
    ApiRouteSettledTransferValue<
      ApiRouteResolvedDownloadValue<TValue, TDownloadValue, TDownloadsOwned>,
      TProcessingKind,
      TUploadKind
    >,
    TWriteBody
  >;
  remove<TValue>(
    declaration: ApiRouteOwnedEffectsDeclaration<ApiRouteOwnedHeadersDeclaration<
      ApiRouteResponseRemoveMutationDeclarationForState<
        TRoute,
        ApiRouteResolvedDownloadValue<TValue, TDownloadValue, TDownloadsOwned>,
        TRequestParams,
        TProcessingKind,
        TUploadKind,
        TDownloadsOwned
      >,
      THeadersOwned
    >, TEffectsOwned>,
  ): ApiDetailResourceFamily<
    TRoute,
    TRequestParams,
    ApiRouteSettledTransferValue<
      ApiRouteResolvedDownloadValue<TValue, TDownloadValue, TDownloadsOwned>,
      TProcessingKind,
      TUploadKind
    >
  >;
  mutation<TValue, TWriteBody, TSemantics extends ApiRouteMutationSemantics>(
    declaration: ApiRouteOwnedEffectsDeclaration<ApiRouteOwnedHeadersDeclaration<
      ApiRouteSemanticMutationDeclarationForState<
        TRoute,
        ApiRouteResolvedDownloadValue<TValue, TDownloadValue, TDownloadsOwned>,
        TWriteBody,
        TRequestParams,
        TProcessingKind,
        TUploadKind,
        TDownloadsOwned
      > & { readonly semantics: TSemantics },
      THeadersOwned
    >, TEffectsOwned>,
  ): ApiDetailResourceFamily<
    TRoute,
    TRequestParams,
    ApiRouteSettledTransferValue<
      ApiRouteResolvedDownloadValue<TValue, TDownloadValue, TDownloadsOwned>,
      TProcessingKind,
      TUploadKind
    >,
    TWriteBody
  >;
  command<TValue, TWriteBody, TSemantics extends ApiRouteCommandSemantics>(
    declaration: ApiRouteOwnedEffectsDeclaration<ApiRouteOwnedHeadersDeclaration<
      ApiRouteResponseCommandMutationDeclarationForState<
        TRoute,
        ApiRouteResolvedDownloadValue<TValue, TDownloadValue, TDownloadsOwned>,
        TWriteBody,
        TRequestParams,
        TProcessingKind,
        TUploadKind,
        TDownloadsOwned
      > & { readonly semantics: TSemantics },
      THeadersOwned
    >, TEffectsOwned>,
  ): ApiDetailResourceFamily<
    TRoute,
    TRequestParams,
    ApiRouteSettledTransferValue<
      ApiRouteResolvedDownloadValue<TValue, TDownloadValue, TDownloadsOwned>,
      TProcessingKind,
      TUploadKind
    >,
    TWriteBody
  >;
};

type ApiRouteResponseDetailBuilder<
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
> = Pick<
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
> & ApiRouteResponseWriteFinalizers<
  TRoute,
  TRequestParams,
  TProcessingKind,
  TUploadKind,
  TBody,
  TDownloadValue,
  TDownloadsOwned,
  THeadersOwned,
  TEffectsOwned
>;

type ApiRouteResponseCollectionBuilder<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TResponse extends ResourceCollectionResponse<any, any, any, any>,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadValue,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TEffectsOwned extends boolean,
  TMethod extends ResourceRequestMethod | undefined,
> = Omit<ApiRouteReconcileBuilder<
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
>, "aspect" | "reconcile" | "summary" | "pageWindowSummary">
  & ApiRouteResponseWriteFinalizers<
    TRoute,
    TRequestParams,
    TProcessingKind,
    TUploadKind,
    TBody,
    TDownloadValue,
    TDownloadsOwned,
    THeadersOwned,
    TEffectsOwned
  >;

export type ApiRouteResponseBuilderForResponse<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TResponse extends
    | ResourceCollectionResponse<any, any, any, any>
    | ResourceDetailResponse<any, any>
    | ResourceSummaryResponse<any>,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadValue,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TEffectsOwned extends boolean,
  TMethod extends ResourceRequestMethod | undefined,
> = TResponse extends ResourceDetailResponse<any, any> | ResourceSummaryResponse<any>
  ? ApiRouteResponseDetailBuilder<
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
    >
  : ApiRouteResponseCollectionBuilder<
      TRoute,
      TRequestParams,
      Extract<TResponse, ResourceCollectionResponse<any, any, any, any>>,
      TProcessingKind,
      TUploadKind,
      TBody,
      TDownloadValue,
      TDownloadsOwned,
      THeadersOwned,
      TEffectsOwned,
      TMethod
    >;
