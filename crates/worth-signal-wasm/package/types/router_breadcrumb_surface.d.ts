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

declare const WorthSignalRouteBreadcrumbDeclarationBrand: unique symbol;
declare const WorthSignalRouteBreadcrumbParentDeclarationBrand: unique symbol;
declare const WorthSignalRouteBreadcrumbEntryDeclarationBrand: unique symbol;
declare const WorthSignalRouteBreadcrumbTrailDeclarationBrand: unique symbol;
declare const WorthSignalRouteBreadcrumbEntryVerificationBrand: unique symbol;
declare const WorthSignalRouteBreadcrumbTrailVerificationBrand: unique symbol;
declare const WorthSignalRouteBreadcrumbProvenanceVerificationBrand: unique symbol;
declare const WorthSignalRouteCarriedBreadcrumbsBrand: unique symbol;
declare const WorthSignalRouteCarriedBreadcrumbsVerificationBrand: unique symbol;
declare const WorthSignalRouteRestoredBreadcrumbsBrand: unique symbol;
declare const WorthSignalRouteRestoredBreadcrumbsVerificationBrand: unique symbol;

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
  readonly [WorthSignalRouteBreadcrumbEntryDeclarationBrand]: "routeBreadcrumbEntryDeclaration";
}

export interface RouteBreadcrumbTrailDeclaration {
  readonly entries: ReadonlyArray<RouteBreadcrumbEntryDeclaration>;
  readonly [WorthSignalRouteBreadcrumbTrailDeclarationBrand]: "routeBreadcrumbTrailDeclaration";
}

export interface RouteBreadcrumbParentDeclaration {
  readonly carry: boolean;
  readonly [WorthSignalRouteBreadcrumbParentDeclarationBrand]: "routeBreadcrumbParentDeclaration";
}

export interface RouteBreadcrumbDeclaration
  extends RouteBreadcrumbEntryDeclaration {
  readonly parent: RouteBreadcrumbParentDeclaration | null;
  readonly [WorthSignalRouteBreadcrumbDeclarationBrand]: "routeBreadcrumbDeclaration";
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
  readonly [WorthSignalRouteBreadcrumbEntryVerificationBrand]: "routeBreadcrumbEntryVerificationPackage";
}

export interface RouteBreadcrumbTrailVerificationPackage {
  readonly breadcrumbTrailDigest: string;
  readonly [WorthSignalRouteBreadcrumbTrailVerificationBrand]: "routeBreadcrumbTrailVerificationPackage";
}

export interface RouteBreadcrumbProvenanceVerificationPackage {
  readonly breadcrumbProvenanceDigest: string;
  readonly [WorthSignalRouteBreadcrumbProvenanceVerificationBrand]: "routeBreadcrumbProvenanceVerificationPackage";
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
  readonly [WorthSignalRouteCarriedBreadcrumbsVerificationBrand]: "routeCarriedBreadcrumbsVerificationPackage";
}

export interface RouteCarriedBreadcrumbs {
  readonly kind: "routeCarriedBreadcrumbs";
  readonly entries: ReadonlyArray<RouteBreadcrumbEntry>;
  verification(): RouteCarriedBreadcrumbsVerificationPackage;
  readonly [WorthSignalRouteCarriedBreadcrumbsBrand]: "routeCarriedBreadcrumbs";
}

export interface RouteRestoredBreadcrumbsVerificationPackage {
  readonly restoredBreadcrumbsDigest: string;
  readonly [WorthSignalRouteRestoredBreadcrumbsVerificationBrand]: "routeRestoredBreadcrumbsVerificationPackage";
}

export interface RouteRestoredBreadcrumbs {
  readonly kind: "routeRestoredBreadcrumbs";
  readonly entries: ReadonlyArray<RouteBreadcrumbEntry>;
  verification(): RouteRestoredBreadcrumbsVerificationPackage;
  readonly [WorthSignalRouteRestoredBreadcrumbsBrand]: "routeRestoredBreadcrumbs";
}
