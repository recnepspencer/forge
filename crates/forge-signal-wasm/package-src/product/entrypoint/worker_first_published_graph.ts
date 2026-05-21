import {
  inspectWorkerFirstGraphDiagnostics,
  inspectWorkerFirstGraphHistory,
} from "./sessions/support/worker_first_graph_inspection.js";
import { normalizePublishedGraphSessionOptions } from "./sessions/support/worker_first_published_graph_definition.js";
import {
  missingReadbackError,
  planWorkerFirstPublishedGraphMutation,
  requireKnownInputDescriptor,
  requireKnownOutputDescriptor,
} from "./sessions/support/worker_first_published_graph_mutation.js";
import { materializeWorkerCachedValue } from "./sessions/support/worker_cached_value.js";
import { createWorkerRuntimeBridge } from "./bridge/worker_runtime_bridge.js";

export async function createWorkerFirstPublishedGraphSession(options) {
  const session = new WorkerFirstPublishedGraphSession(options);
  await session.initialize();
  return session.api();
}

class WorkerFirstPublishedGraphSession {
  #bridge;
  #definition;
  #inputDescriptorsByName;
  #inputAuthoritiesByName;
  #initialValuesBySourceId;
  #outputDescriptorsByName;
  #trackedInputIds;
  #trackedOutputIds;
  #cachedInputs;
  #cachedOutputs;
  #cachedDiagnosticsSummary;
  #cachedDiagnosticsHistory;
  #terminated;

