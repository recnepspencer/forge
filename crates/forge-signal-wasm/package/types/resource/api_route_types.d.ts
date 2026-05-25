import type {
  RouteConstraint,
  RouteParamNames,
  RoutePathParams,
} from "../router/route_types.js";

export type ApiRouteConstraint<TRoute extends string> = RouteConstraint<TRoute>;
export type { RouteParamNames, RoutePathParams };
