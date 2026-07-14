import { FormDeclarationError } from "../form_errors.js";
import { stableValueDigest } from "../values/value_paths.js";

const MESSAGE_CHANNELS = new Set(["inline", "summary", "banner", "toast"]);
const MESSAGE_AUDIENCES = new Set(["user", "developer", "system"]);
const MESSAGE_OPERATIONS = new Set(["show", "update", "dismiss", "clear"]);

export function createMessagePresentationStore() {
  let nextArtifactId = 1;
  let current = null;
  const history = [];
  return Object.freeze({
    report(update) {
      const normalized = normalizeMessageUpdate(update);
      current = messageArtifact(nextArtifactId++, normalized, "report");
      history.push(current);
      return current;
    },
    clear(reason = null) {
      current = null;
      const artifact = messageArtifact(nextArtifactId++, {
        status: "ready",
        target: null,
        reason: reason ?? "message presentation was cleared",
        token: null,
        scope: "wholeForm",
        channel: "banner",
        audience: "user",
        visibleCount: null,
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

function normalizeMessageUpdate(update) {
  if (!update || typeof update !== "object" || Array.isArray(update)) {
    throw new FormDeclarationError("message presentation update must be an object", { update });
  }
  if (
    update.status !== "pending" &&
    update.status !== "busy" &&
    update.status !== "settling" &&
    update.status !== "ready" &&
    update.status !== "failed" &&
    update.status !== "unavailable"
  ) {
    throw new FormDeclarationError("message presentation status is not supported", {
      status: update.status,
    });
  }
  const channel = update.channel ?? "banner";
  if (!MESSAGE_CHANNELS.has(channel)) {
    throw new FormDeclarationError("message presentation channel is not supported", { channel });
  }
  const audience = update.audience ?? "user";
  if (!MESSAGE_AUDIENCES.has(audience)) {
    throw new FormDeclarationError("message presentation audience is not supported", { audience });
  }
  const scope = update.scope ?? "wholeForm";
  if (
    scope !== "field" &&
    scope !== "section" &&
    scope !== "action" &&
    scope !== "control" &&
    scope !== "wholeForm" &&
    scope !== "step"
  ) {
    throw new FormDeclarationError("message presentation scope is not supported", { scope });
  }
  const operation = update.operation ?? "show";
  if (!MESSAGE_OPERATIONS.has(operation)) {
    throw new FormDeclarationError("message presentation operation is not supported", { operation });
  }
  const target = normalizedMessageTarget(update.target, scope);
  return Object.freeze({
    status: update.status,
    target,
    reason: String(update.reason),
    token: update.token === undefined || update.token === null ? null : String(update.token),
    scope,
    channel,
    audience,
    visibleCount: nullableNonNegativeInteger(update.visibleCount),
    operation,
  });
}

function messageArtifact(artifactId, update, source) {
  const artifact = {
    kind: "messagePresentationUpdate",
    artifactId,
    observedAtMs: Date.now(),
    source,
    status: update.status,
    target: update.target,
    reason: update.reason,
    token: update.token,
    scope: update.scope,
    channel: update.channel,
    audience: update.audience,
    visibleCount: update.visibleCount,
    operation: update.operation,
  };
  return Object.freeze({
    ...artifact,
    messageDigest: stableValueDigest(artifact),
  });
}

function nullableNonNegativeInteger(value) {
  if (value === undefined || value === null) {
    return null;
  }
  if (!Number.isInteger(value) || value < 0) {
    throw new FormDeclarationError("message presentation visibleCount must be a non-negative integer", {
      value,
    });
  }
  return value;
}

function normalizedMessageTarget(value, scope) {
  if (scope === "wholeForm") {
    return value === undefined || value === null ? null : String(value);
  }
  if (typeof value !== "string" || value.length === 0) {
    throw new FormDeclarationError("scoped message presentation requires a non-empty target", {
      scope,
      target: value,
    });
  }
  return value;
}
