import type {
  BranchStateProofReport,
  ExecutionHistorySummary,
  FailureSummary,
  FlowSurfaceSummary,
  FrontierExecutionSummary,
  ExecutionHistorySurfaceSummary,
  GraphSummary,
  HealthSummary,
  HostCapabilityTransportReport,
  InvalidationTraceRecord,
  ObservationSurfaceSummary,
  ReplayArtifactProofInput,
  ReplayArtifactProofReport,
  ReplayParityProofReport,
  RollbackDiagnostic,
  RuntimeDefinitionEnvelope,
  RuntimeEnvelope,
  RuntimeProofReport,
  RuntimeBranchHandle,
  RuntimeSnapshotArtifact,
  RuntimeSnapshotEnvelope,
  RuntimePolicySpec,
  VersionSummary,
  WebPerformanceSummary,
  WhySummary,
} from "./diagnostics.js";
import type {
  LineageSummary,
  RecipeSpec,
  ReplaySummary,
  RunSummary,
  SourceSpec,
  TransactionOp,
} from "./model.js";

export interface WorkerRuntimeIdentity {
  deploymentPosture: "workerFirst";
  runtimeAuthority: "workerOwnedRuntime";
  topology: string;
}

export interface WorkerRuntimeShellLock {
  identity: WorkerRuntimeIdentity;
  graphPublicationAdmission: string;
  committedEnvelopeFamily: string;
  callbackPublicationBeforeLowering: string;
}

export interface WorkerRuntimeBootstrapRecord {
  shellLock: WorkerRuntimeShellLock;
  boundarySurface: "workerFirstConstruction";
  transportPosture: string;
  hostCapabilityIngress: string;
  hostEffectEgress: string;
}

export interface WorkerFirstHostCapabilityDiagnosticsReport {
  posture: "workerFirstUnavailable";
  reason: "workerFirstHostCapabilityEventReplayNotImplemented";
  message: string;
}

export interface WorkerFirstHostCapabilityDiagnosticsEvent {
  kind: "unavailable";
  reason: WorkerFirstHostCapabilityDiagnosticsReport["reason"];
  message: string;
}

export interface WorkerPortableGraphPublication {
  policy: RuntimePolicySpec;
  sources: ReadonlyArray<SourceSpec>;
  recipes: ReadonlyArray<RecipeSpec>;
  outputIds?: ReadonlyArray<string>;
}

export interface WorkerGraphPublicationSummary {
  publishedSourceCount: number;
  publishedRecipeCount: number;
  admittedCallbackCount: number;
  deniedCallbackCount: number;
}

export interface WorkerCommittedTransactionEnvelope {
  deploymentPosture: "workerFirst";
  envelopeFamily: "transactionResult";
  runtimeAuthority: "workerOwnedRuntime";
  branchId: number;
  committedTruthDigest: string;
  runSummary: RunSummary;
}

export interface WorkerCommittedProjectionRequest {
  transactionOps: ReadonlyArray<TransactionOp>;
  outputIds: ReadonlyArray<string>;
}

export interface WorkerObservationDeliveryAttachRequest {
  signalId: string;
}

export interface WorkerObservationDeliveryDetachRequest {
  lifecycleSubscriptionId: number;
}

export interface WorkerLifecycleControlPacket {
  runtimeAuthority: "workerOwnedRuntime";
  workerFirstTruthDigest: string;
  activeLifecycleSubscriptionCount: number;
  packetDigest: string;
}

export interface WorkerObservationDeliveryPacket {
  envelopeFamily: "observationDelivery";
  deliveryMode: "CommittedObservationDelivery";
  runtimeAuthority: "workerOwnedRuntime";
  observationDeliveryPacketCount: number;
  observationDeliveryBreadth: number;
  deliveredObservationCount: number;
  rollbackSuppressedDeliveryCount: number;
  callbackNodeCount: number;
  activeLifecycleSubscriptionCount: number;
  workerFirstTruthDigest: string;
  observationDigest: string;
  observationLifecycleDigest: string;
  boundaryPerformance: WorkerHostBoundaryPerformanceEnvelope;
  packetDigest: string;
  observation: ObservationSurfaceSummary;
}

