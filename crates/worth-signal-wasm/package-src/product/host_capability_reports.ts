import {
  canonicalDiagnosticJson,
  digestCanonicalDiagnosticValue,
  digestDiagnosticString,
} from "./canonical_diagnostic_digest.js";

function groupedFamilies(entries, projector) {
  const byFamily = new Map();
  for (const entry of entries) {
    const family = typeof entry?.family === "string" ? entry.family : "unknown";
    const bucket = byFamily.get(family) ?? [];
    bucket.push(projector(entry));
    byFamily.set(family, bucket);
  }
  return [...byFamily.entries()]
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([family, values]) => ({
      family,
      entries: values,
    }));
}

export function buildHostCapabilityDiagnosticsReport(
  performanceSummary,
  recentEvents,
  callbackHostDependencies = null,
) {
  const normalizedCallbackHostDependencies =
    normalizeCallbackHostDependencies(callbackHostDependencies);
  const events = Array.isArray(recentEvents) ? recentEvents : [];
  const lineage = events.map((event) => ({
    sequence: event?.sequence ?? 0,
    family: event?.family ?? "unknown",
    registrationId: event?.registrationId ?? "unknown",
    kind: event?.kind ?? null,
    compatibility: event?.compatibility ?? null,
    invalidationMode: event?.invalidationMode ?? null,
    queuedInvalidationCount: event?.queuedInvalidationCount ?? 0,
    touchedNodes: event?.touchedNodes ?? 0,
    reevaluatedNodes: event?.reevaluatedNodes ?? 0,
    portableImportOutcome: event?.portableImportOutcome ?? null,
    deniedCallbackIds: [...(event?.deniedCallbackIds ?? [])].sort(),
    errorCode: event?.errorCode ?? null,
    denialReason: event?.denialReason ?? null,
    deniedBeforePublication: event?.deniedBeforePublication ?? null,
    failureMessage: event?.failureMessage ?? null,
  }));
  const families = groupedFamilies(events, (event) => ({
    kind: event?.kind ?? null,
    compatibility: event?.compatibility ?? null,
    invalidationMode: event?.invalidationMode ?? null,
    queuedInvalidationCount: event?.queuedInvalidationCount ?? 0,
    touchedNodes: event?.touchedNodes ?? 0,
    reevaluatedNodes: event?.reevaluatedNodes ?? 0,
    deniedCallbackIds: [...(event?.deniedCallbackIds ?? [])].sort(),
    errorCode: event?.errorCode ?? null,
    denialReason: event?.denialReason ?? null,
    deniedBeforePublication: event?.deniedBeforePublication ?? null,
    failureMessage: event?.failureMessage ?? null,
  })).map((family) => ({
    family: family.family,
    eventCount: family.entries.length,
    latestKind: family.entries.at(-1)?.kind ?? null,
    latestCompatibility: family.entries.at(-1)?.compatibility ?? null,
    invalidationModes: [...new Set(family.entries.map((entry) => entry.invalidationMode).filter(Boolean))].sort(),
    maxQueuedInvalidationCount: Math.max(0, ...family.entries.map((entry) => entry.queuedInvalidationCount)),
    maxTouchedNodes: Math.max(0, ...family.entries.map((entry) => entry.touchedNodes)),
    maxReevaluatedNodes: Math.max(0, ...family.entries.map((entry) => entry.reevaluatedNodes)),
    deniedCallbackIds: [...new Set(family.entries.flatMap((entry) => entry.deniedCallbackIds))].sort(),
    failureCount: family.entries.filter((entry) => entry.failureMessage !== null).length,
  }));
  const breadth = {
    maxQueuedInvalidationCount: Math.max(0, ...lineage.map((entry) => entry.queuedInvalidationCount)),
    maxTouchedNodes: Math.max(0, ...lineage.map((entry) => entry.touchedNodes)),
    maxReevaluatedNodes: Math.max(0, ...lineage.map((entry) => entry.reevaluatedNodes)),
    families: families.map((family) => ({
      family: family.family,
      eventCount: family.eventCount,
      maxQueuedInvalidationCount: family.maxQueuedInvalidationCount,
      maxTouchedNodes: family.maxTouchedNodes,
      maxReevaluatedNodes: family.maxReevaluatedNodes,
    })),
  };
  const lineageDigest = digestCanonicalDiagnosticValue(lineage);
  const breadthDigest = digestCanonicalDiagnosticValue(breadth);
  const report = {
    totals: {
      registrationCount: performanceSummary?.hostCapabilityRegistrationCount ?? 0,
      disposalCount: performanceSummary?.hostCapabilityDisposalCount ?? 0,
      readCount: performanceSummary?.hostCapabilityReadCount ?? 0,
      pollCount: performanceSummary?.hostCapabilityPollCount ?? 0,
      noOpPollCount: performanceSummary?.hostCapabilityNoOpPollCount ?? 0,
      manualCommitCount: performanceSummary?.hostCapabilityManualCommitCount ?? 0,
      noOpManualCommitCount: performanceSummary?.hostCapabilityNoOpManualCommitCount ?? 0,
      invalidationCount: performanceSummary?.hostCapabilityInvalidationCount ?? 0,
      invalidationBatchFlushCount: performanceSummary?.hostCapabilityInvalidationBatchFlushCount ?? 0,
      dependencyRefreshCount: performanceSummary?.hostCapabilityDependencyRefreshCount ?? 0,
      dependencyRefreshFailureCount:
        performanceSummary?.hostCapabilityDependencyRefreshFailureCount ?? 0,
      reevaluationCount: performanceSummary?.hostCapabilityReevaluationCount ?? 0,
      invalidationTouchedNodeCount: performanceSummary?.hostCapabilityInvalidationTouchedNodeCount ?? 0,
      noOpInvalidationSuppressedCount: performanceSummary?.hostCapabilityNoOpInvalidationSuppressedCount ?? 0,
      staleInvalidationIgnoredCount: performanceSummary?.hostCapabilityStaleInvalidationIgnoredCount ?? 0,
      compatibilityDenialCount: performanceSummary?.hostCapabilityCompatibilityDenialCount ?? 0,
      unavailabilityArtifactCount: performanceSummary?.hostCapabilityUnavailabilityArtifactCount ?? 0,
      readDenialCount: performanceSummary?.hostCapabilityReadDenialCount ?? 0,
      broadFanoutDenialCount: performanceSummary?.hostCapabilityBroadFanoutDenialCount ?? 0,
      retainedEventCount: events.length,
    },
    lineage,
    lineageDigest,
    breadth,
    breadthDigest,
    families,
    callbackHostDependencies: normalizedCallbackHostDependencies,
  };
  const boundaryPerformanceEnvelope = {
    perReadHostRpcCount: 0,
    hostIngressEventCount: report.totals.invalidationBatchFlushCount,
    hostDependencyRefreshCount: report.totals.dependencyRefreshCount,
    callbackHostDependencyEdgeCount:
      report.callbackHostDependencies.totals.dependencyEdgeCount,
    mainThreadHostReadCount: report.totals.readCount,
    mainThreadRuntimeReevaluationCount: report.totals.reevaluationCount,
  };
  boundaryPerformanceEnvelope.digest = digestCanonicalDiagnosticValue(
    boundaryPerformanceEnvelope,
  );
  const callbackHostReadCertification = buildCallbackHostReadCertification({
    callbackHostDependencies: report.callbackHostDependencies,
    lineageDigest,
    breadthDigest,
    boundaryPerformanceEnvelope,
    ambientHostReadDenialArtifact: latestAmbientHostReadDenialArtifact(lineage),
  });
  const reportWithEnvelope = {
    ...report,
    boundaryPerformanceEnvelope,
    callbackHostReadCertification,
  };
  const digestInput = canonicalDiagnosticJson(reportWithEnvelope);
  return {
    ...reportWithEnvelope,
    digest: digestDiagnosticString(digestInput),
  };
}

