import type {
  NavigationContinuityPolicy,
} from "./router_navigation_surface.js";
import type {
  RouteAdmissionFacts,
  RouteOutcome,
} from "./router_admission_surface.js";
import type {
  RouteResourcePrefetchArtifact,
  RouteResourcePrefetchPosture,
} from "./router_resource_surface.js";
import type {
  AdmittedRouteOutcome,
} from "./router_admission_surface.js";
import type {
  ProjectedRouteCandidate,
} from "./router_projection_surface.js";

declare const WorthSignalRoutePrefetchArtifactBrand: unique symbol;
declare const WorthSignalRouteTransitionArtifactBrand: unique symbol;

export type RoutePrefetchTrigger = RouteResourcePrefetchPosture;

export interface RoutePrefetchVerificationPackage {
  readonly routePrefetchDigest: string;
}

export interface ProjectedRoutePrefetchArtifact {
  readonly kind: "routePrefetchAdmission";
  readonly routeId: string;
  readonly href: string;
  readonly trigger: RoutePrefetchTrigger;
  candidate(): ProjectedRouteCandidate;
  declaredResourceNames(): ReadonlyArray<string>;
  resourceNames(): ReadonlyArray<string>;
  skippedResourceNames(): ReadonlyArray<string>;
  resource(name: string): RouteResourcePrefetchArtifact;
  resources(): ReadonlyArray<RouteResourcePrefetchArtifact>;
  admit(facts?: RouteAdmissionFacts): Promise<RouteOutcome>;
  free(): void;
  [Symbol.dispose](): void;
  verification(): RoutePrefetchVerificationPackage;
  readonly [WorthSignalRoutePrefetchArtifactBrand]: "routePrefetchArtifact";
}

export type RouteTransitionRequestedSource =
  | "directNavigation"
  | "speculativeCommit"
  | "redirect"
  | "prefetchAdmission";

export type RouteVisibleChangeSource =
  | "directNavigation"
  | "speculativeCommit"
  | "redirect"
  | "prefetchAdmission"
  | "resourceContinuityPreservation";

export type RouteTransitionVisiblePolicy =
  | "switch-to-target-route"
  | "preserve-current-route"
  | "preserve-current-route-until-explicit-refresh"
  | "show-target-resource-continuity-while-pending";

export interface RouteTransitionVerificationPackage {
  readonly routeTransitionDigest: string;
}

export interface RouteTransitionDiagnostics {
  readonly requestedSource: RouteTransitionRequestedSource;
  readonly visibleChangeSource: RouteVisibleChangeSource;
  readonly visiblePolicy: RouteTransitionVisiblePolicy;
  readonly continuity: NavigationContinuityPolicy;
  readonly pendingResourceNames: ReadonlyArray<string>;
}

export interface RouteTransitionArtifact {
  readonly kind: "routeTransition";
  readonly currentRouteId: string;
  readonly currentHref: string;
  readonly targetRouteId: string | null;
  readonly targetHref: string | null;
  target(): RouteOutcome;
  diagnostics(): RouteTransitionDiagnostics;
  verification(): RouteTransitionVerificationPackage;
  readonly [WorthSignalRouteTransitionArtifactBrand]: "routeTransitionArtifact";
}

export type RouteTransitionTarget =
  | string
  | import("./router_authority_surface.js").RawLocationAuthority
  | import("./router_authority_surface.js").CanonicalUrlAuthority
  | ProjectedRoutePrefetchArtifact;

export interface RouteTransitionOptions {
  readonly facts?: RouteAdmissionFacts;
  readonly source?: Exclude<RouteTransitionRequestedSource, "prefetchAdmission">;
  readonly continuity?: NavigationContinuityPolicy;
}
