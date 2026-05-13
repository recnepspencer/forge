function normalizeForProof(value) {
  return JSON.parse(JSON.stringify(value));
}

function normalizeVerificationPackageForConvergence(pkg) {
  return {
    committedValue: normalizeForProof(pkg.committedValue),
    requestPosture: {
      authKind: pkg.requestPosture.authKind,
      headerNames: normalizeForProof(pkg.requestPosture.headerNames),
      correlationId: pkg.requestPosture.correlationId,
      branchId: pkg.requestPosture.branchId,
      basisId: pkg.requestPosture.basisId,
      continuationKind: pkg.requestPosture.continuationKind,
      processingKind: pkg.requestPosture.processingKind,
      uploadKind: pkg.requestPosture.uploadKind,
    },
    processing: normalizeForProof(pkg.processing),
    upload: normalizeForProof(pkg.upload),
    lifecycle: {
      status: normalizeForProof(pkg.lifecycle.status),
      freshness: normalizeForProof(pkg.lifecycle.freshness),
      lastOperation: pkg.lifecycle.lastOperation,
      lastOutcome: pkg.lifecycle.lastOutcome,
      pendingOperation: pkg.lifecycle.pendingOperation,
      visibleValueVersion: pkg.lifecycle.visibleValueVersion,
      refreshCount: pkg.lifecycle.refreshCount,
      revalidateCount: pkg.lifecycle.revalidateCount,
      retryAttemptCount: pkg.lifecycle.retryAttemptCount,
      rejectionCount: pkg.lifecycle.rejectionCount,
      timeoutCount: pkg.lifecycle.timeoutCount,
      supersessionCount: pkg.lifecycle.supersessionCount,
      invalidationCount: pkg.lifecycle.invalidationCount,
      patchCount: pkg.lifecycle.patchCount,
      deliveryCount: pkg.lifecycle.deliveryCount,
      basisAdvanceCount: pkg.lifecycle.basisAdvanceCount,
    },
    continuity: normalizeForProof(pkg.continuity),
    reconciliation: {
      broadReplace: pkg.reconciliation.broadReplace,
      narrowItem: pkg.reconciliation.narrowItem,
      narrowSummary: pkg.reconciliation.narrowSummary,
      aspectNames: normalizeForProof(pkg.reconciliation.aspectNames),
      summaryNames: normalizeForProof(pkg.reconciliation.summaryNames),
      lastPatchKind: pkg.reconciliation.lastPatchKind,
      lastPatchScope: pkg.reconciliation.lastPatchScope,
      lastPatchedItemId: pkg.reconciliation.lastPatchedItemId,
      lastPatchedAspect: pkg.reconciliation.lastPatchedAspect,
      lastPatchedSummary: pkg.reconciliation.lastPatchedSummary,
    },
    diagnostics: {
      lastOperation: pkg.diagnostics.lastOperation,
      lastOutcome: pkg.diagnostics.lastOutcome,
      pendingOperation: pkg.diagnostics.pendingOperation,
      lastErrorMessage: pkg.diagnostics.lastErrorMessage,
      summary: {
        current: normalizeForProof(pkg.diagnostics.summary.current),
        activity: normalizeForProof(pkg.diagnostics.summary.activity),
        counts: {
          basisAdvanceCount: pkg.diagnostics.summary.counts.basisAdvanceCount,
          deliveryCount: pkg.diagnostics.summary.counts.deliveryCount,
        },
        latest: {
          basisCurrentId: pkg.diagnostics.summary.latest.basisCurrentId,
          basisAdvanceFromId: pkg.diagnostics.summary.latest.basisAdvanceFromId,
          basisAdvanceToId: pkg.diagnostics.summary.latest.basisAdvanceToId,
          deliveryKind: pkg.diagnostics.summary.latest.deliveryKind,
          deliveryScope: pkg.diagnostics.summary.latest.deliveryScope,
          ...readOptionalIdentityMigrationSummaryLatest(
            pkg.diagnostics.summary.latest,
          ),
          ...readOptionalMutationResponseIdentityMigrationSummaryLatest(
            pkg.diagnostics.summary.latest,
          ),
        },
      },
    },
    historyReplayRestore: {
      branch: normalizeForProof(pkg.historyReplayRestore.branch),
      basis: {
        currentBasisId: pkg.historyReplayRestore.basis.currentBasisId,
        advanceCount: pkg.historyReplayRestore.basis.advanceCount,
        lastAdvanceFromId: pkg.historyReplayRestore.basis.lastAdvanceFromId,
        lastAdvanceToId: pkg.historyReplayRestore.basis.lastAdvanceToId,
      },
      availability: {
        branch: normalizeForProof(pkg.historyReplayRestore.availability.branch),
        restoreExact: normalizeForProof(
          pkg.historyReplayRestore.availability.restoreExact,
        ),
      },
      lifecycleLength: pkg.historyReplayRestore.lifecycleLength,
      lastLifecycleEvent: pkg.historyReplayRestore.lastLifecycleEvent,
      identityMigrationCount: pkg.historyReplayRestore.identityMigrationCount,
      latestIdentityMigration: normalizeForProof(
        pkg.historyReplayRestore.latestIdentityMigration,
      ),
    },
    binaryDownload: normalizeForProof(pkg.binaryDownload),
    deliveryProvenance: {
      deliveryCount: pkg.deliveryProvenance.deliveryCount,
      lastDeliveryKind: pkg.deliveryProvenance.lastDeliveryKind,
      lastDeliveryScope: pkg.deliveryProvenance.lastDeliveryScope,
      lastDeliveryBasisId: pkg.deliveryProvenance.lastDeliveryBasisId,
      basisCurrentId: pkg.deliveryProvenance.basisCurrentId,
      basisAdvanceCount: pkg.deliveryProvenance.basisAdvanceCount,
      basisAdvanceFromId: pkg.deliveryProvenance.basisAdvanceFromId,
      basisAdvanceToId: pkg.deliveryProvenance.basisAdvanceToId,
    },
    boundaryPerformanceEnvelope: normalizeForProof(pkg.boundaryPerformanceEnvelope),
    typedDenials: {
      branch: normalizeForProof(pkg.typedDenials.branch),
      restoreExact: normalizeForProof(pkg.typedDenials.restoreExact),
    },
  };
}

