import type {
  RouteBreadcrumbProvenance,
} from "./router_breadcrumb_surface.js";
import type {
  RouteHistoryReplayResult,
  RouteHistoryRestoreResult,
  RouteReplayHistoryFacade,
  RouteRestoreBoundary,
  RouteRestoreHistoryFacade,
} from "./router_restore_surface.js";
import type {
  WorkerBrowserHistoryInspection,
  WorkerBrowserHistoryInspectionVerificationPackage,
  WorkerBrowserHistoryInspectionSummary,
  WorkerNavigationAuditability,
} from "./worker_runtime_bridge_history_auditability_surface.js";

export interface WorkerHostBoundaryCausality {
  transactionSequence: number;
  generation: number;
  orderingBasis: string;
}

export interface WorkerHostBoundaryPerformanceEnvelope {
  bridgeEnvelopeCount: number;
  submittedItemCount: number;
  coalescedItemCount: number;
  runtimeAdmittedItemCount: number;
  runtimeMutationBreadth: number;
  ambientWorkerReadCount: number;
  diagnosticsColdReconstructionCount: number;
  payloadIdentityByteCount: number;
  performanceDigest: string;
}

export type WorkerHostCapabilityBoundaryArtifact =
  | "admitted"
  | "stale"
  | "denied"
  | "detached"
  | "unavailable";

export interface WorkerHostCapabilityUpdate {
  family: string;
  registrationId: string;
  semanticValueIdentity: string;
  boundaryArtifact?: WorkerHostCapabilityBoundaryArtifact;
  runtimeSourceId?: string;
  runtimeValue?: unknown;
}

export interface WorkerHostCapabilityIngressBatch {
  updates: ReadonlyArray<WorkerHostCapabilityUpdate>;
}

export interface WorkerHostCapabilityIngressReport {
  envelopeFamily: "hostCapabilityIngress";
  causality: WorkerHostBoundaryCausality;
  submittedUpdateCount: number;
  submittedAdmittedUpdateCount: number;
  submittedStaleUpdateCount: number;
  submittedDeniedUpdateCount: number;
  submittedDetachedUpdateCount: number;
  submittedUnavailableUpdateCount: number;
  coalescedAdmittedUpdateCount: number;
  coalescedUpdateCount: number;
  coalescedStaleUpdateCount: number;
  coalescedDeniedUpdateCount: number;
  coalescedDetachedUpdateCount: number;
  coalescedUnavailableUpdateCount: number;
  runtimeAdmittedUpdateCount: number;
  runtimeMutationBreadth: number;
  performance: WorkerHostBoundaryPerformanceEnvelope;
  hostCapabilityEnvelopeDigest: string;
  lifecycleDigest: string;
  truthDigest: string;
  workerFirstTruthDigest: string;
  coalescingDigest: string;
  hostBoundaryArtifactDigest: string;
  ambientWorkerReadDenied: boolean;
}

export interface WorkerBrowserHistoryIngress {
  navigationKind: string;
  rawLocation: string;
  routeIdentity: string;
  runtimeRouteSourceId?: string;
  routeValue?: unknown;
  runtimeContinuitySourceId?: string;
  continuityValue?: unknown;
  coherence?: WorkerBrowserAuthorityCoherence;
}

export type WorkerBrowserAuthorityCoherenceKind =
  | "sameTab"
  | "crossTab"
  | "externalNavigation";

export interface WorkerBrowserAuthorityCoherence {
  coherenceKind: WorkerBrowserAuthorityCoherenceKind;
  coherenceChannelId?: string;
  coherenceSourceTabId?: string;
  expectedRouteId?: string;
}

export interface WorkerBrowserHistoryBoundaryOutcome {
  kind: "admitted" | "notFound";
  routeIdentity: string;
  href: string;
}

export interface WorkerBrowserHistoryBoundaryDiagnostics {
  boundarySource: "browserHistoryIngress" | "browserHistoryWriteback";
  boundaryArtifact:
    | "routeTruthConverged"
    | "routeTruthDriftedFromAuthority"
    | "routeOutcomeNotAdmitted"
    | "externalNavigationEscaped";
  navigationKind: string;
  routeIdentity: string | null;
  coherenceKind: WorkerBrowserAuthorityCoherenceKind | null;
  outcomeKind: "admitted" | "notFound" | null;
  routeId: string | null;
  href: string | null;
  rawLocationHref?: string;
  targetKind?: "local" | "external";
  targetHref?: string;
}

export interface WorkerBrowserHistoryIngressVerificationPackage {
  browserHistoryEnvelopeDigest: string;
  routeTruthDigest: string;
  continuityDigest: string;
  replayRestoreDigest: string;
  workerFirstTruthDigest: string;
}

