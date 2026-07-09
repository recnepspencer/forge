import {
  buildCompatibilityDefinition,
  buildGraphExportDefinition,
  freezeObject,
  requireKnownInput,
  requireKnownOutput,
} from "../graph_support.js";
import {
  inspectWorkerFirstGraphDiagnostics,
  inspectWorkerFirstGraphHistory,
} from "./sessions/support/worker_first_graph_inspection.js";
import {
  buildImportedGraphHydrationTransaction,
  buildImportedInputSignalRecord,
  buildWorkerPendingImportPosture,
  buildImportedSignalRecord,
  buildWorkerHydratedImportPosture,
  normalizeImportedGraphSessionOptions,
} from "./sessions/support/worker_first_imported_graph_support.js";
import {
  buildImportedGraphMutationContext,
  buildImportedGraphOperationalContract,
  buildImportedGraphSnapshotArtifact,
  planImportedGraphMutation,
} from "../imported_graph_surface_support.js";
import { materializeWorkerCachedValue } from "./sessions/support/worker_cached_value.js";
import { createWorkerRuntimeBridge } from "./bridge/worker_runtime_bridge.js";
import { normalizeWorkerRuntimeEnvelope } from "./bridge/worker_runtime_envelope_normalization.js";

export async function createWorkerFirstImportedGraphSession(options) {
  const graph = createPendingWorkerFirstImportedGraphSession(options);
  await graph.ready();
  return graph;
}

export function createPendingWorkerFirstImportedGraphSession(options) {
  return new WorkerFirstImportedGraphSession(options).api();
}

class WorkerFirstImportedGraphSession {
  #bridge;
  #definition;
  #snapshot;
  #graphId;
  #inputDescriptors;
  #outputDescriptors;
  #operationalContract;
  #mutationContext;
  #trackedInputIds;
  #trackedOutputIds;
  #inputs;
  #outputs;
  #cachedInputs;
  #cachedOutputs;
  #cachedDiagnosticsSummary;
  #cachedDiagnosticsHistory;
  #runtimeEnvelopeArtifact;
  #snapshotEnvelope;
  #ready;
  #readyPromise;
  #readyError;
  #terminated;

