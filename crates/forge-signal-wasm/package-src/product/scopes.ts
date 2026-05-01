import {
  GRAPH_LOCAL_ID,
  GRAPH_OWNER_ID,
  GRAPH_SCOPE_DESCRIPTOR,
  GRAPH_SCOPE_ID,
} from "./symbols.js";

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

function requireNonEmptyString(value, message) {
  if (typeof value !== "string" || value.length === 0) {
    throw new TypeError(message);
  }
  return value;
}

function joinScopeId(parentScopeId, localScopeId) {
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

export function reserveAuthoringSignalId(rawSignals, family, canonicalId, scopeId = null, localId = canonicalId) {
  requireNonEmptyString(canonicalId, `${family} authoring requires a non-empty canonical id`);
  const state = getAuthoringState(rawSignals);
  if (state.authoredSignalIds.has(canonicalId)) {
    throw new TypeError(collisionMessage(family, canonicalId, scopeId, localId));
  }
  state.authoredSignalIds.add(canonicalId);
  return () => {
    state.authoredSignalIds.delete(canonicalId);
  };
}

function nextGeneratedScopedId(rawSignals, scopeId, family) {
  const state = getAuthoringState(rawSignals);
  const counterKey = `${scopeId}:${family}`;
  const next = (state.generatedCounters.get(counterKey) ?? 0) + 1;
  state.generatedCounters.set(counterKey, next);
  return `__forgeSignalScoped.${family}.${next}`;
}

function descriptorForScope(scopeId, localScopeId, parentScopeId) {
  const segments = scopeId.split(".");
  const path = Object.freeze(segments.map((segment, index) => Object.freeze({
    id: segments.slice(0, index + 1).join("."),
    localScopeId: segment,
    depth: index + 1,
  })));
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

function canonicalId(scopeId, localId) {
  requireNonEmptyString(localId, "scoped authoring requires a non-empty local id");
  return `${scopeId}.${localId}`;
}

function signalIdentityForScope(descriptor, graphOwnerId, localId) {
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

function looksLikeInputMetadataOptions(value) {
  if (!isPlainObject(value) || typeof value.id !== "string" || value.id.length === 0) {
    return false;
  }
  return Object.keys(value).every((key) => key === "id" || key === "producesAspects");
}

function scopedInputArgs(scopeId, idOrInitial, initialOrOptions, maybeOptions) {
  if (typeof idOrInitial === "string" && !looksLikeInputMetadataOptions(initialOrOptions)) {
    return [canonicalId(scopeId, idOrInitial), initialOrOptions, maybeOptions];
  }
  if (!isPlainObject(initialOrOptions)) {
    return [idOrInitial, initialOrOptions, maybeOptions];
  }
  return [
    idOrInitial,
    {
      ...initialOrOptions,
      id: canonicalId(scopeId, initialOrOptions.id),
    },
  ];
}

function scopedSpecArgs(scopeId, firstArg, secondArg, thirdArg) {
  if (typeof firstArg === "string") {
    return [canonicalId(scopeId, firstArg), secondArg, thirdArg];
  }
  if (!isPlainObject(secondArg)) {
    return [firstArg, secondArg, thirdArg];
  }
  return [
    firstArg,
    {
      ...secondArg,
      id: canonicalId(scopeId, secondArg.id),
    },
  ];
}

function scopedCallbackArgs(rawSignals, scopeId, family, firstArg, secondArg, thirdArg) {
  if (typeof firstArg === "function") {
    if (secondArg === undefined) {
      return [firstArg, { id: canonicalId(scopeId, nextGeneratedScopedId(rawSignals, scopeId, family)) }];
    }
    if (isPlainObject(secondArg)) {
      return [firstArg, {
        ...secondArg,
        id: secondArg.id
          ? canonicalId(scopeId, secondArg.id)
          : canonicalId(scopeId, nextGeneratedScopedId(rawSignals, scopeId, family)),
      }];
    }
    return [firstArg, secondArg, thirdArg];
  }

  if (typeof firstArg === "string" && typeof secondArg === "function") {
    return [canonicalId(scopeId, firstArg), secondArg, thirdArg];
  }

  return null;
}

export function createScopedSignalNamespace(
  callableSignals,
  rawSignals,
  localScopeId,
  parentScope = null,
  explicitGraphOwnerId = undefined,
) {
  requireNonEmptyString(localScopeId, "signals.scope requires a non-empty string scope id");
  const parentScopeId = parentScope?.scopeId ?? null;
  const scopeId = joinScopeId(parentScopeId, localScopeId);
  const graphOwnerId = explicitGraphOwnerId ?? parentScope?.graphOwnerId ?? null;
  const descriptor = Object.freeze({
    ...descriptorForScope(scopeId, localScopeId, parentScopeId),
    graphOwnerId,
  });

  function tagScopedHandle(handle, localId = null) {
    const signalIdentity = typeof localId === "string"
      ? signalIdentityForScope(descriptor, graphOwnerId, localId)
      : null;
    Object.defineProperties(handle, {
      [GRAPH_SCOPE_ID]: {
        enumerable: false,
        value: scopeId,
      },
      [GRAPH_OWNER_ID]: {
        enumerable: false,
        value: graphOwnerId,
      },
      [GRAPH_SCOPE_DESCRIPTOR]: {
        enumerable: false,
        value: descriptor,
      },
      [GRAPH_LOCAL_ID]: {
        enumerable: false,
        value: localId,
      },
    });
    if (signalIdentity) {
      Object.defineProperty(handle, "signalIdentity", {
        enumerable: false,
        value: () => signalIdentity,
      });
    }
    return handle;
  }

  const scopedNamespace = {
    host: callableSignals.host,
    scope(childLocalScopeId) {
      return createScopedSignalNamespace(
        callableSignals,
        rawSignals,
        childLocalScopeId,
        scopedNamespace,
      );
    },
    controller(definition) {
      return callableSignals.controller(definition);
    },
    publicInput(handle, options) {
      return callableSignals.publicInput(handle, options);
    },
    input(idOrInitial, initialOrOptions, maybeOptions) {
      const localId = typeof idOrInitial === "string" && !looksLikeInputMetadataOptions(initialOrOptions)
        ? idOrInitial
        : initialOrOptions?.id;
      return tagScopedHandle(callableSignals.input(...scopedInputArgs(
        scopeId,
        idOrInitial,
        initialOrOptions,
        maybeOptions,
      )), localId ?? null);
    },
    computedSpec(id, spec) {
      return tagScopedHandle(callableSignals.computedSpec(canonicalId(scopeId, id), spec), id);
    },
    computed(idOrSpecOrCompute, specOrComputeOrOptions, maybeOptions) {
      const callbackArgs = scopedCallbackArgs(
        rawSignals,
        scopeId,
        "computed",
        idOrSpecOrCompute,
        specOrComputeOrOptions,
        maybeOptions,
      );
      if (callbackArgs) {
        const localId = typeof idOrSpecOrCompute === "string"
          ? idOrSpecOrCompute
          : specOrComputeOrOptions?.id ?? null;
        return tagScopedHandle(callableSignals.computed(...callbackArgs), localId);
      }
      return tagScopedHandle(callableSignals.computed(...scopedSpecArgs(
        scopeId,
        idOrSpecOrCompute,
        specOrComputeOrOptions,
        maybeOptions,
      )), typeof idOrSpecOrCompute === "string" ? idOrSpecOrCompute : specOrComputeOrOptions?.id ?? null);
    },
    outputSpec(id, spec) {
      return tagScopedHandle(callableSignals.outputSpec(canonicalId(scopeId, id), spec), id);
    },
    output(idOrSpecOrCompute, specOrComputeOrOptions, maybeOptions) {
      const callbackArgs = scopedCallbackArgs(
        rawSignals,
        scopeId,
        "output",
        idOrSpecOrCompute,
        specOrComputeOrOptions,
        maybeOptions,
      );
      if (callbackArgs) {
        const localId = typeof idOrSpecOrCompute === "string"
          ? idOrSpecOrCompute
          : specOrComputeOrOptions?.id ?? null;
        return tagScopedHandle(callableSignals.output(...callbackArgs), localId);
      }
      return tagScopedHandle(callableSignals.output(...scopedSpecArgs(
        scopeId,
        idOrSpecOrCompute,
        specOrComputeOrOptions,
        maybeOptions,
      )), typeof idOrSpecOrCompute === "string" ? idOrSpecOrCompute : specOrComputeOrOptions?.id ?? null);
    },
    outputCallback(id, compute) {
      return tagScopedHandle(callableSignals.outputCallback(canonicalId(scopeId, id), compute), id);
    },
    graph: callableSignals.graph.bind(callableSignals),
    canonicalId(localId) {
      return canonicalId(scopeId, localId);
    },
    signalIdentity(localId) {
      requireNonEmptyString(localId, "scoped authoring requires a non-empty local id");
      return signalIdentityForScope(descriptor, graphOwnerId, localId);
    },
    descriptor() {
      return descriptor;
    },
    get scopeId() {
      return descriptor.id;
    },
    get localScopeId() {
      return descriptor.localScopeId;
    },
    get parentScopeId() {
      return descriptor.parentScopeId;
    },
    get graphOwnerId() {
      return descriptor.graphOwnerId;
    },
  };

  return Object.freeze(scopedNamespace);
}