export interface WorkerBrowserHistoryIngressReport {
  envelopeFamily: "browserHistoryIngress";
  causality: WorkerHostBoundaryCausality;
  navigationKind: string;
  browserHistoryEnvelopeDigest: string;
  routeTruthDigest: string;
  continuityDigest: string;
  replayRestoreDigest: string;
  runtimeAdmittedRouteCount: number;
  runtimeAdmittedContinuityCount: number;
  runtimeMutationBreadth: number;
  workerFirstTruthDigest: string;
  performance: WorkerHostBoundaryPerformanceEnvelope;
  ambientLocationReadDenied: boolean;
  rawLocationHref: string;
  routeIdentity: string;
  outcome(): WorkerBrowserHistoryBoundaryOutcome;
  diagnostics(): WorkerBrowserHistoryBoundaryDiagnostics;
  verification(): WorkerBrowserHistoryIngressVerificationPackage;
}

export interface WorkerBrowserHistoryWriteback {
  navigationKind: "pushstate" | "replacestate" | "external";
  targetKind: "local" | "external";
  targetHref: string;
  routeIdentity?: string;
  runtimeRouteSourceId?: string;
  routeValue?: unknown;
  runtimeContinuitySourceId?: string;
  continuityValue?: unknown;
  coherence?: WorkerBrowserAuthorityCoherence;
}

export interface WorkerBrowserHistoryWritebackVerificationPackage {
  browserHistoryWritebackDigest: string;
  routeTruthDigest: string;
  boundaryStoryDigest: string;
}

export interface WorkerBrowserHistoryWritebackReport {
  envelopeFamily: "browserHistoryWriteback";
  causality: WorkerHostBoundaryCausality | null;
  navigationKind: string;
  targetKind: "local" | "external";
  targetHref: string;
  routeIdentity: string | null;
  boundaryArtifact:
    | "routeTruthConverged"
    | "routeTruthDriftedFromAuthority"
    | "routeOutcomeNotAdmitted"
    | "externalNavigationEscaped";
  browserHistoryWritebackDigest: string;
  routeTruthDigest: string;
  boundaryStoryDigest: string;
  runtimeAdmittedRouteCount: number;
  runtimeMutationBreadth: number;
  workerFirstTruthDigest: string;
  performance: WorkerHostBoundaryPerformanceEnvelope;
  ambientLocationReadDenied: boolean;
  outcome(): WorkerBrowserHistoryBoundaryOutcome | null;
  diagnostics(): WorkerBrowserHistoryBoundaryDiagnostics;
  verification(): WorkerBrowserHistoryWritebackVerificationPackage;
}

export interface WorkerBrowserHistoryBoundaryEvent {
  kind: "browserHistoryBoundaryEvent";
  eventIndex: number;
  envelopeFamily: "browserHistoryIngress" | "browserHistoryWriteback";
  boundarySource: "browserHistoryIngress" | "browserHistoryWriteback";
  boundaryArtifact:
    | "routeTruthConverged"
    | "routeTruthDriftedFromAuthority"
    | "routeOutcomeNotAdmitted"
    | "externalNavigationEscaped";
  navigationKind: string;
  targetHref: string;
  routeIdentity: string | null;
  coherenceKind: WorkerBrowserAuthorityCoherenceKind | null;
  advancedRouteTruth: boolean;
  outcomeKind: "admitted" | "notFound" | null;
  routeId: string | null;
  href: string | null;
  routeTruthEntry: WorkerBrowserHistoryRouteHistoryEntry | null;
  verification(): {
    boundaryEventDigest: string;
  };
}

export interface WorkerBrowserHistoryRouteHistoryEntry {
  kind: "routeHistoryEntry";
  eventIndex: number;
  boundarySource: "browserHistoryIngress" | "browserHistoryWriteback";
  boundaryArtifact: "routeTruthConverged" | "routeTruthDriftedFromAuthority";
  navigationKind: string;
  routeId: string | null;
  href: string;
  routeIdentity: string | null;
  coherenceKind: WorkerBrowserAuthorityCoherenceKind | null;
  previous(): WorkerBrowserHistoryRouteHistoryEntry | null;
  restoreBoundary(): RouteRestoreBoundary | null;
  restore(
    history: RouteRestoreHistoryFacade,
  ): RouteHistoryRestoreResult | Promise<RouteHistoryRestoreResult>;
  replay(
    history: RouteReplayHistoryFacade,
  ): RouteHistoryReplayResult | Promise<RouteHistoryReplayResult>;
  outletComposition(): WorkerBrowserHistoryOutletComposition | null;
  verification(): {
    routeHistoryEntryDigest: string;
  };
}

export interface WorkerBrowserHistoryBackProvenance {
  kind: "browserHistoryBackProvenance";
  available: boolean;
  current: WorkerBrowserHistoryRouteHistoryEntry | null;
  previous: WorkerBrowserHistoryRouteHistoryEntry | null;
  restoreBoundary(): RouteRestoreBoundary | null;
  restore(
    history: RouteRestoreHistoryFacade,
  ): RouteHistoryRestoreResult | Promise<RouteHistoryRestoreResult>;
  replay(
    history: RouteReplayHistoryFacade,
  ): RouteHistoryReplayResult | Promise<RouteHistoryReplayResult>;
  verification(): {
    backProvenanceDigest: string;
  };
}