export interface WorkerDiagnosticsSummaryReadPacket {
  envelopeFamily: "diagnosticsHistoryRead";
  readMode: "SummaryDiagnosticsRead";
  runtimeAuthority: "workerOwnedRuntime";
  diagnosticsSummaryReadCount: number;
  diagnosticsRichReadCount: number;
  diagnosticsColdReconstructionCount: number;
  workerFirstTruthDigest: string;
  diagnosticsSummaryDigest: string;
  richReadAvailabilityDigest: string;
  boundaryPerformance: WorkerHostBoundaryPerformanceEnvelope;
  packetDigest: string;
  summary: GraphSummary;
}

export interface WorkerDiagnosticsHistoryReadPacket {
  envelopeFamily: "diagnosticsHistoryRead";
  readMode: "RichDiagnosticsHistoryRead";
  runtimeAuthority: "workerOwnedRuntime";
  diagnosticsSummaryReadCount: number;
  diagnosticsRichReadCount: number;
  diagnosticsColdReconstructionCount: number;
  workerFirstTruthDigest: string;
  diagnosticsHistoryDigest: string;
  boundaryPerformance: WorkerHostBoundaryPerformanceEnvelope;
  packetDigest: string;
  history: ExecutionHistorySurfaceSummary;
}

export interface WorkerSnapshotEnvelopeArtifact {
  snapshotEnvelope: RuntimeSnapshotEnvelope;
  snapshotEnvelopeRestoreToken: string;
  snapshotEnvelopePortableWire: string;
}

export interface WorkerSnapshotArtifact {
  snapshot: RuntimeSnapshotArtifact;
  snapshotRestoreToken: string;
  snapshotPortableWire: string;
}

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
}

export interface WorkerBrowserHistoryIngressReport {
  envelopeFamily: "browserHistoryIngress";
  causality: WorkerHostBoundaryCausality;
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
  forgeProofReadmissionDigest: string;
  runtimeAdmittedLifecycleCount: number;
  runtimeMutationBreadth: number;
  workerFirstTruthDigest: string;
  performance: WorkerHostBoundaryPerformanceEnvelope;
  hostAcknowledgementIsAuthoritative: boolean;
  workerReadmissionRequired: boolean;
}

export interface WorkerOutputDeliveryRequest {
  outputIds: ReadonlyArray<string>;
}

export interface WorkerSignalReadbackRequest {
  signalIds: ReadonlyArray<string>;
}

export interface WorkerDeliveredOutput {
  id: string;
  value: unknown;
  payloadByteCount: number;
}

export interface WorkerReadbackSignal {
  id: string;
  value: unknown;
  payloadByteCount: number;
}

export interface WorkerOutputDeliveryPacket {
  envelopeFamily: "outputDelivery";
  deliveryMode: "CommittedOutputDelivery";
  runtimeAuthority: "workerOwnedRuntime";
  outputDeliveryPacketCount: number;
  outputDeliveryBreadth: number;
  outputPayloadByteCount: number;
  workerFirstTruthDigest: string;
  outputDigest: string;
  boundaryPerformance: WorkerHostBoundaryPerformanceEnvelope;
  packetDigest: string;
  outputs: ReadonlyArray<WorkerDeliveredOutput>;
}

export interface WorkerRuntimeEnvelopeImportReport {
  envelopeFamily: "workerRuntimeEnvelopeImport";
  importOutcome: string;
  rejectedCallbackCount: number;
  reattachedCallbackCount: number;
  hostCapabilityTransportCount: number;
  fallbackCount: number;
  rejectedCallbackIds: ReadonlyArray<string>;
  reattachedCallbackIds: ReadonlyArray<string>;
  workerFirstTruthDigest: string;
  importDigest: string;
}

