import { createApiFactory, createApiScopeFactory } from "../api/api_namespace.js";
import {
  requireAuthoringOptions,
} from "../authoring_option_validation.js";
import { freezeObject } from "../graph_support.js";
import { CONTROLLER_CONTRACT, PRIVATE_AUTHORING_ID } from "../symbols.js";
import { createWorkerFirstAsyncInputHandle } from "./worker_first_async_input.js";
import { createWorkerFirstAsyncLinkedHandle } from "./worker_first_async_linked.js";
import {
  createWorkerFirstAsyncRecipeHandle,
  normalizeWorkerFirstAsyncRecipeOptions,
} from "./worker_first_async_recipe.js";
import { createWorkerFirstExplicitSpecNamespace } from "./worker_first_explicit_spec_namespace.js";
import { createWorkerFirstFormFactory } from "./worker_first_form_factory.js";
import { createFeatureStoreFactory } from "../feature_store/feature_store_factory.js";
import { createLocalNamespace } from "../local/local_namespace.js";
import {
  createWorkerFirstPublicInputEntry,
  isWorkerFirstPublicGraphInputEntry,
  requireWorkerFirstInputHandle,
  requireWorkerFirstSignalHandle,
} from "./worker_first_public_input_support.js";
import { createWorkerFirstResourceNamespace } from "./worker_first_resource_namespace.js";
import { createWorkerFirstRootGraph } from "./worker_first_root_graph.js";
import { createRouterNamespace } from "../router/router_namespace.js";
import {
  assertSignalsRuntimeCompatibility,
  createSignalsRuntimeContract,
} from "../runtime_contract.js";
import { readRootHistoryFacade } from "./worker_first_root_history.js";
import { createWorkerFirstObservationNamespace } from "./worker_first_observation_namespace.js";
import { decorateWorkerFirstScopedHandle } from "./worker_first_scope_handle.js";
import {
  canonicalWorkerFirstScopedSignalId,
  createWorkerFirstScopeDescriptor,
  createWorkerFirstScopedSignalIdentity,
  requireWorkerFirstScopeLocalId,
} from "./worker_first_scope_identity.js";
import {
  createWorkerFirstSyncComputedCallbackHandle,
  createWorkerFirstSyncInputHandle,
  createWorkerFirstSyncLinkedHandle,
  createWorkerFirstSyncOutputCallbackHandle,
  createWorkerFirstSyncRecipeHandle,
} from "./worker_first_sync_authoring.js";

export function createWorkerFirstScopedNamespace(rootSession, path = []) {
  return freezeObject(createNamespace(rootSession, path));
}

