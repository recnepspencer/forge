import { createRawSignals } from "../raw_surface.js";
import {
  parseComputedCallbackArgs,
  parseOutputCallbackArgs,
  withComputedCallbackFrame,
} from "./callback_frames.js";
import { createControllerContract } from "./controllers.js";
import {
  clockCapability,
  createHostCapabilities,
  hostCapabilityPlan,
  onlineCapability,
  persistenceCapability,
  viewportCapability,
  visibilityCapability,
} from "./host_capabilities.js";
import { wrapDiagnostics } from "./diagnostics.js";
import { createImportedSignalGraph, createPublishedSignalGraph } from "./graphs.js";
import { wrapHistory } from "./history.js";
import { unwrapSignalTarget, wrapInputSignal, wrapReadableSignal } from "./handles.js";
import { createPublicGraphInputEntry } from "./public_inputs.js";
import { createScopedSignalNamespace, reserveAuthoringSignalId } from "./scopes.js";
import { wrapSpecialist } from "./specialist.js";
import { wrapAdapters, wrapTransaction } from "./transactions.js";

const OUTPUT_CALLBACK_PROJECTION_COUNTERS = new WeakMap();

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function cloneSignalValue(value) {
  if (typeof globalThis.structuredClone === "function") {
    try {
      return globalThis.structuredClone(value);
    } catch {
      return value;
    }
  }
  if (Array.isArray(value)) {
    return value.slice();
  }
  if (isPlainObject(value)) {
    return { ...value };
  }
  return value;
}

function requireAuthoringOptions(family, options) {
  if (!isPlainObject(options)) {
    throw new TypeError(`${family} options must be an object when provided`);
  }
  return options;
}

function requireAuthoringId(family, options) {
  if (typeof options.id !== "string" || options.id.length === 0) {
    throw new TypeError(`${family} metadata form requires a non-empty string id`);
  }
  return options.id;
}

function looksLikeInputMetadataOptions(value) {
  if (!isPlainObject(value) || typeof value.id !== "string" || value.id.length === 0) {
    return false;
  }
  return Object.keys(value).every((key) => key === "id" || key === "producesAspects");
}

function parseInputArgs(firstArg, secondArg, thirdArg) {
  if (typeof firstArg === "string" && !looksLikeInputMetadataOptions(secondArg)) {
    return {
      id: firstArg,
      initial: secondArg,
      options: thirdArg,
    };
  }

  const options = requireAuthoringOptions("input", secondArg);
  if (thirdArg !== undefined) {
    throw new TypeError("input metadata form does not accept a third argument");
  }
  const { id, ...inputOptions } = options;
  requireAuthoringId("input", options);
  return {
    id,
    initial: firstArg,
    options: Object.keys(inputOptions).length === 0 ? undefined : inputOptions,
  };
}

function parseSpecAuthoringArgs(family, firstArg, secondArg, thirdArg) {
  if (typeof firstArg === "string") {
    if (thirdArg !== undefined) {
      throw new TypeError(`${family} spec form does not accept a third argument`);
    }
    return {
      id: firstArg,
      spec: secondArg,
    };
  }

  const options = requireAuthoringOptions(family, secondArg);
  if (thirdArg !== undefined) {
    throw new TypeError(`${family} metadata spec form does not accept a third argument`);
  }
  return {
    id: requireAuthoringId(family, options),
    spec: firstArg,
  };
}

function nextOutputProjectionId(rawSignals, outputId) {
  const next = (OUTPUT_CALLBACK_PROJECTION_COUNTERS.get(rawSignals) ?? 0) + 1;
  OUTPUT_CALLBACK_PROJECTION_COUNTERS.set(rawSignals, next);
  return `__forgeSignal.outputProjection.${outputId}.${next}`;
}

function outputProjectionSpec(hiddenComputedId) {
  return {
    reads: [hiddenComputedId],
    expr: {
      kind: "read",
      id: hiddenComputedId,
    },
  };
}

function withReservedSignalId(rawSignals, family, id, callback) {
  const release = reserveAuthoringSignalId(rawSignals, family, id);
  try {
    return callback();
  } catch (error) {
    release();
    throw error;
  }
}

export {
  clockCapability,
  hostCapabilityPlan,
  onlineCapability,
  persistenceCapability,
  viewportCapability,
  visibilityCapability,
};

