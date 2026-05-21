import { isPlainObject } from "./authoring_option_validation.js";

function cloneLinkedValue(value) {
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
  if (value !== null && typeof value === "object") {
    return { ...value };
  }
  return value;
}

function requireLinkedDebugName(options) {
  if (!options || options.debugName === undefined) {
    return null;
  }
  if (typeof options.debugName !== "string" || options.debugName.length === 0) {
    throw new TypeError("signals.linked debugName must be a non-empty string when provided");
  }
  return options.debugName;
}

function forbidLinkedIdOption(options) {
  if (options && Object.prototype.hasOwnProperty.call(options, "id")) {
    throw new TypeError(
      "signals.linked app authoring does not accept id; use signals.spec.* when you need an explicit structural name",
    );
  }
}

export function normalizeLinkedDefinition(sourceOrDefinition, maybeOptions) {
  if (typeof sourceOrDefinition === "function") {
    if (
      maybeOptions !== undefined
      && (!isPlainObject(maybeOptions) || Array.isArray(maybeOptions))
    ) {
      throw new TypeError("signals.linked options must be an object when provided");
    }
    forbidLinkedIdOption(maybeOptions);
    return {
      source: sourceOrDefinition,
      computation(sourceValue) {
        return sourceValue;
      },
      debugName: requireLinkedDebugName(maybeOptions),
    };
  }

  if (!isPlainObject(sourceOrDefinition) || typeof sourceOrDefinition.source !== "function") {
    throw new TypeError(
      "signals.linked expects a source callback or a definition object with a source callback",
    );
  }
  if (maybeOptions !== undefined) {
    throw new TypeError("signals.linked definition form does not accept a second argument");
  }
  const computation = sourceOrDefinition.computation ?? ((sourceValue) => sourceValue);
  if (typeof computation !== "function") {
    throw new TypeError("signals.linked computation must be a function when provided");
  }
  forbidLinkedIdOption(sourceOrDefinition);
  return {
    source: sourceOrDefinition.source,
    computation,
    debugName: requireLinkedDebugName(sourceOrDefinition),
  };
}

export function createLinkedPrevious(value, source) {
  return {
    value: cloneLinkedValue(value),
    source: cloneLinkedValue(source),
  };
}

export function cloneLinkedSignalValue(value) {
  return cloneLinkedValue(value);
}
