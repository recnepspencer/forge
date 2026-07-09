import { stableValueDigest } from "../values/value_paths.js";

export function readFormResourceLifecycleReport(summary, status, freshness) {
  const counts = Object.freeze({
    refreshCount: summary.diagnostics.counts.refreshCount,
    revalidateCount: summary.diagnostics.counts.revalidateCount,
    retryAttemptCount: summary.diagnostics.counts.retryAttemptCount,
    rejectionCount: summary.diagnostics.counts.rejectionCount,
    timeoutCount: summary.diagnostics.counts.timeoutCount,
    supersessionCount: summary.diagnostics.counts.supersessionCount,
    deliveryCount: summary.diagnostics.counts.deliveryCount,
  });
  const retry = Object.freeze({
    kind:
      status.kind === "rejected" || status.kind === "timedOut"
        ? "recommended"
        : "notNeeded",
    operation:
      status.kind === "rejected" || status.kind === "timedOut"
        ? status.operation
        : null,
    attemptCount: summary.diagnostics.counts.retryAttemptCount,
    reason:
      status.kind === "rejected"
        ? `resource line source rejected during ${status.operation}`
        : status.kind === "timedOut"
          ? `resource line source timed out during ${status.operation}`
          : null,
  });
  const supersession = Object.freeze({
    kind:
      summary.diagnostics.counts.supersessionCount > 0
      || summary.diagnostics.latest.supersededOperation != null
        ? "observed"
        : "none",
    count: summary.diagnostics.counts.supersessionCount,
    lastOperation: summary.diagnostics.latest.supersededOperation ?? null,
  });
  const deliveryBasis = Object.freeze({
    kind:
      freshness.kind === "stale" && freshness.reason === "deliveryInvalidate"
        ? "drifted"
        : "stable",
    currentBasisId: summary.diagnostics.latest.basisCurrentId,
    deliveryKind: summary.diagnostics.latest.deliveryKind,
    deliveryScope: summary.diagnostics.latest.deliveryScope,
    deliveryBasisId: summary.diagnostics.latest.deliveryBasisId,
    invalidationCause: summary.diagnostics.latest.invalidationCause,
    invalidationScope: summary.diagnostics.latest.invalidationScope,
  });
  return Object.freeze({
    status,
    freshness,
    activity: summary.diagnostics.activity,
    retry,
    supersession,
    deliveryBasis,
    counts,
    digest: stableValueDigest({
      status,
      freshness,
      activity: summary.diagnostics.activity,
      retry,
      supersession,
      deliveryBasis,
      counts,
    }),
  });
}
