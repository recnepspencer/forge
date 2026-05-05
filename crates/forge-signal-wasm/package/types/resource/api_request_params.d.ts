import type { SignalValue } from "../model.js";
import type {
  CollectionResourceFamily,
  DetailResourceFamily,
  PagedResourceFamily,
  ResourcePatchCapableLine,
} from "./resource_family_surfaces.js";
import type { ResourceLine } from "./resource_lifecycle.js";
import type {
  ResourceCollectionShape,
  ResourceItemAspectValue,
  ResourcePatchForReconcile,
  ResourceReconcileAspectMap,
  ResourceReconcileSummaryMap,
  ResourceReconcileSummaryPatchScope,
  ResourceItemAspectMap,
  ResourceValueSummaryMap,
  ResourceValueSummaryValue,
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

type ExactKeys<TExpected, TActual extends TExpected> =
  Exclude<keyof TActual, keyof TExpected> extends never ? TActual : never;

type ExactNestedParams<
  TExpected,
  TActual extends TExpected,
> = TExpected extends { params: infer TExpectedRequest extends object }
  ? TActual extends { params: infer TActualRequest extends TExpectedRequest }
    ? Exclude<keyof Omit<TActual, "params">, keyof Omit<TExpected, "params">> extends never
      ? Exclude<keyof TActualRequest, keyof TExpectedRequest> extends never
        ? TActual
        : never
      : never
    : never
  : ExactKeys<TExpected, TActual>;

export type ExactApiRouteMemberParams<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TActualParams extends ApiRouteMemberParams<TRoute, TRequestParams>,
> = ExactNestedParams<ApiRouteMemberParams<TRoute, TRequestParams>, TActualParams>;

export type ApiRouteDeclarationParams<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
> = ApiRouteMemberParams<TRoute, TRequestParams>;

export type ApiRouteParamsConstraint<TRoute extends string> =
  "params" extends RouteParamNames<TRoute>
    ? {
        readonly __forgeInvalidApiRequestParams__:
          "api.url(...).params(...) cannot be used when the route already has a :params path placeholder";
      }
    : unknown;

type ApiFamilyPatchReplaceHelper<TValue> = {
  replace(nextValue: TValue): ResourcePatchForReconcile<TValue, never, undefined>;
};

type ApiFamilyPatchItemHelper<TValue, TItem, TReconcile> = [TReconcile] extends [
  ResourceCollectionShape<any, TItem, any, any>,
]
  ? {
      item(options: {
        itemId: string;
        nextItem: TItem;
      }): ResourcePatchForReconcile<TValue, TItem, TReconcile>;
    }
  : {};

type ApiFamilyPatchAspectNames<TItem, TReconcile> =
  keyof ResourceReconcileAspectMap<TReconcile> & string;

type ApiFamilyPatchAspectHelper<TValue, TItem, TReconcile> = [ApiFamilyPatchAspectNames<
  TItem,
  TReconcile
>] extends [never]
  ? {}
  : {
      itemAspect<TAspect extends ApiFamilyPatchAspectNames<TItem, TReconcile>>(
        options: {
          itemId: string;
          aspect: TAspect;
          value: ResourceItemAspectValue<
            ResourceReconcileAspectMap<TReconcile>[TAspect]
          >;
        },
      ): ResourcePatchForReconcile<TValue, TItem, TReconcile>;
    };

type ApiFamilyPatchSummaryNames<TValue, TReconcile, TFamilyKind> =
  TFamilyKind extends "paged"
    ? ResourceReconcileSummaryPatchScope<TReconcile> extends "pageWindow"
      ? keyof ResourceReconcileSummaryMap<TReconcile> & string
      : never
    : keyof ResourceReconcileSummaryMap<TReconcile> & string;

type ApiFamilyPatchSummaryHelper<
  TValue,
  TItem,
  TReconcile,
  TFamilyKind,
> = [ApiFamilyPatchSummaryNames<TValue, TReconcile, TFamilyKind>] extends [never]
  ? {}
  : {
      summary<
        TSummary extends ApiFamilyPatchSummaryNames<TValue, TReconcile, TFamilyKind>,
      >(options: {
        summary: TSummary;
        value: ResourceValueSummaryValue<
          ResourceReconcileSummaryMap<TReconcile>[TSummary]
        >;
      }): ResourcePatchForReconcile<TValue, TItem, TReconcile, TFamilyKind>;
    };

export type ApiFamilyPatchHelpers<
  TValue,
  TItem,
  TReconcile,
  TFamilyKind extends "collection" | "paged",
> =
  & ApiFamilyPatchReplaceHelper<TValue>
  & ApiFamilyPatchItemHelper<TValue, TItem, TReconcile>
  & ApiFamilyPatchAspectHelper<TValue, TItem, TReconcile>
  & ApiFamilyPatchSummaryHelper<TValue, TItem, TReconcile, TFamilyKind>;

export interface ApiDetailResourceFamily<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
> {
  readonly patch: ApiFamilyPatchHelpers<TValue, TItem, TReconcile, "collection">;
  invalidate<
    TActualParams extends ApiRouteMemberParams<TRoute, TRequestParams>,
  >(
    params: ExactApiRouteMemberParams<TRoute, TRequestParams, TActualParams>,
  ): boolean;
  invalidateAll(): number;
  line<
    TActualParams extends ApiRouteMemberParams<TRoute, TRequestParams>,
  >(
    params: ExactApiRouteMemberParams<TRoute, TRequestParams, TActualParams>,
  ): ResourceLine<ApiRouteMemberParams<TRoute, TRequestParams>, TValue | null>;
}

export interface ApiCollectionResourceFamily<
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
> {
  readonly patch: ApiFamilyPatchHelpers<TValue, TItem, TReconcile, "paged">;
  invalidate<
    TActualParams extends ApiRouteMemberParams<TRoute, TRequestParams>,
  >(
    params: ExactApiRouteMemberParams<TRoute, TRequestParams, TActualParams>,
  ): boolean;
  invalidateAll(): number;
  line<
    TActualParams extends ApiRouteMemberParams<TRoute, TRequestParams>,
  >(
    params: ExactApiRouteMemberParams<TRoute, TRequestParams, TActualParams>,
  ): ResourcePatchCapableLine<
    ApiRouteMemberParams<TRoute, TRequestParams>,
    TValue,
    TItem,
    TReconcile,
    "collection"
  >;
}

export interface ApiPagedResourceFamily<
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
> {
  invalidate<
    TActualParams extends ApiRouteMemberParams<TRoute, TRequestParams>,
  >(
    params: ExactApiRouteMemberParams<TRoute, TRequestParams, TActualParams>,
  ): boolean;
  invalidateAll(): number;
  line<
    TActualParams extends ApiRouteMemberParams<TRoute, TRequestParams>,
  >(
    params: ExactApiRouteMemberParams<TRoute, TRequestParams, TActualParams>,
  ): ResourcePatchCapableLine<
    ApiRouteMemberParams<TRoute, TRequestParams>,
    TValue,
    TItem,
    TReconcile,
    "paged"
  >;
}

export type {
  CollectionResourceFamily,
  DetailResourceFamily,
  PagedResourceFamily,
};