function normalizeCallbackHostDependencies(callbackHostDependencies) {
  const normalized = callbackHostDependencies ?? {
    totals: {
      callbackCount: 0,
      dependentCallbackCount: 0,
      dependencyEdgeCount: 0,
      distinctDependencyCount: 0,
    },
    dependencies: [],
    callbacks: [],
  };
  return {
    ...normalized,
    dependencyDigest: normalized.dependencyDigest
      ?? digestCanonicalDiagnosticValue(normalized.dependencies ?? []),
    callbackDigest: normalized.callbackDigest
      ?? digestCanonicalDiagnosticValue(normalized.callbacks ?? []),
    digest: normalized.digest
      ?? digestCanonicalDiagnosticValue({
        totals: normalized.totals,
        dependencies: normalized.dependencies ?? [],
        callbacks: normalized.callbacks ?? [],
      }),
  };
}

function buildCallbackHostReadCertification(evidence) {
  const certification = {
    artifactFamily: "CallbackHostReadDependencyAdmission",
    callbackHostReadDependencyDigest: evidence.callbackHostDependencies.digest,
    hostCapabilityIngressDigest: evidence.lineageDigest,
    callbackRecomputationDigest: evidence.callbackHostDependencies.callbackDigest,
    boundaryPerformanceEnvelopeDigest: evidence.boundaryPerformanceEnvelope.digest,
    workerOwnedDependencyEdgeCount:
      evidence.callbackHostDependencies.totals.dependencyEdgeCount,
    perReadHostRpcCount: evidence.boundaryPerformanceEnvelope.perReadHostRpcCount,
    ambientHostReadDenialArtifact: evidence.ambientHostReadDenialArtifact,
    breadthDigest: evidence.breadthDigest,
  };
  return {
    ...certification,
    digest: digestCanonicalDiagnosticValue(certification),
  };
}

