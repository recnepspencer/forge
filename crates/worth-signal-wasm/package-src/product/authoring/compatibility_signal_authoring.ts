import {
  parseComputedCallbackArgs,
  parseOutputCallbackArgs,
  withComputedCallbackFrame,
} from "../callback_frames.js";
import {
  forbidOpaqueIdOption,
  isPlainObject,
  looksLikeInputMetadataOptions,
  looksLikeOpaqueAuthoringOptions,
  requireAuthoringOptions,
  requireOptionalDebugName,
} from "../authoring_option_validation.js";
import { wrapInputSignal, wrapReadableSignal } from "../handles.js";
import { nextOutputProjectionId, outputProjectionSpec } from "../output_projection_ids.js";
import { withReservedSignalId } from "../reserved_authoring_ids.js";
import { nextGeneratedAuthoringSignalId } from "../scopes.js";
import { PRIVATE_AUTHORING_ID } from "../symbols.js";

export function cloneSignalValue(value) {
  if (typeof globalThis.structuredClone === "function") {
    try {
      return globalThis.structuredClone(value);
    } catch {
      return value;
    }
  }
  if (Array.isArray(value)) return value.slice();
  return isPlainObject(value) ? { ...value } : value;
}

export function parseOpaqueInputArgs(rawSignals, firstArg, secondArg, thirdArg) {
  if (
    (typeof firstArg === "string" && secondArg !== undefined
      && !looksLikeOpaqueAuthoringOptions(secondArg))
    || looksLikeInputMetadataOptions(secondArg)
  ) {
    throw new TypeError(
      "input app authoring does not accept an explicit id; use signals.spec.input(...) when you need an explicit structural name",
    );
  }
  const options = secondArg === undefined
    ? undefined
    : requireAuthoringOptions("input", secondArg);
  if (thirdArg !== undefined) {
    throw new TypeError("input app form does not accept a third argument");
  }
  if (options) forbidOpaqueIdOption("input", options);
  return {
    id: options?.[PRIVATE_AUTHORING_ID] ?? nextGeneratedAuthoringSignalId(rawSignals, "input"),
    initial: firstArg,
    debugName: options ? requireOptionalDebugName("input", options) : null,
    options: options
      ? { ...(options.producesAspects === undefined ? {} : { producesAspects: options.producesAspects }) }
      : undefined,
  };
}

export function parseOpaqueCallbackOptions(rawSignals, family, computeOrSpec, options, maybeOptions) {
  if (typeof computeOrSpec === "string" && typeof options === "function") {
    throw new TypeError(
      `${family} app authoring does not accept an explicit id; use signals.spec.${family}Callback(...) when you need an explicit structural name`,
    );
  }
  const callbackArgs = family === "computed"
    ? parseComputedCallbackArgs(rawSignals, computeOrSpec, options, maybeOptions)
    : parseOutputCallbackArgs(rawSignals, computeOrSpec, options, maybeOptions);
  if (!callbackArgs) return null;
  const callbackOptions = typeof computeOrSpec === "function" ? (options ?? {}) : {};
  if (isPlainObject(callbackOptions)) forbidOpaqueIdOption(family, callbackOptions);
  return { ...callbackArgs, debugName: requireOptionalDebugName(family, callbackOptions) };
}

export function parseOpaqueSpecOptions(rawSignals, family, firstArg, secondArg, thirdArg) {
  if (typeof firstArg === "string") {
    throw new TypeError(
      `${family} app authoring does not accept an explicit id; use signals.spec.${family}(...) when you need an explicit structural name`,
    );
  }
  const options = secondArg === undefined
    ? undefined
    : requireAuthoringOptions(family, secondArg);
  if (thirdArg !== undefined) {
    throw new TypeError(`${family} app spec form does not accept a third argument`);
  }
  if (options) forbidOpaqueIdOption(family, options);
  return {
    id: options?.[PRIVATE_AUTHORING_ID] ?? nextGeneratedAuthoringSignalId(rawSignals, family),
    spec: firstArg,
    debugName: options ? requireOptionalDebugName(family, options) : null,
  };
}

export function explicitSignalSpecNamespace(rawSignals) {
  return Object.freeze({
    input(id, initial, options) {
      const specOptions = authoringOptions("input", options);
      const inputOptions = specOptions
        ? { ...(specOptions.producesAspects === undefined ? {} : { producesAspects: specOptions.producesAspects }) }
        : undefined;
      return withReservedSignalId(rawSignals, "input", id, () => wrapInputSignal(
        rawSignals.input(id, initial, inputOptions), rawSignals,
        cloneSignalValue(initial), debugName("input", specOptions),
      ));
    },
    computed(id, spec, options) {
      return namedSpec(rawSignals, "computed", id, spec, options);
    },
    computedCallback(id, callback, options) {
      const parsed = authoringOptions("computed", options);
      return withReservedSignalId(rawSignals, "computed", id, () => wrapReadableSignal(
        rawSignals.computedCallback(id, withComputedCallbackFrame(rawSignals, callback)),
        rawSignals, "computed", debugName("computed", parsed),
      ));
    },
    output(id, spec, options) {
      return namedSpec(rawSignals, "output", id, spec, options);
    },
    outputCallback(id, callback, options) {
      const parsed = authoringOptions("output", options);
      const hiddenId = nextOutputProjectionId(rawSignals, id);
      const wrapped = withComputedCallbackFrame(rawSignals, callback);
      return withReservedSignalId(rawSignals, "output", id, () => {
        rawSignals.computedCallback(hiddenId, wrapped);
        return wrapReadableSignal(
          rawSignals.outputSpec(id, outputProjectionSpec(hiddenId)),
          rawSignals, "output", debugName("output", parsed),
        );
      });
    },
  });
}

function namedSpec(rawSignals, family, id, spec, options) {
  const parsed = authoringOptions(family, options);
  return withReservedSignalId(rawSignals, family, id, () => createExplicitNamedSignal(
    rawSignals, family, id, spec, debugName(family, parsed),
  ));
}

function createExplicitNamedSignal(rawSignals, family, id, specOrCallback, label) {
  if (isPlainObject(specOrCallback) && typeof specOrCallback.compute === "function") {
    const callback = withComputedCallbackFrame(rawSignals, specOrCallback.compute);
    if (family === "computed") {
      return wrapReadableSignal(rawSignals.computedCallback(id, callback), rawSignals, family, label);
    }
    const hiddenId = nextOutputProjectionId(rawSignals, id);
    rawSignals.computedCallback(hiddenId, callback);
    return wrapReadableSignal(rawSignals.outputSpec(id, outputProjectionSpec(hiddenId)), rawSignals, family, label);
  }
  const raw = family === "computed"
    ? rawSignals.computedSpec(id, specOrCallback)
    : rawSignals.outputSpec(id, specOrCallback);
  return wrapReadableSignal(raw, rawSignals, family, label);
}

function authoringOptions(family, options) {
  return options === undefined ? undefined : requireAuthoringOptions(family, options);
}

function debugName(family, options) {
  return options ? requireOptionalDebugName(family, options) : null;
}
