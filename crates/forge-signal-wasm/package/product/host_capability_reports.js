function stableClone(value) {
  if (Array.isArray(value)) {
    return value.map(stableClone);
  }
  if (value && typeof value === "object") {
    return Object.keys(value)
      .sort()
      .reduce((acc, key) => {
        acc[key] = stableClone(value[key]);
        return acc;
      }, {});
  }
  return value;
}

function canonicalJson(value) {
  return JSON.stringify(stableClone(value));
}

function digestString(value) {
  let hash = 2166136261;
  for (let index = 0; index < value.length; index += 1) {
    hash ^= value.charCodeAt(index);
    hash = Math.imul(hash, 16777619);
  }
  return `f1a-${(hash >>> 0).toString(16).padStart(8, "0")}`;
}

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

export function buildHostCapabilityDiagnosticsReport(performanceSummary, recentEvents) {
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
  }));
  const families = groupedFamilies(events, (event) => ({
    kind: event?.kind ?? null,
    compatibility: event?.compatibility ?? null,
    invalidationMode: event?.invalidationMode ?? null,
    queuedInvalidationCount: event?.queuedInvalidationCount ?? 0,
    touchedNodes: event?.touchedNodes ?? 0,
    reevaluatedNodes: event?.reevaluatedNodes ?? 0,
    deniedCallbackIds: [...(event?.deniedCallbackIds ?? [])].sort(),
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
  const lineageDigest = digestString(canonicalJson(lineage));
  const breadthDigest = digestString(canonicalJson(breadth));
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
      reevaluationCount: performanceSummary?.hostCapabilityReevaluationCount ?? 0,
      invalidationTouchedNodeCount: performanceSummary?.hostCapabilityInvalidationTouchedNodeCount ?? 0,
      noOpInvalidationSuppressedCount: performanceSummary?.hostCapabilityNoOpInvalidationSuppressedCount ?? 0,
      staleInvalidationIgnoredCount: performanceSummary?.hostCapabilityStaleInvalidationIgnoredCount ?? 0,
      compatibilityDenialCount: performanceSummary?.hostCapabilityCompatibilityDenialCount ?? 0,
      unavailabilityArtifactCount: performanceSummary?.hostCapabilityUnavailabilityArtifactCount ?? 0,
      broadFanoutDenialCount: performanceSummary?.hostCapabilityBroadFanoutDenialCount ?? 0,
      retainedEventCount: events.length,
    },
    lineage,
    lineageDigest,
    breadth,
    breadthDigest,
    families,
  };
  const digestInput = canonicalJson(report);
  return {
    ...report,
    digest: digestString(digestInput),
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
  const digestInput = canonicalJson(report);
  return {
    ...report,
    digest: digestString(digestInput),
  };
}
