import { createWorkerRuntimeBridge } from "./bridge/worker_runtime_bridge.js";
import { materializeWorkerCachedValue } from "./sessions/support/worker_cached_value.js";
import { normalizeWorkerBrowserHistoryIngress } from "../router/projection/ingress/router_browser_history_ingress.js";

export async function createWorkerFirstProjectionSession(options) {
  const session = new WorkerFirstProjectionSession(options);
  await session.initialize();
  return session.api();
}

class WorkerFirstProjectionSession {
  #bridge;
  #bootstrap;
  #shellLock;
  #publication;
  #trackedOutputIds;
  #cachedOutputs;
  #cachedDiagnosticsSummary;
  #cachedDiagnosticsHistory;
  #terminated;

  constructor(options = {}) {
    const normalized = normalizeSessionOptions(options);
    this.#bridge = createWorkerRuntimeBridge({
      ...(normalized.workerUrl === null ? {} : { workerUrl: normalized.workerUrl }),
    });
    this.#bootstrap = null;
    this.#shellLock = null;
    this.#publication = normalized.publication;
    this.#trackedOutputIds = normalized.outputIds;
    this.#cachedOutputs = new Map();
    this.#cachedDiagnosticsSummary = null;
    this.#cachedDiagnosticsHistory = null;
    this.#terminated = false;
  }

  async initialize() {
    this.#bootstrap = await this.#bridge.bootstrapRecord();
    this.#shellLock = await this.#bridge.workerRuntimeShellLock();
    await this.#bridge.publishPortableGraph(this.#publication);
    await this.#refreshProjectionCaches(this.#trackedOutputIds);
  }

  api() {
    return Object.freeze({
      bootstrapRecord: () => this.bootstrapRecord(),
      workerRuntimeShellLock: () => this.workerRuntimeShellLock(),
      trackedOutputIds: () => this.trackedOutputIds(),
      readProjectedOutput: (id) => this.readProjectedOutput(id),
      diagnosticsSummary: () => this.diagnosticsSummary(),
      diagnosticsHistory: () => this.diagnosticsHistory(),
      projectCommittedTransaction: (request) => this.projectCommittedTransaction(request),
      refreshProjection: (options) => this.refreshProjection(options),
      admitHostCapabilityIngress: (batch, options) =>
        this.admitHostCapabilityIngress(batch, options),
      admitBrowserHistoryIngress: (ingress, options) =>
        this.admitBrowserHistoryIngress(ingress, options),
      issueHostEffectRequest: (request) => this.issueHostEffectRequest(request),
      admitHostEffectAcknowledgement: (acknowledgement, options) =>
        this.admitHostEffectAcknowledgement(acknowledgement, options),
      terminate: () => this.terminate(),
    });
  }

  bootstrapRecord() {
    this.#requireActive("bootstrapRecord");
    return this.#bootstrap;
  }

  workerRuntimeShellLock() {
    this.#requireActive("workerRuntimeShellLock");
    return this.#shellLock;
  }

  trackedOutputIds() {
    this.#requireActive("trackedOutputIds");
    return [...this.#trackedOutputIds];
  }

  readProjectedOutput(id) {
    this.#requireActive("readProjectedOutput");
    if (!this.#cachedOutputs.has(id)) {
      throw new TypeError(
        `worker-first projection session has no cached output for \`${id}\`; refreshProjection(...) or projectCommittedTransaction(...) must include it first`,
      );
    }
    return this.#cachedOutputs.get(id);
  }

  diagnosticsSummary() {
    this.#requireActive("diagnosticsSummary");
    if (this.#cachedDiagnosticsSummary === null) {
      throw missingProjectionTruthError("diagnosticsSummary");
    }
    return this.#cachedDiagnosticsSummary;
  }

  diagnosticsHistory() {
    this.#requireActive("diagnosticsHistory");
    if (this.#cachedDiagnosticsHistory === null) {
      throw missingProjectionTruthError("diagnosticsHistory");
    }
    return this.#cachedDiagnosticsHistory;
  }

  async projectCommittedTransaction(request) {
    this.#requireActive("projectCommittedTransaction");
    const normalized = normalizeProjectionRequest(request, this.#trackedOutputIds);
    const packet = await this.#bridge.applyTransactionProjection({
      transactionOps: normalized.transactionOps,
      outputIds: normalized.outputIds,
    });
    this.#trackedOutputIds = normalized.outputIds;
    this.#cacheProjectionPacket(packet);
    return packet;
  }

  async refreshProjection(options = {}) {
    this.#requireActive("refreshProjection");
    const outputIds = normalizeTrackedOutputRefresh(
      options.outputIds,
      this.#trackedOutputIds,
    );
    this.#trackedOutputIds = outputIds;
    return this.#refreshProjectionCaches(outputIds);
  }

  async admitHostCapabilityIngress(batch, options) {
    this.#requireActive("admitHostCapabilityIngress");
    const report = await this.#bridge.admitHostCapabilityIngress(batch);
    await this.#maybeRefreshAfterHostMutation(options);
    return report;
  }

  async admitBrowserHistoryIngress(ingress, options) {
    this.#requireActive("admitBrowserHistoryIngress");
    const report = await this.#bridge.admitBrowserHistoryIngress(
      normalizeWorkerBrowserHistoryIngress(
        ingress,
        "worker-first projection session admitBrowserHistoryIngress(...)",
      ),
    );
    await this.#maybeRefreshAfterHostMutation(options);
    return report;
  }

  async issueHostEffectRequest(request) {
    this.#requireActive("issueHostEffectRequest");
    return this.#bridge.issueHostEffectRequest(request);
  }

  async admitHostEffectAcknowledgement(acknowledgement, options) {
    this.#requireActive("admitHostEffectAcknowledgement");
    const report = await this.#bridge.admitHostEffectAcknowledgement(acknowledgement);
    await this.#maybeRefreshAfterHostMutation(options);
    return report;
  }

  async terminate() {
    if (this.#terminated) {
      return;
    }
    this.#terminated = true;
    this.#cachedOutputs.clear();
    this.#cachedDiagnosticsSummary = null;
    this.#cachedDiagnosticsHistory = null;
    await this.#bridge.terminate();
  }

  async #refreshProjectionCaches(outputIds) {
    const [outputs, diagnosticsSummary, diagnosticsHistory] = await Promise.all([
      outputIds.length === 0
        ? Promise.resolve(null)
        : this.#bridge.deliverOutputs({ outputIds }),
      this.#bridge.readDiagnosticsSummary(),
      this.#bridge.readDiagnosticsHistory(),
    ]);
    this.#cacheDiagnosticsPackets(diagnosticsSummary, diagnosticsHistory);
    if (outputs) {
      this.#cacheOutputs(outputs.outputs);
    } else {
      this.#cachedOutputs.clear();
    }
    return Object.freeze({
      outputs,
      diagnosticsSummary,
      diagnosticsHistory,
    });
  }

  async #maybeRefreshAfterHostMutation(options) {
    const refreshPolicy = normalizeRefreshPolicy(options);
    if (refreshPolicy === false) {
      return;
    }
    await this.refreshProjection();
  }

  #cacheProjectionPacket(packet) {
    this.#cacheOutputs(packet.outputs.outputs);
    this.#cacheDiagnosticsPackets(
      packet.diagnosticsSummary,
      packet.diagnosticsHistory,
    );
  }

  #cacheOutputs(outputs) {
    this.#cachedOutputs.clear();
    for (const output of outputs) {
      this.#cachedOutputs.set(output.id, materializeWorkerCachedValue(output.value));
    }
  }

  #cacheDiagnosticsPackets(diagnosticsSummary, diagnosticsHistory) {
    this.#cachedDiagnosticsSummary = materializeWorkerCachedValue(diagnosticsSummary.summary);
    this.#cachedDiagnosticsHistory = materializeWorkerCachedValue(diagnosticsHistory.history);
  }

  #requireActive(operation) {
    if (!this.#terminated) {
      return;
    }
    throw new TypeError(
      `worker-first projection session ${operation}() cannot be used after terminate()`,
    );
  }
}

