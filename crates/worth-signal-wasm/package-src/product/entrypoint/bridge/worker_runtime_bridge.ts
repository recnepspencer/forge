import {
  createBridgeBrowserHistoryIngressReport,
  createBridgeBrowserHistoryStory,
  normalizeBridgeBrowserHistoryIngress,
  normalizeBridgeBrowserHistoryWriteback,
  createBridgeBrowserHistoryWritebackReport,
} from "./worker_runtime_bridge_router_boundary.js";
import {
  attachErrorListener,
  attachMessageListener,
  deserializeError,
  normalizeWorkerBranchId,
  normalizeWorkerMergePreviewRequest,
  normalizeWorkerUrl,
} from "./worker_runtime_bridge_support.js";

export function createWorkerRuntimeBridge(options = {}) {
  return new WorkerRuntimeBridge(options);
}

class WorkerRuntimeBridge {
  #nextRequestId = 0;
  #pending = new Map();
  #worker;
  #terminated = false;
  #terminating = null;

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

  createBranch(name) {
    return this.#request("createBranch", name);
  }

  switchBranch(branchId) {
    return this.#request("switchBranch", normalizeWorkerBranchId(branchId, "switchBranch"));
  }

  planMergeBranches(sourceBranchId, targetBranchId) {
    return this.#request(
      "planMergeBranches",
      normalizeWorkerBranchId(sourceBranchId, "planMergeBranches.sourceBranchId"),
      normalizeWorkerBranchId(targetBranchId, "planMergeBranches.targetBranchId"),
    );
  }

  planMergeBranchesWithProof(sourceBranchId, targetBranchId) {
    return this.#request(
      "planMergeBranchesWithProof",
      normalizeWorkerBranchId(sourceBranchId, "planMergeBranchesWithProof.sourceBranchId"),
      normalizeWorkerBranchId(targetBranchId, "planMergeBranchesWithProof.targetBranchId"),
    );
  }

  mergeBranches(sourceBranchId, targetBranchId) {
    return this.#request(
      "mergeBranches",
      normalizeWorkerBranchId(sourceBranchId, "mergeBranches.sourceBranchId"),
      normalizeWorkerBranchId(targetBranchId, "mergeBranches.targetBranchId"),
    );
  }

  mergeBranchesWithProof(sourceBranchId, targetBranchId) {
    return this.#request(
      "mergeBranchesWithProof",
      normalizeWorkerBranchId(sourceBranchId, "mergeBranchesWithProof.sourceBranchId"),
      normalizeWorkerBranchId(targetBranchId, "mergeBranchesWithProof.targetBranchId"),
    );
  }

  planMergePolicyPreview(request) {
    return this.#request(
      "planMergePolicyPreview",
      normalizeWorkerMergePreviewRequest(request, "planMergePolicyPreview"),
    );
  }

  planMergePolicyPreviewWithProof(request) {
    return this.#request(
      "planMergePolicyPreviewWithProof",
      normalizeWorkerMergePreviewRequest(request, "planMergePolicyPreviewWithProof"),
    );
  }

  mergeBranchesPolicyPreview(request) {
    return this.#request(
      "mergeBranchesPolicyPreview",
      normalizeWorkerMergePreviewRequest(request, "mergeBranchesPolicyPreview"),
    );
  }

  mergeBranchesPolicyPreviewWithProof(request) {
    return this.#request(
      "mergeBranchesPolicyPreviewWithProof",
      normalizeWorkerMergePreviewRequest(request, "mergeBranchesPolicyPreviewWithProof"),
    );
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

  restoreBranchSnapshotArtifact(branchId, snapshot) {
    return this.#request(
      "restoreBranchSnapshotArtifact",
      normalizeWorkerBranchId(branchId, "restoreBranchSnapshotArtifact"),
      snapshot,
    );
  }

  restoreBranchSnapshotWire(branchId, snapshot) {
    return this.#request(
      "restoreBranchSnapshotWire",
      normalizeWorkerBranchId(branchId, "restoreBranchSnapshotWire"),
      snapshot,
    );
  }

  restoreBranchSnapshotPortableWire(branchId, snapshot) {
    return this.#request(
      "restoreBranchSnapshotPortableWire",
      normalizeWorkerBranchId(branchId, "restoreBranchSnapshotPortableWire"),
      snapshot,
    );
  }

  restoreBranchSnapshotById(branchId, snapshotId) {
    return this.#request(
      "restoreBranchSnapshotById",
      normalizeWorkerBranchId(branchId, "restoreBranchSnapshotById"),
      normalizeWorkerBranchId(snapshotId, "restoreBranchSnapshotById.snapshotId"),
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

  evaluateDirty() {
    return this.#request("evaluateDirty");
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

  restoreSnapshotEnvelope(snapshot) {
    return this.#request("restoreSnapshotEnvelope", snapshot);
  }

  restoreSnapshotEnvelopeWire(snapshot) {
    return this.#request("restoreSnapshotEnvelopeWire", snapshot);
  }

  restoreSnapshotEnvelopePortableWire(snapshot) {
    return this.#request("restoreSnapshotEnvelopePortableWire", snapshot);
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

  async admitBrowserHistoryIngress(ingress) {
    const normalizedIngress = normalizeBridgeBrowserHistoryIngress(
      ingress,
      "workerRuntimeBridge.admitBrowserHistoryIngress(...)",
    );
    const report = await this.#request("admitBrowserHistoryIngress", normalizedIngress);
    return createBridgeBrowserHistoryIngressReport(normalizedIngress, report);
  }

  async applyBrowserHistoryWriteback(writeback) {
    const normalizedWriteback = normalizeBridgeBrowserHistoryWriteback(
      writeback,
      "workerRuntimeBridge.applyBrowserHistoryWriteback(...)",
    );
    return createBridgeBrowserHistoryWritebackReport(
      normalizedWriteback,
      async (ingress) => this.#request("admitBrowserHistoryIngress", ingress),
      async () => this.#request("readDiagnosticsSummary"),
    );
  }

  browserHistoryStory(initialReport) {
    return createBridgeBrowserHistoryStory(initialReport);
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
    if (this.#terminated) {
      return;
    }
    if (this.#terminating !== null) {
      await this.#terminating;
      return;
    }
    this.#terminating = this.#terminateWhenIdle();
    try {
      await this.#terminating;
    } finally {
      this.#terminating = null;
    }
  }

  #request(method, ...args) {
    if (this.#terminated) {
      return Promise.reject(new Error("Worker runtime bridge terminated"));
    }
    const id = ++this.#nextRequestId;
    return new Promise((resolve, reject) => {
      this.#pending.set(id, { resolve, reject, method });
      this.#worker.postMessage({ id, method, args });
    });
  }

  async #terminateWhenIdle() {
    await this.#waitForIdle();
    this.#worker.terminate();
    this.#terminated = true;
    const pendingMethods = [...new Set([...this.#pending.values()].map((pending) => pending.method))];
    const suffix = pendingMethods.length === 0
      ? ""
      : ` while requests were pending: ${pendingMethods.join(", ")}`;
    this.#rejectAll(new Error(`Worker runtime bridge terminated${suffix}`));
  }

  async #waitForIdle() {
    let idlePasses = 0;
    for (let attempts = 0; attempts < 50 && idlePasses < 2; attempts += 1) {
      if (this.#pending.size === 0) {
        idlePasses += 1;
      } else {
        idlePasses = 0;
      }
      await new Promise((resolve) => setTimeout(resolve, 0));
    }
  }

  #rejectAll(error) {
    for (const pending of this.#pending.values()) {
      pending.reject(error);
    }
    this.#pending.clear();
  }
}
