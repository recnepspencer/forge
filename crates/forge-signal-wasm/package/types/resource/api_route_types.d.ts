type LowerAlpha =
  | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m"
  | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z";

type Alpha = LowerAlpha | Uppercase<LowerAlpha>;

type Digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";

type ParamStartChar = Alpha | "_";

type ParamPartChar = ParamStartChar | Digit;

type RouteSegmentParamName<TSegment extends string> =
  TSegment extends `:${infer Name}` ? Name : never;

type ParamPartsAreValid<TName extends string> =
  TName extends ""
    ? true
    : TName extends `${infer First}${infer Rest}`
      ? First extends ParamPartChar
        ? ParamPartsAreValid<Rest>
        : false
      : false;

type ParamNameIsValid<TName extends string> =
  TName extends `${infer First}${infer Rest}`
    ? First extends ParamStartChar
      ? ParamPartsAreValid<Rest>
      : false
    : false;

type ValidRouteState<TSeen extends string> = {
  readonly ok: true;
  readonly seen: TSeen;
};

type InvalidRouteState = {
  readonly ok: false;
};

type ValidateRouteSegment<TSegment extends string, TSeen extends string> =
  TSegment extends ""
    ? InvalidRouteState
    : TSegment extends `:${infer Name}`
      ? ParamNameIsValid<Name> extends true
        ? Name extends TSeen
          ? InvalidRouteState
          : ValidRouteState<TSeen | Name>
        : InvalidRouteState
      : ValidRouteState<TSeen>;

type ValidateRoutePath<TPath extends string, TSeen extends string = never> =
  TPath extends `${infer Segment}/${infer Tail}`
    ? ValidateRouteSegment<Segment, TSeen> extends infer TState
      ? TState extends ValidRouteState<infer TNextSeen extends string>
        ? ValidateRoutePath<Tail, TNextSeen>
        : InvalidRouteState
      : InvalidRouteState
    : ValidateRouteSegment<TPath, TSeen>;

type RouteParamNamesFromPath<TPath extends string> =
  TPath extends `${infer Segment}/${infer Tail}`
    ? RouteSegmentParamName<Segment> | RouteParamNamesFromPath<Tail>
    : RouteSegmentParamName<TPath>;

export type RouteParamNames<TRoute extends string> =
  string extends TRoute
    ? never
    : TRoute extends `/${infer Rest}`
      ? RouteParamNamesFromPath<Rest>
      : never;

export type ApiRouteConstraint<TRoute extends string> =
  string extends TRoute
    ? unknown
    : TRoute extends "/"
      ? unknown
      : TRoute extends `/${infer Path}`
        ? ValidateRoutePath<Path> extends ValidRouteState<any>
          ? unknown
          : {
              readonly __forgeInvalidApiRoute__:
                "api.url(...) routes must start with /, avoid empty path segments, and use unique :paramName placeholders";
            }
        : {
            readonly __forgeInvalidApiRoute__:
              "api.url(...) routes must start with /, avoid empty path segments, and use unique :paramName placeholders";
          };

export type RoutePathParams<TRoute extends string> =
  [RouteParamNames<TRoute>] extends [never]
    ? Record<string, never>
    : { [K in RouteParamNames<TRoute>]: string | number | boolean };
