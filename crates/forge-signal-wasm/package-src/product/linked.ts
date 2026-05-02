import {
  registerInputResetPreparer,
  registerInputWriteObserver,
  registerSignalCleanup,
  writeInputBaselineValue,
} from "./handles.js";

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

function normalizeLinkedDefinition(sourceOrDefinition, maybeOptions) {
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

function disposeResource(resource) {
  if (!resource) {
    return;
  }
  if (typeof resource === "function") {
    resource();
    return;
  }
  if (typeof resource[Symbol.dispose] === "function") {
    resource[Symbol.dispose]();
    return;
  }
  if (typeof resource.free === "function") {
    resource.free();
    return;
  }
  if (typeof resource.dispose === "function") {
    resource.dispose();
  }
}

export function createLinkedSignal(namespace, rawSignals, sourceOrDefinition, maybeOptions) {
  const { source, computation, debugName } = normalizeLinkedDefinition(
    sourceOrDefinition,
    maybeOptions,
  );
  const sourceHandle = namespace.computed(source);
  let latestSourceValue = cloneSignalValue(sourceHandle());
  const initialValue = computation(latestSourceValue, null);
  const linkedSignal = namespace.input(initialValue, debugName ? { debugName } : undefined);
  let latestLinkedValue = cloneSignalValue(initialValue);
  let latestBaselineValue = cloneSignalValue(initialValue);

  registerInputWriteObserver(linkedSignal, (nextValue) => {
    latestLinkedValue = cloneSignalValue(nextValue);
  });

  function computeLinkedBaseline(nextSourceValue) {
    const previous = {
      value: cloneSignalValue(latestBaselineValue),
      source: cloneSignalValue(latestSourceValue),
    };
    return {
      sourceValue: cloneSignalValue(nextSourceValue),
      nextValue: computation(nextSourceValue, previous),
    };
  }

  function computeRelinkValue(nextSourceValue) {
    const previous = {
      value: cloneSignalValue(latestLinkedValue),
      source: cloneSignalValue(latestSourceValue),
    };
    return {
      sourceValue: cloneSignalValue(nextSourceValue),
      nextValue: computation(nextSourceValue, previous),
    };
  }

  function applyLinkedValue(nextSourceValue) {
    const { sourceValue, nextValue } = computeRelinkValue(nextSourceValue);
    latestSourceValue = sourceValue;
    latestLinkedValue = cloneSignalValue(nextValue);
    latestBaselineValue = cloneSignalValue(nextValue);
    writeInputBaselineValue(linkedSignal, nextValue);
    return linkedSignal.set(nextValue);
  }

  function relink() {
    return applyLinkedValue(cloneSignalValue(sourceHandle()));
  }

  registerSignalCleanup(linkedSignal, () => {
    disposeResource(sourceHandle);
  });
  registerInputResetPreparer(linkedSignal, () => {
    const { sourceValue, nextValue } = computeLinkedBaseline(cloneSignalValue(sourceHandle()));
    return {
      value: nextValue,
      finalize() {
        latestSourceValue = sourceValue;
        latestLinkedValue = cloneSignalValue(nextValue);
        latestBaselineValue = cloneSignalValue(nextValue);
        writeInputBaselineValue(linkedSignal, nextValue);
      },
    };
  });

  Object.defineProperty(linkedSignal, "relink", {
    enumerable: false,
    value: relink,
  });

  return linkedSignal;
}
