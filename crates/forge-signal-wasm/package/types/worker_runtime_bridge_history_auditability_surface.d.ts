import type {
  RouteBreadcrumbProvenance,
} from "./router_breadcrumb_surface.js";
import type {
  RouteRestoreBoundary,
} from "./router_restore_surface.js";
import type {
  WorkerBrowserAuthorityCoherenceKind,
  WorkerBrowserHistoryBackProvenance,
  WorkerBrowserHistoryBoundaryEvent,
  WorkerBrowserHistoryBreadcrumbTrail,
  WorkerBrowserHistoryOutletComposition,
  WorkerBrowserHistoryRouteHistoryEntry,
} from "./worker_runtime_bridge_boundary.js";

export interface WorkerBrowserHistoryInspectionVerificationPackage {
  historyInspectionDigest: string;
}

export interface WorkerBrowserHistoryInspectionSummary {
  currentEntryAvailable: boolean;
  currentEntryRestoreAvailability: "restoreBoundary" | "unavailable";
  currentEntryReplayAvailability: "replayHistory" | "unavailable";
  backProvenanceAvailable: boolean;
  backRestoreAvailability: "restoreBoundary" | "unavailable";
  backReplayAvailability: "replayHistory" | "unavailable";
  currentOutletCompositionAvailable: boolean;
  backOutletCompositionAvailable: boolean;
  breadcrumbEntryCount: number;
  breadcrumbRestoreAvailability: "none" | "partial" | "all";
  breadcrumbReplayAvailability: "none" | "partial" | "all";
  resolvedBreadcrumbCount: number;
  recomputedBreadcrumbCount: number;
  carriedBreadcrumbCount: number;
  restoredBreadcrumbCount: number;
  fallbackBreadcrumbCount: number;
  routeDeclarationBreadcrumbPresent: boolean;
  recomputedBreadcrumbPresent: boolean;
  carriedBreadcrumbPresent: boolean;
  restoredBreadcrumbPresent: boolean;
  fallbackBreadcrumbPresent: boolean;
  historyFallbackBreadcrumbPresent: boolean;
  latestBoundaryCoherenceKind: WorkerBrowserAuthorityCoherenceKind | null;
  currentRouteTruthCoherenceKind: WorkerBrowserAuthorityCoherenceKind | null;
  sameTabCoherencePresent: boolean;
  crossTabCoherencePresent: boolean;
  externalNavigationCoherencePresent: boolean;
  convergedBoundaryEventCount: number;
  driftedBoundaryEventCount: number;
  notAdmittedBoundaryEventCount: number;
}

export interface WorkerBrowserHistoryInspection {
  kind: "browserHistoryInspection";
  latestBoundaryEvent: WorkerBrowserHistoryBoundaryEvent | null;
  currentRouteTruthEvent: WorkerBrowserHistoryBoundaryEvent | null;
  currentEntry: WorkerBrowserHistoryRouteHistoryEntry | null;
  backProvenance: WorkerBrowserHistoryBackProvenance;
  breadcrumbTrail: WorkerBrowserHistoryBreadcrumbTrail;
  currentOutletComposition(): WorkerBrowserHistoryOutletComposition | null;
  backOutletComposition(): WorkerBrowserHistoryOutletComposition | null;
  breadcrumbProvenance(): ReadonlyArray<RouteBreadcrumbProvenance>;
  summary(): WorkerBrowserHistoryInspectionSummary;
  verification(): WorkerBrowserHistoryInspectionVerificationPackage;
}

export interface WorkerNavigationAuditabilityVerificationPackage {
  navigationAuditabilityDigest: string;
}

export interface WorkerNavigationAuditabilitySummary {
  hydrationBoundaryPresent: false;
  hydrationBoundaryArtifact: null;
  hydrationMatchesCurrentVisibleRoute: null;
  historyCurrentEntryPresent: boolean;
  currentVisibleRouteSource: "routeHistoryEntry" | "none";
  currentVisibilityExplanation:
    | "routeHistoryEntry"
    | "routeHistoryRestoreBoundary"
    | "none";
  currentBoundarySource:
    | "browserHistoryIngress"
    | "browserHistoryWriteback"
    | null;
  currentBoundaryArtifact:
    | "routeTruthConverged"
    | "routeTruthDriftedFromAuthority"
    | "routeOutcomeNotAdmitted"
    | "externalNavigationEscaped"
    | null;
  currentNavigationIntent:
    | "load"
    | "pushstate"
    | "replacestate"
    | "popstate"
    | "manual"
    | "external"
    | null;
  currentCoherenceKind: WorkerBrowserAuthorityCoherenceKind | null;
  currentRouteId: string | null;
  currentHref: string | null;
  currentRestoreAvailability: "restoreBoundary" | "unavailable";
  currentReplayAvailability: "replayHistory" | "unavailable";
  routeHistoryExplainsCurrent: boolean;
  restoreBoundaryExplainsCurrent: boolean;
  latestBoundarySource:
    | "browserHistoryIngress"
    | "browserHistoryWriteback"
    | null;
  latestBoundaryArtifact:
    | "routeTruthConverged"
    | "routeTruthDriftedFromAuthority"
    | "routeOutcomeNotAdmitted"
    | "externalNavigationEscaped"
    | null;
  latestBoundaryCoherenceKind: WorkerBrowserAuthorityCoherenceKind | null;
  sameTabCoherencePresent: boolean;
  crossTabCoherencePresent: boolean;
  externalNavigationCoherencePresent: boolean;
  convergedBoundaryEventCount: number;
  driftedBoundaryEventCount: number;
  notAdmittedBoundaryEventCount: number;
}

export interface WorkerNavigationAuditability {
  kind: "navigationAuditability";
  historyInspection(): WorkerBrowserHistoryInspection;
  currentRestoreBoundary(): RouteRestoreBoundary | null;
  summary(): WorkerNavigationAuditabilitySummary;
  verification(): WorkerNavigationAuditabilityVerificationPackage;
}
