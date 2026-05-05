import type { SignalValue } from "../model.js";
import type { ResourceNamespace } from "./resource_namespace.js";
import type {
  DetailResourceFamily,
  CollectionResourceFamily,
  PagedResourceFamily,
} from "./resource_family_surfaces.js";
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
  ApiRouteConstraint,
  RoutePathParams,
} from "./api_route_types.js";

type ApiRouteHeaders<TParams> =
  | Record<string, string>
  | ((params: TParams) => Record<string, string>);

export interface ApiScopedDefaults<
  TParams extends object = Record<string, SignalValue>,
> {
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

type ApiRouteDetailDeclaration<TRoute extends string, TValue> =
  ApiRouteBoundDeclaration<
    DetailResourceDeclaration<RoutePathParams<TRoute>, TValue>,
    RoutePathParams<TRoute>
  >;

type ApiRouteProcessingDetailDeclaration<TRoute extends string, TValue> =
  ApiRouteBoundDeclaration<
    ProcessingDetailResourceDeclaration<RoutePathParams<TRoute>, TValue>,
    RoutePathParams<TRoute>
  >;

type ApiRouteUploadDetailDeclaration<TRoute extends string, TValue> =
  ApiRouteBoundDeclaration<
    UploadDetailResourceDeclaration<RoutePathParams<TRoute>, TValue>,
    RoutePathParams<TRoute>
  >;

type ApiRouteProcessingUploadDetailDeclaration<TRoute extends string, TValue> =
  ApiRouteBoundDeclaration<
    ProcessingUploadDetailResourceDeclaration<RoutePathParams<TRoute>, TValue>,
    RoutePathParams<TRoute>
  >;

type ApiRouteCollectionDeclaration<
  TRoute extends string,
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
  CollectionResourceDeclaration<RoutePathParams<TRoute>, TValue, TItem, TReconcile>,
  RoutePathParams<TRoute>
>;

type ApiRouteProcessingCollectionDeclaration<
  TRoute extends string,
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
  ProcessingCollectionResourceDeclaration<RoutePathParams<TRoute>, TValue, TItem, TReconcile>,
  RoutePathParams<TRoute>
>;

type ApiRouteUploadCollectionDeclaration<
  TRoute extends string,
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
  UploadCollectionResourceDeclaration<RoutePathParams<TRoute>, TValue, TItem, TReconcile>,
  RoutePathParams<TRoute>
>;

type ApiRouteProcessingUploadCollectionDeclaration<
  TRoute extends string,
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
  ProcessingUploadCollectionResourceDeclaration<RoutePathParams<TRoute>, TValue, TItem, TReconcile>,
  RoutePathParams<TRoute>
>;

type ApiRoutePagedDeclaration<
  TRoute extends string,
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
  PagedResourceDeclaration<RoutePathParams<TRoute>, TValue, TItem, TReconcile>,
  RoutePathParams<TRoute>
>;

type ApiRouteProcessingPagedDeclaration<
  TRoute extends string,
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
  ProcessingPagedResourceDeclaration<RoutePathParams<TRoute>, TValue, TItem, TReconcile>,
  RoutePathParams<TRoute>
>;

type ApiRouteUploadPagedDeclaration<
  TRoute extends string,
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
  UploadPagedResourceDeclaration<RoutePathParams<TRoute>, TValue, TItem, TReconcile>,
  RoutePathParams<TRoute>
>;

type ApiRouteProcessingUploadPagedDeclaration<
  TRoute extends string,
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
  ProcessingUploadPagedResourceDeclaration<RoutePathParams<TRoute>, TValue, TItem, TReconcile>,
  RoutePathParams<TRoute>
>;

export interface ApiRouteBuilder<TRoute extends string> {
  detail<TValue>(
    declaration: ApiRouteProcessingUploadDetailDeclaration<TRoute, TValue>,
  ): DetailResourceFamily<RoutePathParams<TRoute>, TValue | null>;
  detail<TValue>(
    declaration: ApiRouteProcessingDetailDeclaration<TRoute, TValue>,
  ): DetailResourceFamily<RoutePathParams<TRoute>, TValue | null>;
  detail<TValue>(
    declaration: ApiRouteUploadDetailDeclaration<TRoute, TValue>,
  ): DetailResourceFamily<RoutePathParams<TRoute>, TValue | null>;
  detail<TValue>(
    declaration: ApiRouteDetailDeclaration<TRoute, TValue>,
  ): DetailResourceFamily<RoutePathParams<TRoute>, TValue>;
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
    declaration: ApiRouteProcessingUploadCollectionDeclaration<TRoute, TValue, TItem, TReconcile>,
  ): CollectionResourceFamily<RoutePathParams<TRoute>, TValue | null, TItem, TReconcile>;
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
    declaration: ApiRouteProcessingCollectionDeclaration<TRoute, TValue, TItem, TReconcile>,
  ): CollectionResourceFamily<RoutePathParams<TRoute>, TValue | null, TItem, TReconcile>;
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
    declaration: ApiRouteUploadCollectionDeclaration<TRoute, TValue, TItem, TReconcile>,
  ): CollectionResourceFamily<RoutePathParams<TRoute>, TValue | null, TItem, TReconcile>;
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
    declaration: ApiRouteCollectionDeclaration<TRoute, TValue, TItem, TReconcile>,
  ): CollectionResourceFamily<RoutePathParams<TRoute>, TValue, TItem, TReconcile>;
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
    declaration: ApiRouteProcessingUploadPagedDeclaration<TRoute, TValue, TItem, TReconcile>,
  ): PagedResourceFamily<RoutePathParams<TRoute>, TValue | null, TItem, TReconcile>;
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
    declaration: ApiRouteProcessingPagedDeclaration<TRoute, TValue, TItem, TReconcile>,
  ): PagedResourceFamily<RoutePathParams<TRoute>, TValue | null, TItem, TReconcile>;
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
    declaration: ApiRouteUploadPagedDeclaration<TRoute, TValue, TItem, TReconcile>,
  ): PagedResourceFamily<RoutePathParams<TRoute>, TValue | null, TItem, TReconcile>;
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
    declaration: ApiRoutePagedDeclaration<TRoute, TValue, TItem, TReconcile>,
  ): PagedResourceFamily<RoutePathParams<TRoute>, TValue, TItem, TReconcile>;
}

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
