function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function requireFeatureStoreStateHandle(stateHandles, scopeId, key) {
  const handle = stateHandles[key];
  if (handle) {
    return handle;
  }
  throw new TypeError(
    `signals.featureStore(...) action contract referenced unknown state key "${String(key)}" in store "${scopeId}"`,
  );
}

function requireFeatureStoreOptions(options) {
  if (!isPlainObject(options)) {
    throw new TypeError("signals.featureStore(...) requires an options object");
  }
  if (typeof options.id !== "string" || options.id.length === 0) {
    throw new TypeError(
      "signals.featureStore(...) requires a non-empty string id",
    );
  }
  if (!isPlainObject(options.state)) {
    throw new TypeError(
      "signals.featureStore(...) requires a plain object state definition",
    );
  }
  if (typeof options.actions !== "function") {
    throw new TypeError(
      "signals.featureStore(...) requires an actions(...) builder",
    );
  }
  return options;
}

function requireFeatureStoreActions(actions, scopeId) {
  if (!isPlainObject(actions)) {
    throw new TypeError(
      `signals.featureStore(...) actions(...) for "${scopeId}" must return a plain object`,
    );
  }
  for (const [key, value] of Object.entries(actions)) {
    if (typeof value !== "function") {
      throw new TypeError(
        `signals.featureStore(...) action "${key}" in "${scopeId}" must be a function`,
      );
    }
  }
  return actions;
}

function createStateHandles(scope, stateDefinition) {
  return Object.freeze(
    Object.fromEntries(
      Object.entries(stateDefinition).map(([key, initialValue]) => [
        key,
        scope.spec.input(key, initialValue, {
          debugName: `${scope.scopeId}.${key}`,
        }),
      ]),
    ),
  );
}

function readFeatureStoreState(stateHandles) {
  return Object.freeze(
    Object.fromEntries(
      Object.entries(stateHandles).map(([key, handle]) => [key, handle()]),
    ),
  );
}

function createActionContext(scope, stateHandles, snapshot) {
  return Object.freeze({
    scope,
    state: stateHandles,
    set(key, value) {
      return requireFeatureStoreStateHandle(stateHandles, scope.scopeId, key).set(value);
    },
    reset(key) {
      if (key === undefined) {
        return Object.values(stateHandles).map((handle) => handle.reset());
      }
      return requireFeatureStoreStateHandle(stateHandles, scope.scopeId, key).reset();
    },
    read() {
      return snapshot();
    },
  });
}

function freeFeatureStoreState(stateHandles, snapshot) {
  snapshot.free();
  for (const handle of Object.values(stateHandles)) {
    handle.free();
  }
}

function disposeFeatureStoreState(stateHandles, snapshot) {
  snapshot[Symbol.dispose]();
  for (const handle of Object.values(stateHandles)) {
    handle[Symbol.dispose]();
  }
}

export function createFeatureStoreFactory(namespace) {
  return function featureStore(options) {
    const normalized = requireFeatureStoreOptions(options);
    const scope = namespace.scope(normalized.id);
    const state = createStateHandles(scope, normalized.state);
    const snapshot = scope.outputCallback("snapshot", () =>
      readFeatureStoreState(state),
    );
    const actions = Object.freeze(
      requireFeatureStoreActions(
        normalized.actions(createActionContext(scope, state, snapshot)),
        scope.scopeId,
      ),
    );
    return Object.freeze({
      scope,
      scopeId: scope.scopeId,
      state,
      snapshot,
      read() {
        return snapshot();
      },
      actions,
      free() {
        freeFeatureStoreState(state, snapshot);
      },
      [Symbol.dispose]() {
        disposeFeatureStoreState(state, snapshot);
      },
    });
  };
}
