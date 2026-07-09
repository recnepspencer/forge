import { buildControllerContract, createControllerContract } from "./controllers.js";
import {
  cloneSignalValue,
  inputDescriptor,
  isGraphBuilder,
  mergeControllerContracts,
  mergePatchValue,
  normalizePublicInputEntry,
  publishHandle,
  requireBuilderExposure,
  requireCommandSupport,
  requireGraphBuilder,
  requireGraphDefinition,
  requireGraphId,
  requireGraphMutationAllowed,
  requireGraphOwnedHandle,
  requireInputsRecord,
  requireNoConflictingMutations,
  requireOperationObject,
  requireOptionalOperationRecord,
  requireOptionalResetList,
  requireOutputName,
  requireOutputsRecord,
  requireReadableHandle,
} from "./graph_authoring_support.js";
import {
  buildCompatibilityDefinition,
  buildGraphContractDelta,
  buildGraphContractHistory,
  buildGraphContractSurface,
  buildGraphDiagnosticsSurface,
  buildGraphExportDefinition,
  buildGraphExportSnapshot,
  buildGraphHistorySurface,
  buildGraphImportPosture,
  buildGraphOperationalContractSurface,
  requireKnownAuthority,
  requireKnownInput,
  requireKnownOutput,
} from "./graph_support.js";
import { prepareInputReset } from "./handles.js";
import { createPublicGraphInputEntry } from "./public_inputs.js";
import { createScopedSignalNamespace } from "./scopes.js";
import { GRAPH_EXPOSURE } from "./symbols.js";
import { wrapGraphTransaction } from "./transactions.js";

function normalizeBoundaryInputOptions(graphId, boundaryKind, options) {
  if (options && Object.prototype.hasOwnProperty.call(options, "requiredness")) {
    throw new TypeError(
      `signals.graph \`${graphId}\` input.${boundaryKind}(...) does not accept an explicit requiredness override; use input.${boundaryKind}(...) to choose the boundary contract form`,
    );
  }
  return options ?? {};
}

function createGraphConstructionSurface(signalsFacade, rawSignals, graphId) {
  const graphScope = createScopedSignalNamespace(signalsFacade, rawSignals, graphId, null, graphId);
  return Object.freeze({
    id: graphId,
    scope(localScopeId) {
      return graphScope.scope(localScopeId);
    },
    controller(localScopeIdOrDefinition, maybeBuilder) {
      if (typeof localScopeIdOrDefinition === "string") {
        if (typeof maybeBuilder !== "function") {
          throw new TypeError(
            `signals.graph \`${graphId}\` controller(scopeId, builder) requires a builder callback`,
          );
        }
        return buildControllerContract(graphScope.scope(localScopeIdOrDefinition), maybeBuilder);
      }
      return createControllerContract(localScopeIdOrDefinition);
    },
    publicInput(handle, options) {
      return createPublicGraphInputEntry(handle, options);
    },
    input: Object.freeze({
      required(handle, options) {
        return createPublicGraphInputEntry(handle, {
          ...normalizeBoundaryInputOptions(graphId, "required", options),
          requiredness: "required",
        });
      },
      optional(handle, options) {
        return createPublicGraphInputEntry(handle, {
          ...normalizeBoundaryInputOptions(graphId, "optional", options),
          requiredness: "optional",
        });
      },
    }),
    expose(definition) {
      const validated = requireGraphDefinition(definition);
      const merged = mergeControllerContracts(validated, graphId);
      return Object.freeze({
        ...merged,
        [GRAPH_EXPOSURE]: graphId,
      });
    },
  });
}

function resolveGraphDefinition(signalsFacade, rawSignals, graphId, definitionOrBuilder) {
  if (!isGraphBuilder(definitionOrBuilder)) {
    return {
      definition: requireGraphDefinition(definitionOrBuilder),
      graphOwned: false,
    };
  }
  const builder = requireGraphBuilder(definitionOrBuilder);
  return {
    definition: requireBuilderExposure(
      builder(createGraphConstructionSurface(signalsFacade, rawSignals, graphId)),
      graphId,
    ),
    graphOwned: true,
  };
}

