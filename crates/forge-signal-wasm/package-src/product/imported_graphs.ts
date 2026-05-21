import { nullPrototypeRecord } from "./graph_authoring_support.js";
import {
  buildCompatibilityDefinition,
  buildGraphContractHistory,
  buildGraphDiagnosticsSurface,
  buildGraphExportDefinition,
  buildGraphExportSnapshot,
  buildGraphHistorySurface,
  buildGraphImportPosture,
  createImportedReadableSignal,
  freezeObject,
  requireGraphExportDefinition,
  requireGraphExportSnapshot,
  requireKnownInput,
  requireKnownOutput,
  requireMatchingGraphExportPair,
} from "./graph_support.js";
import {
  buildImportedGraphMutationContext,
  buildImportedGraphOperationalContract,
  planImportedGraphMutation,
} from "./imported_graph_surface_support.js";

export function createImportedSignalGraph(signalsFacade, rawSignals, exportedDefinition, exportedSnapshot) {
  const definition = requireGraphExportDefinition(exportedDefinition);
  const snapshot = requireGraphExportSnapshot(exportedSnapshot, definition.id);
  requireMatchingGraphExportPair(definition, snapshot);
  signalsFacade.adapters().restoreExactRuntimeEnvelope(snapshot.runtimeEnvelope);

  const graphId = definition.id;
  const graphSummary = freezeObject(definition.summary);
  const graphContract = freezeObject(definition.contract);
  const inputDescriptors = freezeObject(definition.inputDescriptors);
  const outputDescriptors = freezeObject(definition.descriptors);
  const graphOperationalContract = buildImportedGraphOperationalContract(definition);
  const mutationContext = buildImportedGraphMutationContext(definition, snapshot);
  const graphContractHistory = buildGraphContractHistory(
    graphContract,
    snapshot.contractHistory?.baseline ?? definition.contract,
    snapshot.restoreMode ?? "SameRuntimeExact",
  );
  const graphImportPosture = buildGraphImportPosture(
    graphId,
    snapshot.importPosture?.exactRestoreMode ?? snapshot.restoreMode ?? "SameRuntimeExact",
  );

  const inputs = nullPrototypeRecord();
  for (const descriptor of inputDescriptors) {
    const inputName = descriptor.inputName;
    inputs[descriptor.inputName] = createImportedInputSignal(
      rawSignals,
      descriptor.sourceId,
      (value) => applyImportedGraphMutation({ writes: { [inputName]: value } }),
      () => applyImportedGraphMutation({ reset: [inputName] }),
      (patch) => applyImportedGraphMutation({ patches: { [inputName]: patch } }),
    );
  }

  const outputs = nullPrototypeRecord();
  for (const descriptor of outputDescriptors) {
    outputs[descriptor.outputName] = createImportedReadableSignal(rawSignals, descriptor.publishedId);
  }

  function applyImportedGraphMutation(mutation) {
    const transactionOps = planImportedGraphMutation({
      label: `signals.importGraph \`${graphId}\``,
      graphId,
      inputDescriptorsByName: mutationContext.inputDescriptorsByName,
      inputAuthoritiesByName: mutationContext.inputAuthoritiesByName,
      initialValuesBySourceId: mutationContext.initialValuesBySourceId,
      readCurrentValue(sourceId) {
        return rawSignals.read(sourceId);
      },
      mutation,
    });
    return rawSignals.compatibilityRuntime().transaction(transactionOps);
  }

  return freezeObject({
    id: graphId,
    inputs: freezeObject(inputs),
    outputs: freezeObject(outputs),
    ready() {
      return Promise.resolve();
    },
    contract() { return graphContract; },
    contractHistory() { return graphContractHistory; },
    importPosture() { return graphImportPosture; },
    operationalContract() { return graphOperationalContract; },
    input(name) { return requireKnownInput(inputs, graphId, name); },
    output(name) { return requireKnownOutput(outputs, graphId, name); },
    readInputs() {
      const snapshotRecord = nullPrototypeRecord();
      for (const descriptor of inputDescriptors) {
        snapshotRecord[descriptor.inputName] = inputs[descriptor.inputName]();
      }
      return freezeObject(snapshotRecord);
    },
    read() {
      const snapshotRecord = nullPrototypeRecord();
      for (const descriptor of outputDescriptors) {
        snapshotRecord[descriptor.outputName] = outputs[descriptor.outputName]();
      }
      return freezeObject(snapshotRecord);
    },
    async writeInputs(values) { return applyImportedGraphMutation({ writes: values }); },
    async writeInput(name, value) { return applyImportedGraphMutation({ writes: { [name]: value } }); },
    async patchInputs(patches) { return applyImportedGraphMutation({ patches }); },
    async patchInput(name, patch) { return applyImportedGraphMutation({ patches: { [name]: patch } }); },
    async resetInputs(inputNames = mutationContext.defaultResetInputNames) { return applyImportedGraphMutation({ reset: inputNames }); },
    async resetInput(name) { return applyImportedGraphMutation({ reset: [name] }); },
    async apply(mutation) { return applyImportedGraphMutation(mutation); },
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
    terminate() {},
  });
}

function createImportedInputSignal(rawSignals, sourceId, write, reset, patch) {
  const signal = createImportedReadableSignal(rawSignals, sourceId);
  Object.defineProperty(signal, "set", {
    enumerable: false,
    value(nextValue) {
      return write(nextValue);
    },
  });
  Object.defineProperty(signal, "reset", {
    enumerable: false,
    value() {
      return reset();
    },
  });
  Object.defineProperty(signal, "patch", {
    enumerable: false,
    value(patchValue) {
      return patch(patchValue);
    },
  });
  Object.defineProperty(signal, "assign", {
    enumerable: false,
    value(assignedFields) {
      return patch(assignedFields);
    },
  });
  return signal;
}
