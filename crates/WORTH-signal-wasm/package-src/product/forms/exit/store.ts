import { FormDeclarationError } from "../form_errors.js";
import { stableValueDigest } from "../values/value_paths.js";

const EXIT_SCOPE_KINDS = new Set(["route", "modal", "external"]);
const EXIT_OPERATIONS = new Set(["generic", "block", "confirm", "dismiss", "leave", "stay", "close"]);

export function createExitPresentationStore() {
  let nextArtifactId = 1;
  let current = null;
  const history = [];
  return Object.freeze({
    report(update) {
      const normalized = normalizeExitUpdate(update);
      current = exitArtifact(nextArtifactId++, normalized, "report");
      history.push(current);
      return current;
    },
    clear(reason = null) {
      current = null;
      const artifact = exitArtifact(nextArtifactId++, {
        status: "ready",
        target: null,
        reason: reason ?? "exit presentation was cleared",
        token: null,
        scopeKind: null,
        surfaceId: null,
        operation: "close",
        unavailableReason: null,
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

function normalizeExitUpdate(update) {
  if (!update || typeof update !== "object" || Array.isArray(update)) {
    throw new FormDeclarationError("exit presentation update must be an object", { update });
  }
  if (
    update.status !== "pending" &&
    update.status !== "busy" &&
    update.status !== "settling" &&
    update.status !== "ready" &&
    update.status !== "failed" &&
    update.status !== "unavailable"
  ) {
    throw new FormDeclarationError("exit presentation status is not supported", {
      status: update.status,
    });
  }
  const scopeKind = update.scopeKind ?? null;
  if (scopeKind !== null && !EXIT_SCOPE_KINDS.has(scopeKind)) {
    throw new FormDeclarationError("exit presentation scope kind is not supported", { scopeKind });
  }
  const operation = update.operation ?? "generic";
  if (!EXIT_OPERATIONS.has(operation)) {
    throw new FormDeclarationError("exit presentation operation is not supported", { operation });
  }
  return Object.freeze({
    status: update.status,
    target: update.target === undefined ? null : String(update.target),
    reason: String(update.reason),
    token: update.token === undefined || update.token === null ? null : String(update.token),
    scopeKind,
    surfaceId: update.surfaceId === undefined || update.surfaceId === null ? null : String(update.surfaceId),
    operation,
    unavailableReason: update.unavailableReason === undefined || update.unavailableReason === null
      ? null
      : String(update.unavailableReason),
  });
}

function exitArtifact(artifactId, update, source) {
  const artifact = {
    kind: "exitPresentationUpdate",
    artifactId,
    observedAtMs: Date.now(),
    source,
    status: update.status,
    target: update.target,
    reason: update.reason,
    token: update.token,
    scopeKind: update.scopeKind,
    surfaceId: update.surfaceId,
    operation: update.operation,
    unavailableReason: update.unavailableReason,
  };
  return Object.freeze({
    ...artifact,
    exitDigest: stableValueDigest(artifact),
  });
}
