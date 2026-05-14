import { createRawSignals } from "../raw_surface.js";
import {
  parseComputedCallbackArgs,
  parseOutputCallbackArgs,
  withComputedCallbackFrame,
} from "./callback_frames.js";
import { buildControllerContract } from "./controllers.js";
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
import { createApiFactory } from "./api/api_namespace.js";
import {
  createImportedSignalGraph,
  createPublishedSignalGraph,
} from "./graphs.js";
import { wrapHistory } from "./history.js";
import {
  unwrapSignalTarget,
  wrapInputSignal,
  wrapReadableSignal,
} from "./handles.js";
import { createLinkedSignal } from "./linked.js";
import {
  nextOutputProjectionId,
  outputProjectionSpec,
} from "./output_projection_ids.js";
import { createPublicGraphInputEntry } from "./public_inputs.js";
import {
  createResourceNamespace,
  resourceBinaryDescriptor,
  resourceBinaryValue,
  resourceAuth,
  resourceCollectionShape,
  resourceContinuation,
  resourceDetailFields,
  resourceDetailRegions,
  resourceDetailJsonPaths,
  resourceDelivery,
  resourceDownload,
  resourceItemAspects,
  resourceParamIdentity,
  resourcePatch,
  resourceValueSummaries,
  resourceParams,
  resourcePolicyProfiles,
  resourceProcessingJob,
  resourceProcessingResult,
  resourceMutationResponses,
  resourceResponse,
  resourceUploadResult,
  resourceUploadTransport,
  resourceRequestContext,
} from "./resource/facade.js";
import {
  forbidOpaqueIdOption,
  isPlainObject,
  looksLikeInputMetadataOptions,
  looksLikeOpaqueAuthoringOptions,
  requireAuthoringOptions,
  requireOptionalDebugName,
} from "./authoring_option_validation.js";
import { withReservedSignalId } from "./reserved_authoring_ids.js";
import {
  createScopedSignalNamespace,
  nextGeneratedAuthoringSignalId,
} from "./scopes.js";
import { wrapSpecialist } from "./specialist.js";
import { PRIVATE_AUTHORING_ID, RAW_SIGNALS } from "./symbols.js";
import { wrapAdapters, wrapTransaction } from "./transactions.js";

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

function parseOpaqueInputArgs(rawSignals, firstArg, secondArg, thirdArg) {
  if (
    (typeof firstArg === "string" &&
      secondArg !== undefined &&
      !looksLikeOpaqueAuthoringOptions(secondArg)) ||
    looksLikeInputMetadataOptions(secondArg)
  ) {
    throw new TypeError(
      "input app authoring does not accept an explicit id; use signals.spec.input(...) when you need an explicit structural name",
    );
  }
  const options =
    secondArg === undefined
      ? undefined
      : requireAuthoringOptions("input", secondArg);
  if (thirdArg !== undefined) {
    throw new TypeError("input app form does not accept a third argument");
  }
  if (options) {
    forbidOpaqueIdOption("input", options);
  }
  return {
    id:
      options?.[PRIVATE_AUTHORING_ID] ??
      nextGeneratedAuthoringSignalId(rawSignals, "input"),
    initial: firstArg,
    debugName: options ? requireOptionalDebugName("input", options) : null,
    options: options
      ? {
          ...(options.producesAspects === undefined
            ? {}
            : { producesAspects: options.producesAspects }),
        }
      : undefined,
  };
}

