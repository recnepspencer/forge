import type {
  ReplaySummary,
} from "./diagnostics.js";
import type {
  RuntimeSnapshotEnvelopeArtifact,
} from "./callable_surface.js";

declare const WORTHSignalRouteRestoreBoundaryBrand: unique symbol;
declare const WORTHSignalRouteRestoreBoundaryVerificationBrand: unique symbol;
declare const WORTHSignalRouteHistoryRestoreResultVerificationBrand: unique symbol;
declare const WORTHSignalRouteHistoryReplayResultVerificationBrand: unique symbol;

export interface RouteRestoreHistoryFacade {
  restore_exact_snapshot(
    snapshot: RuntimeSnapshotEnvelopeArtifact,
  ): void | Promise<void>;
}

export interface RouteReplayHistoryFacade {
  replay_for(id: string): ReplaySummary;
}

export interface RouteRestoreBoundaryVerificationPackage {
  readonly routeRestoreBoundaryDigest: string;
  readonly [WORTHSignalRouteRestoreBoundaryVerificationBrand]: "routeRestoreBoundaryVerificationPackage";
}

export interface RouteRestoreBoundaryGuarantees {
  readonly routeTruth: "restoredExactGraphTruth";
  readonly outletComposition: "restoredAdmittedOutletComposition";
  readonly graphOwnedState: "restoredWithinSnapshotBoundary";
}

export interface RouteRestoreBoundary {
  readonly kind: "routeRestoreBoundary";
  snapshotEnvelope(): RuntimeSnapshotEnvelopeArtifact;
  guarantees(): RouteRestoreBoundaryGuarantees;
  verification(): RouteRestoreBoundaryVerificationPackage;
  readonly [WORTHSignalRouteRestoreBoundaryBrand]: "routeRestoreBoundary";
}

export interface RouteHistoryRestoreResultVerificationPackage {
  readonly routeHistoryRestoreDigest: string;
  readonly [WORTHSignalRouteHistoryRestoreResultVerificationBrand]: "routeHistoryRestoreResultVerificationPackage";
}

export interface RouteHistoryRestoreResult {
  readonly kind: "routeHistoryRestoreResult";
  readonly restoreSourceKind: "routeHistoryEntry" | "breadcrumbEntry";
  readonly routeId: string | null;
  readonly href: string;
  readonly restoredEntryDigest: string | null;
  readonly restoreBoundary: RouteRestoreBoundary;
  verification(): RouteHistoryRestoreResultVerificationPackage;
}

export interface RouteHistoryReplayResultVerificationPackage {
  readonly routeHistoryReplayDigest: string;
  readonly [WORTHSignalRouteHistoryReplayResultVerificationBrand]: "routeHistoryReplayResultVerificationPackage";
}

export interface RouteHistoryReplayResult {
  readonly kind: "routeHistoryReplayResult";
  readonly replaySourceKind: "routeHistoryEntry" | "breadcrumbEntry";
  readonly routeId: string | null;
  readonly href: string;
  readonly replayedEntryDigest: string | null;
  readonly routeReplay: ReplaySummary | null;
  readonly continuityReplay: ReplaySummary | null;
  verification(): RouteHistoryReplayResultVerificationPackage;
}
