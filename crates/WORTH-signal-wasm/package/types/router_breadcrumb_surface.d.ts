import type {
  CanonicalUrlAuthority,
  RawLocationAuthority,
} from "./router_authority_surface.js";
import type {
  RouteRestoreBoundary,
} from "./router_restore_surface.js";
import type {
  RouterHashField,
  RouterHashInput,
  RouterSearchMatch,
  RouterSearchSchema,
} from "./router_surface.js";

declare const WORTHSignalRouteBreadcrumbDeclarationBrand: unique symbol;
declare const WORTHSignalRouteBreadcrumbParentDeclarationBrand: unique symbol;
declare const WORTHSignalRouteBreadcrumbEntryDeclarationBrand: unique symbol;
declare const WORTHSignalRouteBreadcrumbTrailDeclarationBrand: unique symbol;
declare const WORTHSignalRouteBreadcrumbEntryVerificationBrand: unique symbol;
declare const WORTHSignalRouteBreadcrumbTrailVerificationBrand: unique symbol;
declare const WORTHSignalRouteBreadcrumbProvenanceVerificationBrand: unique symbol;
declare const WORTHSignalRouteCarriedBreadcrumbsBrand: unique symbol;
declare const WORTHSignalRouteCarriedBreadcrumbsVerificationBrand: unique symbol;
declare const WORTHSignalRouteRestoredBreadcrumbsBrand: unique symbol;
declare const WORTHSignalRouteRestoredBreadcrumbsVerificationBrand: unique symbol;

export interface RouteBreadcrumbContext<
  TRoute extends string = string,
  TSearch extends RouterSearchSchema = Record<string, never>,
  THash extends RouterHashField<unknown> | null = null,
> {
  readonly routeId: string;
  readonly href: string;
  readonly params: import("./router/route_types.js").RoutePathParams<TRoute>;
  readonly search: RouterSearchMatch<TSearch>;
  readonly hash: THash extends RouterHashField<unknown> ? RouterHashInput<THash> | undefined : undefined;
  descriptor(): unknown;
  canonical(): unknown;
}

export type RouteBreadcrumbTarget =
  | string
  | RawLocationAuthority
  | CanonicalUrlAuthority
  | { readonly href: string };

export interface RouteBreadcrumbEntryDeclarationOptions {
  readonly id: string;
  readonly label: string | ((context: RouteBreadcrumbContext<any, any, any>) => string);
  readonly target?:
    | RouteBreadcrumbTarget
    | ((
      context: RouteBreadcrumbContext<any, any, any>,
    ) => RouteBreadcrumbTarget | null | undefined)
    | null;
}

export interface RouteBreadcrumbEntryDeclaration {
  readonly id: string;
  readonly [WORTHSignalRouteBreadcrumbEntryDeclarationBrand]: "routeBreadcrumbEntryDeclaration";
}

export interface RouteBreadcrumbTrailDeclaration {
  readonly entries: ReadonlyArray<RouteBreadcrumbEntryDeclaration>;
  readonly [WORTHSignalRouteBreadcrumbTrailDeclarationBrand]: "routeBreadcrumbTrailDeclaration";
}

export interface RouteBreadcrumbParentDeclaration {
  readonly carry: boolean;
  readonly [WORTHSignalRouteBreadcrumbParentDeclarationBrand]: "routeBreadcrumbParentDeclaration";
}

export interface RouteBreadcrumbDeclaration
  extends RouteBreadcrumbEntryDeclaration {
  readonly parent: RouteBreadcrumbParentDeclaration | null;
  readonly [WORTHSignalRouteBreadcrumbDeclarationBrand]: "routeBreadcrumbDeclaration";
}

export type RouteBreadcrumbStatus =
  | "resolved"
  | "recomputed"
  | "carried"
  | "restored"
  | "fallback";

export type RouteBreadcrumbSourceKind =
  | "routeDeclaration"
  | "recomputed"
  | "carriedProvenance"
  | "restoredProvenance"
  | "fallback"
  | "historyFallback";

