import { stableValueDigest } from "../values/value_paths.js";
import { baseLane } from "./auxiliary_lanes.js";
import { readEntryBootstrapArtifact } from "./entry_bootstrap.js";
import { busyVisibilityStatus } from "./policy_timing.js";

export function entryLane(
  policy,
  sourceAdmission,
  draftRestore,
  sourceCompatibility,
  validation,
  asyncValidationHistory,
  readiness,
  host,
  inputCapabilities,
  accessibility,
  layoutMeasurement,
  nowMs,
) {
  const bootstrap = readEntryBootstrapArtifact(
    policy.bootstrap,
    sourceAdmission,
    draftRestore,
    sourceCompatibility,
    validation,
    asyncValidationHistory,
    readiness,
    host,
    inputCapabilities,
    accessibility,
    layoutMeasurement,
  );
  if (sourceCompatibility.posture === "unavailable" && !policy.bootstrap?.sourceCompatibility) {
    return baseLane("entry", "entry", policy.scope, policy, "unavailable", {
      reason: sourceCompatibility.reason ?? "entry presentation is unavailable because source compatibility drift is unresolved",
      token: stableValueDigest(sourceCompatibility),
      acknowledgementRequired: policy.unavailableAcknowledgement === "required",
      bootstrap,
    });
  }
  if (bootstrap?.posture === "unavailable") {
    return baseLane("entry", "entry", policy.scope, policy, "unavailable", {
      reason: bootstrap.reason,
      token: bootstrap.digest,
      acknowledgementRequired: policy.unavailableAcknowledgement === "required",
      bootstrap,
    });
  }
  const validationPending = validation.summary.pending > 0;
  if (bootstrap?.posture === "pending" || (validationPending && !policy.bootstrap?.validation)) {
    const pendingEntry = asyncValidationHistory.find((entry) => entry.resultKind === "pending") ?? null;
    const pendingValidation = policy.bootstrap?.validation
      ? bootstrap.dependencies.blocking.some((dependency) => dependency.dependency === "validation")
      : validationPending;
    const status = pendingValidation
      ? busyVisibilityStatus(policy, pendingEntry?.observedAtMs ?? nowMs, nowMs)
      : "pending";
    return baseLane("entry", "entry", policy.scope, policy, status, {
      reason: pendingEntryReason(status, bootstrap, validationPending, pendingValidation),
      token: stableValueDigest({
        bootstrapDigest: bootstrap?.digest ?? null,
        pendingValidation: validation.summary.pending,
        asyncValidationOperations: asyncValidationHistory.length,
      }),
      bootstrap,
    });
  }
  return baseLane("entry", "entry", policy.scope, policy, "ready", {
    reason: "entry presentation is settled",
    token: bootstrap?.digest ?? null,
    bootstrap,
  });
}

function pendingEntryReason(status, bootstrap, validationPending, pendingValidation) {
  if (status === "pending") {
    if (bootstrap?.posture === "pending") {
      return pendingValidation
        ? "entry presentation is delaying busy reveal while bootstrap and validation settle"
        : bootstrap.reason;
    }
    return validationPending
      ? "entry presentation is delaying busy reveal while validation bootstrap starts"
      : "entry presentation is waiting for validation bootstrap to start";
  }
  if (bootstrap?.posture === "pending") {
    return pendingValidation
      ? "entry presentation is waiting for bootstrap and validation to settle"
      : bootstrap.reason;
  }
  return "entry presentation is waiting for validation bootstrap to settle";
}
