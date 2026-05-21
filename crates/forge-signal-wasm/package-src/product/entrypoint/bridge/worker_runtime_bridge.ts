export function createWorkerRuntimeBridge(options = {}) {
  return new WorkerRuntimeBridge(options);
}

class WorkerRuntimeBridge {
  #nextRequestId = 0;
  #pending = new Map();
  #worker;

  constructor(options) {
    if (typeof globalThis.Worker !== "function") {
      throw new TypeError(
        "createWorkerRuntimeBridge requires a global Worker constructor",
      );
    }
    const workerUrl = options.workerUrl
      ? normalizeWorkerUrl(options.workerUrl)
      : new URL("./worker_runtime_bridge_worker.js", import.meta.url);
    this.#worker = new globalThis.Worker(workerUrl, { type: "module" });
    attachMessageListener(this.#worker, (message) => {
      const pending = this.#pending.get(message?.id);
      if (!pending) {
        return;
      }
      this.#pending.delete(message.id);
      if (message.ok) {
        pending.resolve(message.value);
        return;
      }
      pending.reject(deserializeError(message.error));
    });
    attachErrorListener(this.#worker, (event) => {
      const error = deserializeError({
        name: "WorkerRuntimeBridgeWorkerError",
        message: event.message || "Worker runtime bridge worker failed",
        stack: null,
      });
      this.#rejectAll(error);
    });
  }

  bootstrapRecord() {
    return this.#request("bootstrapRecord");
  }

  workerRuntimeShellLock() {
    return this.#request("workerRuntimeShellLock");
  }

  publishPortableGraph(publication) {
    return this.#request("publishPortableGraph", publication);
  }

  applyTransaction(transactionOps) {
    return this.#request("applyTransaction", transactionOps);
  }

  applyTransactionProjection(request) {
    return this.#request("applyTransactionProjection", request);
  }

  attachObservationDelivery(request) {
    return this.#request("attachObservationDelivery", request);
  }

  detachObservationDelivery(request) {
    return this.#request("detachObservationDelivery", request);
  }

  why(id) {
    return this.#request("why", id);
  }

  latestFlow() {
    return this.#request("latestFlow");
  }

  latestObservation() {
    return this.#request("latestObservation");
  }

  health() {
    return this.#request("health");
  }

  performanceSummary() {
    return this.#request("performanceSummary");
  }

  latestFailure() {
    return this.#request("latestFailure");
  }

  latestRollback() {
    return this.#request("latestRollback");
  }

  latestFrontierExecution() {
    return this.#request("latestFrontierExecution");
  }

  latestInvalidationTraceRecords() {
    return this.#request("latestInvalidationTraceRecords");
  }

  recentHistory() {
    return this.#request("recentHistory");
  }

  currentBranch() {
    return this.#request("currentBranch");
  }

  branches() {
    return this.#request("branches");
  }

  replayForBranch(branchId) {
    return this.#request("replayForBranch", normalizeWorkerBranchId(branchId, "replayForBranch"));
  }

  branchSnapshotId(branchId) {
    return this.#request("branchSnapshotId", normalizeWorkerBranchId(branchId, "branchSnapshotId"));
  }

  branchSnapshotEnvelope(branchId) {
    return this.#request("branchSnapshotEnvelope", normalizeWorkerBranchId(branchId, "branchSnapshotEnvelope"));
  }

  branchSnapshotArtifact(branchId) {
    return this.#request(
      "branchSnapshotArtifact",
      normalizeWorkerBranchId(branchId, "branchSnapshotArtifact"),
    );
  }

  branchSnapshotEnvelopeArtifact(branchId) {
    return this.#request(
      "branchSnapshotEnvelopeArtifact",
      normalizeWorkerBranchId(branchId, "branchSnapshotEnvelopeArtifact"),
    );
  }

  branchSnapshotEnvelopeWire(branchId) {
    return this.#request(
      "branchSnapshotEnvelopeWire",
      normalizeWorkerBranchId(branchId, "branchSnapshotEnvelopeWire"),
    );
  }

  branchSnapshotEnvelopePortableWire(branchId) {
    return this.#request(
      "branchSnapshotEnvelopePortableWire",
      normalizeWorkerBranchId(branchId, "branchSnapshotEnvelopePortableWire"),
    );
  }

  branchStateProof(branchId) {
    return this.#request("branchStateProof", normalizeWorkerBranchId(branchId, "branchStateProof"));
  }

  replayFor(id) {
    return this.#request("replayFor", id);
  }

  lineageFor(id) {
    return this.#request("lineageFor", id);
  }

  readVersions(ids) {
    return this.#request("readVersions", ids);
  }

  exportDefinitions() {
    return this.#request("exportDefinitions");
  }

  exportWorkerRuntimeEnvelope() {
    return this.#request("exportWorkerRuntimeEnvelope");
  }

  exportWorkerRuntimeEnvelopeWire() {
    return this.#request("exportWorkerRuntimeEnvelopeWire");
  }

  exportWorkerRuntimeEnvelopePortableWire() {
    return this.#request("exportWorkerRuntimeEnvelopePortableWire");
  }

  exportWorkerSnapshotEnvelope() {
    return this.#request("exportWorkerSnapshotEnvelope");
  }

  exportWorkerSnapshotEnvelopeArtifact() {
    return this.#request("exportWorkerSnapshotEnvelopeArtifact");
  }

  exportWorkerSnapshotEnvelopeWire() {
    return this.#request("exportWorkerSnapshotEnvelopeWire");
  }

  exportWorkerSnapshotEnvelopePortableWire() {
    return this.#request("exportWorkerSnapshotEnvelopePortableWire");
  }

  admitWorkerRuntimeEnvelopeImportWire(envelope) {
    return this.#request("admitWorkerRuntimeEnvelopeImportWire", envelope);
  }

  admitWorkerRuntimeEnvelopeImportPortableWire(envelope) {
    return this.#request("admitWorkerRuntimeEnvelopeImportPortableWire", envelope);
  }

  runtimeProofReport() {
    return this.#request("runtimeProofReport");
  }

  admitHostCapabilityIngress(batch) {
    return this.#request("admitHostCapabilityIngress", batch);
  }

  admitBrowserHistoryIngress(ingress) {
    return this.#request("admitBrowserHistoryIngress", ingress);
  }

  issueHostEffectRequest(request) {
    return this.#request("issueHostEffectRequest", request);
  }

  admitHostEffectAcknowledgement(acknowledgement) {
    return this.#request("admitHostEffectAcknowledgement", acknowledgement);
  }

  certifyMainThreadHostBridge() {
    return this.#request("certifyMainThreadHostBridge");
  }

  deliverLatestObservation() {
    return this.#request("deliverLatestObservation");
  }

  deliverOutputs(request) {
    return this.#request("deliverOutputs", request);
  }

  readSignals(request) {
    return this.#request("readSignals", request);
  }

  readDiagnosticsSummary() {
    return this.#request("readDiagnosticsSummary");
  }

  readDiagnosticsHistory() {
    return this.#request("readDiagnosticsHistory");
  }

  async terminate() {
    this.#worker.terminate();
    this.#rejectAll(new Error("Worker runtime bridge terminated"));
  }

  #request(method, ...args) {
    const id = ++this.#nextRequestId;
    return new Promise((resolve, reject) => {
      this.#pending.set(id, { resolve, reject });
      this.#worker.postMessage({ id, method, args });
    });
  }

  #rejectAll(error) {
    for (const pending of this.#pending.values()) {
      pending.reject(error);
    }
    this.#pending.clear();
  }
}

