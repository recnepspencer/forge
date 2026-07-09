import type { SignalValue } from "../model.js";
import type {
  ResourceDetailFieldMap,
  ResourceDetailFieldValue,
  ResourceDetailFields,
} from "./resource_detail_fields.js";
import type {
  ResourceDetailRegionValue,
  ResourceDetailRegions,
} from "./resource_detail_regions.js";
import type {
  ResourceDetailJsonPathValue,
  ResourceDetailJsonPaths,
} from "./resource_detail_json_paths.js";
import type {
  CollectionResourceFamily,
  DisabledResourceFamilySelection,
  DetailResourceFamily,
  PagedResourceFamily,
  ResourceDetailPatchCapableLine,
  ResourcePatchCapableLine,
} from "./resource_family_surfaces.js";
import type {
  ResourceLine,
  ResourceLineExecution,
  ResourceLineExecutionOptions,
} from "./resource_lifecycle.js";
import type {
  ApiFamilyDeliveryHelpers,
  ApiFamilyPatchHelpers,
} from "./api_family_helper_types.js";
export type {
  ApiFamilyDeliveryHelpers,
  ApiFamilyPatchHelpers,
} from "./api_family_helper_types.js";
import type {
  InvalidateResourceDelivery,
  ResourceCollectionShape,
  ResourceDetailReconcile,
  ResourceSummaryPatchScope,
  ResourceItemAspectMap,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";
import type {
  RouteParamNames,
  RoutePathParams,
} from "./api_route_types.js";

export type ApiRequestParamScalar = string | number | boolean;

export type ApiRequestParamValue =
  | ApiRequestParamScalar
  | readonly ApiRequestParamScalar[];

export type ApiRequestParamsShape = Readonly<
  Record<string, ApiRequestParamValue | undefined>
>;

type ApiRouteMemberParamsBase<TRoute extends string> = RoutePathParams<TRoute>;

type ApiRouteHasPathParams<TRoute extends string> =
  [RouteParamNames<TRoute>] extends [never] ? false : true;

export type ApiRouteMemberParams<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
> = [TRequestParams] extends [undefined]
  ? ApiRouteMemberParamsBase<TRoute>
  : ApiRouteHasPathParams<TRoute> extends true
    ? ApiRouteMemberParamsBase<TRoute> & {
        params: TRequestParams;
      }
    : {
        params: TRequestParams;
      };

export type ApiRouteWriteMemberParams<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TBody,
> = ([TRequestParams] extends [undefined]
  ? ApiRouteHasPathParams<TRoute> extends true
    ? ApiRouteMemberParamsBase<TRoute>
    : {}
  : ApiRouteHasPathParams<TRoute> extends true
    ? ApiRouteMemberParamsBase<TRoute> & {
        params: TRequestParams;
      }
    : {
        params: TRequestParams;
      }) & {
  body: TBody;
};

type ExactKeys<TExpected, TActual extends TExpected> =
  Exclude<keyof TActual, keyof TExpected> extends never ? TActual : never;

type ExactNestedParams<
  TExpected,
  TActual extends TExpected,
> = TExpected extends { params: infer TExpectedRequest extends object }
  ? TActual extends { params: infer TActualRequest extends TExpectedRequest }
    ? Exclude<keyof Omit<TActual, "params" | "body">, keyof Omit<TExpected, "params" | "body">> extends never
      ? Exclude<keyof TActualRequest, keyof TExpectedRequest> extends never
        ? ExactBodyKey<TExpected, TActual>
        : never
      : never
    : never
  : ExactBodyKey<TExpected, TActual>;

type ExactBodyKey<TExpected, TActual extends TExpected> =
  TExpected extends { body: infer TExpectedBody }
    ? TActual extends { body: infer TActualBody extends TExpectedBody }
      ? Exclude<keyof Omit<TActual, "body">, keyof Omit<TExpected, "body">> extends never
        ? TActual
        : never
      : never
    : ExactKeys<TExpected, TActual>;

export type ExactApiRouteMemberParams<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TActualParams extends ApiRouteMemberParams<TRoute, TRequestParams>,
> = ExactNestedParams<ApiRouteMemberParams<TRoute, TRequestParams>, TActualParams>;

export type ExactApiRouteWriteMemberParams<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TBody,
  TActualParams extends ApiRouteWriteMemberParams<TRoute, TRequestParams, TBody>,
> = ExactNestedParams<
  ApiRouteWriteMemberParams<TRoute, TRequestParams, TBody>,
  TActualParams
>;

export type ApiRouteDeclarationParams<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
> = ApiRouteMemberParams<TRoute, TRequestParams>;

export type ApiRouteWriteDeclarationParams<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TBody,
> = ApiRouteWriteMemberParams<TRoute, TRequestParams, TBody>;

export type ApiRouteLineParams<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TBody = undefined,
> = [TBody] extends [undefined]
  ? ApiRouteMemberParams<TRoute, TRequestParams>
  : ApiRouteWriteMemberParams<TRoute, TRequestParams, TBody>;

export type ExactApiRouteLineParams<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TBody,
  TActualParams extends ApiRouteLineParams<TRoute, TRequestParams, TBody>,
> = [TBody] extends [undefined]
  ? ExactApiRouteMemberParams<
      TRoute,
      TRequestParams,
      Extract<TActualParams, ApiRouteMemberParams<TRoute, TRequestParams>>
    >
  : ExactApiRouteWriteMemberParams<
      TRoute,
      TRequestParams,
      TBody,
      Extract<TActualParams, ApiRouteWriteMemberParams<TRoute, TRequestParams, TBody>>
    >;

export type ApiRouteParamsConstraint<TRoute extends string> =
  "params" extends RouteParamNames<TRoute>
    ? {
        readonly __WORTHInvalidApiRequestParams__:
          "api.url(...).params(...) cannot be used when the route already has a :params path placeholder";
      }
    : unknown;

export type ApiImplicitArrayReconcile<
  TValue extends readonly TItem[],
  TItem = TValue[number],
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
> = ResourceCollectionShape<
  TValue,
  TItem,
  TAspectMap,
  TSummaryMap,
  TSummaryPatchScope
>;

export type ApiInlineReconcile<
  TValue,
  TItem = SignalValue,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
> = ResourceCollectionShape<
  TValue,
  TItem,
  TAspectMap,
  TSummaryMap,
  TSummaryPatchScope
>;

export interface ApiDetailResourceFamily<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
  TReconcile extends ResourceDetailReconcile<TValue> | undefined = undefined,
> {
  readonly patch: ApiFamilyPatchHelpers<TValue, never, TReconcile, "detail">;
  readonly delivery: ApiFamilyDeliveryHelpers<TValue, never, TReconcile, "detail">;
  invalidate<TActualParams extends ApiRouteLineParams<TRoute, TRequestParams, TBody>>(
    params: ExactApiRouteLineParams<TRoute, TRequestParams, TBody, TActualParams>,
  ): boolean;
  invalidateAll(): number;
  line<TActualParams extends ApiRouteLineParams<TRoute, TRequestParams, TBody>>(
    params: ExactApiRouteLineParams<TRoute, TRequestParams, TBody, TActualParams>,
  ): ResourceDetailPatchCapableLine<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TReconcile
  >;
  optionalLine<TActualParams extends ApiRouteLineParams<TRoute, TRequestParams, TBody>>(
    selection:
      | ExactApiRouteLineParams<TRoute, TRequestParams, TBody, TActualParams>
      | null
      | undefined
      | DisabledResourceFamilySelection,
  ): ResourceDetailPatchCapableLine<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TReconcile
  > | null;
  execute<TActualParams extends ApiRouteLineParams<TRoute, TRequestParams, TBody>>(
    params: ExactApiRouteLineParams<TRoute, TRequestParams, TBody, TActualParams>,
    options?: ResourceLineExecutionOptions,
  ): ResourceLineExecution<ApiRouteLineParams<TRoute, TRequestParams, TBody>, TValue | null>;
}

export interface ApiCollectionResourceFamily<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    any,
    TItem,
    ResourceItemAspectMap<TItem>,
    any,
    any
  > | undefined = undefined,
  TBody = undefined,
> {
  readonly patch: ApiFamilyPatchHelpers<TValue, TItem, TReconcile, "collection">;
  readonly delivery: ApiFamilyDeliveryHelpers<
    TValue,
    TItem,
    TReconcile,
    "collection"
  >;
  invalidate<TActualParams extends ApiRouteLineParams<TRoute, TRequestParams, TBody>>(
    params: ExactApiRouteLineParams<TRoute, TRequestParams, TBody, TActualParams>,
  ): boolean;
  invalidateAll(): number;
  line<TActualParams extends ApiRouteLineParams<TRoute, TRequestParams, TBody>>(
    params: ExactApiRouteLineParams<TRoute, TRequestParams, TBody, TActualParams>,
  ): ResourcePatchCapableLine<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    TReconcile,
    "collection"
  >;
  optionalLine<TActualParams extends ApiRouteLineParams<TRoute, TRequestParams, TBody>>(
    selection:
      | ExactApiRouteLineParams<TRoute, TRequestParams, TBody, TActualParams>
      | null
      | undefined
      | DisabledResourceFamilySelection,
  ): ResourcePatchCapableLine<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    TReconcile,
    "collection"
  > | null;
  execute<TActualParams extends ApiRouteLineParams<TRoute, TRequestParams, TBody>>(
    params: ExactApiRouteLineParams<TRoute, TRequestParams, TBody, TActualParams>,
    options?: ResourceLineExecutionOptions,
  ): ResourceLineExecution<ApiRouteLineParams<TRoute, TRequestParams, TBody>, TValue | null>;
}

export interface ApiPagedResourceFamily<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    any,
    TItem,
    ResourceItemAspectMap<TItem>,
    any,
    any
  > | undefined = undefined,
  TBody = undefined,
> {
  readonly patch: ApiFamilyPatchHelpers<TValue, TItem, TReconcile, "paged">;
  readonly delivery: ApiFamilyDeliveryHelpers<TValue, TItem, TReconcile, "paged">;
  invalidate<TActualParams extends ApiRouteLineParams<TRoute, TRequestParams, TBody>>(
    params: ExactApiRouteLineParams<TRoute, TRequestParams, TBody, TActualParams>,
  ): boolean;
  invalidateAll(): number;
  line<TActualParams extends ApiRouteLineParams<TRoute, TRequestParams, TBody>>(
    params: ExactApiRouteLineParams<TRoute, TRequestParams, TBody, TActualParams>,
  ): ResourcePatchCapableLine<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    TReconcile,
    "paged"
  >;
  optionalLine<TActualParams extends ApiRouteLineParams<TRoute, TRequestParams, TBody>>(
    selection:
      | ExactApiRouteLineParams<TRoute, TRequestParams, TBody, TActualParams>
      | null
      | undefined
      | DisabledResourceFamilySelection,
  ): ResourcePatchCapableLine<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    TReconcile,
    "paged"
  > | null;
  execute<TActualParams extends ApiRouteLineParams<TRoute, TRequestParams, TBody>>(
    params: ExactApiRouteLineParams<TRoute, TRequestParams, TBody, TActualParams>,
    options?: ResourceLineExecutionOptions,
  ): ResourceLineExecution<ApiRouteLineParams<TRoute, TRequestParams, TBody>, TValue | null>;
}

export type {
  CollectionResourceFamily,
  DetailResourceFamily,
  PagedResourceFamily,
};
