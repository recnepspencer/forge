import { FormDeclarationError } from "../form_errors.js";
import { stableValueDigest } from "../values/value_paths.js";

const MEDIA_MODES = new Set(["preview", "capture", "crop", "annotate"]);
const MEDIA_OPERATIONS = new Set(["generic", "open", "replace", "annotate", "close"]);

export function createMediaPresentationStore() {
  let nextArtifactId = 1;
  let current = null;
  const history = [];
  return Object.freeze({
    report(update) {
      const normalized = normalizeMediaUpdate(update);
      current = mediaArtifact(nextArtifactId++, normalized, "report");
      history.push(current);
      return current;
    },
    clear(reason = null) {
      current = null;
      const artifact = mediaArtifact(nextArtifactId++, {
        status: "ready",
        target: null,
        reason: reason ?? "media presentation was cleared",
        token: null,
        mode: null,
        surfaceId: null,
        operation: "close",
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

function normalizeMediaUpdate(update) {
  if (!update || typeof update !== "object" || Array.isArray(update)) {
    throw new FormDeclarationError("media presentation update must be an object", { update });
  }
  if (
    update.status !== "pending" &&
    update.status !== "busy" &&
    update.status !== "settling" &&
    update.status !== "ready" &&
    update.status !== "failed" &&
    update.status !== "unavailable"
  ) {
    throw new FormDeclarationError("media presentation status is not supported", {
      status: update.status,
    });
  }
  const mode = update.mode ?? null;
  if (mode !== null && !MEDIA_MODES.has(mode)) {
    throw new FormDeclarationError("media presentation mode is not supported", { mode });
  }
  const operation = update.operation ?? "generic";
  if (!MEDIA_OPERATIONS.has(operation)) {
    throw new FormDeclarationError("media presentation operation is not supported", {
      operation,
    });
  }
  return Object.freeze({
    status: update.status,
    target: update.target === undefined ? null : String(update.target),
    reason: String(update.reason),
    token: update.token === undefined || update.token === null ? null : String(update.token),
    scopeKind: update.scopeKind ?? null,
    mode,
    surfaceId: update.surfaceId === undefined || update.surfaceId === null ? null : String(update.surfaceId),
    operation,
  });
}

function mediaArtifact(artifactId, update, source) {
  const artifact = {
    kind: "mediaPresentationUpdate",
    artifactId,
    observedAtMs: Date.now(),
    source,
    status: update.status,
    target: update.target,
    reason: update.reason,
    token: update.token,
    scopeKind: update.scopeKind,
    mode: update.mode,
    surfaceId: update.surfaceId,
    operation: update.operation,
  };
  return Object.freeze({
    ...artifact,
    mediaDigest: stableValueDigest(artifact),
  });
}