function normalizeSessionOptions(options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(
      "createWorkerFirstProjectionSession(...) expects an options object",
    );
  }
  const { publication, outputIds, workerUrl, ...unknownOptions } = options;
  const unknownKeys = Object.keys(unknownOptions);
  if (unknownKeys.length > 0) {
    throw new TypeError(
      `createWorkerFirstProjectionSession(...) does not support: ${unknownKeys.join(", ")}`,
    );
  }
  if (!publication || typeof publication !== "object" || Array.isArray(publication)) {
    throw new TypeError(
      "createWorkerFirstProjectionSession(...) requires a portable publication object",
    );
  }
  return Object.freeze({
    publication,
    outputIds: normalizeOutputIds(outputIds, "createWorkerFirstProjectionSession(...) outputIds"),
    workerUrl:
      workerUrl === undefined ? null : workerUrl,
  });
}

function normalizeProjectionRequest(request, defaultOutputIds) {
  if (!request || typeof request !== "object" || Array.isArray(request)) {
    throw new TypeError(
      "projectCommittedTransaction(...) expects a request object",
    );
  }
  const { transactionOps, outputIds, ...unknownOptions } = request;
  const unknownKeys = Object.keys(unknownOptions);
  if (unknownKeys.length > 0) {
    throw new TypeError(
      `projectCommittedTransaction(...) does not support: ${unknownKeys.join(", ")}`,
    );
  }
  if (!Array.isArray(transactionOps)) {
    throw new TypeError(
      "projectCommittedTransaction(...) requires transactionOps as an array",
    );
  }
  return Object.freeze({
    transactionOps,
    outputIds:
      outputIds === undefined
        ? [...defaultOutputIds]
        : normalizeOutputIds(outputIds, "projectCommittedTransaction(...) outputIds"),
  });
}

