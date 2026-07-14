import { readLineDiagnostics } from "../reads/line_diagnostics_read.js";
import { readLineDiagnosticsSummary } from "../reads/line_diagnostics_summary_read.js";
import { readLineDownload } from "../reads/line_download_read.js";
import { readLineFreshness } from "../reads/line_freshness_read.js";
import { readLineStatus } from "../reads/line_status_read.js";
import { readLineValue } from "../reads/line_value_read.js";
import {
  readMutationResponsePlanRecord,
} from "../../mutation/resource_mutation_response_diagnostics_projection.js";

function freezeWithVisibleSelection(value, visibleSelection) {
  Object.defineProperty(value, "visibleSelection", {
    value: visibleSelection,
    enumerable: false,
    configurable: false,
    writable: false,
  });
  return Object.freeze(value);
}

function readLineVerificationPackage(materialization, historyRead) {
  const descriptor = materialization.lineIdentity;
  const request = materialization.requestState.readDescriptor();
  const value = readLineValue(materialization);
  const status = readLineStatus(materialization);
  const freshness = readLineFreshness(materialization);
  const diagnostics = readLineDiagnostics(materialization);
  const summary = readLineDiagnosticsSummary(materialization);
  const download = readLineDownload(materialization);
  const patchCapable = materialization.patch.broadReplace;
  const mutationResponsePlanRecord =
    readMutationResponsePlanRecord(diagnostics);
  const latestIdentityMigration = readLatestIdentityMigrationDigest(
    historyRead.lifecycle,
  );
  return Object.freeze({
    declaration: Object.freeze({
      familyKind: descriptor.family.kind,
      familyId: descriptor.family.familyId,
      canonicalKey: descriptor.canonicalParams.canonicalKey,
      runtimeLineId: descriptor.runtimeLineId,
      scopeId: descriptor.scopeId,
    }),
    committedValue: value,
    requestPosture: Object.freeze({
      authKind: request.auth.kind,
      headerNames: Object.freeze(
        Object.keys(request.context.headers ?? {}).sort(),
      ),
      correlationId: request.context.correlationId,
      branchId: request.context.branchId,
      basisId: request.context.basisId,
      continuationKind: request.continuation.kind,
      processingKind: request.processingJob.kind,
      uploadKind: request.uploadTransport.kind,
      effectsName: request.effects?.name ?? null,
    }),
    processing: Object.freeze({
      kind: diagnostics.processing.kind,
      completionKind: diagnostics.processing.completionKind,
      jobId: diagnostics.processing.jobId,
      message: diagnostics.processing.message,
    }),
    upload: Object.freeze({
      kind: diagnostics.upload.kind,
      transportKind: diagnostics.upload.transportKind,
      uploadId: diagnostics.upload.uploadId,
      finalizeRequired: diagnostics.upload.finalizeRequired,
      awaitingProcessing: diagnostics.upload.awaitingProcessing,
      message: diagnostics.upload.message,
      hasDescriptor: diagnostics.upload.descriptor !== null,
    }),
    lifecycle: freezeWithVisibleSelection({
      status,
      freshness,
      lastOperation: diagnostics.lastOperation,
      lastOutcome: diagnostics.lastOutcome,
      pendingOperation: diagnostics.pendingOperation,
      visibleValueVersion: diagnostics.visibleValueVersion,
      refreshCount: diagnostics.refreshCount,
      revalidateCount: diagnostics.revalidateCount,
      retryAttemptCount: diagnostics.retryAttemptCount,
      rejectionCount: diagnostics.rejectionCount,
      timeoutCount: diagnostics.timeoutCount,
      supersessionCount: diagnostics.supersessionCount,
      invalidationCount: diagnostics.invalidationCount,
      patchCount: diagnostics.patchCount,
      deliveryCount: diagnostics.deliveryCount,
      basisAdvanceCount: diagnostics.basis.advanceCount,
      lastEffect: diagnostics.lastEffect,
    }, diagnostics.visibleSelection),
    continuity: freezeWithVisibleSelection({
      continuity: diagnostics.continuity,
      hasVisibleValue: summary.current.hasVisibleValue,
      visibleValueVersion: summary.current.visibleValueVersion,
    }, summary.current.visibleSelection),
    reconciliation: Object.freeze({
      broadReplace: materialization.patch.broadReplace,
      narrowItem: materialization.patch.narrowItem,
      narrowField: materialization.patch.narrowField,
      narrowRegion: materialization.patch.narrowRegion,
      narrowJsonPath: materialization.patch.narrowJsonPath,
      narrowSummary: materialization.patch.narrowSummary,
      fieldNames: materialization.patch.fieldNames,
      regionNames: materialization.patch.regionNames,
      jsonPathNames: materialization.patch.jsonPathNames,
      aspectNames: materialization.patch.aspectNames,
      summaryNames: materialization.patch.summaryNames,
      lastPatchKind: diagnostics.lastPatchKind,
      lastPatchScope: diagnostics.lastPatchScope,
      lastPatchedItemId: diagnostics.lastPatchedItemId,
      lastPatchedField: diagnostics.lastPatchedField,
      lastPatchedRegion: diagnostics.lastPatchedRegion,
      lastPatchedPath: diagnostics.lastPatchedPath,
      lastPatchedAspect: diagnostics.lastPatchedAspect,
      lastPatchedSummary: diagnostics.lastPatchedSummary,
    }),
    diagnostics: Object.freeze({
      lastOperation: diagnostics.lastOperation,
      lastOutcome: diagnostics.lastOutcome,
      pendingOperation: diagnostics.pendingOperation,
      lastErrorMessage: diagnostics.lastErrorMessage,
      summary: {
        current: summary.current,
        activity: summary.activity,
        counts: summary.counts,
        latest: summary.latest,
      },
    }),
    ...(mutationResponsePlanRecord === null
      ? {}
      : {
          mutationResponse: Object.freeze({
            plan: mutationResponsePlanRecord.plan,
            planCount: mutationResponsePlanRecord.planCount,
          }),
        }),
    historyReplayRestore: Object.freeze({
      replay: historyRead.replay,
      lineage: historyRead.lineage,
      branch: historyRead.branch,
      basis: historyRead.basis,
      availability: historyRead.availability,
      lifecycleLength: historyRead.lifecycle.length,
      lastLifecycleEvent:
        historyRead.lifecycle.at(-1)?.event ?? null,
      identityMigrationCount: historyRead.lifecycle.filter((entry) =>
        entry.event === "identityMigrated").length,
      latestIdentityMigration,
    }),
    binaryDownload: Object.freeze({
      count: download.count,
      readyCount: download.readyCount,
      unavailableCount: download.unavailableCount,
      incompatibleCount: download.incompatibleCount,
      descriptorKinds: Object.freeze(
        download.descriptors.map((descriptorValue) => ({
          kind: descriptorValue.kind,
          downloadKind: descriptorValue.download.kind,
        })),
      ),
    }),
    deliveryProvenance: Object.freeze({
      deliveryCount: diagnostics.deliveryCount,
      lastDeliveryKind: diagnostics.lastDeliveryKind,
      lastDeliveryScope: diagnostics.lastDeliveryScope,
      lastDeliveryPacketId: diagnostics.lastDeliveryPacketId,
      lastDeliveryBasisId: diagnostics.lastDeliveryBasisId,
      lastEffect: diagnostics.lastEffect,
      basisCurrentId: diagnostics.basis.currentBasisId,
      basisAdvanceCount: diagnostics.basis.advanceCount,
      basisAdvanceFromId: diagnostics.basis.lastAdvanceFromBasisId,
      basisAdvanceToId: diagnostics.basis.lastAdvanceToBasisId,
    }),
    externalCompatibility: descriptor.compatibility ?? Object.freeze({
      kind: "native",
    }),
    boundaryPerformanceEnvelope: Object.freeze({
      lifecycleEntryCount: historyRead.lifecycle.length,
      downloadDescriptorCount: download.count,
      summaryReadShape: "inspectionSummary",
      commonLineReadShape: "groupedLineSummary",
    }),
    capabilities: Object.freeze({
      summary: true,
      diagnosticsSummary: true,
      requestRead: true,
      processingRead: true,
      uploadRead: true,
      downloadRead: true,
      historyRead: true,
      patch: patchCapable,
      deliver: patchCapable,
      reconciliationRead: patchCapable,
      broadReplace: materialization.patch.broadReplace,
      narrowItem: materialization.patch.narrowItem,
      narrowField: materialization.patch.narrowField,
      narrowRegion: materialization.patch.narrowRegion,
      narrowJsonPath: materialization.patch.narrowJsonPath,
      narrowSummary: materialization.patch.narrowSummary,
    }),
      typedDenials: Object.freeze({
        replay: historyRead.availability.replay.kind === "unavailable"
          ? historyRead.availability.replay
          : null,
        replayExact: historyRead.availability.replayExact.kind === "unavailable"
          ? historyRead.availability.replayExact
          : null,
        lineage: historyRead.availability.lineage.kind === "unavailable"
          ? historyRead.availability.lineage
          : null,
      branch: historyRead.availability.branch.kind === "unavailable"
        ? historyRead.availability.branch
        : null,
      restoreExact: historyRead.availability.restoreExact.kind === "unavailable"
        ? historyRead.availability.restoreExact
        : null,
    }),
  });
}

function readLatestIdentityMigrationDigest(lifecycle) {
  for (let index = lifecycle.length - 1; index >= 0; index -= 1) {
    const entry = lifecycle[index];
    if (entry.event === "identityMigrated" && entry.identityMigration) {
      return entry.identityMigration;
    }
  }
  return null;
}

export { readLineVerificationPackage };
