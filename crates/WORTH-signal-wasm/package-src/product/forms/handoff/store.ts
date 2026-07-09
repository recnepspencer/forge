import { FormDeclarationError } from "../form_errors.js";
import { stableValueDigest } from "../values/value_paths.js";

const HANDOFF_SCOPE_KINDS = new Set(["route", "modal", "external"]);
const HANDOFF_OPERATIONS = new Set(["generic", "open", "handoff", "dismiss", "return", "close"]);

export function createHandoffStore() {
  let nextArtifactId = 1;
  let current = null;
  const history = [];
  return Object.freeze({
    report(update) {
      const normalized = normalizeHandoffUpdate(update);
      if (
        current &&
        current.token !== null &&
        normalized.token !== null &&
        current.token !== normalized.token
      ) {
        history.push(handoffArtifact(nextArtifactId++, {
          status: current.status,
          target: current.target,
          reason: `handoff presentation handed off to ${normalized.token}`,
          token: current.token,
          scopeKind: current.scopeKind,
          surfaceId: current.surfaceId,
          operation: current.operation,
          unavailableReason: current.unavailableReason,
          supersededByToken: normalized.token,
        }, "handoff"));
      }
      current = handoffArtifact(nextArtifactId++, normalized, "report");
      history.push(current);
      return current;
    },
    clear(reason = null) {
      current = null;
      const artifact = handoffArtifact(nextArtifactId++, {
        status: "ready",
        target: null,
        reason: reason ?? "handoff presentation was cleared",
        token: null,
        scopeKind: null,
        surfaceId: null,
        operation: "close",
        unavailableReason: null,
        supersededByToken: null,
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

function normalizeHandoffUpdate(update) {
  if (!update || typeof update !== "object" || Array.isArray(update)) {
    throw new FormDeclarationError("handoff presentation update must be an object", { update });
  }
  if (
    update.status !== "pending" &&
    update.status !== "busy" &&
    update.status !== "settling" &&
    update.status !== "ready" &&
    update.status !== "failed" &&
    update.status !== "unavailable"
  ) {
    throw new FormDeclarationError("handoff presentation status is not supported", {
      status: update.status,
    });
  }
  const scopeKind = update.scopeKind ?? null;
  if (scopeKind !== null && !HANDOFF_SCOPE_KINDS.has(scopeKind)) {
    throw new FormDeclarationError("handoff presentation scope kind is not supported", {
      scopeKind,
    });
  }
  const operation = update.operation ?? "generic";
  if (!HANDOFF_OPERATIONS.has(operation)) {
    throw new FormDeclarationError("handoff presentation operation is not supported", {
      operation,
    });
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
    supersededByToken: update.supersededByToken === undefined || update.supersededByToken === null
      ? null
      : String(update.supersededByToken),
  });
}

function handoffArtifact(artifactId, update, source) {
  const artifact = {
    kind: "handoffPresentationUpdate",
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
    supersededByToken: update.supersededByToken,
  };
  return Object.freeze({
    ...artifact,
    handoffDigest: stableValueDigest(artifact),
  });
}
