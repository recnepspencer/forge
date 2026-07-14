import { stableValueDigest } from "../values/value_paths.js";

export function readEntryBootstrapArtifact(
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
) {
  if (!policy || Object.values(policy).every((value) => value === false)) {
    return null;
  }
  const hostUnavailableFacts = policy.hostFacts
    ? Object.values(host.facts)
      .filter((fact) => fact.posture === "unavailable")
      .map((fact) => fact.fact)
    : [];
  const inputUnavailableFields = policy.inputCapabilities
    ? inputCapabilities.fields
      .filter((field) => field.posture === "unavailable")
      .map((field) => field.field)
    : [];
  const focusTarget = policy.focusTarget ? accessibility.focusTarget : null;
  const measurementPending = policy.layoutMeasurement && layoutMeasurement.latestSnapshot === null;
  const dependencies = entryBootstrapDependencies(
    policy,
    sourceAdmission,
    draftRestore,
    sourceCompatibility,
    validation,
    readiness,
    hostUnavailableFacts,
    inputUnavailableFields,
    focusTarget,
    measurementPending,
    layoutMeasurement,
  );

  let posture = "ready";
  let reason = "entry bootstrap dependencies are settled";
  if (dependencies.unavailable.length > 0) {
    posture = "unavailable";
    reason = dependencies.unavailable[0].reason;
  } else if (dependencies.blocking.length > 0) {
    posture = "pending";
    reason = dependencies.blocking[0].reason;
  }

  const artifact = {
    posture,
    reason,
    requirements: Object.freeze({
      sourceAdmission: policy.sourceAdmission,
      draftRestore: policy.draftRestore,
      sourceCompatibility: policy.sourceCompatibility,
      validation: policy.validation,
      readiness: policy.readiness,
      hostFacts: policy.hostFacts,
      inputCapabilities: policy.inputCapabilities,
      focusTarget: policy.focusTarget,
      layoutMeasurement: policy.layoutMeasurement,
    }),
    sourceAdmission: sourceAdmission ? Object.freeze({
      status: sourceAdmission.status,
      reason: sourceAdmission.reason,
      token: sourceAdmission.token ?? null,
    }) : null,
    draftRestore: draftRestore ? Object.freeze({
      status: draftRestore.status,
      reason: draftRestore.reason,
      token: draftRestore.token ?? null,
    }) : null,
    hostUnavailableFacts: Object.freeze(hostUnavailableFacts),
    inputUnavailableFields: Object.freeze(inputUnavailableFields),
    focusTarget: focusTarget ? Object.freeze({
      posture: focusTarget.posture,
      field: focusTarget.field,
      target: focusTarget.target,
      reason: focusTarget.reason,
    }) : null,
    layoutMeasurementPending: measurementPending,
    dependencies,
  };
  return Object.freeze({
    ...artifact,
    digest: stableValueDigest(artifact),
  });
}

function entryBootstrapDependencies(
  policy,
  sourceAdmission,
  draftRestore,
  sourceCompatibility,
  validation,
  readiness,
  hostUnavailableFacts,
  inputUnavailableFields,
  focusTarget,
  measurementPending,
  layoutMeasurement,
) {
  const required = [];
  if (policy.sourceAdmission) {
    required.push(sourceBootstrapDependencyArtifact("sourceAdmission", sourceAdmission));
  }
  if (policy.draftRestore) {
    required.push(sourceBootstrapDependencyArtifact("draftRestore", draftRestore));
  }
  if (policy.sourceCompatibility) {
    required.push(sourceCompatibilityDependencyArtifact(sourceCompatibility));
  }
  if (policy.validation) {
    required.push(validationDependencyArtifact(validation));
  }
  if (policy.readiness) {
    required.push(readinessDependencyArtifact(readiness));
  }
  if (policy.hostFacts) {
    required.push(hostFactsDependencyArtifact(hostUnavailableFacts));
  }
  if (policy.inputCapabilities) {
    required.push(inputCapabilitiesDependencyArtifact(inputUnavailableFields));
  }
  if (policy.focusTarget) {
    required.push(focusTargetDependencyArtifact(focusTarget));
  }
  if (policy.layoutMeasurement) {
    required.push(layoutMeasurementDependencyArtifact(measurementPending, layoutMeasurement));
  }
  const summary = {
    required: Object.freeze(required),
    blocking: Object.freeze(required.filter((dependency) => (
      dependency.status === "pending" ||
      dependency.status === "busy" ||
      dependency.status === "settling"
    ))),
    unavailable: Object.freeze(required.filter((dependency) => dependency.status === "unavailable")),
    satisfied: Object.freeze(required.filter((dependency) => dependency.status === "ready")),
  };
  return Object.freeze({
    ...summary,
    digest: stableValueDigest(summary),
  });
}