function normalizeWorkerUrl(workerUrl) {
  if (workerUrl instanceof URL) {
    return workerUrl;
  }
  if (typeof workerUrl === "string") {
    return new URL(workerUrl, import.meta.url);
  }
  throw new TypeError("createWorkerRuntimeBridge workerUrl must be a string or URL");
}

function normalizeWorkerBranchId(branchId, operation) {
  if (typeof branchId === "bigint") {
    if (branchId < 0n) {
      throw new RangeError(`${operation} expects a non-negative branch id`);
    }
    return branchId;
  }
  if (!Number.isSafeInteger(branchId) || branchId < 0) {
    throw new TypeError(`${operation} expects a non-negative safe integer branch id`);
  }
  return BigInt(branchId);
}

function deserializeError(error) {
  const normalized = error && typeof error === "object" ? error : {};
  const message = typeof normalized.message === "string"
    ? normalized.message
    : trySerializeUnknownError(error);
  const reconstructed = new Error(message);
  reconstructed.name = typeof normalized.name === "string"
    ? normalized.name
    : "WorkerRuntimeBridgeError";
  if (typeof normalized.code === "string") {
    reconstructed.code = normalized.code;
  }
  if (typeof normalized.stack === "string") {
    reconstructed.stack = normalized.stack;
  }
  return reconstructed;
}

function trySerializeUnknownError(error) {
  try {
    return JSON.stringify(error);
  } catch {
    return String(error);
  }
}

function attachMessageListener(worker, listener) {
  if (typeof worker.addEventListener === "function") {
    worker.addEventListener("message", (event) => listener(event.data));
    return;
  }
  if (typeof worker.on === "function") {
    worker.on("message", listener);
    return;
  }
  worker.onmessage = (event) => listener(event.data);
}

function attachErrorListener(worker, listener) {
  if (typeof worker.addEventListener === "function") {
    worker.addEventListener("error", listener);
    return;
  }
  if (typeof worker.on === "function") {
    worker.on("error", listener);
    return;
  }
  worker.onerror = listener;
}