function parseOpaqueCallbackOptions(
  rawSignals,
  family,
  computeOrSpec,
  options,
  maybeOptions,
) {
  if (typeof computeOrSpec === "string" && typeof options === "function") {
    throw new TypeError(
      `${family} app authoring does not accept an explicit id; use signals.spec.${family}Callback(...) when you need an explicit structural name`,
    );
  }
  const callbackArgs =
    family === "computed"
      ? parseComputedCallbackArgs(
          rawSignals,
          computeOrSpec,
          options,
          maybeOptions,
        )
      : parseOutputCallbackArgs(
          rawSignals,
          computeOrSpec,
          options,
          maybeOptions,
        );
  if (!callbackArgs) {
    return null;
  }
  const callbackOptions =
    typeof computeOrSpec === "function" ? (options ?? {}) : {};
  if (isPlainObject(callbackOptions)) {
    forbidOpaqueIdOption(family, callbackOptions);
  }
  return {
    ...callbackArgs,
    debugName: requireOptionalDebugName(family, callbackOptions),
  };
}

function parseOpaqueSpecOptions(
  rawSignals,
  family,
  firstArg,
  secondArg,
  thirdArg,
) {
  if (typeof firstArg === "string") {
    throw new TypeError(
      `${family} app authoring does not accept an explicit id; use signals.spec.${family}(...) when you need an explicit structural name`,
    );
  }
  const options =
    secondArg === undefined
      ? undefined
      : requireAuthoringOptions(family, secondArg);
  if (thirdArg !== undefined) {
    throw new TypeError(
      `${family} app spec form does not accept a third argument`,
    );
  }
  if (options) {
    forbidOpaqueIdOption(family, options);
  }
  return {
    id:
      options?.[PRIVATE_AUTHORING_ID] ??
      nextGeneratedAuthoringSignalId(rawSignals, family),
    spec: firstArg,
    debugName: options ? requireOptionalDebugName(family, options) : null,
  };
}

function isNamedCallbackDefinition(spec) {
  return isPlainObject(spec) && typeof spec.compute === "function";
}

function createExplicitNamedSignal(rawSignals, family, id, specOrCallback, debugName) {
  if (isNamedCallbackDefinition(specOrCallback)) {
    const callback = withComputedCallbackFrame(rawSignals, specOrCallback.compute);
    if (family === "computed") {
      return wrapReadableSignal(
        rawSignals.computedCallback(id, callback),
        rawSignals,
        "computed",
        debugName,
      );
    }
    const hiddenComputedId = nextOutputProjectionId(rawSignals, id);
    rawSignals.computedCallback(hiddenComputedId, callback);
    return wrapReadableSignal(
      rawSignals.outputSpec(id, outputProjectionSpec(hiddenComputedId)),
      rawSignals,
      "output",
      debugName,
    );
  }
  if (family === "computed") {
    return wrapReadableSignal(
      rawSignals.computedSpec(id, specOrCallback),
      rawSignals,
      "computed",
      debugName,
    );
  }
  return wrapReadableSignal(
    rawSignals.outputSpec(id, specOrCallback),
    rawSignals,
    "output",
    debugName,
  );
}