function createNamespace(rootSession, path) {
  const operationPrefix = path.length === 0
    ? "signals"
    : `signals.scope(${path.map((segment) => JSON.stringify(segment)).join(").scope(")})`;
  let form = null;
  let resource = null;
  let api = null;
  let apiScope = null;
  let featureStore = null;
  let history = null;
  let local = null;
  let router = null;
  let spec = null;
  const descriptor = createWorkerFirstScopeDescriptor(path);
  const contract = createSignalsRuntimeContract({
    surfaceFamily: "workerFirstScoped",
    deployment: "workerFirst",
    scopeId: descriptor.id || null,
    capabilities: {
      callableSurface: false,
      scopedAuthoring: true,
      specNamespace: true,
      workerRuntime: true,
    },
  });
  const tagHandle = (handle, localId = null) => decorateWorkerFirstScopedHandle(
    handle,
    descriptor,
    typeof localId === "string" ? createWorkerFirstScopedSignalIdentity(path, localId) : null,
  );

  const namespace = {
    host: rootSession.hostSurface(),
    get spec() {
      spec ??= createWorkerFirstExplicitSpecNamespace(rootSession, path);
      return spec;
    },
    get form() {
      form ??= createWorkerFirstFormFactory(this);
      return form;
    },
    get resource() {
      resource ??= createWorkerFirstResourceNamespace(namespace, rootSession);
      return resource;
    },
    get api() {
      api ??= createApiFactory(namespace);
      return api;
    },
    get apiScope() {
      apiScope ??= createApiScopeFactory(namespace);
      return apiScope;
    },
    get featureStore() {
      featureStore ??= createFeatureStoreFactory(namespace);
      return featureStore;
    },
    get local() {
      local ??= createLocalNamespace(namespace);
      return local;
    },
    get router() {
      router ??= createRouterNamespace(descriptor.id);
      return router;
    },
    history() {
      history ??= readRootHistoryFacade(rootSession);
      return history;
    },
    settleAuthoredWork() {
      return rootSession.settleAuthoredWork();
    },
    ...createWorkerFirstObservationNamespace(rootSession),
    scope(localScopeId) {
      requireNonEmptyString(localScopeId, `${operationPrefix}.scope`);
      return createWorkerFirstScopedNamespace(rootSession, [...path, localScopeId]);
    },
    controller(definitionOrBuilder) {
      return buildControllerContract(rootSession, path, definitionOrBuilder);
    },
    publicInput(handle, options) {
      return createWorkerFirstPublicInputEntry(rootSession, handle, options);
    },
    input() {
      const initial = arguments[0];
      const options = arguments[1];
      const normalizedOptions = normalizeWorkerFirstScopedInputOptions(
        operationPrefix,
        options,
      );
      const localId = normalizedOptions?.[PRIVATE_AUTHORING_ID] ?? null;
      const canonicalId = localId === null
        ? rootSession.nextGeneratedStandaloneSignalId("input", path.join(".") || null)
        : canonicalWorkerFirstScopedSignalId(path, localId);
      return tagHandle(createWorkerFirstSyncInputHandle(
        rootSession,
        canonicalId,
        initial,
        normalizedOptions,
      ), localId);
    },
    async inputAsync(initial, options) {
      const normalizedOptions = normalizeWorkerFirstScopedInputOptions(
        operationPrefix,
        options,
      );
      const localId = normalizedOptions?.[PRIVATE_AUTHORING_ID] ?? null;
      const canonicalId = localId === null
        ? rootSession.nextGeneratedStandaloneSignalId("input", path.join(".") || null)
        : canonicalWorkerFirstScopedSignalId(path, localId);
      await rootSession.createStandaloneInput(canonicalId, initial, normalizedOptions);
      return tagHandle(createWorkerFirstAsyncInputHandle(
        rootSession,
        canonicalId,
        normalizedOptions?.debugName ?? null,
      ), localId);
    },
    linked() {
      return tagHandle(createWorkerFirstSyncLinkedHandle(
        rootSession,
        rootSession.nextGeneratedStandaloneSignalId("input", path.join(".") || null),
        arguments[0],
        arguments[1],
      ));
    },
    async linkedAsync(sourceOrDefinition, options) {
      return tagHandle(
        await createWorkerFirstAsyncLinkedHandle(
          rootSession,
          rootSession.nextGeneratedStandaloneSignalId("input", path.join(".") || null),
          sourceOrDefinition,
          options,
        ),
      );
    },
    computedSpec() {
      return tagHandle(this.spec.computed(...arguments), arguments[0]);
    },
    computed() {
      return tagHandle(createWorkerFirstSyncRecipeHandle(
        rootSession,
        "computed",
        rootSession.nextGeneratedStandaloneSignalId("computed", path.join(".") || null),
        arguments[0],
        arguments[1],
        `${operationPrefix}.computed`,
      ));
    },
    async computedAsync(specOrCompute, options) {
      const normalizedOptions = normalizeWorkerFirstAsyncRecipeOptions("computed", options);
      return tagHandle(
        await createWorkerFirstAsyncRecipeHandle(
          rootSession,
          "computed",
          rootSession.nextGeneratedStandaloneSignalId("computed", path.join(".") || null),
          specOrCompute,
          normalizedOptions,
        ),
      );
    },
    computedCallback() {
      return tagHandle(createWorkerFirstSyncComputedCallbackHandle(
        rootSession,
        canonicalWorkerFirstScopedSignalId(path, arguments[0]),
        arguments[1],
        arguments[2],
      ), arguments[0]);
    },
    outputSpec() {
      return tagHandle(this.spec.output(...arguments), arguments[0]);
    },
    output() {
      return tagHandle(createWorkerFirstSyncRecipeHandle(
        rootSession,
        "output",
        rootSession.nextGeneratedStandaloneSignalId("output", path.join(".") || null),
        arguments[0],
        arguments[1],
        `${operationPrefix}.output`,
      ));
    },
    async outputAsync(specOrCompute, options) {
      const normalizedOptions = normalizeWorkerFirstAsyncRecipeOptions("output", options);
      return tagHandle(
        await createWorkerFirstAsyncRecipeHandle(
          rootSession,
          "output",
          rootSession.nextGeneratedStandaloneSignalId("output", path.join(".") || null),
          specOrCompute,
          normalizedOptions,
        ),
      );
    },
    outputCallback() {
      return tagHandle(createWorkerFirstSyncOutputCallbackHandle(
        rootSession,
        canonicalWorkerFirstScopedSignalId(path, arguments[0]),
        arguments[1],
        arguments[2],
      ), arguments[0]);
    },
    graph() {
      return createWorkerFirstRootGraph(rootSession, path, ...arguments);
    },
    canonicalId(localId) {
      return canonicalWorkerFirstScopedSignalId(path, localId);
    },
    signalIdentity(localId) {
      requireWorkerFirstScopeLocalId(localId);
      return createWorkerFirstScopedSignalIdentity(path, localId);
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
        contract() {
            return contract;
        },
        assertCompatibility(options) {
            return assertSignalsRuntimeCompatibility(
              contract,
              options,
              `${operationPrefix}.assertCompatibility`,
            );
        },
    };
  return namespace;
}

function buildControllerContract(rootSession, path, definitionOrBuilder) {
  if (typeof definitionOrBuilder === "function") {
    const authoringSurface = createWorkerFirstScopedNamespace(rootSession, path);
    return buildControllerContract(rootSession, path, definitionOrBuilder(authoringSurface));
  }
  return createControllerContract(rootSession, definitionOrBuilder);
}

function createControllerContract(rootSession, definition) {
  if (!isPlainObject(definition)) {
    throw new TypeError("signals.controller requires a controller definition object");
  }
  return freezeObject({
    inputs: requireControllerInputRecord(rootSession, requireRecord(definition.inputs, "inputs")),
    outputs: requireControllerOutputRecord(rootSession, requireRecord(definition.outputs, "outputs")),
    internal: requireControllerInternalRecord(requireRecord(definition.internal, "internal")),
    [CONTROLLER_CONTRACT]: true,
  });
}

function requireControllerInputRecord(rootSession, record) {
  const clone = nullPrototypeRecord();
  for (const [name, value] of Object.entries(record)) {
    if (isWorkerFirstPublicGraphInputEntry(value)) {
      requireWorkerFirstInputHandle(rootSession, value.handle, `controller.inputs.\`${name}\``);
      clone[name] = value;
      continue;
    }
    clone[name] = requireWorkerFirstInputHandle(
      rootSession,
      value,
      `controller.inputs.\`${name}\``,
    );
  }
  return freezeObject(clone);
}

function requireControllerOutputRecord(rootSession, record) {
  const clone = nullPrototypeRecord();
  for (const [name, value] of Object.entries(record)) {
    if (isWorkerFirstPublicGraphInputEntry(value)) {
      throw new TypeError(
        `controller.outputs.\`${name}\` cannot use signals.publicInput(...); public input authority belongs only in controller.inputs`,
      );
    }
    clone[name] = requireWorkerFirstSignalHandle(
      rootSession,
      value,
      `controller.outputs.\`${name}\` must be a worker-first signal handle from the active imported graph`,
    );
  }
  return freezeObject(clone);
}

function requireControllerInternalRecord(record) {
  const clone = nullPrototypeRecord();
  for (const [name, value] of Object.entries(record)) {
    if (isWorkerFirstPublicGraphInputEntry(value)) {
      throw new TypeError(
        `controller.internal.\`${name}\` cannot use signals.publicInput(...); public authority wrappers belong only in controller.inputs`,
      );
    }
    clone[name] = value;
  }
  return freezeObject(clone);
}

function normalizeWorkerFirstScopedInputOptions(operationPrefix, options) {
  if (options === undefined) {
    return undefined;
  }
  const normalized = requireAuthoringOptions("input", options);
  if (
    typeof normalized.id === "string"
    && normalized.id.length > 0
  ) {
    return {
      ...normalized,
      [PRIVATE_AUTHORING_ID]: normalized.id,
    };
  }
  return normalized;
}

function requireRecord(candidate, fieldName) {
  if (candidate === undefined) {
    return freezeObject(nullPrototypeRecord());
  }
  if (!isPlainObject(candidate)) {
    throw new TypeError(`controller.${fieldName} must be an object when provided`);
  }
  const clone = nullPrototypeRecord();
  for (const [key, value] of Object.entries(candidate)) {
    clone[key] = value;
  }
  return freezeObject(clone);
}

function requireNonEmptyString(value, operation) {
  if (typeof value !== "string" || value.length === 0) {
    throw new TypeError(`${operation} requires a non-empty string scope id`);
  }
  return value;
}


function nullPrototypeRecord() {
  return Object.create(null);
}

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}
