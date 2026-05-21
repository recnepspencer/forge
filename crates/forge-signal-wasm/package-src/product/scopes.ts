import { buildControllerContract } from "./controllers.js";
import { createApiFactory } from "./api/api_namespace.js";
import { createLinkedSignal } from "./linked.js";
import { createResourceNamespace } from "./resource/facade.js";
import {
  DEBUG_NAME,
  GRAPH_LOCAL_ID,
  GRAPH_OWNER_ID,
  GRAPH_SCOPE_DESCRIPTOR,
  GRAPH_SCOPE_ID,
  PRIVATE_AUTHORING_ID,
  RAW_SIGNALS,
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

function generatedCounterKey(scopeId, family) {
  return `${scopeId ?? "__root__"}:${family}`;
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

function nextGeneratedScopedId(rawSignals, scopeId, family) {
  return nextGeneratedAuthoringSignalId(rawSignals, family, scopeId);
}

function withPrivateAuthoringId(options, authoringId) {
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

function hasExplicitAuthoringIdOption(options) {
  return (
    isPlainObject(options) &&
    typeof options.id === "string" &&
    options.id.length > 0
  );
}

function stripExplicitAuthoringIdOption(options) {
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

function canonicalId(scopeId, localId) {
  requireNonEmptyString(
    localId,
    "scoped authoring requires a non-empty local id",
  );
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

export function createScopedSignalNamespace(
  callableSignals,
  rawSignals,
  localScopeId,
  parentScope = null,
  explicitGraphOwnerId = undefined,
) {
  requireNonEmptyString(
    localScopeId,
    "signals.scope requires a non-empty string scope id",
  );
  const parentScopeId = parentScope?.scopeId ?? null;
  const scopeId = joinScopeId(parentScopeId, localScopeId);
  const graphOwnerId =
    explicitGraphOwnerId ?? parentScope?.graphOwnerId ?? null;
  const descriptor = Object.freeze({
    ...descriptorForScope(scopeId, localScopeId, parentScopeId),
    graphOwnerId,
  });

  function tagScopedHandle(handle, localId = null) {
    const signalIdentity =
      typeof localId === "string"
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
    const inheritedDebugName = handle[DEBUG_NAME] ?? null;
    if (inheritedDebugName !== null) {
      Object.defineProperty(handle, DEBUG_NAME, {
        enumerable: false,
        value: inheritedDebugName,
      });
    }
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
    resource: null,
    api: null,
    spec: Object.freeze({
      input(id, initial, options) {
        return tagScopedHandle(
          callableSignals.spec.input(
            canonicalId(scopeId, id),
            initial,
            options,
          ),
          id,
        );
      },
      computed(id, spec) {
        return tagScopedHandle(
          callableSignals.spec.computed(canonicalId(scopeId, id), spec),
          id,
        );
      },
      computedCallback(id, callback, options) {
        return tagScopedHandle(
          callableSignals.spec.computedCallback(
            canonicalId(scopeId, id),
            callback,
            options,
          ),
          id,
        );
      },
      output(id, spec) {
        return tagScopedHandle(
          callableSignals.spec.output(canonicalId(scopeId, id), spec),
          id,
        );
      },
      outputCallback(id, callback, options) {
        return tagScopedHandle(
          callableSignals.spec.outputCallback(
            canonicalId(scopeId, id),
            callback,
            options,
          ),
          id,
        );
      },
    }),
    scope(childLocalScopeId) {
      return createScopedSignalNamespace(
        callableSignals,
        rawSignals,
        childLocalScopeId,
        scopedNamespace,
      );
    },
    controller(definitionOrBuilder) {
      return buildControllerContract(scopedNamespace, definitionOrBuilder);
    },
    publicInput(handle, options) {
      return callableSignals.publicInput(handle, options);
    },
    input(firstArg, secondArg, thirdArg) {
      if (hasExplicitAuthoringIdOption(secondArg)) {
        return tagScopedHandle(
          callableSignals.spec.input(
            canonicalId(scopeId, secondArg.id),
            firstArg,
            stripExplicitAuthoringIdOption(secondArg),
          ),
          secondArg.id,
        );
      }
      const authoringId = nextGeneratedScopedId(rawSignals, scopeId, "input");
      return tagScopedHandle(
        callableSignals.input(
          firstArg,
          withPrivateAuthoringId(secondArg, authoringId),
        ),
      );
    },
    inputAsync(firstArg, secondArg, thirdArg) {
      if (hasExplicitAuthoringIdOption(secondArg)) {
        return Promise.resolve(
          tagScopedHandle(
            callableSignals.spec.input(
              canonicalId(scopeId, secondArg.id),
              firstArg,
              stripExplicitAuthoringIdOption(secondArg),
            ),
            secondArg.id,
          ),
        );
      }
      const authoringId = nextGeneratedScopedId(rawSignals, scopeId, "input");
      return Promise.resolve(
        tagScopedHandle(
          callableSignals.input(
            firstArg,
            withPrivateAuthoringId(secondArg, authoringId),
          ),
        ),
      );
    },
    computedSpec(id, spec, options) {
      return tagScopedHandle(
        callableSignals.spec.computed(
          canonicalId(scopeId, id),
          spec,
          options,
        ),
        id,
      );
    },
    linked(sourceOrDefinition, options) {
      return tagScopedHandle(
        createLinkedSignal(
          scopedNamespace,
          rawSignals,
          sourceOrDefinition,
          options,
        ),
      );
    },
    linkedAsync(sourceOrDefinition, options) {
      return Promise.resolve(this.linked(sourceOrDefinition, options));
    },
    computed(firstArg, secondArg, thirdArg) {
      if (typeof firstArg === "string") {
        if (typeof secondArg === "function") {
          if (thirdArg !== undefined) {
            throw new TypeError(
              "scoped computed callback form does not accept options after an explicit id",
            );
          }
          return tagScopedHandle(
            callableSignals.spec.computedCallback(
              canonicalId(scopeId, firstArg),
              secondArg,
            ),
            firstArg,
          );
        }
        if (thirdArg !== undefined) {
          throw new TypeError(
            "scoped computed spec form does not accept a third argument after an explicit id",
          );
        }
        return tagScopedHandle(
          callableSignals.spec.computed(
            canonicalId(scopeId, firstArg),
            secondArg,
          ),
          firstArg,
        );
      }
      if (hasExplicitAuthoringIdOption(secondArg)) {
        const localId = secondArg.id;
        if (typeof firstArg === "function") {
          return tagScopedHandle(
            callableSignals.spec.computedCallback(
              canonicalId(scopeId, localId),
              firstArg,
              stripExplicitAuthoringIdOption(secondArg),
            ),
            localId,
          );
        }
        return tagScopedHandle(
          callableSignals.spec.computed(
            canonicalId(scopeId, localId),
            firstArg,
            stripExplicitAuthoringIdOption(secondArg),
          ),
          localId,
        );
      }
      const authoringId = nextGeneratedScopedId(
        rawSignals,
        scopeId,
        "computed",
      );
      return tagScopedHandle(
        callableSignals.computed(
          firstArg,
          withPrivateAuthoringId(secondArg, authoringId),
        ),
      );
    },
    computedAsync(firstArg, secondArg, thirdArg) {
      return Promise.resolve(
        scopedNamespace.computed(firstArg, secondArg, thirdArg),
      );
    },
    outputSpec(id, spec, options) {
      return tagScopedHandle(
        callableSignals.spec.output(
          canonicalId(scopeId, id),
          spec,
          options,
        ),
        id,
      );
    },
    output(firstArg, secondArg, thirdArg) {
      if (typeof firstArg === "string") {
        if (typeof secondArg === "function") {
          if (thirdArg !== undefined) {
            throw new TypeError(
              "scoped output callback form does not accept options after an explicit id",
            );
          }
          return tagScopedHandle(
            callableSignals.spec.outputCallback(
              canonicalId(scopeId, firstArg),
              secondArg,
            ),
            firstArg,
          );
        }
        if (thirdArg !== undefined) {
          throw new TypeError(
            "scoped output spec form does not accept a third argument after an explicit id",
          );
        }
        return tagScopedHandle(
          callableSignals.spec.output(
            canonicalId(scopeId, firstArg),
            secondArg,
          ),
          firstArg,
        );
      }
      if (hasExplicitAuthoringIdOption(secondArg)) {
        const localId = secondArg.id;
        if (typeof firstArg === "function") {
          return tagScopedHandle(
            callableSignals.spec.outputCallback(
              canonicalId(scopeId, localId),
              firstArg,
              stripExplicitAuthoringIdOption(secondArg),
            ),
            localId,
          );
        }
        return tagScopedHandle(
          callableSignals.spec.output(
            canonicalId(scopeId, localId),
            firstArg,
            stripExplicitAuthoringIdOption(secondArg),
          ),
          localId,
        );
      }
      const authoringId = nextGeneratedScopedId(rawSignals, scopeId, "output");
      return tagScopedHandle(
        callableSignals.output(
          firstArg,
          withPrivateAuthoringId(secondArg, authoringId),
        ),
      );
    },
    outputAsync(firstArg, secondArg, thirdArg) {
      return Promise.resolve(
        scopedNamespace.output(firstArg, secondArg, thirdArg),
      );
    },
    outputCallback(id, callback, options) {
      return tagScopedHandle(
        callableSignals.spec.outputCallback(
          canonicalId(scopeId, id),
          callback,
          options,
        ),
        id,
      );
    },
    graph: callableSignals.graph.bind(callableSignals),
    history: callableSignals.history.bind(callableSignals),
    canonicalId(localId) {
      return canonicalId(scopeId, localId);
    },
    signalIdentity(localId) {
      requireNonEmptyString(
        localId,
        "scoped authoring requires a non-empty local id",
      );
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
    [RAW_SIGNALS]: rawSignals,
  };

  scopedNamespace.resource = createResourceNamespace(
    scopedNamespace,
    rawSignals,
  );
  scopedNamespace.api = createApiFactory(scopedNamespace);

  return Object.freeze(scopedNamespace);
}
