import { createControllerContract, isControllerContract } from "./controllers.js";
import { createPublicGraphInputEntry, isPublicGraphInputEntry } from "./public_inputs.js";
import { wrapReadableSignal } from "./handles.js";
import {
  activeComputedCallbackFrame,
  denySignalMutationDuringCallbackAuthoring,
} from "./callback_frames.js";
import {
  GRAPH_EXPOSURE,
  GRAPH_OWNER_ID,
  INPUT_BASELINE_VALUE,
  PRODUCT_SIGNAL_KIND,
  RAW_SIGNAL_HANDLE,
  RAW_SIGNALS,
} from "./symbols.js";

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

export function requireGraphId(graphId) {
  if (typeof graphId !== "string" || graphId.length === 0) {
    throw new TypeError("signals.graph requires a non-empty string graph id");
  }
  return graphId;
}

export function requireGraphDefinition(definition) {
  if (!isPlainObject(definition)) {
    throw new TypeError("signals.graph requires a graph definition object");
  }
  return definition;
}

export function requireGraphBuilder(builder) {
  if (typeof builder !== "function") {
    throw new TypeError("signals.graph builder form requires a function");
  }
  return builder;
}

export function isGraphBuilder(candidate) {
  return typeof candidate === "function";
}

export function requireBuilderExposure(definition, graphId) {
  if (!isPlainObject(definition) || definition[GRAPH_EXPOSURE] !== graphId) {
    throw new TypeError(
      "signals.graph builder form must return the result of graph.expose(...) from the same graph boundary",
    );
  }
  return definition;
}

export function requireInputsRecord(inputs) {
  if (inputs === undefined) {
    return [];
  }
  if (!isPlainObject(inputs)) {
    throw new TypeError("signals.graph requires an inputs object when provided");
  }
  return Object.entries(inputs);
}

export function requireOutputsRecord(outputs) {
  if (!isPlainObject(outputs)) {
    throw new TypeError("signals.graph requires an outputs object");
  }
  const entries = Object.entries(outputs);
  if (entries.length === 0) {
    throw new TypeError("signals.graph requires at least one published output");
  }
  return entries;
}

export function requireOptionalOutputsRecord(outputs) {
  if (outputs === undefined) {
    return [];
  }
  if (!isPlainObject(outputs)) {
    throw new TypeError("graph.expose outputs must be an object when provided");
  }
  return Object.entries(outputs);
}

export function requireControllerCollection(controllers) {
  if (controllers === undefined) {
    return [];
  }
  if (!Array.isArray(controllers)) {
    throw new TypeError("graph.expose controllers must be an array when provided");
  }
  return controllers;
}

export function requireOutputName(name) {
  if (typeof name !== "string" || name.length === 0) {
    throw new TypeError("signals.graph output names must be non-empty strings");
  }
  return name;
}

function isReadableProductHandle(target) {
  return (
    target &&
    typeof target === "function" &&
    RAW_SIGNAL_HANDLE in target &&
    RAW_SIGNALS in target &&
    PRODUCT_SIGNAL_KIND in target
  );
}

export function requireReadableHandle(target, rawSignals, graphId, outputName) {
  if (!isReadableProductHandle(target)) {
    throw new TypeError(
      `signals.graph output \`${graphId}.${outputName}\` expects a product signal handle created by this package`,
    );
  }
  if (target[RAW_SIGNALS] !== rawSignals) {
    throw new TypeError(
      `signals.graph output \`${graphId}.${outputName}\` cannot use signal \`${target.id ?? "<unknown>"}\` from a different Signals runtime`,
    );
  }
  return target;
}

export function requireGraphOwnedHandle(
  target,
  rawSignals,
  graphId,
  outputName,
  contractFamily,
) {
  const handle = requireReadableHandle(target, rawSignals, graphId, outputName);
  if (handle[GRAPH_OWNER_ID] !== graphId) {
    throw new TypeError(
      `signals.graph ${contractFamily} \`${graphId}.${outputName}\` must come from graph-owned scope \`${graphId}\`, not from ambient runtime authoring`,
    );
  }
  return handle;
}

