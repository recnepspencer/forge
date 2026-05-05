import type { SignalValue } from "../model.js";
import type { ResourceNamespace } from "./resource_namespace.js";
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
  ResourceAuthPosture,
  ResourceContinuationPosture,
  ResourceProcessingJobPosture,
  ResourceRequestContext,
  ResourceUploadTransportPosture,
} from "./resource_postures.js";
import type {
  ResourceCollectionShape,
  ResourceItemAspectMap,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";
import type {
  ApiCollectionResourceFamily,
  ApiDetailResourceFamily,
  ApiPagedResourceFamily,
  ApiRequestParamsShape,
  ApiRouteDeclarationParams,
  ApiRouteParamsConstraint,
} from "./api_request_params.js";
import type {
  ApiRouteConstraint,
  RoutePathParams,
} from "./api_route_types.js";

type ApiRouteHeaders<TParams> =
  | Record<string, string>
  | ((params: TParams) => Record<string, string>);

export interface ApiScopedDefaults<
  TParams extends object = Record<string, SignalValue>,
> {
  baseUrl?: string | ((params: TParams) => string);
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  headers?: ApiRouteHeaders<TParams>;
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  processingJob?:
    | ResourceProcessingJobPosture
    | ((params: TParams) => ResourceProcessingJobPosture);
  uploadTransport?:
    | ResourceUploadTransportPosture
    | ((params: TParams) => ResourceUploadTransportPosture);
}

type ApiRouteBoundDeclaration<TDeclaration, TParams> =
  Omit<TDeclaration, "params" | "normalizeParams"> & {
    headers?: ApiRouteHeaders<TParams>;
    params?: never;
    normalizeParams?: never;
    baseUrl?: never;
  };

type ApiRouteDetailDeclaration<
  TRoute extends string,
  TValue,
  TRequestParams extends ApiRequestParamsShape | undefined = undefined,
> =
  ApiRouteBoundDeclaration<
    DetailResourceDeclaration<ApiRouteDeclarationParams<TRoute, TRequestParams>, TValue>,
    ApiRouteDeclarationParams<TRoute, TRequestParams>
  >;

type ApiRouteProcessingDetailDeclaration<
  TRoute extends string,
  TValue,
  TRequestParams extends ApiRequestParamsShape | undefined = undefined,
> =
  ApiRouteBoundDeclaration<
    ProcessingDetailResourceDeclaration<ApiRouteDeclarationParams<TRoute, TRequestParams>, TValue>,
    ApiRouteDeclarationParams<TRoute, TRequestParams>
  >;

type ApiRouteUploadDetailDeclaration<
  TRoute extends string,
  TValue,
  TRequestParams extends ApiRequestParamsShape | undefined = undefined,
> =
  ApiRouteBoundDeclaration<
    UploadDetailResourceDeclaration<ApiRouteDeclarationParams<TRoute, TRequestParams>, TValue>,
    ApiRouteDeclarationParams<TRoute, TRequestParams>
  >;

type ApiRouteProcessingUploadDetailDeclaration<
  TRoute extends string,
  TValue,
  TRequestParams extends ApiRequestParamsShape | undefined = undefined,
> =
  ApiRouteBoundDeclaration<
    ProcessingUploadDetailResourceDeclaration<ApiRouteDeclarationParams<TRoute, TRequestParams>, TValue>,
    ApiRouteDeclarationParams<TRoute, TRequestParams>
  >;

type ApiRouteCollectionDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>,
    any
  > | undefined = undefined,
> = ApiRouteBoundDeclaration<
  CollectionResourceDeclaration<ApiRouteDeclarationParams<TRoute, TRequestParams>, TValue, TItem, TReconcile>,
  ApiRouteDeclarationParams<TRoute, TRequestParams>
>;

type ApiRouteProcessingCollectionDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>,
    any
  > | undefined = undefined,
> = ApiRouteBoundDeclaration<
  ProcessingCollectionResourceDeclaration<ApiRouteDeclarationParams<TRoute, TRequestParams>, TValue, TItem, TReconcile>,
  ApiRouteDeclarationParams<TRoute, TRequestParams>
>;

type ApiRouteUploadCollectionDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>,
    any
  > | undefined = undefined,
> = ApiRouteBoundDeclaration<
  UploadCollectionResourceDeclaration<ApiRouteDeclarationParams<TRoute, TRequestParams>, TValue, TItem, TReconcile>,
  ApiRouteDeclarationParams<TRoute, TRequestParams>
>;

type ApiRouteProcessingUploadCollectionDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>,
    any
  > | undefined = undefined,
> = ApiRouteBoundDeclaration<
  ProcessingUploadCollectionResourceDeclaration<ApiRouteDeclarationParams<TRoute, TRequestParams>, TValue, TItem, TReconcile>,
  ApiRouteDeclarationParams<TRoute, TRequestParams>
>;

type ApiRoutePagedDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>,
    any
  > | undefined = undefined,
> = ApiRouteBoundDeclaration<
  PagedResourceDeclaration<ApiRouteDeclarationParams<TRoute, TRequestParams>, TValue, TItem, TReconcile>,
  ApiRouteDeclarationParams<TRoute, TRequestParams>
>;

type ApiRouteProcessingPagedDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>,
    any
  > | undefined = undefined,
> = ApiRouteBoundDeclaration<
  ProcessingPagedResourceDeclaration<ApiRouteDeclarationParams<TRoute, TRequestParams>, TValue, TItem, TReconcile>,
  ApiRouteDeclarationParams<TRoute, TRequestParams>
>;

type ApiRouteUploadPagedDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>,
    any
  > | undefined = undefined,
> = ApiRouteBoundDeclaration<
  UploadPagedResourceDeclaration<ApiRouteDeclarationParams<TRoute, TRequestParams>, TValue, TItem, TReconcile>,
  ApiRouteDeclarationParams<TRoute, TRequestParams>
>;

type ApiRouteProcessingUploadPagedDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>,
    any
  > | undefined = undefined,
> = ApiRouteBoundDeclaration<
  ProcessingUploadPagedResourceDeclaration<ApiRouteDeclarationParams<TRoute, TRequestParams>, TValue, TItem, TReconcile>,
  ApiRouteDeclarationParams<TRoute, TRequestParams>
>;

type ApiRouteBuilderParamsStep<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
> = [TRequestParams] extends [undefined]
  ? ApiRouteParamsConstraint<TRoute> extends {
      readonly __forgeInvalidApiRequestParams__: string;
    }
    ? {}
    : {
        params<TNextRequestParams extends ApiRequestParamsShape>(): ApiRouteBuilder<TRoute, TNextRequestParams>;
      }
  : {};

interface ApiRouteBuilderBase<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
> {
  detail<TValue>(
    declaration: ApiRouteProcessingUploadDetailDeclaration<TRoute, TValue, TRequestParams>,
  ): ApiDetailResourceFamily<TRoute, TRequestParams, TValue | null>;
  detail<TValue>(
    declaration: ApiRouteProcessingDetailDeclaration<TRoute, TValue, TRequestParams>,
  ): ApiDetailResourceFamily<TRoute, TRequestParams, TValue | null>;
  detail<TValue>(
    declaration: ApiRouteUploadDetailDeclaration<TRoute, TValue, TRequestParams>,
  ): ApiDetailResourceFamily<TRoute, TRequestParams, TValue | null>;
  detail<TValue>(
    declaration: ApiRouteDetailDeclaration<TRoute, TValue, TRequestParams>,
  ): ApiDetailResourceFamily<TRoute, TRequestParams, TValue>;
  list<
    TValue,
    TItem = SignalValue,
    TReconcile extends ResourceCollectionShape<
      TValue,
      TItem,
      ResourceItemAspectMap<TItem>,
      ResourceValueSummaryMap<TValue>,
      any
    > | undefined = undefined,
  >(
    declaration: ApiRouteProcessingUploadCollectionDeclaration<TRoute, TRequestParams, TValue, TItem, TReconcile>,
  ): ApiCollectionResourceFamily<TRoute, TRequestParams, TValue | null, TItem, TReconcile>;
  list<
    TValue,
    TItem = SignalValue,
    TReconcile extends ResourceCollectionShape<
      TValue,
      TItem,
      ResourceItemAspectMap<TItem>,
      ResourceValueSummaryMap<TValue>,
      any
    > | undefined = undefined,
  >(
    declaration: ApiRouteProcessingCollectionDeclaration<TRoute, TRequestParams, TValue, TItem, TReconcile>,
  ): ApiCollectionResourceFamily<TRoute, TRequestParams, TValue | null, TItem, TReconcile>;
  list<
    TValue,
    TItem = SignalValue,
    TReconcile extends ResourceCollectionShape<
      TValue,
      TItem,
      ResourceItemAspectMap<TItem>,
      ResourceValueSummaryMap<TValue>,
      any
    > | undefined = undefined,
  >(
    declaration: ApiRouteUploadCollectionDeclaration<TRoute, TRequestParams, TValue, TItem, TReconcile>,
  ): ApiCollectionResourceFamily<TRoute, TRequestParams, TValue | null, TItem, TReconcile>;
  list<
    TValue,
    TItem = SignalValue,
    TReconcile extends ResourceCollectionShape<
      TValue,
      TItem,
      ResourceItemAspectMap<TItem>,
      ResourceValueSummaryMap<TValue>,
      any
    > | undefined = undefined,
  >(
    declaration: ApiRouteCollectionDeclaration<TRoute, TRequestParams, TValue, TItem, TReconcile>,
  ): ApiCollectionResourceFamily<TRoute, TRequestParams, TValue, TItem, TReconcile>;
  paged<
    TValue,
    TItem = SignalValue,
    TReconcile extends ResourceCollectionShape<
      TValue,
      TItem,
      ResourceItemAspectMap<TItem>,
      ResourceValueSummaryMap<TValue>,
      any
    > | undefined = undefined,
  >(
    declaration: ApiRouteProcessingUploadPagedDeclaration<TRoute, TRequestParams, TValue, TItem, TReconcile>,
  ): ApiPagedResourceFamily<TRoute, TRequestParams, TValue | null, TItem, TReconcile>;
  paged<
    TValue,
    TItem = SignalValue,
    TReconcile extends ResourceCollectionShape<
      TValue,
      TItem,
      ResourceItemAspectMap<TItem>,
      ResourceValueSummaryMap<TValue>,
      any
    > | undefined = undefined,
  >(
    declaration: ApiRouteProcessingPagedDeclaration<TRoute, TRequestParams, TValue, TItem, TReconcile>,
  ): ApiPagedResourceFamily<TRoute, TRequestParams, TValue | null, TItem, TReconcile>;
  paged<
    TValue,
    TItem = SignalValue,
    TReconcile extends ResourceCollectionShape<
      TValue,
      TItem,
      ResourceItemAspectMap<TItem>,
      ResourceValueSummaryMap<TValue>,
      any
    > | undefined = undefined,
  >(
    declaration: ApiRouteUploadPagedDeclaration<TRoute, TRequestParams, TValue, TItem, TReconcile>,
  ): ApiPagedResourceFamily<TRoute, TRequestParams, TValue | null, TItem, TReconcile>;
  paged<
    TValue,
    TItem = SignalValue,
    TReconcile extends ResourceCollectionShape<
      TValue,
      TItem,
      ResourceItemAspectMap<TItem>,
      ResourceValueSummaryMap<TValue>,
      any
    > | undefined = undefined,
  >(
    declaration: ApiRoutePagedDeclaration<TRoute, TRequestParams, TValue, TItem, TReconcile>,
  ): ApiPagedResourceFamily<TRoute, TRequestParams, TValue, TItem, TReconcile>;
}

export type ApiRouteBuilder<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined = undefined,
> = ApiRouteBuilderBase<TRoute, TRequestParams>
  & ApiRouteBuilderParamsStep<TRoute, TRequestParams>;

export interface ApiNamespace
  extends Pick<ResourceNamespace, "detail" | "collection" | "paged"> {
  scope<TParams extends object = Record<string, SignalValue>>(
    options?: ApiScopedDefaults<TParams>,
  ): ApiNamespace;
  url<TRoute extends string>(
    route: TRoute & ApiRouteConstraint<TRoute>,
  ): ApiRouteBuilder<TRoute>;
}

export interface ApiFactory {
  <TParams extends object = Record<string, SignalValue>>(
    options?: ApiScopedDefaults<TParams>,
  ): ApiNamespace;
}
