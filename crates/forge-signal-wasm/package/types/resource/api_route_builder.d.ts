import type { SignalValue } from "../model.js";
import type {
  ApiCollectionResourceFamily,
  ApiDetailResourceFamily,
  ApiPagedResourceFamily,
  ApiRequestParamsShape,
} from "./api_request_params.js";
import type {
  ApiRouteBuilderItemsStep,
  ApiRouteBuilderParamsStep,
  ApiRouteBuilderRequestShapeStep,
  ApiRouteBuilderTransferStep,
  ApiRouteProcessingKind,
  ApiRouteUploadKind,
} from "./api_route_builder_steps.js";
import type { ResourceRequestMethod } from "./resource_postures.js";
import type {
  ApiRouteCollectionDeclarationForState,
  ApiRouteCreateDeclarationForState,
  ApiRouteDetailDeclarationForState,
  ApiRouteOwnedHeadersDeclaration,
  ApiRoutePagedDeclarationForState,
  ApiRouteReconcile,
  ApiRouteResolvedDownloadValue,
  ApiRouteTransferValue,
} from "./api_route_builder_declarations.js";

interface ApiRouteBuilderBase<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody = undefined,
  TDownloadValue = never,
  TDownloadsOwned extends boolean = false,
  THeadersOwned extends boolean = false,
> {
  detail<
    TValue extends ApiRouteResolvedDownloadValue<
      TValue,
      TDownloadValue,
      TDownloadsOwned
    >,
  >(
    declaration: ApiRouteOwnedHeadersDeclaration<
      ApiRouteDetailDeclarationForState<
        TRoute,
        TValue,
        TRequestParams,
        TProcessingKind,
        TUploadKind,
        TDownloadsOwned,
        TBody
      >,
      THeadersOwned
    >,
  ): ApiDetailResourceFamily<
    TRoute,
    TRequestParams,
    ApiRouteTransferValue<TValue, TProcessingKind, TUploadKind>,
    TBody
  >;
  list<
    TValue extends ApiRouteResolvedDownloadValue<
      TValue,
      TDownloadValue,
      TDownloadsOwned
    >,
    TItem = SignalValue,
    TReconcile extends ApiRouteReconcile<TValue, TItem> | undefined = undefined,
  >(
    declaration: ApiRouteOwnedHeadersDeclaration<
      ApiRouteCollectionDeclarationForState<
        TRoute,
        TRequestParams,
        TValue,
        TItem,
        TReconcile,
        TProcessingKind,
        TUploadKind,
        TDownloadsOwned,
        TBody
      >,
      THeadersOwned
    >,
  ): ApiCollectionResourceFamily<
    TRoute,
    TRequestParams,
    ApiRouteTransferValue<TValue, TProcessingKind, TUploadKind>,
    TItem,
    TReconcile,
    TBody
  >;
  paged<
    TValue extends ApiRouteResolvedDownloadValue<
      TValue,
      TDownloadValue,
      TDownloadsOwned
    >,
    TItem = SignalValue,
    TReconcile extends ApiRouteReconcile<TValue, TItem> | undefined = undefined,
  >(
    declaration: ApiRouteOwnedHeadersDeclaration<
      ApiRoutePagedDeclarationForState<
        TRoute,
        TRequestParams,
        TValue,
        TItem,
        TReconcile,
        TProcessingKind,
        TUploadKind,
        TDownloadsOwned,
        TBody
      >,
      THeadersOwned
    >,
  ): ApiPagedResourceFamily<
    TRoute,
    TRequestParams,
    ApiRouteTransferValue<TValue, TProcessingKind, TUploadKind>,
    TItem,
    TReconcile,
    TBody
  >;
}

interface ApiRouteBuilderStandardFinalizers<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TDownloadValue = never,
  TDownloadsOwned extends boolean = false,
  THeadersOwned extends boolean = false,