export type RouteBreadcrumbTargetKind =
  | "routeHref"
  | "externalHref"
  | "none";

export type RouteBreadcrumbRestoreAvailability =
  | "restoreBoundary"
  | "unavailable";

export type RouteBreadcrumbReplayAvailability =
  | "replayHistory"
  | "unavailable";

export interface RouteBreadcrumbEntryVerificationPackage {
  readonly breadcrumbEntryDigest: string;
  readonly [WORTHSignalRouteBreadcrumbEntryVerificationBrand]: "routeBreadcrumbEntryVerificationPackage";
}

export interface RouteBreadcrumbTrailVerificationPackage {
  readonly breadcrumbTrailDigest: string;
  readonly [WORTHSignalRouteBreadcrumbTrailVerificationBrand]: "routeBreadcrumbTrailVerificationPackage";
}

export interface RouteBreadcrumbProvenanceVerificationPackage {
  readonly breadcrumbProvenanceDigest: string;
  readonly [WORTHSignalRouteBreadcrumbProvenanceVerificationBrand]: "routeBreadcrumbProvenanceVerificationPackage";
}

export interface RouteBreadcrumbProvenance {
  readonly kind: "routeBreadcrumbProvenance";
  readonly crumbId: string;
  readonly routeId: string | null;
  readonly href: string;
  readonly targetHref: string | null;
  readonly status: RouteBreadcrumbStatus;
  readonly sourceKind: RouteBreadcrumbSourceKind;
  readonly restoreAvailability: RouteBreadcrumbRestoreAvailability;
  readonly replayAvailability: RouteBreadcrumbReplayAvailability;
  restoreBoundary(): RouteRestoreBoundary | null;
  verification(): RouteBreadcrumbProvenanceVerificationPackage;
}

export interface RouteBreadcrumbEntry {
  readonly kind: "routeBreadcrumbEntry";
  readonly crumbId: string;
  readonly routeId: string | null;
  readonly href: string;
  readonly label: string;
  readonly status: RouteBreadcrumbStatus;
  readonly sourceKind: RouteBreadcrumbSourceKind;
  readonly targetKind: RouteBreadcrumbTargetKind;
  readonly targetHref: string | null;
  provenance(): RouteBreadcrumbProvenance;
  verification(): RouteBreadcrumbEntryVerificationPackage;
}

export interface RouteBreadcrumbTrail {
  readonly kind: "routeBreadcrumbTrail";
  readonly entries: ReadonlyArray<RouteBreadcrumbEntry>;
  verification(): RouteBreadcrumbTrailVerificationPackage;
}

export interface RouteCarriedBreadcrumbsVerificationPackage {
  readonly carriedBreadcrumbsDigest: string;
  readonly [WORTHSignalRouteCarriedBreadcrumbsVerificationBrand]: "routeCarriedBreadcrumbsVerificationPackage";
}

export interface RouteCarriedBreadcrumbs {
  readonly kind: "routeCarriedBreadcrumbs";
  readonly entries: ReadonlyArray<RouteBreadcrumbEntry>;
  verification(): RouteCarriedBreadcrumbsVerificationPackage;
  readonly [WORTHSignalRouteCarriedBreadcrumbsBrand]: "routeCarriedBreadcrumbs";
}

export interface RouteRestoredBreadcrumbsVerificationPackage {
  readonly restoredBreadcrumbsDigest: string;
  readonly [WORTHSignalRouteRestoredBreadcrumbsVerificationBrand]: "routeRestoredBreadcrumbsVerificationPackage";
}

export interface RouteRestoredBreadcrumbs {
  readonly kind: "routeRestoredBreadcrumbs";
  readonly entries: ReadonlyArray<RouteBreadcrumbEntry>;
  verification(): RouteRestoredBreadcrumbsVerificationPackage;
  readonly [WORTHSignalRouteRestoredBreadcrumbsBrand]: "routeRestoredBreadcrumbs";
}