export interface WorkerFirstRuntimeEnvelopeArtifact extends RuntimeEnvelope {
  runtimeEnvelopeRestoreToken: string;
  runtimeEnvelopeRestoreMode: "SameRuntimeExact";
  runtimeEnvelopePortableWire: string;
}

export interface WorkerSignalReadbackPacket {
  envelopeFamily: "signalReadback";
  readbackMode: "CommittedSignalReadback";
  runtimeAuthority: "workerOwnedRuntime";
  signalReadbackPacketCount: number;
  signalReadbackBreadth: number;
  signalPayloadByteCount: number;
  workerFirstTruthDigest: string;
  signalDigest: string;
  boundaryPerformance: WorkerHostBoundaryPerformanceEnvelope;
  packetDigest: string;
  signals: ReadonlyArray<WorkerReadbackSignal>;
}

export interface WorkerCommittedProjectionPacket {
  envelopeFamily: "workerCommittedProjection";
  deploymentPosture: "workerFirst";
  runtimeAuthority: "workerOwnedRuntime";
  workerFirstTruthDigest: string;
  projectionDigest: string;
  packetDigest: string;
  transaction: WorkerCommittedTransactionEnvelope;
  outputs: WorkerOutputDeliveryPacket;
  diagnosticsSummary: WorkerDiagnosticsSummaryReadPacket;
  diagnosticsHistory: WorkerDiagnosticsHistoryReadPacket;
}

export interface WorkerMainThreadHostBridgeCertificationPackage {
  certificationFamily: "mainThreadHostBridgeCertification";
  coveredSuiteCount: number;
  hostCapabilityEnvelopeDigest: string;
  hostCapabilityLifecycleDigest: string;
  hostCapabilityTruthDigest: string;
  hostCapabilityCoalescingDigest: string;
  hostCapabilityArtifactDigest: string;
  browserHistoryEnvelopeDigest: string;
  browserHistoryRouteTruthDigest: string;
  browserHistoryContinuityDigest: string;
  browserHistoryReplayRestoreDigest: string;
  hostEffectRequestDigest: string;
  hostEffectAcknowledgedRequestDigest: string;
  hostEffectAcknowledgementDigest: string;
  hostEffectLifecycleArtifact: string;
  hostEffectLifecycleIntegrityDigest: string;
  forgeProofReadmissionDigest: string;
  hostBoundaryCausalityDigest: string;
  boundaryPerformanceDigest: string;
  workerFirstTruthDigest: string;
  ambientHostReadDenied: boolean;
  hostAcknowledgementIsAuthoritative: boolean;
  certificationDigest: string;
}

export interface CreateWorkerRuntimeBridgeOptions {
  workerUrl?: string | URL;
}