function latestAmbientHostReadDenialArtifact(lineage) {
  const denial = lineage.findLast((event) => event.kind === "HostCapabilityReadDenied");
  if (!denial) {
    return {
      errorCode: "computeCallbackForeignRuntimeReadDenied",
      deniedBeforePublication: true,
    };
  }
  return {
    errorCode: denial.errorCode,
    family: denial.family,
    registrationId: denial.registrationId,
    compatibility: denial.compatibility,
    denialReason: denial.denialReason,
    deniedBeforePublication: denial.deniedBeforePublication === true,
  };
}

export function buildHostCapabilityTransportReport(unavailableCallbacks) {
  const artifacts = Array.isArray(unavailableCallbacks) ? unavailableCallbacks : [];
  const flatTransports = artifacts.flatMap((artifact) => {
    const transports = Array.isArray(artifact?.hostCapabilityTransports)
      ? artifact.hostCapabilityTransports
      : [];
    return transports.map((transport) => ({
      callbackId: artifact?.id ?? null,
      family: transport?.family ?? null,
      compatibility: transport?.compatibility ?? null,
      exactRestoreOutcome: transport?.exactRestoreOutcome ?? null,
      portableImportOutcome: transport?.portableImportOutcome ?? null,
      portableImportReason: transport?.portableImportReason ?? null,
    }));
  });
  const families = groupedFamilies(flatTransports, (entry) => entry).map((family) => {
    const entries = family.entries;
    return {
      family: family.family,
      callbackIds: [...new Set(entries.map((entry) => entry.callbackId).filter(Boolean))].sort(),
      compatibilities: [...new Set(entries.map((entry) => entry.compatibility).filter(Boolean))].sort(),
      exactRestoreOutcomes: [...new Set(entries.map((entry) => entry.exactRestoreOutcome).filter(Boolean))].sort(),
      portableImportOutcomes: [...new Set(entries.map((entry) => entry.portableImportOutcome).filter(Boolean))].sort(),
      deniedCallbackIds: [...new Set(entries
        .filter((entry) => entry.portableImportOutcome === "Denied" || entry.portableImportOutcome === "Incompatible")
        .map((entry) => entry.callbackId)
        .filter(Boolean))].sort(),
      unavailableCallbackIds: [...new Set(entries
        .filter((entry) => entry.portableImportOutcome === "Unavailable")
        .map((entry) => entry.callbackId)
        .filter(Boolean))].sort(),
    };
  });
  const report = {
    totals: {
      unavailableArtifactCount: artifacts.length,
      transportEntryCount: flatTransports.length,
      deniedFamilyCount: families.filter((family) => family.portableImportOutcomes.includes("Denied") || family.portableImportOutcomes.includes("Incompatible")).length,
      unavailableFamilyCount: families.filter((family) => family.portableImportOutcomes.includes("Unavailable")).length,
      snapshotPortableFamilyCount: families.filter((family) => family.compatibilities.includes("SnapshotPortable")).length,
    },
    families,
  };
  const digestInput = canonicalDiagnosticJson(report);
  return {
    ...report,
    digest: digestDiagnosticString(digestInput),
  };
}
