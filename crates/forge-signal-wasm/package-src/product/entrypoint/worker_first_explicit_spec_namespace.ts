import { freezeObject } from "../graph_support.js";
import { PRODUCT_SIGNAL_KIND } from "../symbols.js";
import {
  denyWorkerFirstMutationDuringCallbackAuthoring,
  readWorkerFirstTrackedSignal,
} from "./worker_first_callback_tracking.js";

export function createWorkerFirstExplicitSpecNamespace(rootSession, path = []) {
  return freezeObject({
    input(localId, initial, options) {
      const id = canonicalSpecId(path, localId);
      void initial;
      void options;
      requireWorkerFirstInputAvailability(rootSession, id);
      return createWorkerFirstSpecInputHandle(rootSession, id);
    },
    computed(localId, spec, options) {
      const id = canonicalSpecId(path, localId);
      requireWorkerFirstRecipeDeclaration(rootSession, id, "computed", spec, options);
      return createWorkerFirstSpecReadableHandle(
        rootSession,
        id,
        "computed",
      );
    },
    computedCallback(localId, callback, options) {
      void localId;
      void callback;
      void options;
      throwWorkerFirstSpecCallbackUnavailable("signals.spec.computedCallback");
    },
    output(localId, spec, options) {
      const id = canonicalSpecId(path, localId);
      requireWorkerFirstRecipeDeclaration(rootSession, id, "output", spec, options);
      return createWorkerFirstSpecReadableHandle(
        rootSession,
        id,
        "output",
      );
    },
    outputCallback(localId, callback, options) {
      void localId;
      void callback;
      void options;
      throwWorkerFirstSpecCallbackUnavailable("signals.spec.outputCallback");
    },
  });
}

function createWorkerFirstSpecInputHandle(rootSession, id) {
  requireWorkerFirstSpecDescriptor(rootSession, id, "input");
  const read = () => readWorkerFirstTrackedSignal(
    rootSession,
    id,
    () => readWorkerFirstSpecSignalValue(rootSession, id, "input"),
  );
  const handle = function workerFirstSpecInputSignal() {
    return read();
  };
  handle.get = read;
  handle.value = read;
  handle.free = () => {};
  handle[Symbol.dispose] = () => {};
  handle.id = id;
  handle.debugName = null;
  handle[PRODUCT_SIGNAL_KIND] = "input";
  handle.set = (value) => {
    denyWorkerFirstMutationDuringCallbackAuthoring();
    return rootSession.applyActiveInputMutation(id, { kind: "set", value });
  };
  handle.reset = () => {
    denyWorkerFirstMutationDuringCallbackAuthoring();
    return rootSession.applyActiveInputMutation(id, { kind: "reset" });
  };
  handle.patch = (value) => {
    denyWorkerFirstMutationDuringCallbackAuthoring();
    return rootSession.applyActiveInputMutation(id, { kind: "patch", value });
  };
  handle.assign = (fields) => {
    denyWorkerFirstMutationDuringCallbackAuthoring();
    return rootSession.applyActiveInputMutation(id, { kind: "patch", value: fields });
  };
  return freezeObject(handle);
}

function createWorkerFirstSpecReadableHandle(rootSession, id, family) {
  requireWorkerFirstSpecDescriptor(rootSession, id, family);
  const read = () => readWorkerFirstTrackedSignal(
    rootSession,
    id,
    () => readWorkerFirstSpecSignalValue(rootSession, id, family),
  );
  const handle = function workerFirstSpecReadableSignal() {
    return read();
  };
  handle.get = read;
  handle.value = read;
  handle.free = () => {};
  handle[Symbol.dispose] = () => {};
  handle.id = id;
  handle.debugName = null;
  handle[PRODUCT_SIGNAL_KIND] = family;
  return freezeObject(handle);
}

function readWorkerFirstSpecSignalValue(rootSession, id, family) {
  return requireWorkerFirstSpecDescriptor(rootSession, id, family).value;
}

function requireWorkerFirstSpecDescriptor(rootSession, id, family) {
  const context = rootSession.currentImportContext();
  if (family === "input") {
    const descriptor = context.inputDescriptorBySourceId.get(id);
    if (descriptor) {
      return {
        value: context.signalValueById.get(id),
      };
    }
  } else {
    const descriptor = context.outputDescriptorBySourceId.get(id);
    if (descriptor && descriptor.sourceKind === family) {
      return {
        value: context.signalValueById.get(id),
      };
    }
  }
  throw new TypeError(
    `worker-first signals.spec.${family}(...) binds only to ${family} ids from the active imported graph; \`${id}\` is not currently available`,
  );
}

function requireWorkerFirstInputAvailability(rootSession, id) {
  const context = rootSession.currentImportContext();
  const descriptor = context.inputDescriptorBySourceId.get(id);
  if (!descriptor) {
    throw new TypeError(
      `worker-first signals.spec.input(...) binds only to input ids from the active imported graph; \`${id}\` is not currently available`,
    );
  }
}

function requireWorkerFirstRecipeDeclaration(rootSession, id, family, spec, options) {
  if (options !== undefined) {
    throw new TypeError(
      `worker-first signals.spec.${family}(...) does not accept authoring options; it requires the exact imported declaration for \`${id}\``,
    );
  }
  const context = rootSession.currentImportContext();
  const descriptor = context.outputDescriptorBySourceId.get(id);
  const recipeDefinition = context.recipeDefinitionById.get(id);
  if (!descriptor || !recipeDefinition || descriptor.sourceKind !== family) {
    throw new TypeError(
      `worker-first signals.spec.${family}(...) binds only to ${family} ids from the active imported graph; \`${id}\` is not currently available`,
    );
  }
  if (!sameJsonValue(normalizeRecipeSpec(recipeDefinition), normalizeRecipeSpec(spec))) {
    throw new TypeError(
      `worker-first signals.spec.${family}(...) requires the imported recipe declaration for \`${id}\`; the provided declaration does not match the active imported graph`,
    );
  }
}

function normalizeRecipeSpec(spec) {
  if (!spec || typeof spec !== "object" || Array.isArray(spec)) {
    return spec;
  }
  return {
    reads: spec.reads ?? null,
    expr: spec.expr ?? null,
    when: spec.when ?? null,
    identity: spec.identity ?? null,
    producesAspects: normalizeOptionalArray(spec.producesAspects),
  };
}

function normalizeOptionalArray(value) {
  return value === undefined ? null : value;
}

function sameJsonValue(left, right) {
  return JSON.stringify(left) === JSON.stringify(right);
}

function throwWorkerFirstSpecCallbackUnavailable(operation) {
  const error = new Error(
    `${operation}(...) is unavailable on the current worker-first spec surface because callback declarations are not portable through active imported-graph binding; use deployment: "mainThreadCompatibility" for callback explicit authoring`,
  );
  error.name = "WorkerFirstSpecCallbackUnavailable";
  error.code = "workerFirstSpecCallbackUnavailable";
  error.compatibilityRecovery = freezeObject({
    deployment: "mainThreadCompatibility",
    message:
      'Retry with deployment: "mainThreadCompatibility" to use callback explicit authoring.',
  });
  throw error;
}

function canonicalSpecId(path, localId) {
  if (typeof localId !== "string" || localId.length === 0) {
    throw new TypeError("worker-first signals.spec(...) requires a non-empty signal id");
  }
  return path.length === 0 ? localId : `${path.join(".")}.${localId}`;
}
