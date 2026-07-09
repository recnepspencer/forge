import { FormDeclarationError } from "../form_errors.js";
import { cloneFormValue, stableValueDigest } from "../values/value_paths.js";
import { requireDeclaredValidationMessageTarget } from "../messages/targets.js";
import { requireDeclaredField } from "./declarations.js";

const VALIDATION_KINDS = new Set([
  "valid",
  "warning",
  "invalid",
  "pending",
  "blocked",
  "unavailable",
  "parseFailure",
]);
const MESSAGE_SEVERITIES = new Set(["info", "warning", "error"]);
const MESSAGE_AUDIENCES = new Set(["user", "developer", "system"]);
const MESSAGE_VISIBILITIES = new Set(["hidden", "visible", "summary", "blocked"]);
const BLOCKING_KINDS = new Set(["invalid", "pending", "blocked", "unavailable", "parseFailure"]);

export function parseFailureArtifact(field, error, rawValue) {
  return Object.freeze({
    kind: "parseFailure",
    field: field.id,
    message: Object.freeze({
      code: "form.parse.failure",
      message: error instanceof Error ? error.message : "Unable to parse input",
      severity: "error",
      target: field.id,
      audience: "user",
      visibility: "visible",
      recovery: Object.freeze([
        Object.freeze({
          kind: "edit",
          label: "Revise input",
        }),
      ]),
    }),
    rawDigest: stableValueDigest(rawValue),
  });
}

export function normalizeValidationArtifact(artifact, declaration = null, declaredFieldIds = null) {
  if (artifact == null || artifact === true) {
    return validArtifact(declaration?.field ?? undefined, null);
  }
  if (!artifact || typeof artifact !== "object" || !VALIDATION_KINDS.has(artifact.kind)) {
    throw new FormDeclarationError("validator returned an undeclared validation artifact shape", {
      artifact,
    });
  }
  if (artifact.kind === "valid") {
    const field = artifact.field ?? declaration?.field ?? undefined;
    requireDeclaredArtifactField(declaredFieldIds, field);
    return validArtifact(field, artifact.digest ?? null);
  }
  if (artifact.kind === "pending") {
    return normalizePendingValidationArtifact(artifact, declaration, declaredFieldIds);
  }
  if (artifact.kind === "blocked") {
    return normalizeBlockedValidationArtifact(artifact, declaration, declaredFieldIds);
  }
  if (artifact.kind === "unavailable") {
    return normalizeUnavailableValidationArtifact(artifact, declaration, declaredFieldIds);
  }
  const field = artifact.field ?? declaration?.field ?? undefined;
  requireDeclaredArtifactField(declaredFieldIds, field);
  return Object.freeze({
    kind: artifact.kind,
    field,
    message: normalizeMessage(artifact.message, field, declaredFieldIds),
    ...(artifact.rawDigest === undefined ? {} : { rawDigest: artifact.rawDigest }),
  });
}

export function validationReadinessBlockers(validation) {
  return validation.artifacts
    .filter((artifact) => BLOCKING_KINDS.has(artifact.kind))
    .map((artifact) => ({
      kind: `validation:${artifact.kind}`,
      field: artifact.field,
      reason: validationBlockerReason(artifact),
    }));
}

export function visibleMessages(validation) {
  return Object.freeze(
    validation.artifacts
      .map((artifact) => artifact.message ?? null)
      .filter((message) => message && message.visibility !== "hidden"),
  );
}

function normalizePendingValidationArtifact(artifact, declaration, declaredFieldIds) {
  if (typeof artifact.asyncValidationId !== "string" || artifact.asyncValidationId.length === 0) {
    throw new FormDeclarationError("pending validation artifacts require asyncValidationId");
  }
  const field = artifact.field ?? declaration?.field ?? undefined;
  requireDeclaredArtifactField(declaredFieldIds, field);
  return Object.freeze({
    kind: "pending",
    field,
    asyncValidationId: artifact.asyncValidationId,
  });
}

function normalizeBlockedValidationArtifact(artifact, declaration, declaredFieldIds) {
  if (!Array.isArray(artifact.blockers)) {
    throw new FormDeclarationError("blocked validation artifacts require blockers");
  }
  const field = artifact.field ?? declaration?.field ?? undefined;
  requireDeclaredArtifactField(declaredFieldIds, field);
  return Object.freeze({
    kind: "blocked",
    field,
    reason: requireNonEmptyString(artifact.reason, "blocked validation reason"),
    blockers: Object.freeze([...artifact.blockers]),
  });
}

function normalizeUnavailableValidationArtifact(artifact, declaration, declaredFieldIds) {
  const field = artifact.field ?? declaration?.field ?? undefined;
  requireDeclaredArtifactField(declaredFieldIds, field);
  return Object.freeze({
    kind: "unavailable",
    field,
    reason: requireNonEmptyString(artifact.reason, "unavailable validation reason"),
    detail: requireNonEmptyString(artifact.detail, "unavailable validation detail"),
  });
}

function validArtifact(field, value) {
  return Object.freeze({
    kind: "valid",
    ...(field === undefined || field === null ? {} : { field }),
    digest: typeof value === "string" ? value : stableValueDigest(value),
  });
}

function normalizeMessage(message, target, declaredFieldIds) {
  if (!message || typeof message !== "object") {
    throw new FormDeclarationError("validation message artifacts require a message object");
  }
  const severity = message.severity ?? "error";
  const audience = message.audience ?? "user";
  const visibility = message.visibility ?? "visible";
  if (!MESSAGE_SEVERITIES.has(severity) || !MESSAGE_AUDIENCES.has(audience) || !MESSAGE_VISIBILITIES.has(visibility)) {
    throw new FormDeclarationError("validation message artifact uses an unsupported severity/audience/visibility", {
      message,
    });
  }
  const messageTarget = message.target ?? target;
  requireDeclaredValidationMessageTarget(declaredFieldIds, messageTarget);
  return Object.freeze({
    code: requireNonEmptyString(message.code, "validation message code"),
    ...(message.message === undefined ? {} : { message: String(message.message) }),
    severity,
    target: messageTarget,
    audience,
    visibility,
    ...(message.accessibility === undefined ? {} : { accessibility: cloneFormValue(message.accessibility) }),
    ...(message.recovery === undefined ? {} : { recovery: cloneFormValue(message.recovery) }),
  });
}

function validationBlockerReason(artifact) {
  if (artifact.message?.message) {
    return artifact.message.message;
  }
  return artifact.reason ?? artifact.kind;
}

function requireDeclaredArtifactField(declaredFieldIds, fieldId) {
  if (fieldId === undefined || fieldId === null || !declaredFieldIds) {
    return;
  }
  requireDeclaredField(declaredFieldIds, fieldId);
}

function requireNonEmptyString(value, name) {
  if (typeof value !== "string" || value.length === 0) {
    throw new FormDeclarationError(`${name} must be a non-empty string`);
  }
  return value;
}