function projectAuthoringConvergenceDigest(pkg) {
  return {
    committedValue: normalizeForProof(pkg.committedValue),
    requestPosture: {
      authKind: pkg.requestPosture.authKind,
      headerNames: normalizeForProof(pkg.requestPosture.headerNames),
      correlationId: pkg.requestPosture.correlationId,
      branchId: pkg.requestPosture.branchId,
      basisId: pkg.requestPosture.basisId,
      continuationKind: pkg.requestPosture.continuationKind,
      processingKind: pkg.requestPosture.processingKind,
      uploadKind: pkg.requestPosture.uploadKind,
    },
    processing: normalizeForProof(pkg.processing),
    upload: normalizeForProof(pkg.upload),
    lifecycle: {
      status: normalizeForProof(pkg.lifecycle.status),
      freshness: normalizeForProof(pkg.lifecycle.freshness),
      lastOperation: pkg.lifecycle.lastOperation,
      lastOutcome: pkg.lifecycle.lastOutcome,
      pendingOperation: pkg.lifecycle.pendingOperation,
      visibleValueVersion: pkg.lifecycle.visibleValueVersion,
      basisAdvanceCount: pkg.lifecycle.basisAdvanceCount,
    },
    continuity: normalizeForProof(pkg.continuity),
    reconciliation: {
      broadReplace: pkg.reconciliation.broadReplace,
      narrowItem: pkg.reconciliation.narrowItem,
      narrowSummary: pkg.reconciliation.narrowSummary,
      aspectNames: normalizeForProof(pkg.reconciliation.aspectNames),
      summaryNames: normalizeForProof(pkg.reconciliation.summaryNames),
      lastPatchKind: pkg.reconciliation.lastPatchKind,
      lastPatchScope: pkg.reconciliation.lastPatchScope,
      lastPatchedItemId: pkg.reconciliation.lastPatchedItemId,
      lastPatchedAspect: pkg.reconciliation.lastPatchedAspect,
      lastPatchedSummary: pkg.reconciliation.lastPatchedSummary,
    },
    diagnostics: {
      lastOperation: pkg.diagnostics.lastOperation,
      lastOutcome: pkg.diagnostics.lastOutcome,
      pendingOperation: pkg.diagnostics.pendingOperation,
      lastErrorMessage: pkg.diagnostics.lastErrorMessage,
      summary: {
        current: normalizeForProof(pkg.diagnostics.summary.current),
        activity: normalizeForProof(pkg.diagnostics.summary.activity),
        latest: {
          basisCurrentId: pkg.diagnostics.summary.latest.basisCurrentId,
          basisAdvanceFromId: pkg.diagnostics.summary.latest.basisAdvanceFromId,
          basisAdvanceToId: pkg.diagnostics.summary.latest.basisAdvanceToId,
          ...readOptionalIdentityMigrationSummaryLatest(
            pkg.diagnostics.summary.latest,
          ),
          ...readOptionalMutationResponseIdentityMigrationSummaryLatest(
            pkg.diagnostics.summary.latest,
          ),
        },
      },
    },
    historyReplayRestore: {
      branch: normalizeForProof(pkg.historyReplayRestore.branch),
      basis: {
        currentBasisId: pkg.historyReplayRestore.basis.currentBasisId,
        advanceCount: pkg.historyReplayRestore.basis.advanceCount,
        lastAdvanceFromId: pkg.historyReplayRestore.basis.lastAdvanceFromId,
        lastAdvanceToId: pkg.historyReplayRestore.basis.lastAdvanceToId,
      },
      availability: {
        replayExact:
          pkg.historyReplayRestore.availability.replayExact.kind === "available"
            ? {
                kind: "available",
                mode: pkg.historyReplayRestore.availability.replayExact.mode,
              }
            : normalizeForProof(pkg.historyReplayRestore.availability.replayExact),
        branch: normalizeForProof(pkg.historyReplayRestore.availability.branch),
        restoreExact: normalizeForProof(
          pkg.historyReplayRestore.availability.restoreExact,
        ),
      },
      lifecycleLength: pkg.historyReplayRestore.lifecycleLength,
      lastLifecycleEvent: pkg.historyReplayRestore.lastLifecycleEvent,
      identityMigrationCount: pkg.historyReplayRestore.identityMigrationCount,
      latestIdentityMigration: normalizeForProof(
        pkg.historyReplayRestore.latestIdentityMigration,
      ),
    },
    binaryDownload: normalizeForProof(pkg.binaryDownload),
  };
}

