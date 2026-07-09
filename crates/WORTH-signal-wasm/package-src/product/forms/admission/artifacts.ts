import { FormDeclarationError } from "../form_errors.js";
import { stableValueDigest } from "../values/value_paths.js";

const ADMISSION_POSTURES = new Set([
  "admitted",
  "denied",
  "blocked",
  "unavailable",
  "requiresApproval",
  "requiresSignature",
  "requiresReview",
  "requiresReason",
]);
const BLOCKING_POSTURES = new Set([
  "denied",
  "blocked",
  "unavailable",
  "requiresApproval",
  "requiresSignature",
  "requiresReview",
  "requiresReason",
]);
const REGULATED_POSTURES = new Set([
  "requiresApproval",
  "requiresSignature",
  "requiresReview",
  "requiresReason",
]);

export function normalizeAdmissionArtifact(artifact, declaration, binding = null) {
  if (artifact == null || artifact === true) {
    return admissionArtifact(declaration, { posture: "admitted" }, binding);
  }
  if (typeof artifact === "string") {
    return admissionArtifact(declaration, { posture: artifact }, binding);
  }
  if (!artifact || typeof artifact !== "object") {
    throw new FormDeclarationError("admission resolver returned an undeclared artifact shape", {
      artifact,
    });
  }
  return admissionArtifact(declaration, artifact, binding);
}

export function admissionReadinessBlockers(admission) {
  return admission.artifacts
    .filter((artifact) => BLOCKING_POSTURES.has(artifact.posture))
    .map((artifact) => ({
      kind: `admission:${artifact.posture}`,
      field: artifact.scope === "field" ? artifact.ownerId : undefined,
      action: artifact.scope === "action" ? artifact.ownerId : undefined,
      capability: artifact.capability,
      reason: admissionBlockerReason(artifact),
    }));
}

export function admissionCapabilityBlocker(admission, fieldId, capability) {
  const artifact = admission.artifacts.find((entry) => (
    entry.scope === "field" &&
    entry.ownerId === fieldId &&
    entry.capability === capability
  ));
  if (!artifact || !BLOCKING_POSTURES.has(artifact.posture)) {
    return null;
  }
  return Object.freeze({
    kind: `admission:${artifact.posture}`,
    field: fieldId,
    capability,
    reason: admissionBlockerReason(artifact),
    admission: artifact,
  });
}

function admissionArtifact(declaration, artifact, binding) {
  const posture = artifact.posture ?? "admitted";
  if (!ADMISSION_POSTURES.has(posture)) {
    throw new FormDeclarationError("admission artifact posture is not supported", {
      posture,
    });
  }
  const regulatedBinding = regulatedAdmissionBinding(posture, artifact, binding);
  return Object.freeze({
    kind: "admission",
    id: declaration.id,
    scope: declaration.scope,
    ownerId: declaration.ownerId,
    capability: declaration.capability,
    posture,
    dependencies: declaration.dependencies,
    ...(artifact.actorDigest === undefined ? {} : { actorDigest: String(artifact.actorDigest) }),
    ...(artifact.policyDigest === undefined ? {} : { policyDigest: String(artifact.policyDigest) }),
    ...(regulatedBinding === null ? {} : regulatedBinding),
    ...(artifact.reason === undefined ? {} : { reason: String(artifact.reason) }),
  });
}

function regulatedAdmissionBinding(posture, artifact, binding) {
  if (!REGULATED_POSTURES.has(posture)) {
    return null;
  }
  const actorDigest = requireNonEmptyString(artifact.actorDigest, `${posture} actorDigest`);
  const policyDigest = requireNonEmptyString(artifact.policyDigest, `${posture} policyDigest`);
  const currentActorDigest = optionalString(artifact.currentActorDigest, actorDigest);
  const currentPolicyDigest = optionalString(artifact.currentPolicyDigest, policyDigest);
  const expected = Object.freeze({
    actorDigest,
    policyDigest,
    sourceDigest: optionalString(artifact.sourceDigest, binding?.sourceDigest),
    patchDigest: optionalString(artifact.patchDigest, binding?.patchDigest),
    schemaDigest: optionalString(artifact.schemaDigest, binding?.schemaDigest),
  });
  const current = Object.freeze({
    actorDigest: currentActorDigest,
    policyDigest: currentPolicyDigest,
    sourceDigest: binding?.sourceDigest ?? null,
    patchDigest: binding?.patchDigest ?? null,
    schemaDigest: binding?.schemaDigest ?? null,
  });
  const staleReasons = staleBindingReasons(expected, current);
  return {
    actorDigest,
    policyDigest,
    binding: Object.freeze({
      expected,
      current,
      bindingDigest: stableValueDigest({
        expected,
        current,
        formBindingDigest: binding?.bindingDigest ?? null,
      }),
    }),
    stale: Object.freeze({
      isStale: staleReasons.length > 0,
      reasons: Object.freeze(staleReasons),
    }),
  };
}

function staleBindingReasons(expected, current) {
  return ["actorDigest", "policyDigest", "sourceDigest", "patchDigest", "schemaDigest"]
    .filter((key) => expected[key] !== null && expected[key] !== current[key])
    .map((key) => `${key} changed`);
}

function admissionBlockerReason(artifact) {
  if (artifact.stale?.isStale) {
    return `admission binding is stale: ${artifact.stale.reasons.join(", ")}`;
  }
  return artifact.reason ?? `${artifact.ownerId} admission is ${artifact.posture}`;
}

function optionalString(value, fallback) {
  if (value === undefined) {
    return fallback ?? null;
  }
  return String(value);
}

function requireNonEmptyString(value, name) {
  if (typeof value !== "string" || value.length === 0) {
    throw new FormDeclarationError(`${name} must be a non-empty string`);
  }
  return value;
}