  constructor(options = {}) {
    const normalized = normalizeImportedGraphSessionOptions(options);
    this.#bridge = createWorkerRuntimeBridge(
      normalized.workerUrl === null ? {} : { workerUrl: normalized.workerUrl },
    );
    this.#definition = normalized.definition;
    this.#snapshot = normalized.snapshot;
    this.#graphId = normalized.definition.id;
    this.#inputDescriptors = normalized.definition.inputDescriptors;
    this.#outputDescriptors = normalized.definition.descriptors;
    this.#operationalContract = buildImportedGraphOperationalContract(normalized.definition);
    this.#mutationContext = buildImportedGraphMutationContext(
      normalized.definition,
      normalized.snapshot,
    );
    this.#trackedInputIds = normalized.trackedInputIds;
    this.#trackedOutputIds = normalized.trackedOutputIds;
    this.#cachedInputs = new Map();
    this.#cachedOutputs = new Map();
    this.#cachedDiagnosticsSummary = null;
    this.#cachedDiagnosticsHistory = null;
    this.#runtimeEnvelopeArtifact = null;
    this.#snapshotEnvelope = null;
    this.#ready = false;
    this.#readyError = null;
    this.#terminated = false;
    this.#inputs = buildImportedInputSignalRecord(
      this,
      this.#inputDescriptors,
      "inputName",
      (descriptor) => descriptor.sourceId,
      (descriptor) => this.#readCachedInput(descriptor.sourceId),
      (mutation) => this.apply(mutation),
    );
    this.#outputs = buildImportedSignalRecord(
      this,
      this.#outputDescriptors,
      "outputName",
      (descriptor) => descriptor.publishedId,
      (descriptor) => this.#readCachedOutput(descriptor.publishedId),
    );
    this.#readyPromise = this.#initialize().catch((error) => {
      this.#readyError = error;
      throw error;
    });
  }

  api() {
    return freezeObject({
      id: this.#graphId,
      inputs: this.#inputs,
      outputs: this.#outputs,
      ready: () => this.ready(),
      contract: () => this.#definition.contract,
      contractHistory: () => this.#snapshot.contractHistory,
      importPosture: () => this.importPosture(),
      operationalContract: () => this.#operationalContract,
      input: (name) => this.input(name),
      output: (name) => this.output(name),
      read: () => this.read(),
      readInputs: () => this.readInputs(),
      writeInputs: (values) => this.writeInputs(values),
      writeInput: (name, value) => this.writeInput(name, value),
      patchInputs: (patches) => this.patchInputs(patches),
      patchInput: (name, patch) => this.patchInput(name, patch),
      resetInputs: (names) => this.resetInputs(names),
      resetInput: (name) => this.resetInput(name),
      apply: (mutation) => this.apply(mutation),
      summary: () => this.#definition.summary,
      inputDescriptors: () => this.#inputDescriptors,
      descriptors: () => this.#outputDescriptors,
      inspectDiagnostics: () => this.inspectDiagnostics(),
      inspectHistory: () => this.inspectHistory(),
      exportCompatibilityDefinition: () => this.exportCompatibilityDefinition(),
      exportDefinition: () => this.exportDefinition(),
      exportSnapshot: () => this.exportSnapshot(),
      terminate: () => this.terminate(),
    });
  }

  ready() { return this.#readyPromise; }
  input(name) { this.#requireActive("input"); return requireKnownInput(this.#inputs, this.#graphId, name); }
  output(name) { this.#requireActive("output"); return requireKnownOutput(this.#outputs, this.#graphId, name); }

  readInputs() {
    this.#requireHydrated("readInputs");
    const snapshot = Object.create(null);
    for (const descriptor of this.#inputDescriptors) {
      snapshot[descriptor.inputName] = this.#readCachedInput(descriptor.sourceId);
    }
    return freezeObject(snapshot);
  }

  read() {
    this.#requireHydrated("read");
    const snapshot = Object.create(null);
    for (const descriptor of this.#outputDescriptors) {
      snapshot[descriptor.outputName] = this.#readCachedOutput(descriptor.publishedId);
    }
    return freezeObject(snapshot);
  }

  async writeInputs(values) { return this.apply({ writes: values }); }
  async writeInput(name, value) { return this.apply({ writes: { [name]: value } }); }
  async patchInputs(patches) { return this.apply({ patches }); }
  async patchInput(name, patch) { return this.apply({ patches: { [name]: patch } }); }
  async resetInputs(names = this.#mutationContext.defaultResetInputNames) { return this.apply({ reset: names }); }
  async resetInput(name) { return this.apply({ reset: [name] }); }

  async apply(mutation) {
    await this.ready();
    this.#requireActive("apply");
    const transactionOps = planImportedGraphMutation({
      label: `worker-first imported graph ${this.#graphId}`,
      graphId: this.#graphId,
      inputDescriptorsByName: this.#mutationContext.inputDescriptorsByName,
      inputAuthoritiesByName: this.#mutationContext.inputAuthoritiesByName,
      initialValuesBySourceId: this.#mutationContext.initialValuesBySourceId,
      readCurrentValue: (sourceId) => this.#readCachedInput(sourceId),
      mutation,
    });
    const projectionPacket = await this.#bridge.applyTransactionProjection({
      transactionOps,
      outputIds: this.#trackedOutputIds,
    });
    await this.#refreshCaches();
    return projectionPacket.transaction.runSummary;
  }

  async inspectDiagnostics() {
    await this.ready();
    this.#requireActive("inspectDiagnostics");
    return inspectWorkerFirstGraphDiagnostics(this.#inspectionContext());
  }

  async inspectHistory() {
    await this.ready();
    this.#requireActive("inspectHistory");
    return inspectWorkerFirstGraphHistory(this.#inspectionContext());
  }

  exportCompatibilityDefinition() {
    this.#requireActive("exportCompatibilityDefinition");
    return buildCompatibilityDefinition(
      this.#graphId,
      this.#definition.summary,
      this.#inputDescriptors,
      this.#outputDescriptors,
      this.#definition.compatibility.definitions,
    );
  }

  exportDefinition() {
    this.#requireActive("exportDefinition");
    return buildGraphExportDefinition(
      this.#graphId,
      this.#definition.summary,
      this.#definition.contract,
      this.#operationalContract,
      this.#inputDescriptors,
      this.#outputDescriptors,
      this.#definition.compatibility.definitions,
    );
  }

  exportSnapshot() {
    this.#requireActive("exportSnapshot");
    this.#requireHydrated("exportSnapshot");
    if (this.#runtimeEnvelopeArtifact === null || this.#snapshotEnvelope === null) {
      throw new TypeError(
        `worker-first imported graph ${this.#graphId} exportSnapshot() requires cached worker-owned runtime artifacts`,
      );
    }
    return buildImportedGraphSnapshotArtifact({
      definition: this.exportDefinition(),
      runtimeEnvelope: this.#runtimeEnvelopeArtifact,
      snapshotEnvelope: this.#snapshotEnvelope,
      restoreMode: this.#runtimeEnvelopeArtifact.runtimeEnvelopeRestoreMode,
      contractHistory: this.#snapshot.contractHistory,
      importPosture: this.importPosture(),
    });
  }

  importPosture() {
    this.#requireActive("importPosture");
    return this.#ready
      ? buildWorkerHydratedImportPosture(this.#snapshot, this.#graphId)
      : buildWorkerPendingImportPosture(this.#snapshot, this.#graphId);
  }

  async terminate() {
    if (this.#terminated) {
      return;
    }
    this.#terminated = true;
    this.#cachedInputs.clear();
    this.#cachedOutputs.clear();
    this.#cachedDiagnosticsSummary = null;
    this.#cachedDiagnosticsHistory = null;
    await this.#bridge.terminate();
  }

  diagnosticsSummary() {
    this.#requireHydrated("diagnosticsSummary");
    return this.#cachedDiagnosticsSummary;
  }

  diagnosticsHistory() {
    this.#requireHydrated("diagnosticsHistory");
    return this.#cachedDiagnosticsHistory;
  }

  async #initialize() {
    try {
      await this.#bridge.bootstrapRecord();
      await this.#bridge.workerRuntimeShellLock();
      await this.#bridge.publishPortableGraph({
        ...this.#definition.compatibility.definitions,
        outputIds: this.#trackedOutputIds,
      });
      await this.#bridge.applyTransaction(
        buildImportedGraphHydrationTransaction(
          this.#snapshot,
          this.#trackedInputIds,
          this.#graphId,
        ),
      );
      await this.#refreshCaches();
      this.#ready = true;
    } catch (error) {
      if (!this.#terminated) {
        await this.#bridge.terminate().catch(() => {});
        this.#terminated = true;
      }
      throw error;
    }
  }

  async #refreshDiagnosticsCache() {
    const [summaryPacket, historyPacket] = await Promise.all([
      this.#bridge.readDiagnosticsSummary(),
      this.#bridge.readDiagnosticsHistory(),
    ]);
    this.#cachedDiagnosticsSummary = materializeWorkerCachedValue(summaryPacket.summary);
    this.#cachedDiagnosticsHistory = materializeWorkerCachedValue(historyPacket.history);
  }

  async #refreshCaches() {
    const [inputPacket, outputPacket] = await Promise.all([
      this.#bridge.readSignals({ signalIds: this.#trackedInputIds }),
      this.#bridge.deliverOutputs({ outputIds: this.#trackedOutputIds }),
      this.#refreshDiagnosticsCache(),
      this.#refreshRuntimeArtifacts(),
    ]);
    this.#cachedInputs.clear();
    for (const signal of inputPacket.signals) {
      this.#cachedInputs.set(signal.id, materializeWorkerCachedValue(signal.value));
    }
    this.#cachedOutputs.clear();
    for (const output of outputPacket.outputs) {
      this.#cachedOutputs.set(output.id, materializeWorkerCachedValue(output.value));
    }
  }

  async #refreshRuntimeArtifacts() {
    const [runtimeEnvelope, runtimeEnvelopeRestoreToken, runtimeEnvelopePortableWire, snapshotEnvelope] =
      await Promise.all([
        this.#bridge.exportWorkerRuntimeEnvelope(),
        this.#bridge.exportWorkerRuntimeEnvelopeWire(),
        this.#bridge.exportWorkerRuntimeEnvelopePortableWire(),
        this.#bridge.exportWorkerSnapshotEnvelope(),
      ]);
    const normalizedRuntimeEnvelope = normalizeWorkerRuntimeEnvelope(runtimeEnvelope);
    this.#runtimeEnvelopeArtifact = freezeObject({
      ...normalizedRuntimeEnvelope,
      runtimeEnvelopeRestoreToken,
      runtimeEnvelopeRestoreMode: "SameRuntimeExact",
      runtimeEnvelopePortableWire,
    });
    this.#snapshotEnvelope = snapshotEnvelope;
  }

  #inspectionContext() {
    return freezeObject({
      bridge: this.#bridge,
      definition: this.#definition,
      trackedInputIds: this.#trackedInputIds,
      trackedOutputIds: this.#trackedOutputIds,
      diagnosticsSummary: () => this.diagnosticsSummary(),
      diagnosticsHistory: () => this.diagnosticsHistory(),
    });
  }

  #readCachedInput(id) {
    this.#requireHydrated("input.get");
    if (!this.#cachedInputs.has(id)) {
      throw new TypeError(
        `worker-first imported graph ${this.#graphId} has no cached input readback for \`${id}\``,
      );
    }
    return this.#cachedInputs.get(id);
  }

  #readCachedOutput(id) {
    this.#requireHydrated("output.get");
    if (!this.#cachedOutputs.has(id)) {
      throw new TypeError(
        `worker-first imported graph ${this.#graphId} has no cached output readback for \`${id}\``,
      );
    }
    return this.#cachedOutputs.get(id);
  }

  #requireActive(operation) {
    if (!this.#terminated) {
      return;
    }
    throw new TypeError(
      `worker-first imported graph ${this.#graphId} ${operation}() cannot be used after terminate()`,
    );
  }

  #requireHydrated(operation) {
    this.#requireActive(operation);
    if (this.#ready) {
      return;
    }
    if (this.#readyError !== null) {
      throw this.#readyError;
    }
    throw new TypeError(
      `worker-first imported graph ${this.#graphId} ${operation}() requires await importedGraph.ready() before worker-owned truth can be read`,
    );
  }
}
