import { FormDeclarationError } from "../form_errors.js";
import { stableValueDigest } from "../values/value_paths.js";

const ATTACHMENT_OPERATIONS = new Set(["generic", "select", "stage", "preview", "remove", "clear"]);

export function createAttachmentPresentationStore() {
  let nextArtifactId = 1;
  let current = null;
  const history = [];
  return Object.freeze({
    report(update) {
      const normalized = normalizeAttachmentUpdate(update);
      current = attachmentArtifact(nextArtifactId++, normalized, "report");
      history.push(current);
      return current;
    },
    clear(reason = null) {
      current = null;
      const artifact = attachmentArtifact(nextArtifactId++, {
        status: "ready",
        target: null,
        reason: reason ?? "attachment presentation was cleared",
        token: null,
        section: null,
        selectedCount: null,
        stagedCount: null,
        failedCount: null,
        operation: "clear",
      }, "clear");
      history.push(artifact);
      return artifact;
    },
    current() {
      return current;
    },
    history() {
      return Object.freeze([...history]);
    },
  });
}

function normalizeAttachmentUpdate(update) {
  if (!update || typeof update !== "object" || Array.isArray(update)) {
    throw new FormDeclarationError("attachment presentation update must be an object", { update });
  }
  if (
    update.status !== "pending" &&
    update.status !== "busy" &&
    update.status !== "settling" &&
    update.status !== "ready" &&
    update.status !== "failed" &&
    update.status !== "unavailable"
  ) {
    throw new FormDeclarationError("attachment presentation status is not supported", {
      status: update.status,
    });
  }
  const operation = update.operation ?? "generic";
  if (!ATTACHMENT_OPERATIONS.has(operation)) {
    throw new FormDeclarationError("attachment presentation operation is not supported", {
      operation,
    });
  }
  return Object.freeze({
    status: update.status,
    target: update.target === undefined ? null : String(update.target),
    reason: String(update.reason),
    token: update.token === undefined || update.token === null ? null : String(update.token),
    section: update.section === undefined || update.section === null ? null : String(update.section),
    selectedCount: nullableNonNegativeInteger(update.selectedCount, "selectedCount"),
    stagedCount: nullableNonNegativeInteger(update.stagedCount, "stagedCount"),
    failedCount: nullableNonNegativeInteger(update.failedCount, "failedCount"),
    operation,
  });
}

function attachmentArtifact(artifactId, update, source) {
  const artifact = {
    kind: "attachmentPresentationUpdate",
    artifactId,
    observedAtMs: Date.now(),
    source,
    status: update.status,
    target: update.target,
    reason: update.reason,
    token: update.token,
    section: update.section,
    selectedCount: update.selectedCount,
    stagedCount: update.stagedCount,
    failedCount: update.failedCount,
    operation: update.operation,
  };
  return Object.freeze({
    ...artifact,
    attachmentDigest: stableValueDigest(artifact),
  });
}

function nullableNonNegativeInteger(value, label) {
  if (value === undefined || value === null) {
    return null;
  }
  if (!Number.isInteger(value) || value < 0) {
    throw new FormDeclarationError(`attachment presentation ${label} must be a non-negative integer`, { value });
  }
  return value;
}
