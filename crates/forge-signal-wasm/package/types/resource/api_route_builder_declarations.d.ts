import type {
  ResourceCollectionShape,
  ResourceItemAspectMap,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";
import type { ApiRequestParamsShape } from "./api_request_params.js";
import type { ApiRouteProcessingKind, ApiRouteUploadKind } from "./api_route_transfer_kinds.js";
import type {
  ApiRouteCollectionDeclaration,
  ApiRouteCreateDeclaration,
  ApiRouteDetailDeclaration,
  ApiRoutePagedDeclaration,
  ApiRouteProcessingCollectionDeclaration,
  ApiRouteProcessingCreateDeclaration,
  ApiRouteProcessingDetailDeclaration,
  ApiRouteProcessingPagedDeclaration,
  ApiRouteProcessingUploadCollectionDeclaration,
  ApiRouteProcessingUploadCreateDeclaration,
  ApiRouteProcessingUploadDetailDeclaration,
  ApiRouteProcessingUploadPagedDeclaration,
  ApiRouteUploadCollectionDeclaration,
  ApiRouteUploadCreateDeclaration,
  ApiRouteUploadDetailDeclaration,
  ApiRouteUploadPagedDeclaration,
} from "./api_route_declarations.js";

export type ApiRouteTransferValue<
  TValue,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
> = TProcessingKind extends "none"
  ? TUploadKind extends "none"
    ? TValue
    : TValue | null
  : TValue | null;

export type ApiRouteReconcile<
  TValue,
  TItem,
> = ResourceCollectionShape<
  TValue,
  TItem,
  ResourceItemAspectMap<TItem>,
  ResourceValueSummaryMap<TValue>,
  any
>;

export type ApiRouteOwnedHeadersDeclaration<
  TDeclaration,
  THeadersOwned extends boolean,
> = THeadersOwned extends true
  ? Omit<TDeclaration, "headers"> & { headers?: never }
  : TDeclaration;

export type ApiRouteResolvedDownloadValue<
  TValue,
  TDownloadValue,
  TDownloadsOwned extends boolean,
> = TDownloadsOwned extends true ? TDownloadValue : TValue;

export type ApiRouteSettledTransferValue<
  TValue,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
> = Awaited<ApiRouteTransferValue<Awaited<TValue>, TProcessingKind, TUploadKind>>;

export type ApiRouteDetailDeclarationForState<
  TRoute extends string,
  TValue,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TDownloadsOwned extends boolean,
  TBody = undefined,
> = TProcessingKind extends "none"
  ? TUploadKind extends "none"
    ? ApiRouteDetailDeclaration<TRoute, TValue, TRequestParams, TBody, false, false, TDownloadsOwned>
    : ApiRouteUploadDetailDeclaration<TRoute, TValue, TRequestParams, TBody, false, true, TDownloadsOwned>
  : TUploadKind extends "none"
    ? ApiRouteProcessingDetailDeclaration<TRoute, TValue, TRequestParams, TBody, true, false, TDownloadsOwned>
    : ApiRouteProcessingUploadDetailDeclaration<TRoute, TValue, TRequestParams, TBody, true, true, TDownloadsOwned>;

export type ApiRouteCreateDeclarationForState<
  TRoute extends string,
  TValue,
  TBody,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TDownloadsOwned extends boolean,
> = TProcessingKind extends "none"
  ? TUploadKind extends "none"
    ? ApiRouteCreateDeclaration<TRoute, TValue, TBody, TRequestParams, false, false, TDownloadsOwned>
    : ApiRouteUploadCreateDeclaration<TRoute, TValue, TBody, TRequestParams, false, true, TDownloadsOwned>
  : TUploadKind extends "none"
    ? ApiRouteProcessingCreateDeclaration<TRoute, TValue, TBody, TRequestParams, true, false, TDownloadsOwned>
    : ApiRouteProcessingUploadCreateDeclaration<TRoute, TValue, TBody, TRequestParams, true, true, TDownloadsOwned>;

export type ApiRouteCollectionDeclarationForState<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem,
  TReconcile extends ApiRouteReconcile<TValue, TItem> | undefined,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TDownloadsOwned extends boolean,
  TBody = undefined,
> = TProcessingKind extends "none"
  ? TUploadKind extends "none"
    ? ApiRouteCollectionDeclaration<TRoute, TRequestParams, TValue, TBody, TItem, TReconcile, false, false, TDownloadsOwned>
    : ApiRouteUploadCollectionDeclaration<TRoute, TRequestParams, TValue, TBody, TItem, TReconcile, false, true, TDownloadsOwned>
  : TUploadKind extends "none"
    ? ApiRouteProcessingCollectionDeclaration<TRoute, TRequestParams, TValue, TBody, TItem, TReconcile, true, false, TDownloadsOwned>
    : ApiRouteProcessingUploadCollectionDeclaration<TRoute, TRequestParams, TValue, TBody, TItem, TReconcile, true, true, TDownloadsOwned>;

export type ApiRoutePagedDeclarationForState<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem,
  TReconcile extends ApiRouteReconcile<TValue, TItem> | undefined,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TDownloadsOwned extends boolean,
  TBody = undefined,
> = TProcessingKind extends "none"
  ? TUploadKind extends "none"
    ? ApiRoutePagedDeclaration<TRoute, TRequestParams, TValue, TBody, TItem, TReconcile, false, false, TDownloadsOwned>
    : ApiRouteUploadPagedDeclaration<TRoute, TRequestParams, TValue, TBody, TItem, TReconcile, false, true, TDownloadsOwned>
  : TUploadKind extends "none"
    ? ApiRouteProcessingPagedDeclaration<TRoute, TRequestParams, TValue, TBody, TItem, TReconcile, true, false, TDownloadsOwned>
    : ApiRouteProcessingUploadPagedDeclaration<TRoute, TRequestParams, TValue, TBody, TItem, TReconcile, true, true, TDownloadsOwned>;
