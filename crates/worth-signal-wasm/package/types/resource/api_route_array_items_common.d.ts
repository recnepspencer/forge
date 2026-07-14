export type ApiRouteArrayItemsSummaryMode = "none" | "line" | "pageWindow";

export type ApiRouteArrayItemsSummaryPatchScope<
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
> = TSummaryMode extends "pageWindow" ? "pageWindow" : "line";

export type ApiRouteUnusedDefinitionName<TName extends string, TMap> =
  TName extends keyof TMap ? never : TName;
