import type { SignalValue } from "../model.js";
import type {
  DetailResourceDeclaration,
  ProcessingDetailResourceDeclaration,
  UploadDetailResourceDeclaration,
  ProcessingUploadDetailResourceDeclaration,
  CollectionResourceDeclaration,
  ProcessingCollectionResourceDeclaration,
  UploadCollectionResourceDeclaration,
  ProcessingUploadCollectionResourceDeclaration,
  PagedResourceDeclaration,
  ProcessingPagedResourceDeclaration,
  UploadPagedResourceDeclaration,
  ProcessingUploadPagedResourceDeclaration,
} from "./resource_declarations.js";
import type {
  ResourceCollectionShape,
  ResourceItemAspectMap,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";
import type {
  ApiRequestParamsShape,
  ApiRouteDeclarationParams,
  ApiRouteLineParams,
  ApiRouteWriteDeclarationParams,
} from "./api_request_params.js";
import type { ApiRouteDownloadsDeclaration } from "./api_route_downloads.js";

type ApiRouteHeaders<TParams> =
  | Record<string, string>
  | ((params: TParams) => Record<string, string>);

type ApiRouteOwnedProcessingField<
  TDeclaration,
  TOwned extends boolean,
> = TOwned extends true
  ? { processingJob?: never }
  : TDeclaration extends { processingJob: infer TProcessingJob }
    ? { processingJob: TProcessingJob }
    : TDeclaration extends { processingJob?: infer TProcessingJob }
      ? { processingJob?: TProcessingJob }
      : {};

type ApiRouteOwnedUploadField<
  TDeclaration,
  TOwned extends boolean,
> = TOwned extends true
  ? { uploadTransport?: never }
  : TDeclaration extends { uploadTransport: infer TUploadTransport }
    ? { uploadTransport: TUploadTransport }
    : TDeclaration extends { uploadTransport?: infer TUploadTransport }
      ? { uploadTransport?: TUploadTransport }
      : {};

type ApiRouteOwnedDownloadsField<
  TParams,
  TValue,
  TOwned extends boolean,
> = TOwned extends true
  ? { downloads?: never }
  : {
      downloads?: ApiRouteDownloadsDeclaration<TParams, TValue>;
    };

export type ApiRouteBoundDeclaration<
  TDeclaration,
  TParams,
  TProcessingOwned extends boolean = false,
  TUploadOwned extends boolean = false,
> =
  Omit<TDeclaration, "params" | "normalizeParams" | "method" | "requestBody" | "processingJob" | "uploadTransport"> & {
    headers?: ApiRouteHeaders<TParams>;
    params?: never;
    normalizeParams?: never;
    baseUrl?: never;
    method?: never;
    requestBody?: never;
  }
  & ApiRouteOwnedProcessingField<TDeclaration, TProcessingOwned>
  & ApiRouteOwnedUploadField<TDeclaration, TUploadOwned>;

type ApiRouteDeclarationMemberParams<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TBody = undefined,
> = ApiRouteLineParams<TRoute, TRequestParams, TBody>;

export type ApiRouteDetailDeclaration<
  TRoute extends string,
  TValue,
  TRequestParams extends ApiRequestParamsShape | undefined = undefined,
  TBody = undefined,
  TProcessingOwned extends boolean = false,
  TUploadOwned extends boolean = false,
  TDownloadsOwned extends boolean = false,
> = ApiRouteBoundDeclaration<
  DetailResourceDeclaration<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue>,
  ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>,
  TProcessingOwned,
  TUploadOwned
> & ApiRouteOwnedDownloadsField<
  ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>,
  TValue,
  TDownloadsOwned
>;

export type ApiRouteProcessingDetailDeclaration<
  TRoute extends string,
  TValue,
  TRequestParams extends ApiRequestParamsShape | undefined = undefined,
  TBody = undefined,
  TProcessingOwned extends boolean = false,
  TUploadOwned extends boolean = false,
  TDownloadsOwned extends boolean = false,
> = ApiRouteBoundDeclaration<
  ProcessingDetailResourceDeclaration<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue>,
  ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>,
  TProcessingOwned,
  TUploadOwned
> & ApiRouteOwnedDownloadsField<
  ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>,
  TValue,
  TDownloadsOwned
>;

export type ApiRouteUploadDetailDeclaration<
  TRoute extends string,
  TValue,
  TRequestParams extends ApiRequestParamsShape | undefined = undefined,
  TBody = undefined,
  TProcessingOwned extends boolean = false,
  TUploadOwned extends boolean = false,
  TDownloadsOwned extends boolean = false,
> = ApiRouteBoundDeclaration<
  UploadDetailResourceDeclaration<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue>,
  ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>,
  TProcessingOwned,
  TUploadOwned
> & ApiRouteOwnedDownloadsField<
  ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>,
  TValue,
  TDownloadsOwned
>;

export type ApiRouteProcessingUploadDetailDeclaration<
  TRoute extends string,
  TValue,
  TRequestParams extends ApiRequestParamsShape | undefined = undefined,
  TBody = undefined,
  TProcessingOwned extends boolean = false,
  TUploadOwned extends boolean = false,
  TDownloadsOwned extends boolean = false,
> = ApiRouteBoundDeclaration<
  ProcessingUploadDetailResourceDeclaration<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue>,
  ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>,
  TProcessingOwned,
  TUploadOwned
> & ApiRouteOwnedDownloadsField<
  ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>,
  TValue,
  TDownloadsOwned
>;

export type ApiRouteCreateDeclaration<
  TRoute extends string,
  TValue,
  TBody,
  TRequestParams extends ApiRequestParamsShape | undefined = undefined,
  TProcessingOwned extends boolean = false,
  TUploadOwned extends boolean = false,
  TDownloadsOwned extends boolean = false,
> = ApiRouteBoundDeclaration<
  DetailResourceDeclaration<ApiRouteWriteDeclarationParams<TRoute, TRequestParams, TBody>, TValue>,
  ApiRouteWriteDeclarationParams<TRoute, TRequestParams, TBody>,
  TProcessingOwned,
  TUploadOwned
> & ApiRouteOwnedDownloadsField<
  ApiRouteWriteDeclarationParams<TRoute, TRequestParams, TBody>,
  TValue,
  TDownloadsOwned
>;

export type ApiRouteProcessingCreateDeclaration<
  TRoute extends string,
  TValue,
  TBody,
  TRequestParams extends ApiRequestParamsShape | undefined = undefined,
  TProcessingOwned extends boolean = false,
  TUploadOwned extends boolean = false,
  TDownloadsOwned extends boolean = false,
> = ApiRouteBoundDeclaration<
  ProcessingDetailResourceDeclaration<ApiRouteWriteDeclarationParams<TRoute, TRequestParams, TBody>, TValue>,
  ApiRouteWriteDeclarationParams<TRoute, TRequestParams, TBody>,
  TProcessingOwned,
  TUploadOwned
> & ApiRouteOwnedDownloadsField<
  ApiRouteWriteDeclarationParams<TRoute, TRequestParams, TBody>,
  TValue,
  TDownloadsOwned
>;

export type ApiRouteUploadCreateDeclaration<
  TRoute extends string,
  TValue,
  TBody,
  TRequestParams extends ApiRequestParamsShape | undefined = undefined,
  TProcessingOwned extends boolean = false,
  TUploadOwned extends boolean = false,
  TDownloadsOwned extends boolean = false,
> = ApiRouteBoundDeclaration<
  UploadDetailResourceDeclaration<ApiRouteWriteDeclarationParams<TRoute, TRequestParams, TBody>, TValue>,
  ApiRouteWriteDeclarationParams<TRoute, TRequestParams, TBody>,
  TProcessingOwned,
  TUploadOwned
> & ApiRouteOwnedDownloadsField<
  ApiRouteWriteDeclarationParams<TRoute, TRequestParams, TBody>,
  TValue,
  TDownloadsOwned
>;

export type ApiRouteProcessingUploadCreateDeclaration<
  TRoute extends string,
  TValue,
  TBody,
  TRequestParams extends ApiRequestParamsShape | undefined = undefined,
  TProcessingOwned extends boolean = false,
  TUploadOwned extends boolean = false,
  TDownloadsOwned extends boolean = false,
> = ApiRouteBoundDeclaration<
  ProcessingUploadDetailResourceDeclaration<ApiRouteWriteDeclarationParams<TRoute, TRequestParams, TBody>, TValue>,
  ApiRouteWriteDeclarationParams<TRoute, TRequestParams, TBody>,
  TProcessingOwned,
  TUploadOwned
> & ApiRouteOwnedDownloadsField<
  ApiRouteWriteDeclarationParams<TRoute, TRequestParams, TBody>,
  TValue,
  TDownloadsOwned
>;

type ApiRouteReconcile<
  TValue,
  TItem,
> = ResourceCollectionShape<
  TValue,
  TItem,
  ResourceItemAspectMap<TItem>,
  ResourceValueSummaryMap<TValue>,
  any
>;

export type ApiRouteCollectionDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
  TItem = SignalValue,
  TReconcile extends ApiRouteReconcile<TValue, TItem> | undefined = undefined,
  TProcessingOwned extends boolean = false,
  TUploadOwned extends boolean = false,
  TDownloadsOwned extends boolean = false,
> = ApiRouteBoundDeclaration<
  CollectionResourceDeclaration<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue, TItem, TReconcile>,
  ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>,
  TProcessingOwned,
  TUploadOwned
> & ApiRouteOwnedDownloadsField<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue, TDownloadsOwned>;

export type ApiRouteProcessingCollectionDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
  TItem = SignalValue,
  TReconcile extends ApiRouteReconcile<TValue, TItem> | undefined = undefined,
  TProcessingOwned extends boolean = false,
  TUploadOwned extends boolean = false,
  TDownloadsOwned extends boolean = false,
> = ApiRouteBoundDeclaration<
  ProcessingCollectionResourceDeclaration<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue, TItem, TReconcile>,
  ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>,
  TProcessingOwned,
  TUploadOwned
> & ApiRouteOwnedDownloadsField<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue, TDownloadsOwned>;

export type ApiRouteUploadCollectionDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
  TItem = SignalValue,
  TReconcile extends ApiRouteReconcile<TValue, TItem> | undefined = undefined,
  TProcessingOwned extends boolean = false,
  TUploadOwned extends boolean = false,
  TDownloadsOwned extends boolean = false,
> = ApiRouteBoundDeclaration<
  UploadCollectionResourceDeclaration<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue, TItem, TReconcile>,
  ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>,
  TProcessingOwned,
  TUploadOwned
> & ApiRouteOwnedDownloadsField<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue, TDownloadsOwned>;

export type ApiRouteProcessingUploadCollectionDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
  TItem = SignalValue,
  TReconcile extends ApiRouteReconcile<TValue, TItem> | undefined = undefined,
  TProcessingOwned extends boolean = false,
  TUploadOwned extends boolean = false,
  TDownloadsOwned extends boolean = false,
> = ApiRouteBoundDeclaration<
  ProcessingUploadCollectionResourceDeclaration<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue, TItem, TReconcile>,
  ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>,
  TProcessingOwned,
  TUploadOwned
> & ApiRouteOwnedDownloadsField<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue, TDownloadsOwned>;

export type ApiRoutePagedDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
  TItem = SignalValue,
  TReconcile extends ApiRouteReconcile<TValue, TItem> | undefined = undefined,
  TProcessingOwned extends boolean = false,
  TUploadOwned extends boolean = false,
  TDownloadsOwned extends boolean = false,
> = ApiRouteBoundDeclaration<
  PagedResourceDeclaration<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue, TItem, TReconcile>,
  ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>,
  TProcessingOwned,
  TUploadOwned
> & ApiRouteOwnedDownloadsField<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue, TDownloadsOwned>;

export type ApiRouteProcessingPagedDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
  TItem = SignalValue,
  TReconcile extends ApiRouteReconcile<TValue, TItem> | undefined = undefined,
  TProcessingOwned extends boolean = false,
  TUploadOwned extends boolean = false,
  TDownloadsOwned extends boolean = false,
> = ApiRouteBoundDeclaration<
  ProcessingPagedResourceDeclaration<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue, TItem, TReconcile>,
  ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>,
  TProcessingOwned,
  TUploadOwned
> & ApiRouteOwnedDownloadsField<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue, TDownloadsOwned>;

export type ApiRouteUploadPagedDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
  TItem = SignalValue,
  TReconcile extends ApiRouteReconcile<TValue, TItem> | undefined = undefined,
  TProcessingOwned extends boolean = false,
  TUploadOwned extends boolean = false,
  TDownloadsOwned extends boolean = false,
> = ApiRouteBoundDeclaration<
  UploadPagedResourceDeclaration<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue, TItem, TReconcile>,
  ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>,
  TProcessingOwned,
  TUploadOwned
> & ApiRouteOwnedDownloadsField<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue, TDownloadsOwned>;

export type ApiRouteProcessingUploadPagedDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
  TItem = SignalValue,
  TReconcile extends ApiRouteReconcile<TValue, TItem> | undefined = undefined,
  TProcessingOwned extends boolean = false,
  TUploadOwned extends boolean = false,
  TDownloadsOwned extends boolean = false,
> = ApiRouteBoundDeclaration<
  ProcessingUploadPagedResourceDeclaration<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue, TItem, TReconcile>,
  ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>,
  TProcessingOwned,
  TUploadOwned
> & ApiRouteOwnedDownloadsField<ApiRouteDeclarationMemberParams<TRoute, TRequestParams, TBody>, TValue, TDownloadsOwned>;