function normalizeTrackedOutputRefresh(outputIds, trackedOutputIds) {
  return outputIds === undefined
    ? [...trackedOutputIds]
    : normalizeOutputIds(outputIds, "refreshProjection(...) outputIds");
}

function normalizeOutputIds(outputIds, label) {
  if (outputIds === undefined) {
    return [];
  }
  if (!Array.isArray(outputIds)) {
    throw new TypeError(`${label} must be an array when provided`);
  }
  const seen = new Set();
  const normalized = outputIds.map((id, index) => {
    if (typeof id !== "string" || id.trim().length === 0) {
      throw new TypeError(`${label}[${index}] must be a non-empty string`);
    }
    if (seen.has(id)) {
      throw new TypeError(`${label} rejects duplicate output id \`${id}\``);
    }
    seen.add(id);
    return id;
  });
  return Object.freeze(normalized);
}

function normalizeRefreshPolicy(options) {
  if (options === undefined) {
    return false;
  }
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError("host-boundary refresh options must be an object when provided");
  }
  const { refreshProjection = false, ...unknownOptions } = options;
  const unknownKeys = Object.keys(unknownOptions);
  if (unknownKeys.length > 0) {
    throw new TypeError(
      `host-boundary refresh options do not support: ${unknownKeys.join(", ")}`,
    );
  }
  if (typeof refreshProjection !== "boolean") {
    throw new TypeError("host-boundary refreshProjection must be a boolean when provided");
  }
  return refreshProjection;
}

function missingProjectionTruthError(operation) {
  return new TypeError(
    `worker-first projection session ${operation}() requires cached worker truth; initialize the session or refreshProjection(...) first`,
  );
}