export interface WorkerRuntimeBridge {
  bootstrapRecord(): Promise<WorkerRuntimeBootstrapRecord>;
  workerRuntimeShellLock(): Promise<WorkerRuntimeShellLock>;
  publishPortableGraph(
    publication: WorkerPortableGraphPublication,
  ): Promise<WorkerGraphPublicationSummary>;
  applyTransaction(
    transactionOps: ReadonlyArray<TransactionOp>,
  ): Promise<WorkerCommittedTransactionEnvelope>;
  applyTransactionProjection(
    request: WorkerCommittedProjectionRequest,
  ): Promise<WorkerCommittedProjectionPacket>;
  attachObservationDelivery(
    request: WorkerObservationDeliveryAttachRequest,
  ): Promise<WorkerLifecycleControlPacket>;
  detachObservationDelivery(
    request: WorkerObservationDeliveryDetachRequest,
  ): Promise<WorkerLifecycleControlPacket>;
  why(id: string): Promise<WhySummary>;
  health(): Promise<HealthSummary>;
  latestFlow(): Promise<FlowSurfaceSummary | null>;
  latestObservation(): Promise<ObservationSurfaceSummary | null>;
  performanceSummary(): Promise<WebPerformanceSummary>;
  latestFailure(): Promise<FailureSummary | null>;
  latestRollback(): Promise<RollbackDiagnostic | null>;
  latestFrontierExecution(): Promise<FrontierExecutionSummary | null>;
  latestInvalidationTraceRecords(): Promise<ReadonlyArray<InvalidationTraceRecord>>;
  recentHistory(): Promise<ReadonlyArray<ExecutionHistorySummary>>;
  currentBranch(): Promise<RuntimeBranchHandle>;
  branches(): Promise<ReadonlyArray<RuntimeBranchHandle>>;
  createBranch(name: string): Promise<RuntimeBranchHandle>;
  switchBranch(branchId: bigint | number): Promise<unknown>;
  planMergeBranches(
    sourceBranchId: bigint | number,
    targetBranchId: bigint | number,
  ): Promise<MergePlanArtifact>;
  planMergeBranchesWithProof(
    sourceBranchId: bigint | number,
    targetBranchId: bigint | number,
  ): Promise<MergePlanProofEnvelope>;
  mergeBranches(
    sourceBranchId: bigint | number,
    targetBranchId: bigint | number,
  ): Promise<MergeResultArtifact>;
  mergeBranchesWithProof(
    sourceBranchId: bigint | number,
    targetBranchId: bigint | number,
  ): Promise<MergeResultProofEnvelope>;
  planMergePolicyPreview(request: MergePolicyPreviewRequest): Promise<MergePlanArtifact>;
  planMergePolicyPreviewWithProof(
    request: MergePolicyPreviewRequest,
  ): Promise<MergePlanProofEnvelope>;
  mergeBranchesPolicyPreview(request: MergePolicyPreviewRequest): Promise<MergeResultArtifact>;
  mergeBranchesPolicyPreviewWithProof(
    request: MergePolicyPreviewRequest,
  ): Promise<MergeResultProofEnvelope>;
  replayForBranch(branchId: bigint | number): Promise<ReplaySummary>;
  branchSnapshotId(branchId: bigint | number): Promise<bigint | number>;
  branchSnapshotEnvelope(branchId: bigint | number): Promise<RuntimeSnapshotEnvelope>;
  branchSnapshotArtifact(branchId: bigint | number): Promise<WorkerSnapshotArtifact>;
  branchSnapshotEnvelopeArtifact(
    branchId: bigint | number,
  ): Promise<WorkerSnapshotEnvelopeArtifact>;
  branchSnapshotEnvelopeWire(branchId: bigint | number): Promise<string>;
  branchSnapshotEnvelopePortableWire(branchId: bigint | number): Promise<string>;
  restoreBranchSnapshotArtifact(
    branchId: bigint | number,
    snapshot: RuntimeSnapshotArtifact,
  ): Promise<unknown>;
  restoreBranchSnapshotWire(branchId: bigint | number, snapshot: string): Promise<unknown>;
  restoreBranchSnapshotPortableWire(branchId: bigint | number, snapshot: string): Promise<unknown>;
  restoreBranchSnapshotById(branchId: bigint | number, snapshotId: bigint | number): Promise<unknown>;
  branchStateProof(branchId: bigint | number): Promise<BranchStateProofReport>;
  replayFor(id: string): Promise<ReplaySummary>;
  lineageFor(id: string): Promise<LineageSummary>;
  readVersions(ids: ReadonlyArray<string>): Promise<ReadonlyArray<VersionSummary>>;
  evaluateDirty(): Promise<RunSummary>;
  exportDefinitions(): Promise<RuntimeDefinitionEnvelope>;
  exportWorkerRuntimeEnvelope(): Promise<RuntimeEnvelope>;
  exportWorkerSnapshotEnvelope(): Promise<RuntimeSnapshotEnvelope>;
  exportWorkerSnapshotEnvelopeArtifact(): Promise<WorkerSnapshotEnvelopeArtifact>;
  exportWorkerSnapshotEnvelopeWire(): Promise<string>;
  exportWorkerSnapshotEnvelopePortableWire(): Promise<string>;
  restoreSnapshotEnvelope(snapshot: RuntimeSnapshotEnvelope): Promise<unknown>;
  restoreSnapshotEnvelopeWire(snapshot: string): Promise<unknown>;
  restoreSnapshotEnvelopePortableWire(snapshot: string): Promise<unknown>;
  exportWorkerRuntimeEnvelopeWire(): Promise<string>;
  exportWorkerRuntimeEnvelopePortableWire(): Promise<string>;
  admitWorkerRuntimeEnvelopeImportWire(
    envelope: string,
  ): Promise<WorkerRuntimeEnvelopeImportReport>;
  admitWorkerRuntimeEnvelopeImportPortableWire(
    envelope: string,
  ): Promise<WorkerRuntimeEnvelopeImportReport>;
  runtimeProofReport(): Promise<RuntimeProofReport>;
  admitHostCapabilityIngress(
    batch: WorkerHostCapabilityIngressBatch,
  ): Promise<WorkerHostCapabilityIngressReport>;
  admitBrowserHistoryIngress(
    ingress: WorkerBrowserHistoryIngress,
  ): Promise<WorkerBrowserHistoryIngressReport>;
  issueHostEffectRequest(
    request: WorkerHostEffectRequest,
  ): Promise<WorkerHostEffectRequestEnvelope>;
  admitHostEffectAcknowledgement(
    acknowledgement: WorkerHostEffectAcknowledgement,
  ): Promise<WorkerHostEffectAcknowledgementReport>;
  certifyMainThreadHostBridge(): Promise<WorkerMainThreadHostBridgeCertificationPackage>;
  deliverLatestObservation(): Promise<WorkerObservationDeliveryPacket>;
  deliverOutputs(request: WorkerOutputDeliveryRequest): Promise<WorkerOutputDeliveryPacket>;
  readSignals(request: WorkerSignalReadbackRequest): Promise<WorkerSignalReadbackPacket>;
  readDiagnosticsSummary(): Promise<WorkerDiagnosticsSummaryReadPacket>;
  readDiagnosticsHistory(): Promise<WorkerDiagnosticsHistoryReadPacket>;
  terminate(): Promise<void>;
}

