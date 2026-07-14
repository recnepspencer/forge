import type {
  NavigationArtifactPolicy,
  NavigationCommitPolicy,
  NavigationContinuityPolicy,
  NavigationDeployment,
  NavigationRedirectPolicy,
  NavigationIntentKind,
  NavigationProjectionRefreshPolicy,
} from "./router_navigation_surface.js";
import type {
  CanonicalRouteArtifact,
} from "./router_surface.js";

export interface RouteNavigationCost {
  readonly costClass:
    | "url-only-navigation"
    | "deferred-visible-refresh"
    | "explicit-visible-staleness"
    | "restore-navigation"
    | "speculative-transition";
  readonly intentKind: NavigationIntentKind;
  readonly routeDelta: number;
  readonly projectionRefresh: NavigationProjectionRefreshPolicy;
  readonly continuity: NavigationContinuityPolicy;
  readonly commit: NavigationCommitPolicy;
  readonly redirect: NavigationRedirectPolicy;
  readonly artifactPolicy: NavigationArtifactPolicy;
  readonly looksExpensive: boolean;
}

export interface RouteNavigationTransitionPolicy {
  readonly navigationFamily:
    | "direct-route"
    | "canonicalization"
    | "soft-refresh"
    | "same-route-mutation"
    | "restore-navigation";
  readonly historyEffect: "pushstate" | "replacestate" | "none";
  readonly artifactPolicy: NavigationArtifactPolicy;
  readonly commit: NavigationCommitPolicy;
  readonly redirect: NavigationRedirectPolicy;
  readonly continuity: NavigationContinuityPolicy;
  readonly deployment: NavigationDeployment;
}

export interface RouteNavigationExecutionContract {
  readonly navigationFamily:
    | "direct-route"
    | "canonicalization"
    | "soft-refresh"
    | "same-route-mutation"
    | "restore-navigation";
  readonly historyEffect: "pushstate" | "replacestate" | "none";
  readonly routeTruthEffect:
    | "advance-admitted-route-truth"
    | "canonicalize-admitted-route-truth"
    | "re-admit-current-route-truth"
    | "re-admit-current-route-with-mutation"
    | "restore-admitted-route-truth";
  readonly visibleProjectionEffect:
    | "refresh-visible-projection-immediately"
    | "refresh-visible-projection-after-admission"
    | "preserve-visible-projection-until-explicit-refresh";
  readonly artifactEffect:
    | "materialize-minimal-navigation-artifacts"
    | "materialize-diagnostic-navigation-artifacts";
  readonly commitBoundary: NavigationCommitPolicy;
  readonly redirectBoundary: NavigationRedirectPolicy;
  readonly deployment: NavigationDeployment;
}

export interface RouteNavigationProjectionPolicy {
  readonly projectionRefresh: NavigationProjectionRefreshPolicy;
  readonly continuity: NavigationContinuityPolicy;
  readonly deployment: NavigationDeployment;
  readonly visibleFreshness:
    | "freshly-refreshed"
    | "continuity-preserved"
    | "intentionally-stale";
  readonly admittedRouteTruth:
    | "converges-with-visible-refresh"
    | "may-advance-before-visible-refresh";
  readonly refreshAttribution:
    | "refreshes-visible-projection-immediately"
    | "refreshes-visible-projection-after-admission"
    | "requires-explicit-visible-refresh";
  readonly continuityAttribution:
    | "no-visible-continuity-preservation"
    | "preserve-visible-while-pending"
    | "preserve-visible-until-explicit-refresh";
}

export interface RouteNavigationFreshnessDiagnostics {
  readonly visibleFreshness:
    | "freshly-refreshed"
    | "continuity-preserved"
    | "intentionally-stale";
  readonly admittedRouteTruth:
    | "converges-with-visible-refresh"
    | "may-advance-before-visible-refresh";
  readonly projectionRefresh: NavigationProjectionRefreshPolicy;
  readonly continuity: NavigationContinuityPolicy;
  readonly commit: NavigationCommitPolicy;
  readonly redirect: NavigationRedirectPolicy;
  readonly deployment: NavigationDeployment;
  readonly refreshAttribution:
    | "refreshes-visible-projection-immediately"
    | "refreshes-visible-projection-after-admission"
    | "requires-explicit-visible-refresh";
  readonly continuityAttribution:
    | "no-visible-continuity-preservation"
    | "preserve-visible-while-pending"
    | "preserve-visible-until-explicit-refresh";
  readonly staleVisibilityReason:
    | null
    | "waiting-for-admission-refresh"
    | "waiting-for-explicit-refresh";
}

export interface RouteNavigationExplanation {
  readonly kind: NavigationIntentKind;
  readonly routeId: string;
  readonly href: string;
  readonly deployment: NavigationDeployment;
  readonly artifactPolicy: NavigationArtifactPolicy;
  readonly canonical: CanonicalRouteArtifact;
  readonly transitionPolicy: RouteNavigationTransitionPolicy;
  readonly executionContract: RouteNavigationExecutionContract;
  readonly projectionPolicy: RouteNavigationProjectionPolicy;
  readonly freshness: RouteNavigationFreshnessDiagnostics;
  readonly cost: RouteNavigationCost;
}