export function createPublishedSignalGraph(signalsFacade, rawSignals, graphId, definitionOrBuilder) {
  requireGraphId(graphId);
  const { definition: validatedDefinition, graphOwned } = resolveGraphDefinition(
    signalsFacade,
    rawSignals,
    graphId,
    definitionOrBuilder,
  );
  const inputEntries = requireInputsRecord(validatedDefinition.inputs);
  const outputEntries = requireOutputsRecord(validatedDefinition.outputs);
  const inputs = Object.create(null);
  const outputs = Object.create(null);
  const inputDescriptors = [];
  const outputDescriptors = [];

  for (const [inputName, candidateHandle] of inputEntries) {
    requireOutputName(inputName);
    const normalizedInput = normalizePublicInputEntry(
      candidateHandle,
      rawSignals,
      graphId,
      inputName,
      graphOwned,
    );
    inputs[inputName] = normalizedInput.handle;
    inputDescriptors.push(
      inputDescriptor(
        inputName,
        normalizedInput.handle,
        normalizedInput.authority,
        normalizedInput.requiredness,
      ),
    );
  }

  for (const [outputName, candidateHandle] of outputEntries) {
    requireOutputName(outputName);
    const sourceHandle = graphOwned
      ? requireGraphOwnedHandle(candidateHandle, rawSignals, graphId, outputName, "output")
      : requireReadableHandle(candidateHandle, rawSignals, graphId, outputName);
    const published = publishHandle(rawSignals, graphId, outputName, sourceHandle);
    outputs[outputName] = published.handle;
    outputDescriptors.push(published.descriptor);
  }

  Object.freeze(inputs);
  Object.freeze(outputs);
  Object.freeze(inputDescriptors);
  Object.freeze(outputDescriptors);

  const inputNames = Object.freeze(inputDescriptors.map((descriptor) => descriptor.inputName));
  const inputSourceIds = Object.freeze(inputDescriptors.map((descriptor) => descriptor.sourceId));
  const outputNames = Object.freeze(outputDescriptors.map((descriptor) => descriptor.outputName));
  const publishedOutputIds = Object.freeze(outputDescriptors.map((descriptor) => descriptor.publishedId));
  const outputSourceIds = outputDescriptors.map((descriptor) => descriptor.sourceId);
  const sourceIds = Object.freeze([...new Set([...inputSourceIds, ...outputSourceIds])]);
  const graphSummary = Object.freeze({
    id: graphId,
    inputCount: inputDescriptors.length,
    inputNames,
    inputSourceIds,
    outputCount: outputDescriptors.length,
    outputNames,
    publishedOutputIds,
    sourceIds,
    synthesizedOutputCount: outputDescriptors.filter(
      (descriptor) => descriptor.publicationKind === "synthesizedOutput",
    ).length,
  });
  const graphContract = buildGraphContractSurface(graphSummary, inputDescriptors, outputDescriptors);
  const graphOperationalContract = buildGraphOperationalContractSurface(
    graphSummary,
    inputDescriptors,
    inputs,
  );
  const graphContractHistory = buildGraphContractHistory(graphContract);
  const graphImportPosture = buildGraphImportPosture(graphId);

  function applyGraphMutation(definition) {
    requireGraphMutationAllowed();
    const operation = requireOperationObject(definition, graphId);
    const writes = requireOptionalOperationRecord(operation.writes, graphId, "writes");
    const patches = requireOptionalOperationRecord(operation.patches, graphId, "patches");
    const resetNames = requireOptionalResetList(operation.reset, graphId);
    requireCommandSupport(operation.commands, graphId);
    requireNoConflictingMutations(graphId, writes, patches, resetNames);
    const plannedSets = [];
    const plannedResetFinalizers = [];

    for (const [inputName, nextValue] of Object.entries(writes)) {
      requireKnownInput(inputs, graphId, inputName);
      requireKnownAuthority(
        graphOperationalContract.authorities,
        graphId,
        inputName,
        "apply(...)",
        "supportsWrite",
      );
      plannedSets.push([inputName, nextValue]);
    }

    for (const [inputName, patchValue] of Object.entries(patches)) {
      const input = requireKnownInput(inputs, graphId, inputName);
      const authority = graphOperationalContract.authorities[inputName];
      if (!authority) {
        throw new TypeError(
          `signals.graph \`${graphId}\` patchInputs(...) cannot use unknown public input \`${inputName}\``,
        );
      }
      if (authority.authority !== "writable") {
        requireKnownAuthority(
          graphOperationalContract.authorities,
          graphId,
          inputName,
          "patchInputs(...)",
          "supportsPatch",
        );
      }
      if (!authority.supportsPatch) {
        throw new TypeError(
          `signals.graph \`${graphId}\` patchInputs cannot patch public input \`${inputName}\` because the graph operational contract does not admit patches for it`,
        );
      }
      plannedSets.push([
        inputName,
        mergePatchValue(graphId, inputName, input(), patchValue),
      ]);
    }

    for (const inputName of resetNames) {
      const input = requireKnownInput(inputs, graphId, inputName);
      requireKnownAuthority(
        graphOperationalContract.authorities,
        graphId,
        inputName,
        "apply(...)",
        "supportsReset",
      );
      const preparedReset = prepareInputReset(input);
      plannedSets.push([inputName, preparedReset.value]);
      plannedResetFinalizers.push(preparedReset.finalize);
    }

    const result = rawSignals.transaction((rawTx) => {
      const tx = wrapGraphTransaction(
        rawTx,
        rawSignals,
        graphId,
        inputs,
        graphOperationalContract.authorities,
      );
      for (const [inputName, nextValue] of plannedSets) {
        tx.set(inputName, cloneSignalValue(nextValue));
      }
    });
    for (const finalizeReset of plannedResetFinalizers) {
      finalizeReset();
    }
    return result;
  }

  return Object.freeze({
    id: graphId,
    inputs,
    outputs,
    contract() { return graphContract; },
    contractDelta(previousContract) { return buildGraphContractDelta(graphContract, previousContract); },
    contractHistory() { return graphContractHistory; },
    importPosture() { return graphImportPosture; },
    operationalContract() { return graphOperationalContract; },
    input(name) { return requireKnownInput(inputs, graphId, name); },
    output(name) { return requireKnownOutput(outputs, graphId, name); },
    readInputs() {
      const snapshot = Object.create(null);
      for (const inputName of inputNames) snapshot[inputName] = inputs[inputName]();
      return Object.freeze(snapshot);
    },
    writeInputs(nextInputs) { return applyGraphMutation({ writes: nextInputs }); },
    writeInput(inputName, nextValue) { return applyGraphMutation({ writes: { [inputName]: nextValue } }); },
    patchInputs(nextPatches) { return applyGraphMutation({ patches: nextPatches }); },
    patchInput(inputName, patchValue) { return applyGraphMutation({ patches: { [inputName]: patchValue } }); },
    resetInputs(inputNamesToReset = inputNames) { return applyGraphMutation({ reset: inputNamesToReset }); },
    resetInput(inputName) { return applyGraphMutation({ reset: [inputName] }); },
    apply(definition) { return applyGraphMutation(definition); },
    transaction(callback) {
      requireGraphMutationAllowed();
      if (typeof callback !== "function") {
        throw new TypeError(`signals.graph \`${graphId}\` transaction(...) requires a callback`);
      }
      return rawSignals.transaction((rawTx) => callback(
        wrapGraphTransaction(rawTx, rawSignals, graphId, inputs, graphOperationalContract.authorities),
      ));
    },
    transactionAsync(callback) {
      return Promise.resolve(this.transaction(callback));
    },
    batchAsync(callback) {
      return Promise.resolve(this.transaction(callback));
    },
    read() {
      const snapshot = Object.create(null);
      for (const outputName of outputNames) snapshot[outputName] = outputs[outputName]();
      return Object.freeze(snapshot);
    },
    why(name) {
      const output = requireKnownOutput(outputs, graphId, name);
      return signalsFacade.diagnostics().why(output.id);
    },
    replayFor(name) {
      const output = requireKnownOutput(outputs, graphId, name);
      return signalsFacade.history().replay_for(output.id);
    },
    lineageFor(name) {
      const output = requireKnownOutput(outputs, graphId, name);
      return signalsFacade.history().lineage_for(output.id);
    },
    readVersions() { return signalsFacade.specialist().readVersions(publishedOutputIds); },
    summary() { return graphSummary; },
    inputDescriptors() { return inputDescriptors; },
    descriptors() { return outputDescriptors; },
    inspectDiagnostics() {
      return buildGraphDiagnosticsSurface(
        signalsFacade,
        graphSummary,
        inputDescriptors,
        outputDescriptors,
        signalsFacade.adapters().exportDefinitions(),
      );
    },
    inspectHistory() {
      return buildGraphHistorySurface(
        signalsFacade,
        graphSummary,
        inputDescriptors,
        outputDescriptors,
        signalsFacade.adapters().exportDefinitions(),
      );
    },
    exportCompatibilityDefinition() {
      return buildCompatibilityDefinition(
        graphId,
        graphSummary,
        inputDescriptors,
        outputDescriptors,
        signalsFacade.adapters().exportDefinitions(),
      );
    },
    exportDefinition() {
      return buildGraphExportDefinition(
        graphId,
        graphSummary,
        graphContract,
        graphOperationalContract,
        inputDescriptors,
        outputDescriptors,
        signalsFacade.adapters().exportDefinitions(),
      );
    },
    exportSnapshot() { return buildGraphExportSnapshot(this.exportDefinition(), signalsFacade); },
    diagnostics() { return signalsFacade.diagnostics(); },
    history() { return signalsFacade.history(); },
    specialist() { return signalsFacade.specialist(); },
    adapters() { return signalsFacade.adapters(); },
    compatibilityApp() { return signalsFacade.compatibilityApp(); },
    compatibilityRuntime() { return signalsFacade.compatibilityRuntime(); },
  });
}