function projectReplayReconstructionDigest(appPackage) {
  return {
    detail: projectReplayLineDigest(appPackage.detail),
    nativeCollection: projectReplayLineDigest(appPackage.nativeCollection),
    externalCollection: projectReplayLineDigest(appPackage.externalCollection),
    paged: projectReplayLineDigest(appPackage.paged),
    retryDetail: projectReplayLineDigest(appPackage.retryDetail),
    transferDetail: projectReplayLineDigest(appPackage.transferDetail),
  };
}

function projectReplayLineDigest(pkg) {
  return {
    committedValue: normalizeForProof(pkg.committedValue),
    requestPosture: normalizeForProof(pkg.requestPosture),
    processing: normalizeForProof(pkg.processing),
    upload: normalizeForProof(pkg.upload),
    continuity: {
      continuity: pkg.continuity.continuity,
      hasVisibleValue: pkg.continuity.hasVisibleValue,
    },
    reconciliation: normalizeForProof(pkg.reconciliation),
    diagnostics: {
      summary: {
        current: {
          freshness: normalizeForProof(pkg.diagnostics.summary.current.freshness),
          hasVisibleValue: pkg.diagnostics.summary.current.hasVisibleValue,
        },
        counts: normalizeForProof(pkg.diagnostics.summary.counts),
        latest: normalizeForProof(pkg.diagnostics.summary.latest),
      },
    },
    historyReplayRestore: {
      basis: normalizeForProof(pkg.historyReplayRestore.basis),
      availability: {
        replayExact: normalizeForProof(
          pkg.historyReplayRestore.availability.replayExact,
        ),
        restoreExact: normalizeForProof(
          pkg.historyReplayRestore.availability.restoreExact,
        ),
      },
    },
    binaryDownload: normalizeForProof(pkg.binaryDownload),
    deliveryProvenance: normalizeForProof(pkg.deliveryProvenance),
    externalCompatibility: normalizeForProof(pkg.externalCompatibility),
    typedDenials: {
      replay: normalizeForProof(pkg.typedDenials.replay),
      replayExact: normalizeForProof(pkg.typedDenials.replayExact),
      restoreExact: normalizeForProof(pkg.typedDenials.restoreExact),
    },
  };
}