function requireInputHandle(target, rawSignals, graphId, inputName) {
  const handle = requireReadableHandle(target, rawSignals, graphId, inputName);
  if (handle[PRODUCT_SIGNAL_KIND] !== "input") {
    throw new TypeError(
      `signals.graph input \`${graphId}.${inputName}\` expects an input handle, but received \`${handle.id}\` (${handle[PRODUCT_SIGNAL_KIND]})`,
    );
  }
  return handle;
}

export function inputDescriptor(inputName, sourceHandle, authority) {
  return Object.freeze({
    inputName,
    sourceId: sourceHandle.id,
    sourceKind: "input",
    authority,
  });
}

export function publicationId(graphId, outputName) {
  return `${graphId}.${outputName}`;
}

export function readProjectionSpec(sourceId) {
  return {
    reads: [sourceId],
    expr: {
      kind: "read",
      id: sourceId,
    },
  };
}

export function publicationDescriptor(outputName, sourceHandle, publishedHandle, publicationKind) {
  return Object.freeze({
    outputName,
    sourceId: sourceHandle.id,
    sourceKind: sourceHandle[PRODUCT_SIGNAL_KIND],
    publishedId: publishedHandle.id,
    publicationKind,
  });
}

export function nullPrototypeRecord() {
  return Object.create(null);
}