function sourceBootstrapDependencyArtifact(dependency, artifact) {
  if (artifact === null) {
    return Object.freeze({
      dependency,
      status: "unavailable",
      target: null,
      reason: `entry bootstrap requires ${dependency} but the source does not declare it`,
      digest: null,
    });
  }
  return Object.freeze({
    dependency,
    status: artifact.status,
    target: null,
    reason: artifact.reason,
    digest: stableValueDigest(artifact),
  });
}

function sourceCompatibilityDependencyArtifact(sourceCompatibility) {
  return Object.freeze({
    dependency: "sourceCompatibility",
    status: sourceCompatibility.posture === "unavailable" ? "unavailable" : "ready",
    target: sourceCompatibility.currentSchemaVersion,
    reason: sourceCompatibility.reason ?? sourceCompatibilityReason(sourceCompatibility.posture),
    digest: stableValueDigest({
      posture: sourceCompatibility.posture,
      currentSchemaVersion: sourceCompatibility.currentSchemaVersion,
      draftSchemaVersion: sourceCompatibility.draftSchemaVersion,
      reason: sourceCompatibility.reason,
    }),
  });
}

function validationDependencyArtifact(validation) {
  const pending = validation.summary.pending > 0;
  return Object.freeze({
    dependency: "validation",
    status: pending ? "pending" : "ready",
    target: null,
    reason: pending
      ? "entry bootstrap is waiting for initial validation to settle"
      : validation.summary.invalid > 0 || validation.summary.blocked > 0 || validation.summary.unavailable > 0
        ? "initial validation is derived with visible blockers"
        : "initial validation is derived",
    digest: stableValueDigest(validation.summary),
  });
}

function readinessDependencyArtifact(readiness) {
  return Object.freeze({
    dependency: "readiness",
    status: "ready",
    target: null,
    reason: readiness.canSubmit
      ? "initial readiness is derived and can submit"
      : `initial readiness is derived with ${readiness.blockers.length} blocker(s)`,
    digest: stableValueDigest({
      canSubmit: readiness.canSubmit,
      blockers: readiness.blockers,
    }),
  });
}

function hostFactsDependencyArtifact(hostUnavailableFacts) {
  return Object.freeze({
    dependency: "hostFacts",
    status: hostUnavailableFacts.length > 0 ? "unavailable" : "ready",
    target: null,
    reason: hostUnavailableFacts.length > 0
      ? `entry bootstrap is unavailable because host facts are missing: ${hostUnavailableFacts.join(", ")}`
      : "declared host facts are available",
    digest: stableValueDigest(hostUnavailableFacts),
  });
}

function inputCapabilitiesDependencyArtifact(inputUnavailableFields) {
  return Object.freeze({
    dependency: "inputCapabilities",
    status: inputUnavailableFields.length > 0 ? "unavailable" : "ready",
    target: inputUnavailableFields[0] ?? null,
    reason: inputUnavailableFields.length > 0
      ? `entry bootstrap is unavailable because declared adapter capabilities are missing for: ${inputUnavailableFields.join(", ")}`
      : "declared adapter capabilities are available",
    digest: stableValueDigest(inputUnavailableFields),
  });
}

function focusTargetDependencyArtifact(focusTarget) {
  if (focusTarget?.posture === "unavailable") {
    return Object.freeze({
      dependency: "focusTarget",
      status: "unavailable",
      target: focusTarget.field,
      reason: focusTarget.reason,
      digest: stableValueDigest(focusTarget),
    });
  }
  return Object.freeze({
    dependency: "focusTarget",
    status: "ready",
    target: focusTarget?.target ?? null,
    reason: focusTarget?.reason ?? "focus target is settled",
    digest: focusTarget ? stableValueDigest(focusTarget) : null,
  });
}

function layoutMeasurementDependencyArtifact(measurementPending, layoutMeasurement) {
  return Object.freeze({
    dependency: "layoutMeasurement",
    status: measurementPending ? "pending" : "ready",
    target: null,
    reason: measurementPending
      ? "entry bootstrap is waiting for the first declared layout measurement snapshot"
      : "declared layout measurement is settled",
    digest: layoutMeasurement.latestSnapshot?.snapshotDigest ?? null,
  });
}

function sourceCompatibilityReason(posture) {
  if (posture === "migrated") {
    return "draft schema compatibility was restored through migration";
  }
  if (posture === "compatible") {
    return "draft schema compatibility remained valid across source drift";
  }
  if (posture === "current") {
    return "source schema compatibility is current";
  }
  return "source schema compatibility is not declared";
}
