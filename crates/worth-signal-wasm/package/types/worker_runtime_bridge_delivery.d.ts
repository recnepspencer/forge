import type {
  ExecutionHistorySurfaceSummary,
  GraphSummary,
  ObservationSurfaceSummary,
} from "./diagnostics.js";
import type { RunSummary, TransactionOp } from "./model.js";
import type { WorkerHostBoundaryPerformanceEnvelope } from "./worker_runtime_bridge_boundary.js";

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