export function wrapSignals(rawSignals, options) {
  const hostCapabilities = createHostCapabilities(rawSignals, options);
  let diagnostics = null;
  const callableSignals = {
    host: hostCapabilities.host,
    scope(localScopeId) {
      return createScopedSignalNamespace(callableSignals, rawSignals, localScopeId);
    },
    controller(definition) {
      return createControllerContract(definition);
    },
    publicInput(handle, options) {
      return createPublicGraphInputEntry(handle, options);
    },
    input(idOrInitial, initialOrOptions, maybeOptions) {
      const { id, initial, options } = parseInputArgs(idOrInitial, initialOrOptions, maybeOptions);
      return withReservedSignalId(rawSignals, "input", id, () => (
        wrapInputSignal(
          rawSignals.input(id, initial, options),
          rawSignals,
          cloneSignalValue(initial),
        )
      ));
    },
    computedSpec(id, spec) {
      return withReservedSignalId(rawSignals, "computed", id, () => (
        wrapReadableSignal(rawSignals.computedSpec(id, spec), rawSignals, "computed")
      ));
    },
    computed(idOrCompute, specOrCompute, maybeOptions) {
      const callbackArgs = parseComputedCallbackArgs(
        rawSignals,
        idOrCompute,
        specOrCompute,
        maybeOptions,
      );
      if (callbackArgs) {
        const callback = withComputedCallbackFrame(rawSignals, callbackArgs.callback);
        return withReservedSignalId(rawSignals, "computed", callbackArgs.id, () => (
          wrapReadableSignal(
            rawSignals.computedCallback(callbackArgs.id, callback),
            rawSignals,
            "computed",
          )
        ));
      }
      const { id, spec } = parseSpecAuthoringArgs(
        "computed",
        idOrCompute,
        specOrCompute,
        maybeOptions,
      );
      return withReservedSignalId(rawSignals, "computed", id, () => (
        wrapReadableSignal(rawSignals.computedSpec(id, spec), rawSignals, "computed")
      ));
    },
    outputSpec(id, spec) {
      return withReservedSignalId(rawSignals, "output", id, () => (
        wrapReadableSignal(rawSignals.outputSpec(id, spec), rawSignals, "output")
      ));
    },
    output(idOrSpec, specOrCompute, maybeOptions) {
      const callbackArgs = parseOutputCallbackArgs(
        rawSignals,
        idOrSpec,
        specOrCompute,
        maybeOptions,
      );
      if (callbackArgs) {
        const wrappedCallback = withComputedCallbackFrame(rawSignals, callbackArgs.callback);
        const hiddenComputedId = nextOutputProjectionId(rawSignals, callbackArgs.id);
        return withReservedSignalId(rawSignals, "output", callbackArgs.id, () => {
          rawSignals.computedCallback(hiddenComputedId, wrappedCallback);
          return wrapReadableSignal(
            rawSignals.outputSpec(callbackArgs.id, outputProjectionSpec(hiddenComputedId)),
            rawSignals,
            "output",
          );
        });
      }
      const { id, spec } = parseSpecAuthoringArgs(
        "output",
        idOrSpec,
        specOrCompute,
        maybeOptions,
      );
      return withReservedSignalId(rawSignals, "output", id, () => (
        wrapReadableSignal(rawSignals.outputSpec(id, spec), rawSignals, "output")
      ));
    },
    outputCallback(id, callback) {
      const wrappedCallback = withComputedCallbackFrame(rawSignals, callback);
      const hiddenComputedId = nextOutputProjectionId(rawSignals, id);
      return withReservedSignalId(rawSignals, "output", id, () => {
        rawSignals.computedCallback(hiddenComputedId, wrappedCallback);
        return wrapReadableSignal(
          rawSignals.outputSpec(id, outputProjectionSpec(hiddenComputedId)),
          rawSignals,
          "output",
        );
      });
    },
    graph(id, definition) {
      return createPublishedSignalGraph(callableSignals, rawSignals, id, definition);
    },
    importGraph(definition, snapshot) {
      return createImportedSignalGraph(callableSignals, rawSignals, definition, snapshot);
    },
    read(target) {
      return rawSignals.read(unwrapSignalTarget(target, rawSignals, "signals.read"));
    },
    watch(target, callback) {
      return rawSignals.watch(
        unwrapSignalTarget(target, rawSignals, "signals.watch"),
        callback,
      );
    },
    effect(target, callback) {
      return rawSignals.effect(
        unwrapSignalTarget(target, rawSignals, "signals.effect"),
        callback,
      );
    },
    transaction(callback) {
      return rawSignals.transaction((rawTx) => callback(wrapTransaction(rawTx, rawSignals)));
    },
    batch(callback) {
      return rawSignals.batch((rawTx) => callback(wrapTransaction(rawTx, rawSignals)));
    },
    nuke: rawSignals.nuke.bind(rawSignals),
    diagnostics() {
      if (!diagnostics) {
        diagnostics = wrapDiagnostics(rawSignals.diagnostics(), hostCapabilities);
      }
      return diagnostics;
    },
    history() {
      return wrapHistory(rawSignals.history());
    },
    specialist() {
      return wrapSpecialist(rawSignals.specialist());
    },
    adapters() {
      return wrapAdapters(rawSignals.adapters(), hostCapabilities);
    },
    compatibilityApp: rawSignals.compatibilityApp.bind(rawSignals),
    compatibilityRuntime: rawSignals.compatibilityRuntime.bind(rawSignals),
    free() {
      hostCapabilities.dispose();
      rawSignals.free();
    },
    [Symbol.dispose]() {
      hostCapabilities.dispose();
      if (typeof rawSignals[Symbol.dispose] === "function") {
        rawSignals[Symbol.dispose]();
        return;
      }
      rawSignals.free();
    },
  };
  return callableSignals;
}

export function createCallableSignals(options) {
  return wrapSignals(createRawSignals(), options);
}

export function createSignals(options) {
  return createCallableSignals(options);
}