export interface WorkerFirstDiagnosticsFacade {
  why(id: string): Promise<WhySummary>;
  health(): Promise<HealthSummary>;
  summaryNow(): Promise<GraphSummary>;
  historyNow(): Promise<ExecutionHistorySurfaceSummary>;
  latestFlow(): Promise<FlowSurfaceSummary | null>;
  latestObservation(): Promise<ObservationSurfaceSummary | null>;
  latestHostCapabilityEvent(): WorkerFirstHostCapabilityDiagnosticsEvent | null;
  recentHostCapabilityEvents(): ReadonlyArray<WorkerFirstHostCapabilityDiagnosticsEvent>;
  performanceSummary(): Promise<WebPerformanceSummary>;
  hostCapabilityReport(): Promise<WorkerFirstHostCapabilityDiagnosticsReport>;
  latestFailure(): Promise<FailureSummary | null>;
  latestRollback(): Promise<RollbackDiagnostic | null>;
  latestFrontierExecution(): Promise<FrontierExecutionSummary | null>;
  latestInvalidationTraceRecords(): Promise<ReadonlyArray<InvalidationTraceRecord>>;
  recentHistory(): Promise<ReadonlyArray<ExecutionHistorySummary>>;
}

export interface WorkerFirstHistoryFacade {
  replay_for(id: string): Promise<ReplaySummary>;
  lineage_for(id: string): Promise<LineageSummary>;
  recentHistory(): Promise<ReadonlyArray<ExecutionHistorySummary>>;
  snapshot(): Promise<WorkerSnapshotEnvelopeArtifact>;
  restore_snapshot(snapshot: RuntimeSnapshotEnvelope): Promise<unknown>;
  restore_exact_snapshot(snapshot: WorkerSnapshotEnvelopeArtifact): Promise<unknown>;
  current_branch(): Promise<RuntimeBranchHandle>;
  branches(): Promise<ReadonlyArray<RuntimeBranchHandle>>;
  create_branch(name: string): Promise<RuntimeBranchHandle>;
  switch_branch(branchId: bigint | number): Promise<unknown>;
  replay_for_branch(branchId: bigint | number): Promise<ReplaySummary>;
  branch_snapshot(branchId: bigint | number): Promise<WorkerSnapshotArtifact>;
  branch_snapshot_id(branchId: bigint | number): Promise<bigint | number>;
  branch_snapshot_envelope(branchId: bigint | number): Promise<WorkerSnapshotEnvelopeArtifact>;
  restore_branch_snapshot(branchId: bigint | number, snapshot: RuntimeSnapshotArtifact): Promise<unknown>;
  restore_exact_branch_snapshot(
    branchId: bigint | number,
    snapshot: WorkerSnapshotArtifact,
  ): Promise<unknown>;
  restore_branch_snapshot_by_id(branchId: bigint | number, snapshotId: bigint | number): Promise<unknown>;
  plan_merge_branches(
    sourceBranchId: bigint | number,
    targetBranchId: bigint | number,
  ): Promise<MergePlanArtifact>;
  plan_merge_branches_with_proof(
    sourceBranchId: bigint | number,
    targetBranchId: bigint | number,
  ): Promise<MergePlanProofEnvelope>;
  merge_branches(
    sourceBranchId: bigint | number,
    targetBranchId: bigint | number,
  ): Promise<MergeResultArtifact>;
  merge_branches_with_proof(
    sourceBranchId: bigint | number,
    targetBranchId: bigint | number,
  ): Promise<MergeResultProofEnvelope>;
  plan_merge_policy_preview(request: MergePolicyPreviewRequest): Promise<MergePlanArtifact>;
  plan_merge_policy_preview_with_proof(
    request: MergePolicyPreviewRequest,
  ): Promise<MergePlanProofEnvelope>;
  merge_branches_policy_preview(request: MergePolicyPreviewRequest): Promise<MergeResultArtifact>;
  merge_branches_policy_preview_with_proof(
    request: MergePolicyPreviewRequest,
  ): Promise<MergeResultProofEnvelope>;
  branch_state_proof(branchId: bigint | number): Promise<BranchStateProofReport>;
  replay_parity_proof(
    expectedBranchId: bigint | number,
    replayedBranchId: bigint | number,
  ): Promise<ReplayParityProofReport>;
  replay_artifact_proof(
    expected: ReplayArtifactProofInput,
    replayedBranchId: bigint | number,
  ): Promise<ReplayArtifactProofReport>;
}

