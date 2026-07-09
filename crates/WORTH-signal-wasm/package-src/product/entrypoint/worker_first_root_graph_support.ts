import {
  buildGraphContractSurface,
  buildGraphOperationalContractSurface,
  freezeObject,
} from "../graph_support.js";
import { inputDescriptor } from "../graph_authoring_support.js";
import { CONTROLLER_CONTRACT, GRAPH_EXPOSURE, INPUT_BASELINE_VALUE, PUBLIC_GRAPH_INPUT } from "../symbols.js";
import { createWorkerFirstScopedNamespace } from "./worker_first_authoring_namespace.js";

export function resolveWorkerFirstRootGraphDefinition(rootSession, path, graphId, definitionOrBuilder) {
  if (typeof definitionOrBuilder !== "function") {
    return requireGraphDefinitionObject(definitionOrBuilder, graphId);
  }
  const graphScope = createWorkerFirstScopedNamespace(rootSession, [...path, graphId]);
  const graphSurface = freezeObject({
    id: graphId,
    scope(localScopeId) {
      return graphScope.scope(localScopeId);
    },
    controller(localScopeIdOrDefinition, maybeBuilder) {
      if (typeof localScopeIdOrDefinition === "string") {
        return graphScope.scope(localScopeIdOrDefinition).controller(maybeBuilder);
      }
      return graphScope.controller(localScopeIdOrDefinition);
    },
    publicInput(handle, options) {
      return graphScope.publicInput(handle, options);
    },
    input: freezeObject({
      required(handle, options) {
        return graphScope.publicInput(handle, { ...options, requiredness: "required" });
      },
      optional(handle, options) {
        return graphScope.publicInput(handle, { ...options, requiredness: "optional" });
      },
    }),
    expose(definition) {
      return freezeObject({
        ...mergeGraphExposure(definition, graphId),
        [GRAPH_EXPOSURE]: graphId,
      });
    },
  });
  const built = definitionOrBuilder(graphSurface);
  if (!built || built[GRAPH_EXPOSURE] !== graphId) {
    throw new TypeError(
      "signals.graph builder form must return the result of graph.expose(...) from the same graph boundary",
    );
  }
  return built;
}

export function normalizeWorkerFirstRootGraphDefinition(context, graphId, definition) {
  const inputDescriptors = [];
  const outputDescriptors = [];
  const inputs = Object.create(null);
  const outputs = Object.create(null);
  const inputDescriptorsByName = Object.create(null);
  const inputDescriptorById = new Map(
    context.definition.inputDescriptors.map((descriptor) => [descriptor.sourceId, descriptor]),
  );
  const outputDescriptorById = new Map(
    context.definition.descriptors.map((descriptor) => [descriptor.publishedId, descriptor]),
  );
  for (const [inputName, candidate] of Object.entries(requireInputsRecord(definition.inputs))) {
    const entry = normalizeGraphInputEntry(context, candidate, inputName, inputDescriptorById);
    inputs[inputName] = attachBaselineValue(entry.handle);
    inputDescriptors.push(entry.descriptor);
    inputDescriptorsByName[inputName] = entry.descriptor;
  }
  for (const [outputName, handle] of Object.entries(requireOutputsRecord(definition.outputs, graphId))) {
    const descriptor = normalizeGraphOutputDescriptor(outputDescriptorById, handle, outputName, graphId);
    outputs[outputName] = handle;
    outputDescriptors.push(descriptor);
  }
  const graphSummary = freezeObject({
    id: graphId,
    inputCount: inputDescriptors.length,
    inputNames: freezeObject(inputDescriptors.map((descriptor) => descriptor.inputName)),
    inputSourceIds: freezeObject(inputDescriptors.map((descriptor) => descriptor.sourceId)),
    outputCount: outputDescriptors.length,
    outputNames: freezeObject(outputDescriptors.map((descriptor) => descriptor.outputName)),
    publishedOutputIds: freezeObject(outputDescriptors.map((descriptor) => descriptor.publishedId)),
    sourceIds: freezeObject([
      ...new Set([
        ...inputDescriptors.map((descriptor) => descriptor.sourceId),
        ...outputDescriptors.map((descriptor) => descriptor.sourceId),
      ]),
    ]),
    synthesizedOutputCount: 0,
  });
  const operationalContract = buildGraphOperationalContractSurface(
    graphSummary,
    freezeObject(inputDescriptors),
    inputs,
  );
  return freezeObject({
    graphSummary,
    contract: buildGraphContractSurface(
      graphSummary,
      freezeObject(inputDescriptors),
      freezeObject(outputDescriptors),
    ),
    operationalContract,
    inputDescriptors: freezeObject(inputDescriptors),
    outputDescriptors: freezeObject(outputDescriptors),
    inputs,
    outputs,
    mutationContext: freezeObject({
      inputDescriptorsByName: freezeObject(inputDescriptorsByName),
      inputDescriptorById: new Map(inputDescriptors.map((descriptor) => [descriptor.sourceId, descriptor])),
      inputAuthoritiesByName: graphSummary.inputNames.reduce((record, inputName) => {
        record[inputName] = operationalContract.authorities[inputName];
        return record;
      }, Object.create(null)),
      initialValuesBySourceId: new Map(
        inputDescriptors.map((descriptor) => [descriptor.sourceId, inputs[descriptor.inputName][INPUT_BASELINE_VALUE]]),
      ),
    }),
  });
}

