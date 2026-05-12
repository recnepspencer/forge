import type { SignalValue } from "../model.js";
import type {
  CollectionResourceFamily,
  DetailResourceFamily,
  PagedResourceFamily,
  ResourcePatchCapableLine,
} from "./resource_family_surfaces.js";
import type { ResourceLine } from "./resource_lifecycle.js";
import type {
  InvalidateResourceDelivery,
  PatchResourceDelivery,
  ReplaceResourceDelivery,
  ResourceCollectionShape,
  ResourceSummaryPatchScope,
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
        readonly __forgeInvalidApiRequestParams__:
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

type ApiFamilyPatchReplaceHelper<TValue> = {
  replace(nextValue: TValue): ResourcePatchForReconcile<TValue, never, undefined>;
};

type ApiFamilyDeliveryBaseOptions = {
  packetId: string;
  basisId?: string | null;
  nextBasisId?: string | null;
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
  TFamilyKind extends "collection" | "paged",
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

type ApiFamilyDeliveryReplaceHelper<
  TValue,
  TItem,
  TReconcile,
  TFamilyKind extends "collection" | "paged",
> = {
  replace(
    options: ApiFamilyDeliveryBaseOptions & {
      nextValue: TValue;
    },
  ): ReplaceResourceDelivery<TValue>;
  patch(
    options: ApiFamilyDeliveryBaseOptions & {
      patch: ResourcePatchForReconcile<TValue, TItem, TReconcile, TFamilyKind>;
    },
  ): PatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>;
  invalidate(options: ApiFamilyDeliveryBaseOptions): InvalidateResourceDelivery;
};

type ApiFamilyDeliveryItemHelper<
  TValue,
  TItem,
  TReconcile,
  TFamilyKind extends "collection" | "paged",
> = [TReconcile] extends [
  ResourceCollectionShape<any, TItem, any, any>,
]
  ? {
      item(
        options: ApiFamilyDeliveryBaseOptions & {
          itemId: string;
          nextItem: TItem;
        },
      ): PatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>;
    }
  : {};

type ApiFamilyDeliveryAspectHelper<
  TValue,
  TItem,
  TReconcile,
  TFamilyKind extends "collection" | "paged",
> = [ApiFamilyPatchAspectNames<TItem, TReconcile>] extends [never]
  ? {}
  : {
      itemAspect<TAspect extends ApiFamilyPatchAspectNames<TItem, TReconcile>>(
        options: ApiFamilyDeliveryBaseOptions & {
          itemId: string;
          aspect: TAspect;
          value: ResourceItemAspectValue<
            ResourceReconcileAspectMap<TReconcile>[TAspect]
          >;
        },
      ): PatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>;
    };

type ApiFamilyDeliverySummaryHelper<
  TValue,
  TItem,
  TReconcile,
  TFamilyKind extends "collection" | "paged",
> = [ApiFamilyPatchSummaryNames<TValue, TReconcile, TFamilyKind>] extends [never]
  ? {}
  : {
      summary<
        TSummary extends ApiFamilyPatchSummaryNames<TValue, TReconcile, TFamilyKind>,
      >(
        options: ApiFamilyDeliveryBaseOptions & {
          summary: TSummary;
          value: ResourceValueSummaryValue<
            ResourceReconcileSummaryMap<TReconcile>[TSummary]
          >;
        },
      ): PatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>;
    };

export type ApiFamilyDeliveryHelpers<
  TValue,
  TItem,
  TReconcile,
  TFamilyKind extends "collection" | "paged",
> =
  & ApiFamilyDeliveryReplaceHelper<TValue, TItem, TReconcile, TFamilyKind>
  & ApiFamilyDeliveryItemHelper<TValue, TItem, TReconcile, TFamilyKind>
  & ApiFamilyDeliveryAspectHelper<TValue, TItem, TReconcile, TFamilyKind>
  & ApiFamilyDeliverySummaryHelper<TValue, TItem, TReconcile, TFamilyKind>;

export interface ApiDetailResourceFamily<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
> {
  invalidate<TActualParams extends ApiRouteLineParams<TRoute, TRequestParams, TBody>>(
    params: ExactApiRouteLineParams<TRoute, TRequestParams, TBody, TActualParams>,
  ): boolean;
  invalidateAll(): number;
  line<TActualParams extends ApiRouteLineParams<TRoute, TRequestParams, TBody>>(
    params: ExactApiRouteLineParams<TRoute, TRequestParams, TBody, TActualParams>,
  ): ResourceLine<ApiRouteLineParams<TRoute, TRequestParams, TBody>, TValue | null>;
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
}

export type {
  CollectionResourceFamily,
  DetailResourceFamily,
  PagedResourceFamily,
};
