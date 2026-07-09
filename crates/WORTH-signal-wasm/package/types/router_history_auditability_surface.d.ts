import type {
  RouteBreadcrumbProvenance,
} from "./router_breadcrumb_surface.js";
import type {
  RouterHydrationAdmissionReport,
} from "./router_hydration_surface.js";
import type {
  RouteRestoreBoundary,
} from "./router_restore_surface.js";
import type {
  BrowserAuthorityCoherenceKind,
  RouterBrowserHistoryBackProvenance,
  RouterBrowserHistoryBoundaryEvent,
  RouterBrowserHistoryBreadcrumbTrail,
  RouterBrowserHistoryStoryEntry,
} from "./router_history_surface.js";

declare const WORTHSignalRouterBrowserHistoryInspectionVerificationBrand: unique symbol;
declare const WORTHSignalRouterBrowserHistoryOutletCompositionVerificationBrand: unique symbol;
declare const WORTHSignalRouterNavigationAuditabilityVerificationBrand: unique symbol;

export interface RouterBrowserHistoryOutletCompositionVerificationPackage {
  readonly outletCompositionDigest: string;
  readonly [WORTHSignalRouterBrowserHistoryOutletCompositionVerificationBrand]: "routerBrowserHistoryOutletCompositionVerificationPackage";
}

export interface RouterBrowserHistoryOutletComposition {
  readonly kind: "browserHistoryOutletComposition";
  readonly routeId: string;
  readonly href: string;
  layouts(): ReadonlyArray<import("./router_projection_surface.js").ProjectedLayoutPlacement>;
  outlet(): import("./router_projection_surface.js").ProjectedOutletContract;
  outlets(): ReadonlyArray<import("./router_projection_surface.js").ProjectedOutletContract>;
  summary(): Readonly<{ layoutCount: number; outletCount: number; layoutRouteIds: ReadonlyArray<string>; outletIds: ReadonlyArray<string | null>; occupantRouteIds: ReadonlyArray<string>; }>;
  verification(): RouterBrowserHistoryOutletCompositionVerificationPackage;
}

export interface RouterBrowserHistoryInspectionVerificationPackage {
  readonly historyInspectionDigest: string;
  readonly [WORTHSignalRouterBrowserHistoryInspectionVerificationBrand]: "routerBrowserHistoryInspectionVerificationPackage";
}

export interface RouterBrowserHistoryInspectionSummary {
  readonly currentEntryAvailable: boolean;
  readonly currentEntryRestoreAvailability: "restoreBoundary" | "unavailable";
  readonly currentEntryReplayAvailability: "replayHistory" | "unavailable";
  readonly backProvenanceAvailable: boolean;
  readonly backRestoreAvailability: "restoreBoundary" | "unavailable";
  readonly backReplayAvailability: "replayHistory" | "unavailable";
  readonly currentOutletCompositionAvailable: boolean;
  readonly backOutletCompositionAvailable: boolean;
  readonly breadcrumbEntryCount: number;
  readonly breadcrumbRestoreAvailability: "none" | "partial" | "all";
  readonly breadcrumbReplayAvailability: "none" | "partial" | "all";
  readonly resolvedBreadcrumbCount: number;
  readonly recomputedBreadcrumbCount: number;
  readonly carriedBreadcrumbCount: number;
  readonly restoredBreadcrumbCount: number;
  readonly fallbackBreadcrumbCount: number;
  readonly routeDeclarationBreadcrumbPresent: boolean;
  readonly recomputedBreadcrumbPresent: boolean;
  readonly carriedBreadcrumbPresent: boolean;
  readonly restoredBreadcrumbPresent: boolean;
  readonly fallbackBreadcrumbPresent: boolean;
  readonly historyFallbackBreadcrumbPresent: boolean;
  readonly latestBoundaryCoherenceKind: BrowserAuthorityCoherenceKind | null;
  readonly currentRouteTruthCoherenceKind: BrowserAuthorityCoherenceKind | null;
  readonly sameTabCoherencePresent: boolean;
  readonly crossTabCoherencePresent: boolean;
  readonly externalNavigationCoherencePresent: boolean;
  readonly convergedBoundaryEventCount: number;
  readonly driftedBoundaryEventCount: number;
  readonly notAdmittedBoundaryEventCount: number;
}

