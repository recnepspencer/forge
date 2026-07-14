import { FormDeclarationError } from "../form_errors.js";
import { requireDeclaredPresentationScopeTarget } from "./scope_registry.js";

export function normalizeScopedPresentationUpdate(update, policy, laneId, scopeRegistry = null) {
  if (!update || typeof update !== "object" || Array.isArray(update)) {
    throw new FormDeclarationError("presentation lane update must be an object", { update });
  }
  const normalized = {
    section: null,
    scopeKind: null,
    surfaceId: null,
  };
  if (laneId === "handoff" || laneId === "exit") {
    normalized.scopeKind = requireDeclaredScopeKind(update.scopeKind, laneId);
    normalized.surfaceId = requireNonEmptyString(update.surfaceId, `${laneId} presentation surfaceId`);
    return Object.freeze(normalized);
  }
  if (laneId === "media") {
    normalized.scopeKind = requireMatchingScopeKind(update.scopeKind, "modal", laneId);
    normalized.surfaceId = requireNonEmptyString(update.surfaceId, `${laneId} presentation surfaceId`);
    return Object.freeze(normalized);
  }
  if (policy.scope === "section") {
    normalized.section = requireNonEmptyString(update.section, `${laneId} presentation section`);
    if (scopeRegistry !== null) {
      requireDeclaredPresentationScopeTarget(scopeRegistry, "section", normalized.section, `${laneId} presentation section`);
    }
    return Object.freeze(normalized);
  }
  return Object.freeze(normalized);
}

function requireDeclaredScopeKind(value, laneId) {
  if (value !== "route" && value !== "modal" && value !== "external") {
    throw new FormDeclarationError(`${laneId} presentation scope kind is not supported`, {
      scopeKind: value,
    });
  }
  return value;
}

function requireMatchingScopeKind(value, expected, laneId) {
  if (value === undefined || value === null) {
    return expected;
  }
  if (value !== expected) {
    throw new FormDeclarationError(`${laneId} presentation scope kind must match declared scope`, {
      expected,
      scopeKind: value,
    });
  }
  return value;
}

function requireNonEmptyString(value, label) {
  if (typeof value !== "string" || value.length === 0) {
    throw new FormDeclarationError(`${label} must be a non-empty string`, { value });
  }
  return value;
}