function readOptionalIdentityMigrationSummaryLatest(latest) {
  if (!("identityMigrationCount" in latest)) {
    return {};
  }
  return {
    identityMigrationCount: latest.identityMigrationCount,
    lastIdentityMigration: normalizeForProof(latest.lastIdentityMigration),
  };
}

function readOptionalMutationResponseIdentityMigrationSummaryLatest(latest) {
  if (!("mutationResponseIdentityMigrationDigest" in latest)) {
    return "mutationResponsePlanId" in latest
      ? { mutationResponsePlanId: latest.mutationResponsePlanId }
      : {};
  }
  return {
    ...( "mutationResponsePlanId" in latest
      ? { mutationResponsePlanId: latest.mutationResponsePlanId }
      : {}),
    mutationResponseIdentityMigrationDigest:
      latest.mutationResponseIdentityMigrationDigest,
    mutationResponseIdentityMigrationNeeded:
      latest.mutationResponseIdentityMigrationNeeded,
    mutationResponseIdentityMigrationPartialAdmission:
      latest.mutationResponseIdentityMigrationPartialAdmission,
    mutationResponseIdentityMigrationTargetCount:
      latest.mutationResponseIdentityMigrationTargetCount,
    mutationResponseIdentityMigrationExactTargetCount:
      latest.mutationResponseIdentityMigrationExactTargetCount,
    mutationResponseIdentityMigrationExecutionDigest:
      latest.mutationResponseIdentityMigrationExecutionDigest,
    mutationResponseIdentityMigrationFallbackDigest:
      latest.mutationResponseIdentityMigrationFallbackDigest,
  };
}

function projectHostileAppPackage(lines) {
  return Object.freeze({
    detail: lines.detail.history().verificationPackage(),
    nativeCollection: lines.nativeCollection.history().verificationPackage(),
    externalCollection: lines.externalCollection.history().verificationPackage(),
    paged: lines.paged.history().verificationPackage(),
    retryDetail: lines.retryDetail.history().verificationPackage(),
    transferDetail: lines.transferDetail.history().verificationPackage(),
  });
}

function projectConvergenceDigest(appPackage) {
  return {
    detail: normalizeVerificationPackageForConvergence(appPackage.detail),
    nativeCollection: normalizeVerificationPackageForConvergence(
      appPackage.nativeCollection,
    ),
    externalCollection: normalizeVerificationPackageForConvergence(
      appPackage.externalCollection,
    ),
    paged: normalizeVerificationPackageForConvergence(appPackage.paged),
    retryDetail: normalizeVerificationPackageForConvergence(
      appPackage.retryDetail,
    ),
    transferDetail: normalizeVerificationPackageForConvergence(
      appPackage.transferDetail,
    ),
  };
}

export {
  normalizeForProof,
  projectAuthoringConvergenceDigest,
  normalizeVerificationPackageForConvergence,
  projectConvergenceDigest,
  projectHostileAppPackage,
  projectReplayReconstructionDigest,
};