export interface RouterBrowserHistoryInspection {
  readonly kind: "browserHistoryInspection";
  readonly latestBoundaryEvent: RouterBrowserHistoryBoundaryEvent | null;
  readonly currentRouteTruthEvent: RouterBrowserHistoryBoundaryEvent | null;
  readonly currentEntry: RouterBrowserHistoryStoryEntry | null;
  readonly backProvenance: RouterBrowserHistoryBackProvenance;
  readonly breadcrumbTrail: RouterBrowserHistoryBreadcrumbTrail;
  currentOutletComposition(): RouterBrowserHistoryOutletComposition | null;
  backOutletComposition(): RouterBrowserHistoryOutletComposition | null;
  breadcrumbProvenance(): ReadonlyArray<RouteBreadcrumbProvenance>;
  summary(): RouterBrowserHistoryInspectionSummary;
  verification(): RouterBrowserHistoryInspectionVerificationPackage;
}

export interface RouterNavigationAuditabilityVerificationPackage {
  readonly navigationAuditabilityDigest: string;
  readonly [WORTHSignalRouterNavigationAuditabilityVerificationBrand]: "routerNavigationAuditabilityVerificationPackage";
}

export interface RouterNavigationAuditabilitySummary {
  readonly hydrationBoundaryPresent: boolean;
  readonly hydrationBoundaryArtifact:
    | "routeTruthMatchedServer"
    | "routeTruthDriftedFromServer"
    | "routeOutcomeNotAdmitted"
    | null;
  readonly hydrationMatchesCurrentVisibleRoute: boolean | null;
  readonly historyCurrentEntryPresent: boolean;
  readonly currentVisibleRouteSource:
    | "routeHistoryEntry"
    | "hydrationAdmission"
    | "none";
  readonly currentVisibilityExplanation:
    | "routeHistoryEntry"
    | "routeHistoryRestoreBoundary"
    | "hydrationBoundary"
    | "none";
  readonly currentBoundarySource:
    | "hydrationHandoff"
    | "browserHistoryIngress"
    | "browserHistoryWriteback"
    | null;
  readonly currentBoundaryArtifact:
    | "routeTruthMatchedServer"
    | "routeTruthDriftedFromServer"
    | "routeOutcomeNotAdmitted"
    | "routeTruthConverged"
    | "routeTruthDriftedFromAuthority"
    | "externalNavigationEscaped"
    | null;
  readonly currentNavigationIntent:
    | "load"
    | "pushstate"
    | "replacestate"
    | "popstate"
    | "manual"
    | "external"
    | null;
  readonly currentCoherenceKind: BrowserAuthorityCoherenceKind | null;
  readonly currentRouteId: string | null;
  readonly currentHref: string | null;
  readonly currentRestoreAvailability: "restoreBoundary" | "unavailable";
  readonly currentReplayAvailability: "replayHistory" | "unavailable";
  readonly routeHistoryExplainsCurrent: boolean;
  readonly restoreBoundaryExplainsCurrent: boolean;
  readonly latestBoundarySource:
    | "browserHistoryIngress"
    | "browserHistoryWriteback"
    | null;
  readonly latestBoundaryArtifact:
    | "routeTruthConverged"
    | "routeTruthDriftedFromAuthority"
    | "routeOutcomeNotAdmitted"
    | "externalNavigationEscaped"
    | null;
  readonly latestBoundaryCoherenceKind: BrowserAuthorityCoherenceKind | null;
  readonly sameTabCoherencePresent: boolean;
  readonly crossTabCoherencePresent: boolean;
  readonly externalNavigationCoherencePresent: boolean;
  readonly convergedBoundaryEventCount: number;
  readonly driftedBoundaryEventCount: number;
  readonly notAdmittedBoundaryEventCount: number;
}

export interface RouterNavigationAuditability {
  readonly kind: "navigationAuditability";
  hydrationBoundary(): RouterHydrationAdmissionReport | null;
  historyInspection(): RouterBrowserHistoryInspection;
  currentRestoreBoundary(): RouteRestoreBoundary | null;
  summary(): RouterNavigationAuditabilitySummary;
  verification(): RouterNavigationAuditabilityVerificationPackage;
}
