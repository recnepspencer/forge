import { mergeApiDeclaration, normalizeApiLayer } from "./api_request_defaults.js";
import { createApiRouteBuilder } from "./route/api_route_builder.js";

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function plainObjectEntriesMatch(left, right) {
  const leftKeys = Object.keys(left);
  const rightKeys = Object.keys(right);
  if (leftKeys.length !== rightKeys.length) {
    return false;
  }
  for (const key of leftKeys) {
    if (!Object.prototype.hasOwnProperty.call(right, key)) {
      return false;
    }
    if (!fieldValuesMatch(left[key], right[key])) {
      return false;
    }
  }
  return true;
}

function fieldValuesMatch(left, right) {
  if (Object.is(left, right)) {
    return true;
  }
  if (isPlainObject(left) && isPlainObject(right)) {
    return plainObjectEntriesMatch(left, right);
  }
  return false;
}

function apiLayerConfigsMatch(left, right) {
  return (
    fieldValuesMatch(left.baseUrl, right.baseUrl)
    && fieldValuesMatch(left.auth, right.auth)
    && fieldValuesMatch(left.headers, right.headers)
    && fieldValuesMatch(left.requestContext, right.requestContext)
    && fieldValuesMatch(left.continuation, right.continuation)
    && fieldValuesMatch(left.processingJob, right.processingJob)
    && fieldValuesMatch(left.uploadTransport, right.uploadTransport)
    && fieldValuesMatch(left.effects, right.effects)
  );
}

function hasDynamicScopedDefaults(options) {
  return (
    typeof options.baseUrl === "function"
    || typeof options.auth === "function"
    || typeof options.headers === "function"
    || typeof options.requestContext === "function"
    || typeof options.continuation === "function"
    || typeof options.processingJob === "function"
    || typeof options.uploadTransport === "function"
    || typeof options.effects === "function"
  );
}

function requireStableScopedDefaults(operation, scopeId, options) {
  if (hasDynamicScopedDefaults(options)) {
    throw new TypeError(
      `${operation}("${scopeId}") only admits static scoped defaults; use signals.api(...).scope({...}) when defaults depend on params`,
    );
  }
}

function createApiFactory(signalNamespace) {
  const state = {
    nextScopeId: 1,
    scopedNamespaces: new Map(),
  };
  const api = function api(options = {}) {
    const rootLayer = normalizeApiLayer("apiRoot", options);
    return createApiNamespace(signalNamespace, [rootLayer], state, null);
  };
  Object.defineProperty(api, "state", {
    enumerable: false,
    value: state,
  });
  return api;
}

function createApiScopeFactory(signalNamespace) {
  const createApi = createApiFactory(signalNamespace);
  const state = createApi.state;
  return function apiScope(scopeId, options = {}) {
    if (typeof scopeId !== "string" || scopeId.length === 0) {
      throw new TypeError("signals.apiScope(...) requires a non-empty string scope id");
    }
    requireStableScopedDefaults("signals.apiScope", scopeId, options);
    const rootLayer = normalizeApiLayer(`apiScopeRoot[${scopeId}]`, options);
    return createScopedApiNamespace(
      signalNamespace,
      [rootLayer],
      state,
      scopeId,
      rootLayer,
      null,
    );
  };
}

function readScopeCacheKey(parentScopeId, scopeId) {
  return parentScopeId === null ? scopeId : `${parentScopeId}/${scopeId}`;
}

function createScopedApiNamespace(signalNamespace, layers, state, scopeId, layer, parentScopeId) {
  const cacheKey = readScopeCacheKey(parentScopeId, scopeId);
  const cached = state.scopedNamespaces.get(cacheKey);
  if (cached) {
    if (!apiLayerConfigsMatch(cached.layer, layer)) {
      throw new TypeError(
        `signals.apiScope("${cacheKey}") was requested with conflicting scoped defaults for the same runtime`,
      );
    }
    return cached.namespace;
  }
  const namespace = createApiNamespace(signalNamespace, layers, state, scopeId);
  state.scopedNamespaces.set(cacheKey, {
    layer,
    namespace,
  });
  return namespace;
}

function createApiNamespace(signalNamespace, layers, state, scopeId) {
  return Object.freeze({
    ...(scopeId === null ? {} : { scopeId }),
    scope(scopeIdOrOptions = {}, maybeOptions) {
      if (typeof scopeIdOrOptions === "string") {
        requireStableScopedDefaults("api.scope", scopeIdOrOptions, maybeOptions ?? {});
        const scopedLayer = normalizeApiLayer(
          `apiScope[${scopeIdOrOptions}]`,
          maybeOptions ?? {},
        );
        return createScopedApiNamespace(
          signalNamespace,
          [...layers, scopedLayer],
          state,
          scopeIdOrOptions,
          scopedLayer,
          scopeId,
        );
      }
      const layer = normalizeApiLayer(`apiScope[${state.nextScopeId}]`, scopeIdOrOptions);
      state.nextScopeId += 1;
      return createApiNamespace(signalNamespace, [...layers, layer], state, null);
    },
    url(route) {
      return createApiRouteBuilder(signalNamespace, layers, route);
    },
    detail(declaration) {
      return signalNamespace.resource.detail(
        mergeApiDeclaration(layers, declaration),
      );
    },
    collection(declaration) {
      return signalNamespace.resource.collection(
        mergeApiDeclaration(layers, declaration),
      );
    },
    paged(declaration) {
      return signalNamespace.resource.paged(
        mergeApiDeclaration(layers, declaration),
      );
    },
  });
}

export { createApiFactory, createApiScopeFactory };
