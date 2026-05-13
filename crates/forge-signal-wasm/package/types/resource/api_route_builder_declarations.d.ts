import type {
  ResourceCollectionShape,
  ResourceDetailReconcile,
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
import type {
  ResourceMutationResponseIdentityDeclaration,
  ResourceMutationResponseAnyTargetDeclaration,
  ResourceMutationResponseDiagnosticDeclaration,
  ResourceMutationResponseFallbackTargetDeclaration,
} from "./resource_mutation_response.js";

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
  any,
  TItem,
  ResourceItemAspectMap<TItem>,
  any,
  any
>;

export type ApiRouteOwnedHeadersDeclaration<
  TDeclaration,
  THeadersOwned extends boolean,
> = THeadersOwned extends true
  ? Omit<TDeclaration, "headers"> & { headers?: never }
  : TDeclaration;

export type ApiRouteOwnedEffectsDeclaration<
  TDeclaration,
  TEffectsOwned extends boolean,
> = TEffectsOwned extends true
  ? Omit<TDeclaration, "effects"> & { effects?: never }
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
  TReconcile extends ResourceDetailReconcile<TValue> | undefined,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TDownloadsOwned extends boolean,
  TBody = undefined,
> = TProcessingKind extends "none"
  ? TUploadKind extends "none"
    ? ApiRouteDetailDeclaration<TRoute, TValue, TReconcile, TRequestParams, TBody, false, false, TDownloadsOwned>
    : ApiRouteUploadDetailDeclaration<TRoute, TValue, TReconcile, TRequestParams, TBody, false, true, TDownloadsOwned>
  : TUploadKind extends "none"
    ? ApiRouteProcessingDetailDeclaration<TRoute, TValue, TReconcile, TRequestParams, TBody, true, false, TDownloadsOwned>
    : ApiRouteProcessingUploadDetailDeclaration<TRoute, TValue, TReconcile, TRequestParams, TBody, true, true, TDownloadsOwned>;

export type ApiRouteCreateDeclarationForState<
  TRoute extends string,
  TValue,
  TBody,
  TReconcile extends ResourceDetailReconcile<TValue> | undefined,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TDownloadsOwned extends boolean,
> = TProcessingKind extends "none"
  ? TUploadKind extends "none"
    ? ApiRouteCreateDeclaration<TRoute, TValue, TBody, TReconcile, TRequestParams, false, false, TDownloadsOwned>
    : ApiRouteUploadCreateDeclaration<TRoute, TValue, TBody, TReconcile, TRequestParams, false, true, TDownloadsOwned>
  : TUploadKind extends "none"
    ? ApiRouteProcessingCreateDeclaration<TRoute, TValue, TBody, TReconcile, TRequestParams, true, false, TDownloadsOwned>
    : ApiRouteProcessingUploadCreateDeclaration<TRoute, TValue, TBody, TReconcile, TRequestParams, true, true, TDownloadsOwned>;

export type ApiRouteResponseCreateMutationDeclarationForState<
  TRoute extends string,
  TValue,
  TBody,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TDownloadsOwned extends boolean,
> = ApiRouteCreateDeclarationForState<
  TRoute,
  TValue,
  TBody,
  undefined,
  TRequestParams,
  TProcessingKind,
  TUploadKind,
  TDownloadsOwned
> & {
  reconciles?: readonly ResourceMutationResponseFallbackTargetDeclaration<
    import("./api_request_params.js").ApiRouteWriteDeclarationParams<
      TRoute,
      TRequestParams,
      TBody
    >
  >[];
  identity?: ResourceMutationResponseIdentityDeclaration<
    import("./api_request_params.js").ApiRouteWriteDeclarationParams<
      TRoute,
      TRequestParams,
      TBody
    >,
    TValue
  >;
};

export type ApiRouteResponseUpdateMutationDeclarationForState<
  TRoute extends string,
  TValue,
  TBody,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TDownloadsOwned extends boolean,
> = ApiRouteCreateDeclarationForState<
  TRoute,
  TValue,
  TBody,
  undefined,
  TRequestParams,
  TProcessingKind,
  TUploadKind,
  TDownloadsOwned
> & {
  reconciles?: readonly ResourceMutationResponseAnyTargetDeclaration<
    import("./api_request_params.js").ApiRouteWriteDeclarationParams<
      TRoute,
      TRequestParams,
      TBody
    >
  >[];
  diagnostics?: readonly ResourceMutationResponseDiagnosticDeclaration[];
  identity?: ResourceMutationResponseIdentityDeclaration<
    import("./api_request_params.js").ApiRouteWriteDeclarationParams<
      TRoute,
      TRequestParams,
      TBody
    >,
    TValue
  >;
};

export type ApiRouteResponseMutationDeclarationForState<
  TRoute extends string,
  TValue,
  TBody,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TDownloadsOwned extends boolean,
> = ApiRouteResponseUpdateMutationDeclarationForState<
  TRoute,
  TValue,
  TBody,
  TRequestParams,
  TProcessingKind,
  TUploadKind,
  TDownloadsOwned
>;

export type ApiRouteResponseRemoveMutationDeclarationForState<
  TRoute extends string,
  TValue,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TDownloadsOwned extends boolean,
> = ApiRouteDetailDeclarationForState<
  TRoute,
  TValue,
  undefined,
  TRequestParams,
  TProcessingKind,
  TUploadKind,
  TDownloadsOwned
> & {
  reconciles?: readonly ResourceMutationResponseFallbackTargetDeclaration<
    import("./api_request_params.js").ApiRouteDeclarationParams<
      TRoute,
      TRequestParams
    >
  >[];
  identity?: never;
};

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