export interface WorkerBrowserHistoryBreadcrumbEntry {
  readonly crumbId: string;
  readonly routeId: string | null;
  readonly href: string;
  readonly label: string;
  readonly status: string;
  readonly sourceKind: string;
  readonly targetKind: string;
  readonly targetHref: string | null;
  restoreBoundary(): RouteRestoreBoundary | null;
  restore(
    history: RouteRestoreHistoryFacade,
  ): RouteHistoryRestoreResult | Promise<RouteHistoryRestoreResult>;
  replay(
    history: RouteReplayHistoryFacade,
  ): RouteHistoryReplayResult | Promise<RouteHistoryReplayResult>;
  provenance(): RouteBreadcrumbProvenance;
  verification(): {
    breadcrumbEntryDigest: string;
  };
}

export interface WorkerBrowserHistoryBreadcrumbTrail {
  kind: "browserHistoryBreadcrumbTrail";
  entries: ReadonlyArray<WorkerBrowserHistoryBreadcrumbEntry>;
  verification(): {
    breadcrumbTrailDigest: string;
  };
}

export interface WorkerBrowserHistoryOutletComposition {
  kind: "browserHistoryOutletComposition";
  routeId: string;
  href: string;
  layouts(): ReadonlyArray<import("./router_projection_surface.js").ProjectedLayoutPlacement>;
  outlet(): import("./router_projection_surface.js").ProjectedOutletContract;
  outlets(): ReadonlyArray<import("./router_projection_surface.js").ProjectedOutletContract>;
  summary(): Readonly<{ layoutCount: number; outletCount: number; layoutRouteIds: ReadonlyArray<string>; outletIds: ReadonlyArray<string | null>; occupantRouteIds: ReadonlyArray<string>; }>;
  verification(): { outletCompositionDigest: string };
}

export interface WorkerBrowserHistoryStoryVerificationPackage {
  historyStoryDigest: string;
  latestBoundaryEventDigest: string | null;
  currentRouteTruthEventDigest: string | null;
  currentEntryDigest: string | null;
  backEntryDigest: string | null;
}

export interface WorkerBrowserHistoryStory {
  record(
    report: WorkerBrowserHistoryIngressReport | WorkerBrowserHistoryWritebackReport,
  ): WorkerBrowserHistoryBoundaryEvent;
  events(): ReadonlyArray<WorkerBrowserHistoryBoundaryEvent>;
  admittedEntries(): ReadonlyArray<WorkerBrowserHistoryRouteHistoryEntry>;
  current(): WorkerBrowserHistoryRouteHistoryEntry | null;
  latestBoundaryEvent(): WorkerBrowserHistoryBoundaryEvent | null;
  currentRouteTruthEvent(): WorkerBrowserHistoryBoundaryEvent | null;
  back(): WorkerBrowserHistoryRouteHistoryEntry | null;
  breadcrumbs(): ReadonlyArray<WorkerBrowserHistoryRouteHistoryEntry>;
  backProvenance(): WorkerBrowserHistoryBackProvenance;
  breadcrumbTrail(): WorkerBrowserHistoryBreadcrumbTrail;
  inspection(): WorkerBrowserHistoryInspection;
  auditability(): WorkerNavigationAuditability;
  verification(): WorkerBrowserHistoryStoryVerificationPackage;
}

export interface WorkerHostEffectRequest {
  effectId: string;
  hostCapabilityFamily: string;
  closedPayloadIdentity: string;
}

export interface WorkerHostEffectRequestEnvelope {
  envelopeFamily: "hostEffectEgress";
  causality: WorkerHostBoundaryCausality;
  requestDigest: string;
  hostExecutionBoundary: "mainThreadHostEffect";
  performance: WorkerHostBoundaryPerformanceEnvelope;
}

export type WorkerHostEffectOutcome =
  | "completed"
  | "failed"
  | "detached"
  | "unavailable"
  | "denied";

export interface WorkerHostEffectAcknowledgement {
  requestDigest: string;
  outcome: WorkerHostEffectOutcome;
  artifactIdentity: string;
  runtimeLifecycleSourceId?: string;
  lifecycleValue?: unknown;
}

export interface WorkerHostEffectAcknowledgementReport {
  envelopeFamily: "hostEffectEgress";
  causality: WorkerHostBoundaryCausality;
  acknowledgedRequestDigest: string;
  acknowledgementDigest: string;
  hostEffectLifecycleArtifact: string;
  lifecycleIntegrityDigest: string;
  WorthProofReadmissionDigest: string;
  runtimeAdmittedLifecycleCount: number;
  runtimeMutationBreadth: number;
  workerFirstTruthDigest: string;
  performance: WorkerHostBoundaryPerformanceEnvelope;
  hostAcknowledgementIsAuthoritative: boolean;
  workerReadmissionRequired: boolean;
}