function explicitSignalSpecNamespace(rawSignals) {
  return Object.freeze({
    input(id, initial, options) {
      const specOptions =
        options === undefined
          ? undefined
          : requireAuthoringOptions("input", options);
      const debugName = specOptions
        ? requireOptionalDebugName("input", specOptions)
        : null;
      const inputOptions = specOptions
        ? {
            ...(specOptions.producesAspects === undefined
              ? {}
              : { producesAspects: specOptions.producesAspects }),
          }
        : undefined;
      return withReservedSignalId(rawSignals, "input", id, () =>
        wrapInputSignal(
          rawSignals.input(id, initial, inputOptions),
          rawSignals,
          cloneSignalValue(initial),
          debugName,
        ),
      );
    },
    computed(id, spec, options) {
      const specOptions =
        options === undefined
          ? undefined
          : requireAuthoringOptions("computed", options);
      const debugName = specOptions
          ? requireOptionalDebugName("computed", specOptions)
          : null;
      return withReservedSignalId(rawSignals, "computed", id, () =>
        createExplicitNamedSignal(rawSignals, "computed", id, spec, debugName),
      );
    },
    computedCallback(id, callback, options) {
      const callbackOptions =
        options === undefined
          ? undefined
          : requireAuthoringOptions("computed", options);
      const debugName = callbackOptions
        ? requireOptionalDebugName("computed", callbackOptions)
        : null;
      return withReservedSignalId(rawSignals, "computed", id, () =>
        wrapReadableSignal(
          rawSignals.computedCallback(
            id,
            withComputedCallbackFrame(rawSignals, callback),
          ),
          rawSignals,
          "computed",
          debugName,
        ),
      );
    },
    output(id, spec, options) {
      const specOptions =
        options === undefined
          ? undefined
          : requireAuthoringOptions("output", options);
      const debugName = specOptions
          ? requireOptionalDebugName("output", specOptions)
          : null;
      return withReservedSignalId(rawSignals, "output", id, () =>
        createExplicitNamedSignal(rawSignals, "output", id, spec, debugName),
      );
    },
    outputCallback(id, callback, options) {
      const callbackOptions =
        options === undefined
          ? undefined
          : requireAuthoringOptions("output", options);
      const debugName = callbackOptions
        ? requireOptionalDebugName("output", callbackOptions)
        : null;
      const wrappedCallback = withComputedCallbackFrame(rawSignals, callback);
      const hiddenComputedId = nextOutputProjectionId(rawSignals, id);
      return withReservedSignalId(rawSignals, "output", id, () => {
        rawSignals.computedCallback(hiddenComputedId, wrappedCallback);
        return wrapReadableSignal(
          rawSignals.outputSpec(id, outputProjectionSpec(hiddenComputedId)),
          rawSignals,
          "output",
          debugName,
        );
      });
    },
  });
}

export {
  clockCapability,
  hostCapabilityPlan,
  onlineCapability,
  persistenceCapability,
  resourceBinaryDescriptor,
  resourceBinaryValue,
  resourceAuth,
  resourceCollectionShape,
  resourceContinuation,
  resourceDetailFields,
  resourceDetailRegions,
  resourceDetailJsonPaths,
  resourceDelivery,
  resourceDownload,
  resourceItemAspects,
  resourceParamIdentity,
  resourcePatch,
  resourceValueSummaries,
  resourceParams,
  resourcePolicyProfiles,
  resourceProcessingJob,
  resourceProcessingResult,
  resourceMutationResponses,
  resourceUploadResult,
  resourceUploadTransport,
  resourceRequestContext,
  resourceResponse,
  viewportCapability,
  visibilityCapability,
};

