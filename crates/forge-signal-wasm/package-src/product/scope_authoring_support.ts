import { PRIVATE_AUTHORING_ID } from "./symbols.js";

const AUTHORING_STATE = new WeakMap();

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function getAuthoringState(rawSignals) {
  let state = AUTHORING_STATE.get(rawSignals);
  if (!state) {
    state = {
      authoredSignalIds: new Set(),
      generatedCounters: new Map(),
    };
    AUTHORING_STATE.set(rawSignals, state);
  }
  return state;
}

function generatedCounterKey(scopeId, family) {
  return `${scopeId ?? "__root__"}:${family}`;
}

export function requireNonEmptyString(value, message) {
  if (typeof value !== "string" || value.length === 0) {
    throw new TypeError(message);
  }
  return value;
}

export function joinScopeId(parentScopeId, localScopeId) {
  return parentScopeId ? `${parentScopeId}.${localScopeId}` : localScopeId;
}

function collisionMessage(family, canonicalId, scopeId, localId) {
  if (!scopeId && canonicalId.includes(".")) {
    const boundary = canonicalId.lastIndexOf(".");
    scopeId = canonicalId.slice(0, boundary);
    localId = canonicalId.slice(boundary + 1);
  }
  if (!scopeId) {
    return `${family} authoring cannot reuse canonical id \`${canonicalId}\` in the same Signals runtime`;
  }
  return `${family} authoring in scope \`${scopeId}\` cannot reuse local id \`${localId}\` because canonical id \`${canonicalId}\` is already owned in this Signals runtime`;
}

export function reserveAuthoringSignalId(
  rawSignals,
  family,
  canonicalId,
  scopeId = null,
  localId = canonicalId,
) {
  requireNonEmptyString(
    canonicalId,
    `${family} authoring requires a non-empty canonical id`,
  );
  const state = getAuthoringState(rawSignals);
  if (state.authoredSignalIds.has(canonicalId)) {
    throw new TypeError(
      collisionMessage(family, canonicalId, scopeId, localId),
    );
  }
  state.authoredSignalIds.add(canonicalId);
  return () => {
    state.authoredSignalIds.delete(canonicalId);
  };
}

export function nextGeneratedAuthoringSignalId(
  rawSignals,
  family,
  scopeId = null,
) {
  const state = getAuthoringState(rawSignals);
  const counterKey = generatedCounterKey(scopeId, family);
  const next = (state.generatedCounters.get(counterKey) ?? 0) + 1;
  state.generatedCounters.set(counterKey, next);
  if (scopeId) {
    return `__forgeSignalScoped.${scopeId}.${family}.${next}`;
  }
  return `__forgeSignal.${family}.${next}`;
}

export function withPrivateAuthoringId(options, authoringId) {
  if (options === undefined) {
    return {
      [PRIVATE_AUTHORING_ID]: authoringId,
    };
  }
  if (!isPlainObject(options)) {
    throw new TypeError(
      "scoped authoring options must be an object when provided",
    );
  }
  return {
    ...options,
    [PRIVATE_AUTHORING_ID]: authoringId,
  };
}

export function hasExplicitAuthoringIdOption(options) {
  return (
    isPlainObject(options)
    && typeof options.id === "string"
    && options.id.length > 0
  );
}

export function stripExplicitAuthoringIdOption(options) {
  if (!hasExplicitAuthoringIdOption(options)) {
    return options;
  }
  const { id: _id, ...rest } = options;
  return Object.keys(rest).length === 0 ? undefined : rest;
}

function descriptorForScope(scopeId, localScopeId, parentScopeId) {
  const segments = scopeId.split(".");
  const path = Object.freeze(
    segments.map((segment, index) =>
      Object.freeze({
        id: segments.slice(0, index + 1).join("."),
        localScopeId: segment,
        depth: index + 1,
      }),
    ),
  );
  return Object.freeze({
    id: scopeId,
    localScopeId,
    parentScopeId,
    depth: path.length,
    path,
    identity: Object.freeze({
      scopeId,
      parentScopeId,
      path,
      depth: path.length,
    }),
  });
}

export function canonicalId(scopeId, localId) {
  requireNonEmptyString(
    localId,
    "scoped authoring requires a non-empty local id",
  );
  return `${scopeId}.${localId}`;
}

export function createScopeDescriptor(scopeId, localScopeId, parentScopeId, graphOwnerId) {
  return Object.freeze({
    ...descriptorForScope(scopeId, localScopeId, parentScopeId),
    graphOwnerId,
  });
}

export function signalIdentityForScope(descriptor, graphOwnerId, localId) {
  const canonicalSignalId = canonicalId(descriptor.id, localId);
  const rootScopeId = descriptor.path[0]?.localScopeId ?? null;
  const graphId = graphOwnerId ?? null;
  return Object.freeze({
    localId,
    canonicalId: canonicalSignalId,
    scopeId: descriptor.id,
    graphOwnerId,
    graphId,
    rootScopeId,
    scopePath: descriptor.path,
  });
}
