import { createApiFactory } from "../api/api_namespace.js";
import {
  requireAuthoringOptions,
  requireOptionalDebugName,
} from "../authoring_option_validation.js";
import { freezeObject } from "../graph_support.js";
import { CONTROLLER_CONTRACT, PRIVATE_AUTHORING_ID, PUBLIC_GRAPH_INPUT } from "../symbols.js";
import { createWorkerFirstAsyncInputHandle } from "./worker_first_async_input.js";
import { createWorkerFirstAsyncLinkedHandle } from "./worker_first_async_linked.js";
import {
  createWorkerFirstAsyncRecipeHandle,
  normalizeWorkerFirstAsyncRecipeOptions,
} from "./worker_first_async_recipe.js";
import { createWorkerFirstExplicitSpecNamespace } from "./worker_first_explicit_spec_namespace.js";
import { createWorkerFirstFormFactory } from "./worker_first_form_factory.js";
import { createWorkerFirstResourceNamespace } from "./worker_first_resource_namespace.js";
import { createWorkerFirstRootGraph } from "./worker_first_root_graph.js";

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
  let spec = null;

  return {
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
      resource ??= createWorkerFirstResourceNamespace(rootSession);
      return resource;
    },
    get api() {
      api ??= createApiFactory(this);
      return api;
    },
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
      throwWorkerFirstCallableUnavailable(`${operationPrefix}.input`);
    },
    async inputAsync(initial, options) {
      const normalizedOptions = normalizeWorkerFirstScopedInputOptions(
        operationPrefix,
        options,
      );
      const localId = normalizedOptions?.[PRIVATE_AUTHORING_ID] ?? null;
      const canonicalId = localId === null
        ? rootSession.nextGeneratedStandaloneSignalId("input", path.join(".") || null)
        : canonicalScopedInputId(path, localId);
      await rootSession.createStandaloneInput(canonicalId, initial, normalizedOptions);
      return createWorkerFirstAsyncInputHandle(
        rootSession,
        canonicalId,
        normalizedOptions ? requireOptionalDebugName("input", normalizedOptions) : null,
      );
    },
    linked() {
      throwWorkerFirstCallableUnavailable(`${operationPrefix}.linked`);
    },
    async linkedAsync(sourceOrDefinition, options) {
      return createWorkerFirstAsyncLinkedHandle(
        rootSession,
        rootSession.nextGeneratedStandaloneSignalId("input", path.join(".") || null),
        sourceOrDefinition,
        options,
      );
    },
    computedSpec() {
      return this.spec.computed(...arguments);
    },
    computed() {
      throwWorkerFirstCallableUnavailable(`${operationPrefix}.computed`);
    },
    async computedAsync(specOrCompute, options) {
      const normalizedOptions = normalizeWorkerFirstAsyncRecipeOptions("computed", options);
      return createWorkerFirstAsyncRecipeHandle(
        rootSession,
        "computed",
        rootSession.nextGeneratedStandaloneSignalId("computed", path.join(".") || null),
        specOrCompute,
        normalizedOptions,
      );
    },
    outputSpec() {
      return this.spec.output(...arguments);
    },
    output() {
      throwWorkerFirstCallableUnavailable(`${operationPrefix}.output`);
    },
    async outputAsync(specOrCompute, options) {
      const normalizedOptions = normalizeWorkerFirstAsyncRecipeOptions("output", options);
      return createWorkerFirstAsyncRecipeHandle(
        rootSession,
        "output",
        rootSession.nextGeneratedStandaloneSignalId("output", path.join(".") || null),
        specOrCompute,
        normalizedOptions,
      );
    },
    outputCallback() {
      throwWorkerFirstCallableUnavailable(`${operationPrefix}.outputCallback`);
    },
    graph() {
      return createWorkerFirstRootGraph(rootSession, path, ...arguments);
    },
  };
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

function createWorkerFirstPublicInputEntry(rootSession, handle, options) {
  const normalizedHandle = requireWorkerFirstInputHandle(
    rootSession,
    handle,
    "signals.publicInput(...)",
  );
  const normalizedOptions = normalizePublicInputOptions(options);
  return freezeObject({
    handle: normalizedHandle,
    authority: normalizedOptions.authority,
    requiredness: normalizedOptions.requiredness,
    [PUBLIC_GRAPH_INPUT]: true,
  });
}

function normalizePublicInputOptions(options) {
  if (options === undefined) {
    return { authority: "writable", requiredness: "required" };
  }
  if (!isPlainObject(options)) {
    throw new TypeError("signals.publicInput(...) options must be an object when provided");
  }
  return {
    authority: requireAuthority(options.authority),
    requiredness: requireRequiredness(options.requiredness),
  };
}

function requireWorkerFirstInputHandle(rootSession, handle, operation) {
  const normalizedHandle = requireWorkerFirstSignalHandle(
    rootSession,
    handle,
    `${operation} expects a worker-first input handle`,
  );
  if (
    !rootSession.hasMutableInputId(normalizedHandle.id)
    || typeof normalizedHandle.set !== "function"
    || typeof normalizedHandle.reset !== "function"
    || typeof normalizedHandle.patch !== "function"
    || typeof normalizedHandle.assign !== "function"
  ) {
    throw new TypeError(`${operation} expects a worker-first input handle`);
  }
  return normalizedHandle;
}

function requireWorkerFirstSignalHandle(rootSession, handle, message) {
  if (
    typeof handle !== "function"
    || typeof handle.id !== "string"
    || handle.id.length === 0
    || !rootSession.hasKnownSignalId(handle.id)
  ) {
    throw new TypeError(message);
  }
  return handle;
}

function isWorkerFirstPublicGraphInputEntry(candidate) {
  return isPlainObject(candidate) && candidate[PUBLIC_GRAPH_INPUT] === true;
}

function requireAuthority(authority) {
  if (authority === undefined) {
    return "writable";
  }
  if (authority !== "writable" && authority !== "readOnly" && authority !== "imported") {
    throw new TypeError(
      `signals.publicInput(...) authority must be "writable", "readOnly", or "imported" when provided`,
    );
  }
  return authority;
}

function requireRequiredness(requiredness) {
  if (requiredness === undefined) {
    return "required";
  }
  if (requiredness !== "required" && requiredness !== "optional") {
    throw new TypeError(
      'signals.publicInput(...) requiredness must be "required" or "optional" when provided',
    );
  }
  return requiredness;
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

function canonicalScopedInputId(path, localId) {
  requireNonEmptyString(localId, "worker-first scoped inputAsync requires a non-empty local id");
  return path.length === 0 ? localId : `${path.join(".")}.${localId}`;
}

function createUnavailableNamespace(operation) {
  return new Proxy(
    freezeObject({}),
    {
      get() {
        throwWorkerFirstCallableUnavailable(operation);
      },
      has() {
        return false;
      },
      ownKeys() {
        return [];
      },
      getOwnPropertyDescriptor() {
        return undefined;
      },
    },
  );
}

function throwWorkerFirstCallableUnavailable(operation) {
  const error = new Error(
    `${operation} is unavailable on the current worker-first callable surface; use deployment: "mainThreadCompatibility" for authoring and root runtime operations beyond imported graph hydration`,
  );
  error.name = "WorkerFirstCallableSurfaceUnavailable";
  error.code = "workerFirstCallableSurfaceUnavailable";
  error.compatibilityRecovery = Object.freeze({
    deployment: "mainThreadCompatibility",
    message:
      'Retry with deployment: "mainThreadCompatibility" to use the full callable root surface.',
  });
  throw error;
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