export function cloneSignalValue(value) {
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

export function isPatchableSignalValue(value) {
  return isPlainObject(value) || Array.isArray(value);
}

export function normalizePublicInputEntry(candidateHandle, rawSignals, graphId, inputName, graphOwned) {
  if (isPublicGraphInputEntry(candidateHandle)) {
    const handle = requireInputHandle(candidateHandle.handle, rawSignals, graphId, inputName);
    if (graphOwned) {
      requireGraphOwnedHandle(handle, rawSignals, graphId, inputName, "input");
    }
    return createPublicGraphInputEntry(handle, { authority: candidateHandle.authority });
  }
  const handle = requireInputHandle(candidateHandle, rawSignals, graphId, inputName);
  if (graphOwned) {
    requireGraphOwnedHandle(handle, rawSignals, graphId, inputName, "input");
  }
  return createPublicGraphInputEntry(handle);
}

export function requireOperationObject(definition, graphId) {
  if (!isPlainObject(definition)) {
    throw new TypeError(`signals.graph \`${graphId}\` input operations must be expressed as an object`);
  }
  return definition;
}

export function requireOptionalOperationRecord(value, graphId, fieldName) {
  if (value === undefined) {
    return nullPrototypeRecord();
  }
  if (!isPlainObject(value)) {
    throw new TypeError(`signals.graph \`${graphId}\` ${fieldName} must be an object when provided`);
  }
  return value;
}

export function requireOptionalResetList(value, graphId) {
  if (value === undefined) {
    return [];
  }
  if (!Array.isArray(value) || !value.every((entry) => typeof entry === "string" && entry.length > 0)) {
    throw new TypeError(`signals.graph \`${graphId}\` reset must be an array of public input names when provided`);
  }
  return value;
}

export function requireCommandSupport(commands, graphId) {
  if (commands === undefined) {
    return nullPrototypeRecord();
  }
  if (!isPlainObject(commands) || Object.keys(commands).length > 0) {
    throw new TypeError(
      `signals.graph \`${graphId}\` commands are not supported yet; use writes, patches, or reset only`,
    );
  }
  return commands;
}

export function mergePatchValue(graphId, inputName, currentValue, patchValue) {
  if (!isPatchableSignalValue(currentValue) || !isPatchableSignalValue(patchValue)) {
    throw new TypeError(
      `signals.graph \`${graphId}\` patch for public input \`${inputName}\` requires object or array values`,
    );
  }
  if (Array.isArray(currentValue) || Array.isArray(patchValue)) {
    return cloneSignalValue(patchValue);
  }
  return {
    ...currentValue,
    ...patchValue,
  };
}

export function requireNoConflictingMutations(graphId, writes, patches, resetNames) {
  const touchedByWrite = new Set(Object.keys(writes));
  const touchedByPatch = new Set(Object.keys(patches));
  const touchedByReset = new Set(resetNames);
  for (const inputName of touchedByWrite) {
    if (touchedByPatch.has(inputName)) {
      throw new TypeError(
        `signals.graph \`${graphId}\` cannot both write and patch public input \`${inputName}\` in the same graph mutation request`,
      );
    }
    if (touchedByReset.has(inputName)) {
      throw new TypeError(
        `signals.graph \`${graphId}\` cannot both write and reset public input \`${inputName}\` in the same graph mutation request`,
      );
    }
  }
  for (const inputName of touchedByPatch) {
    if (touchedByReset.has(inputName)) {
      throw new TypeError(
        `signals.graph \`${graphId}\` cannot both patch and reset public input \`${inputName}\` in the same graph mutation request`,
      );
    }
  }
}

export function requireGraphMutationAllowed() {
  if (activeComputedCallbackFrame()) {
    denySignalMutationDuringCallbackAuthoring();
  }
}

export function mergeControllerContracts(definition, graphId) {
  const controllers = requireControllerCollection(definition.controllers);
  const mergedInputs = nullPrototypeRecord();
  const mergedOutputs = nullPrototypeRecord();
  for (const controller of controllers) {
    if (!isControllerContract(controller)) {
      throw new TypeError(
        `graph.expose controllers for \`${graphId}\` must be a controller artifact created by signals.controller(...) or scope.controller(...)`,
      );
    }
    for (const [inputName, inputHandle] of Object.entries(controller.inputs)) {
      if (inputName in mergedInputs) {
        throw new TypeError(
          `graph.expose controllers for \`${graphId}\` cannot publish duplicate public input name \`${inputName}\``,
        );
      }
      mergedInputs[inputName] = inputHandle;
    }
    for (const [outputName, outputHandle] of Object.entries(controller.outputs)) {
      if (outputName in mergedOutputs) {
        throw new TypeError(
          `graph.expose controllers for \`${graphId}\` cannot publish duplicate public output name \`${outputName}\``,
        );
      }
      mergedOutputs[outputName] = outputHandle;
    }
  }

  const explicitInputs = requireOperationObject(definition.inputs ?? {}, graphId);
  const explicitOutputs = requireOperationObject(definition.outputs ?? {}, graphId);
  for (const inputName of Object.keys(explicitInputs)) {
    if (inputName in mergedInputs) {
      throw new TypeError(
        `graph.expose controllers for \`${graphId}\` cannot publish duplicate input name \`${inputName}\``,
      );
    }
  }
  for (const outputName of Object.keys(explicitOutputs)) {
    if (outputName in mergedOutputs) {
      throw new TypeError(
        `graph.expose controllers for \`${graphId}\` cannot publish duplicate output name \`${outputName}\``,
      );
    }
  }

  return {
    inputs: {
      ...mergedInputs,
      ...explicitInputs,
    },
    outputs: {
      ...mergedOutputs,
      ...explicitOutputs,
    },
  };
}

export function publishHandle(rawSignals, graphId, outputName, sourceHandle) {
  if (sourceHandle[PRODUCT_SIGNAL_KIND] === "output") {
    return {
      handle: sourceHandle,
      descriptor: publicationDescriptor(
        outputName,
        sourceHandle,
        sourceHandle,
        "existingOutput",
      ),
    };
  }
  const publishedId = publicationId(graphId, outputName);
  const publishedRawHandle = rawSignals.outputSpec(
    publishedId,
    readProjectionSpec(sourceHandle.id),
  );
  const publishedHandle = wrapReadableSignal(publishedRawHandle, rawSignals, "output");
  return {
    handle: publishedHandle,
    descriptor: publicationDescriptor(
      outputName,
      sourceHandle,
      publishedHandle,
      "synthesizedOutput",
    ),
  };
}