export interface WorkerFirstAdaptersFacade {
  exportDefinitions(): Promise<RuntimeDefinitionEnvelope>;
  exportRuntimeEnvelope(): Promise<WorkerFirstRuntimeEnvelopeArtifact>;
  replaceRuntimeEnvelope(
    envelope: WorkerFirstRuntimeEnvelopeArtifact | RuntimeEnvelope,
  ): Promise<WorkerRuntimeEnvelopeImportReport>;
  restoreExactRuntimeEnvelope(
    envelope: WorkerFirstRuntimeEnvelopeArtifact,
  ): Promise<WorkerRuntimeEnvelopeImportReport>;
  runtimeProofReport(): Promise<RuntimeProofReport>;
  hostCapabilityTransportReport(
    envelope?: WorkerFirstRuntimeEnvelopeArtifact | RuntimeEnvelope,
  ): Promise<HostCapabilityTransportReport>;
}

export function createWorkerRuntimeBridge(
  options?: CreateWorkerRuntimeBridgeOptions,
): WorkerRuntimeBridge;

export function createWorkerFirstDiagnosticsFacade(session: {
  bridge: WorkerRuntimeBridge;
}): WorkerFirstDiagnosticsFacade;

export function createWorkerFirstHistoryFacade(session: {
  bridge: WorkerRuntimeBridge;
}): WorkerFirstHistoryFacade;

export function createWorkerFirstAdaptersFacade(session: {
  bridge: WorkerRuntimeBridge;
}): WorkerFirstAdaptersFacade;
