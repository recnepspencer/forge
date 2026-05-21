import {
  registerInputResetPreparer,
  registerInputWriteObserver,
  registerSignalCleanup,
  writeInputBaselineValue,
} from "./handles.js";
import {
  cloneLinkedSignalValue as cloneSignalValue,
  createLinkedPrevious,
  normalizeLinkedDefinition,
} from "./linked_definition.js";

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
    const previous = createLinkedPrevious(latestBaselineValue, latestSourceValue);
    return {
      sourceValue: cloneSignalValue(nextSourceValue),
      nextValue: computation(nextSourceValue, previous),
    };
  }

  function computeRelinkValue(nextSourceValue) {
    const previous = createLinkedPrevious(latestLinkedValue, latestSourceValue);
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