> {
  create<
    TValue extends ApiRouteResolvedDownloadValue<
      TValue,
      TDownloadValue,
      TDownloadsOwned
    >,
    TBody,
  >(
    declaration: ApiRouteOwnedHeadersDeclaration<
      ApiRouteCreateDeclarationForState<
        TRoute,
        TValue,
        TBody,
        TRequestParams,
        TProcessingKind,
        TUploadKind,
        TDownloadsOwned
      >,
      THeadersOwned
    >,
  ): ApiDetailResourceFamily<
    TRoute,
    TRequestParams,
    ApiRouteTransferValue<TValue, TProcessingKind, TUploadKind>,
    TBody
  >;
  update<
    TValue extends ApiRouteResolvedDownloadValue<
      TValue,
      TDownloadValue,
      TDownloadsOwned
    >,
    TBody,
  >(
    declaration: ApiRouteOwnedHeadersDeclaration<
      ApiRouteCreateDeclarationForState<
        TRoute,
        TValue,
        TBody,
        TRequestParams,
        TProcessingKind,
        TUploadKind,
        TDownloadsOwned
      >,
      THeadersOwned
    >,
  ): ApiDetailResourceFamily<
    TRoute,
    TRequestParams,
    ApiRouteTransferValue<TValue, TProcessingKind, TUploadKind>,
    TBody
  >;
  remove<
    TValue extends ApiRouteResolvedDownloadValue<
      TValue,
      TDownloadValue,
      TDownloadsOwned
    >,
  >(
    declaration: ApiRouteOwnedHeadersDeclaration<
      ApiRouteDetailDeclarationForState<
        TRoute,
        TValue,
        TRequestParams,
        TProcessingKind,
        TUploadKind,
        TDownloadsOwned
      >,
      THeadersOwned
    >,
  ): ApiDetailResourceFamily<
    TRoute,
    TRequestParams,
    ApiRouteTransferValue<TValue, TProcessingKind, TUploadKind>
  >;
}

export type ApiRouteBuilder<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined = undefined,
  TProcessingKind extends ApiRouteProcessingKind = "none",
  TUploadKind extends ApiRouteUploadKind = "none",
  TBody = undefined,
  TDownloadValue = never,
  TDownloadsOwned extends boolean = false,
  THeadersOwned extends boolean = false,
  TMethod extends ResourceRequestMethod | undefined = undefined,
> = ApiRouteBuilderBase<
  TRoute,
  TRequestParams,
  TProcessingKind,
  TUploadKind,
  TBody,
  TDownloadValue,
  TDownloadsOwned,
  THeadersOwned
>
  & ([TBody] extends [undefined]
    ? [TMethod] extends [undefined]
      ? ApiRouteBuilderStandardFinalizers<
          TRoute,
          TRequestParams,
          TProcessingKind,
          TUploadKind,
          TDownloadValue,
          TDownloadsOwned,
          THeadersOwned
        >
      : {}
    : {})
  & ApiRouteBuilderParamsStep<
    TRoute,
    TRequestParams,
    TProcessingKind,
    TUploadKind,
    TBody,
    TDownloadValue,
    TDownloadsOwned,
    THeadersOwned,
    TMethod
  >
  & ApiRouteBuilderTransferStep<
    TRoute,
    TRequestParams,
    TProcessingKind,
    TUploadKind,
    TBody,
    TDownloadValue,
    TDownloadsOwned,
    THeadersOwned,
    TMethod
  >
  & ApiRouteBuilderRequestShapeStep<
    TRoute,
    TRequestParams,
    TProcessingKind,
    TUploadKind,
    TBody,
    TDownloadValue,
    TDownloadsOwned,
    THeadersOwned,
    TMethod
  >
  & ApiRouteBuilderItemsStep<
    TRoute,
    TRequestParams,
    TProcessingKind,
    TUploadKind,
    TBody,
    TDownloadValue,
    TDownloadsOwned,
    THeadersOwned,
    TMethod
  >;