export function wrapSignals(rawSignals, options) {
  const hostCapabilities = createHostCapabilities(rawSignals, options);
  let diagnostics = null;
  const explicitSpec = explicitSignalSpecNamespace(rawSignals);
  const callableSignals = {
    host: hostCapabilities.host,
    resource: createResourceNamespace(null, rawSignals),
    api: null,
    spec: explicitSpec,
    scope(localScopeId) {
      return createScopedSignalNamespace(
        callableSignals,
        rawSignals,
        localScopeId,
      );
    },
    controller(definitionOrBuilder) {
      return buildControllerContract(callableSignals, definitionOrBuilder);
    },
    publicInput(handle, options) {
      return createPublicGraphInputEntry(handle, options);
    },
    input(idOrInitial, initialOrOptions, maybeOptions) {
      const { id, initial, options, debugName } = parseOpaqueInputArgs(
        rawSignals,
        idOrInitial,
        initialOrOptions,
        maybeOptions,
      );
      return withReservedSignalId(rawSignals, "input", id, () =>
        wrapInputSignal(
          rawSignals.input(id, initial, options),
          rawSignals,
          cloneSignalValue(initial),
          debugName,
        ),
      );
    },
    linked(sourceOrDefinition, options) {
      return createLinkedSignal(
        callableSignals,
        rawSignals,
        sourceOrDefinition,
        options,
      );
    },
    computedSpec(id, spec, options) {
      return explicitSpec.computed(id, spec, options);
    },
    computed(idOrCompute, specOrCompute, maybeOptions) {
      const callbackArgs = parseOpaqueCallbackOptions(
        rawSignals,
        "computed",
        idOrCompute,
        specOrCompute,
        maybeOptions,
      );
      if (callbackArgs) {
        const callback = withComputedCallbackFrame(
          rawSignals,
          callbackArgs.callback,
        );
        return withReservedSignalId(
          rawSignals,
          "computed",
          callbackArgs.id,
          () =>
            wrapReadableSignal(
              rawSignals.computedCallback(callbackArgs.id, callback),
              rawSignals,
              "computed",
              callbackArgs.debugName,
            ),
        );
      }
      const { id, spec, debugName } = parseOpaqueSpecOptions(
        rawSignals,
        "computed",
        idOrCompute,
        specOrCompute,
        maybeOptions,
      );
      return withReservedSignalId(rawSignals, "computed", id, () =>
        wrapReadableSignal(
          rawSignals.computedSpec(id, spec),
          rawSignals,
          "computed",
          debugName,
        ),
      );
    },
    outputSpec(id, spec, options) {
      return explicitSpec.output(id, spec, options);
    },
    output(idOrSpec, specOrCompute, maybeOptions) {
      const callbackArgs = parseOpaqueCallbackOptions(
        rawSignals,
        "output",
        idOrSpec,
        specOrCompute,
        maybeOptions,
      );
      if (callbackArgs) {
        const wrappedCallback = withComputedCallbackFrame(
          rawSignals,
          callbackArgs.callback,
        );
        const hiddenComputedId = nextOutputProjectionId(
          rawSignals,
          callbackArgs.id,
        );
        return withReservedSignalId(
          rawSignals,
          "output",
          callbackArgs.id,
          () => {
            rawSignals.computedCallback(hiddenComputedId, wrappedCallback);
            return wrapReadableSignal(
              rawSignals.outputSpec(
                callbackArgs.id,
                outputProjectionSpec(hiddenComputedId),
              ),
              rawSignals,
              "output",
              callbackArgs.debugName,
            );
          },
        );
      }
      const { id, spec, debugName } = parseOpaqueSpecOptions(
        rawSignals,
        "output",
        idOrSpec,
        specOrCompute,
        maybeOptions,
      );
      return withReservedSignalId(rawSignals, "output", id, () =>
        wrapReadableSignal(
          rawSignals.outputSpec(id, spec),
          rawSignals,
          "output",
          debugName,
        ),
      );
    },
    outputCallback(id, callback, options) {
      return explicitSpec.outputCallback(id, callback, options);
    },
    graph(id, definition) {
      return createPublishedSignalGraph(
        callableSignals,
        rawSignals,
        id,
        definition,
      );
    },
    importGraph(definition, snapshot) {
      return createImportedSignalGraph(
        callableSignals,
        rawSignals,
        definition,
        snapshot,
      );
    },
    read(target) {
      return rawSignals.read(
        unwrapSignalTarget(target, rawSignals, "signals.read"),
      );
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
      return rawSignals.transaction((rawTx) =>
        callback(wrapTransaction(rawTx, rawSignals)),
      );
    },
    batch(callback) {
      return rawSignals.batch((rawTx) =>
        callback(wrapTransaction(rawTx, rawSignals)),
      );
    },
    nuke: rawSignals.nuke.bind(rawSignals),
    diagnostics() {
      if (!diagnostics) {
        diagnostics = wrapDiagnostics(
          rawSignals.diagnostics(),
          hostCapabilities,
        );
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
    [RAW_SIGNALS]: rawSignals,
  };
  callableSignals.resource = createResourceNamespace(
    callableSignals,
    rawSignals,
  );
  callableSignals.api = createApiFactory(callableSignals);
  return callableSignals;
}

export function createCallableSignals(options) {
  return wrapSignals(createRawSignals(), options);
}

export function createSignals(options) {
  return createCallableSignals(options);
}