  constructor(options = {}) {
    const normalized = normalizePublishedGraphSessionOptions(options);
    this.#bridge = createWorkerRuntimeBridge(
      normalized.workerUrl === null ? {} : { workerUrl: normalized.workerUrl },
    );
    this.#definition = normalized.definition;
    this.#inputDescriptorsByName = normalized.inputDescriptorsByName;
    this.#inputAuthoritiesByName = normalized.inputAuthoritiesByName;
    this.#initialValuesBySourceId = normalized.initialValuesBySourceId;
    this.#outputDescriptorsByName = normalized.outputDescriptorsByName;
    this.#trackedInputIds = normalized.trackedInputIds;
    this.#trackedOutputIds = normalized.trackedOutputIds;
    this.#cachedInputs = new Map();
    this.#cachedOutputs = new Map();
    this.#cachedDiagnosticsSummary = null;
    this.#cachedDiagnosticsHistory = null;
    this.#terminated = false;
  }

  async initialize() {
    await this.#bridge.bootstrapRecord();
    await this.#bridge.workerRuntimeShellLock();
    await this.#bridge.publishPortableGraph({
      ...this.#definition.compatibility.definitions,
      outputIds: this.#trackedOutputIds,
    });
    await this.#refreshCaches();
  }

  api() {
    return Object.freeze({
      id: this.#definition.id,
      summary: () => this.#definition.summary,
      contract: () => this.#definition.contract,
      operationalContract: () => this.#definition.operationalContract,
      inputDescriptors: () => this.#definition.inputDescriptors,
      descriptors: () => this.#definition.descriptors,
      read: () => this.read(),
      readInputs: () => this.readInputs(),
      readInput: (name) => this.readInput(name),
      readOutput: (name) => this.readOutput(name),
      why: (name) => this.why(name),
      replayFor: (name) => this.replayFor(name),
      lineageFor: (name) => this.lineageFor(name),
      readVersions: () => this.readVersions(),
      writeInputs: (values) => this.writeInputs(values),
      writeInput: (name, value) => this.writeInput(name, value),
      patchInputs: (patches) => this.patchInputs(patches),
      patchInput: (name, patch) => this.patchInput(name, patch),
      resetInputs: (names) => this.resetInputs(names),
      resetInput: (name) => this.resetInput(name),
      apply: (mutation) => this.apply(mutation),
      diagnosticsSummary: () => this.diagnosticsSummary(),
      diagnosticsHistory: () => this.diagnosticsHistory(),
      inspectDiagnostics: () => this.inspectDiagnostics(),
      inspectHistory: () => this.inspectHistory(),
      exportCompatibilityDefinition: () => this.exportCompatibilityDefinition(),
      exportDefinition: () => this.exportDefinition(),
      runtimeProofReport: () => this.runtimeProofReport(),
      terminate: () => this.terminate(),
    });
  }

  read() {
    this.#requireActive("read");
    const snapshot = Object.create(null);
    for (const descriptor of this.#definition.descriptors) {
      snapshot[descriptor.outputName] = this.readOutput(descriptor.outputName);
    }
    return Object.freeze(snapshot);
  }

  readInputs() {
    this.#requireActive("readInputs");
    const snapshot = Object.create(null);
    for (const descriptor of this.#definition.inputDescriptors) {
      snapshot[descriptor.inputName] = this.readInput(descriptor.inputName);
    }
    return Object.freeze(snapshot);
  }

  readInput(name) {
    this.#requireActive("readInput");
    const descriptor = requireKnownInputDescriptor(this.#inputDescriptorsByName, this.#definition.id, name);
    if (!this.#cachedInputs.has(descriptor.sourceId)) {
      throw missingReadbackError(this.#definition.id, "input", name);
    }
    return this.#cachedInputs.get(descriptor.sourceId);
  }

  readOutput(name) {
    this.#requireActive("readOutput");
    const descriptor = requireKnownOutputDescriptor(this.#outputDescriptorsByName, this.#definition.id, name);
    if (!this.#cachedOutputs.has(descriptor.publishedId)) {
      throw missingReadbackError(this.#definition.id, "output", name);
    }
    return this.#cachedOutputs.get(descriptor.publishedId);
  }

  async why(name) {
    this.#requireActive("why");
    const descriptor = requireKnownOutputDescriptor(
      this.#outputDescriptorsByName,
      this.#definition.id,
      name,
    );
    return this.#bridge.why(descriptor.publishedId);
  }

  async replayFor(name) {
    this.#requireActive("replayFor");
    const descriptor = requireKnownOutputDescriptor(
      this.#outputDescriptorsByName,
      this.#definition.id,
      name,
    );
    return this.#bridge.replayFor(descriptor.publishedId);
  }

  async lineageFor(name) {
    this.#requireActive("lineageFor");
    const descriptor = requireKnownOutputDescriptor(
      this.#outputDescriptorsByName,
      this.#definition.id,
      name,
    );
    return this.#bridge.lineageFor(descriptor.publishedId);
  }

  async readVersions() {
    this.#requireActive("readVersions");
    return this.#bridge.readVersions(this.#trackedOutputIds);
  }
  async writeInputs(values) {
    return this.apply({ writes: values });
  }
  async writeInput(name, value) {
    return this.apply({ writes: { [name]: value } });
  }
  async patchInputs(patches) {
    return this.apply({ patches });
  }
  async patchInput(name, patch) {
    return this.apply({ patches: { [name]: patch } });
  }
  async resetInputs(names) {
    return this.apply({ reset: names });
  }
  async resetInput(name) {
    return this.apply({ reset: [name] });
  }
  async apply(mutation) {
    this.#requireActive("apply");
    const transactionOps = planWorkerFirstPublishedGraphMutation({
      definition: this.#definition,
      inputDescriptorsByName: this.#inputDescriptorsByName,
      inputAuthoritiesByName: this.#inputAuthoritiesByName,
      initialValuesBySourceId: this.#initialValuesBySourceId,
      cachedInputs: this.#cachedInputs,
      mutation,
    });
    const projectionPacket = await this.#bridge.applyTransactionProjection({
      transactionOps,
      outputIds: this.#trackedOutputIds,
    });
    this.#cacheOutputs(projectionPacket.outputs.outputs);
    this.#cacheDiagnosticsPackets(
      projectionPacket.diagnosticsSummary,
      projectionPacket.diagnosticsHistory,
    );
    await this.#refreshInputCache();
    return projectionPacket.transaction.runSummary;
  }
  diagnosticsSummary() {
    this.#requireActive("diagnosticsSummary");
    if (this.#cachedDiagnosticsSummary === null) {
      throw new TypeError("worker-first published graph diagnosticsSummary() requires cached worker truth");
    }
    return this.#cachedDiagnosticsSummary;
  }
  diagnosticsHistory() {
    this.#requireActive("diagnosticsHistory");
    if (this.#cachedDiagnosticsHistory === null) {
      throw new TypeError("worker-first published graph diagnosticsHistory() requires cached worker truth");
    }
    return this.#cachedDiagnosticsHistory;
  }

  async inspectDiagnostics() {
    this.#requireActive("inspectDiagnostics");
    return inspectWorkerFirstGraphDiagnostics(this.#inspectionContext());
  }

  async inspectHistory() {
    this.#requireActive("inspectHistory");
    return inspectWorkerFirstGraphHistory(this.#inspectionContext());
  }

  exportCompatibilityDefinition() {
    this.#requireActive("exportCompatibilityDefinition");
    return this.#definition.compatibility;
  }

  exportDefinition() {
    this.#requireActive("exportDefinition");
    return this.#definition;
  }

  async runtimeProofReport() {
    this.#requireActive("runtimeProofReport");
    return this.#bridge.runtimeProofReport();
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
  async #refreshCaches() {
    await Promise.all([
      this.#refreshInputCache(),
      this.#refreshOutputCache(),
      this.#refreshDiagnosticsCache(),
    ]);
  }
  async #refreshInputCache() {
    const packet = await this.#bridge.readSignals({ signalIds: this.#trackedInputIds });
    this.#cachedInputs.clear();
    for (const signal of packet.signals) {
      this.#cachedInputs.set(signal.id, materializeWorkerCachedValue(signal.value));
    }
  }
  async #refreshOutputCache() {
    const packet = await this.#bridge.deliverOutputs({ outputIds: this.#trackedOutputIds });
    this.#cacheOutputs(packet.outputs);
  }
  async #refreshDiagnosticsCache() {
    const [summaryPacket, historyPacket] = await Promise.all([
      this.#bridge.readDiagnosticsSummary(),
      this.#bridge.readDiagnosticsHistory(),
    ]);
    this.#cacheDiagnosticsPackets(summaryPacket, historyPacket);
  }
  #cacheOutputs(outputs) {
    this.#cachedOutputs.clear();
    for (const output of outputs) {
      this.#cachedOutputs.set(output.id, materializeWorkerCachedValue(output.value));
    }
  }
  #cacheDiagnosticsPackets(summaryPacket, historyPacket) {
    this.#cachedDiagnosticsSummary = materializeWorkerCachedValue(summaryPacket.summary);
    this.#cachedDiagnosticsHistory = materializeWorkerCachedValue(historyPacket.history);
  }

  #inspectionContext() {
    return Object.freeze({
      bridge: this.#bridge,
      definition: this.#definition,
      trackedInputIds: this.#trackedInputIds,
      trackedOutputIds: this.#trackedOutputIds,
      diagnosticsSummary: () => this.diagnosticsSummary(),
      diagnosticsHistory: () => this.diagnosticsHistory(),
    });
  }
  #requireActive(operation) {
    if (!this.#terminated) {
      return;
    }
    throw new TypeError(
      `worker-first published graph ${this.#definition.id} ${operation}() cannot be used after terminate()`,
    );
  }
}