function normalizeGraphInputEntry(context, candidate, inputName, descriptorById) {
  const entry = candidate?.[PUBLIC_GRAPH_INPUT] === true
    ? candidate
    : { handle: candidate, authority: "writable", requiredness: "required" };
  const sourceDescriptor = descriptorById.get(entry.handle?.id);
  if (!sourceDescriptor || typeof entry.handle?.set !== "function") {
    throw new TypeError(`signals.graph input \`${inputName}\` expects an input handle from the active imported graph`);
  }
  const sourceAuthority =
    context.definition.operationalContract?.authorities?.[sourceDescriptor.inputName];
  const authority = entry.authority ?? "writable";
  const requiredness = entry.requiredness ?? "required";
  if (authority !== "writable" && authority !== "readOnly" && authority !== "imported") {
    throw new TypeError(
      `signals.graph input \`${inputName}\` authority must be "writable", "readOnly", or "imported"`,
    );
  }
  if (requiredness !== "required" && requiredness !== "optional") {
    throw new TypeError(
      `signals.graph input \`${inputName}\` requiredness must be "required" or "optional"`,
    );
  }
  return freezeObject({
    handle: entry.handle,
    descriptor: inputDescriptor(inputName, entry.handle, authority, requiredness),
    sourceAuthority,
  });
}

function normalizeGraphOutputDescriptor(descriptorById, handle, outputName, graphId) {
  const source = descriptorById.get(handle?.id);
  if (!source) {
    throw new TypeError(
      `signals.graph \`${graphId}\` outputs.\`${outputName}\` must be an active imported-graph published output handle`,
    );
  }
  return freezeObject({
    outputName,
    sourceId: source.sourceId,
    sourceKind: source.sourceKind,
    publishedId: source.publishedId,
    publicationKind: "existingOutput",
  });
}

function mergeGraphExposure(definition, graphId) {
  const mergedInputs = Object.create(null);
  const mergedOutputs = Object.create(null);
  for (const controller of definition?.controllers ?? []) {
    if (!controller || controller[CONTROLLER_CONTRACT] !== true) {
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
  for (const inputName of Object.keys(definition?.inputs ?? {})) {
    if (inputName in mergedInputs) {
      throw new TypeError(
        `graph.expose controllers for \`${graphId}\` cannot publish duplicate input name \`${inputName}\``,
      );
    }
  }
  for (const outputName of Object.keys(definition?.outputs ?? {})) {
    if (outputName in mergedOutputs) {
      throw new TypeError(
        `graph.expose controllers for \`${graphId}\` cannot publish duplicate output name \`${outputName}\``,
      );
    }
  }
  return {
    inputs: { ...mergedInputs, ...(definition?.inputs ?? {}) },
    outputs: { ...mergedOutputs, ...(definition?.outputs ?? {}) },
  };
}

function attachBaselineValue(handle) {
  if (typeof handle !== "function") {
    return handle;
  }
  const wrapper = function workerFirstRootGraphInput() {
    return handle();
  };
  Object.assign(wrapper, handle);
  wrapper[INPUT_BASELINE_VALUE] = handle();
  return freezeObject(wrapper);
}

function requireGraphDefinitionObject(definition, graphId) {
  if (!definition || typeof definition !== "object" || Array.isArray(definition)) {
    throw new TypeError(`signals.graph \`${graphId}\` requires a graph definition object`);
  }
  return definition;
}

function requireInputsRecord(record) {
  if (record === undefined) {
    return Object.create(null);
  }
  if (!record || typeof record !== "object" || Array.isArray(record)) {
    throw new TypeError("signals.graph inputs must be an object when provided");
  }
  return record;
}

function requireOutputsRecord(record, graphId) {
  if (!record || typeof record !== "object" || Array.isArray(record)) {
    throw new TypeError("signals.graph outputs must be an object");
  }
  if (Object.keys(record).length === 0) {
    throw new TypeError(`signals.graph \`${graphId}\` requires at least one published output`);
  }
  return record;
}
