import { createWorkerRuntimeBridge } from "./bridge/worker_runtime_bridge.js";
import { createWorkerFirstHostCapabilities } from "./worker_first_host_capabilities.js";
import {
  createWorkerFirstRootHistoryLifecycle,
  refreshWorkerFirstRootAfterHistoryMutation,
} from "./worker_first_root_history_lifecycle.js";
import { createWorkerFirstRootMutation } from "./worker_first_root_mutation.js";
import { createWorkerFirstRootObservationManager } from "./worker_first_root_observations.js";
import { createWorkerFirstRootRuntimeReplacement } from "./worker_first_root_runtime_replacement.js";
import { createWorkerFirstRootAuthoredRuntime } from "./sessions/support/worker_first_root_authored_runtime.js";
import { buildActiveImportContext } from "./sessions/support/worker_first_root_import_context.js";

export function createWorkerFirstRootSession(options = {}) {
  return new WorkerFirstRootSession(options);
}

class WorkerFirstRootSession {
  #bridge;
  #bootstrap;
  #hostCapabilities;
  #observations;
  #activeImportController;
  #activeImportDependents;
  #activeImportContext;
  #importChain;
  #authoredRuntime;
  #historyLifecycle;
  #mutation;
  #runtimeReplacement;
  #cachedCurrentBranch;
  #cachedBranches;
  #terminated;

  constructor(options) {
    this.#bridge = createWorkerRuntimeBridge(
      options.workerUrl === undefined ? {} : { workerUrl: options.workerUrl },
    );
    this.#observations = createWorkerFirstRootObservationManager();
    this.#bootstrap = this.#bootstrapBridge();
    this.#activeImportController = null;
    this.#activeImportDependents = new Set();
    this.#activeImportContext = null;
    this.#importChain = Promise.resolve();
    this.#authoredRuntime = createWorkerFirstRootAuthoredRuntime(
      this.#bridge,
      () => this.#activeImportContext,
      (operation) => this.#requireActive(operation),
      this,
    );
    this.#historyLifecycle = createWorkerFirstRootHistoryLifecycle({
      ready: () => this.ready(),
      requireActive: (operation) => this.#requireActive(operation),
      requireControllerActive: (controller, operation) => this.#requireControllerActive(controller, operation),
      bridge: this.#bridge,
      observations: this.#observations,
      authoredRuntime: this.#authoredRuntime,
      activeImportContext: () => this.#activeImportContext,
      activeImportController: () => this.#activeImportController,
      setActiveImportContext: (context) => {
        this.#activeImportContext = context;
      },
      refreshBranchCache: () => this.#refreshBranchCache(),
      refreshActiveImportContext: () => this.refreshActiveImportContext(),
      refreshAfterHistoryMutation: (operation, activeImportContext) => this.#refreshAfterHistoryMutation(operation, activeImportContext),
    });
    this.#mutation = createWorkerFirstRootMutation({
      ready: () => this.ready(),
      requireActive: (operation) => this.#requireActive(operation),
      requireControllerActive: (controller, operation) => this.#requireControllerActive(controller, operation),
      bridge: this.#bridge,
      observations: this.#observations,
      authoredRuntime: this.#authoredRuntime,
      activeImportContext: () => this.#activeImportContext,
      activeImportController: () => this.#activeImportController,
      setActiveImportContext: (context) => {
        this.#activeImportContext = context;
      },
      refreshBranchCache: () => this.#refreshBranchCache(),
      currentImportContext: () => this.currentImportContext(),
      hasMutableInputId: (id) => this.hasMutableInputId(id),
      applyImportMutation: (controller, transactionOps, outputIds) => this.applyImportMutation(controller, transactionOps, outputIds),
    });
    this.#cachedCurrentBranch = null;
    this.#cachedBranches = [];
    this.#hostCapabilities = createWorkerFirstHostCapabilities(this, options.hostCapabilities ?? null);
    this.#runtimeReplacement = createWorkerFirstRootRuntimeReplacement({
      ready: () => this.ready(),
      requireActive: (operation) => this.#requireActive(operation),
      bridge: this.#bridge,
      observations: this.#observations,
      authoredRuntime: this.#authoredRuntime,
      hostCapabilities: this.#hostCapabilities,
      invalidateActiveImport: (message) => this.#invalidateActiveImport(message),
    });
    this.#terminated = false;
  }

  bridge() { return this.#bridge; }
  ready() { return this.#bootstrap; }
  hostSurface() { return this.#hostCapabilities.host; }
  latestHostCapabilityEvent() { return this.#hostCapabilities.latestEvent(); }
  recentHostCapabilityEvents() { return this.#hostCapabilities.recentEvents(); }
  hostCapabilityReport() { return this.#hostCapabilities.report(); }
  watch(target, callback) { return this.#observations.watch(this.#bridge, target, callback); }
  effect(target, callback) { return this.#observations.effect(this.#bridge, target, callback); }
  nuke(handle) { return this.#observations.nuke(this.#bridge, handle); }

  beginExactImport(definition, snapshot, controller) {
    const portableWire = snapshot?.runtimeEnvelope?.runtimeEnvelopePortableWire;
    if (typeof portableWire !== "string") {
      throw new TypeError(
        "worker-first root importGraph(...) requires a snapshot.runtimeEnvelope artifact returned by adapters.exportRuntimeEnvelope()",
      );
    }
    this.#invalidateActiveImport(
      "worker-first imported graph was superseded by a newer root importGraph() call",
    );
    this.#authoredRuntime.invalidate(
      "worker-first imported graph importGraph(...) replaced the worker-owned runtime",
    );
    this.#activeImportController = controller;
    const importPromise = this.#importChain.then(async () => {
      this.#requireActive("importGraph");
      await this.#authoredRuntime.settlePendingPublications();
      await this.ready();
      await this.#observations.clearContext(this.#bridge);
      this.#requireControllerActive(controller, "importGraph");
      const report = await this.#bridge.admitWorkerRuntimeEnvelopeImportPortableWire(portableWire);
      if (report?.importOutcome !== "Admitted") {
        throw new TypeError(
          `worker-first root importGraph(...) could not admit the portable runtime envelope: ${report?.importOutcome ?? "Unknown"}`,
        );
      }
      await this.#hostCapabilities.replayCurrentIngress();
      this.#activeImportContext = await buildActiveImportContext(
        this.#bridge,
        definition,
        snapshot,
      );
      await this.#observations.replaceContext(this.#bridge, this.#activeImportContext);
      this.#requireControllerActive(controller, "importGraph");
    });
    this.#importChain = importPromise.catch(() => {});
    return importPromise;
  }

  detachImport(controller) {
    if (this.#activeImportController === controller) this.#activeImportController = null;
  }

  registerActiveImportDependent(dependent) { this.#activeImportDependents.add(dependent); }
  unregisterActiveImportDependent(dependent) { this.#activeImportDependents.delete(dependent); }

  currentImportContext() {
    if (this.#activeImportContext === null) {
      throw new TypeError(
        "worker-first root surface requires an active imported graph; await importedGraph.ready() first",
      );
    }
    return this.#activeImportContext;
  }

  nextGeneratedStandaloneSignalId(family, scopeId = null) {
    return this.#authoredRuntime.nextGeneratedStandaloneSignalId(family, scopeId);
  }

  hasKnownSignalId(id) {
    return this.#authoredRuntime.hasKnownSignalId(id);
  }

  hasMutableInputId(id) {
    return this.#authoredRuntime.hasMutableInputId(id);
  }

  readSignalValue(id) {
    return this.#authoredRuntime.readSignalValue(id);
  }

  readAuthoredInputBaseline(id) {
    return this.#authoredRuntime.readAuthoredInputBaseline(id);
  }

  currentBranchSummary() {
    return this.#activeImportContext?.currentBranch ?? this.#cachedCurrentBranch;
  }

  branchesSummary() {
    return this.#activeImportContext?.branches ?? this.#cachedBranches;
  }

  writeAuthoredInputBaseline(id, value) {
    this.#authoredRuntime.writeAuthoredInputBaseline(id, value);
  }

  async createStandaloneInput(id, initial, options = {}) {
    await this.ready(); await this.#authoredRuntime.createStandaloneInput(id, initial, options);
  }
  createEagerStandaloneInput(id, initial, options = {}) {
    this.#authoredRuntime.createEagerStandaloneInput(id, initial, options);
  }
  async createStandaloneReadable(id, family, spec) {
    await this.ready(); await this.#authoredRuntime.createStandaloneReadable(id, family, spec);
  }
  createEagerStandaloneReadable(id, family, spec, initialValue, dependencyIds) {
    this.#authoredRuntime.createEagerStandaloneReadable(
      id,
      family,
      spec,
      initialValue,
      dependencyIds,
    );
  }
  async createStandaloneCallbackReadable(id, family, callback) {
    await this.ready(); await this.#authoredRuntime.createStandaloneCallbackReadable(id, family, callback);
  }
  createEagerStandaloneCallbackReadable(id, family, callback) {
    this.#authoredRuntime.createEagerStandaloneCallbackReadable(id, family, callback);
  }

  async replaceRuntimeEnvelope(envelope) {
    return this.#runtimeReplacement.replaceRuntimeEnvelope(envelope);
  }

  async restoreExactRuntimeEnvelope(envelope) {
    return this.#runtimeReplacement.restoreExactRuntimeEnvelope(envelope);
  }

  async createHistoryBranch(name) {
    return this.#historyLifecycle.createBranch(name);
  }

  async switchHistoryBranch(branchId) {
    return this.#historyLifecycle.switchBranch(branchId);
  }

  async restoreHistorySnapshotEnvelope(snapshotEnvelope) {
    return this.#historyLifecycle.restoreSnapshotEnvelope(snapshotEnvelope);
  }

  async restoreExactHistorySnapshotEnvelope(restoreToken) {
    return this.#historyLifecycle.restoreExactSnapshotEnvelope(restoreToken);
  }

  async restorePortableHistorySnapshotEnvelope(portableWire) {
    return this.#historyLifecycle.restorePortableSnapshotEnvelope(portableWire);
  }

  async restoreHistoryBranchSnapshot(branchId, snapshot) {
    return this.#historyLifecycle.restoreBranchSnapshot(branchId, snapshot);
  }

  async restoreExactHistoryBranchSnapshot(branchId, restoreToken) {
    return this.#historyLifecycle.restoreExactBranchSnapshot(branchId, restoreToken);
  }

  async restorePortableHistoryBranchSnapshot(branchId, portableWire) {
    return this.#historyLifecycle.restorePortableBranchSnapshot(branchId, portableWire);
  }

  async restoreHistoryBranchSnapshotById(branchId, snapshotId) {
    return this.#historyLifecycle.restoreBranchSnapshotById(branchId, snapshotId);
  }

  async mergeHistoryBranches(sourceBranchId, targetBranchId) {
    return this.#historyLifecycle.mergeBranches(sourceBranchId, targetBranchId);
  }

  async mergeHistoryBranchesWithProof(sourceBranchId, targetBranchId) {
    return this.#historyLifecycle.mergeBranchesWithProof(sourceBranchId, targetBranchId);
  }

  async evaluateDirty() {
    return this.#historyLifecycle.evaluateDirty();
  }

  async terminate() {
    if (this.#terminated) {
      return;
    }
    this.#terminated = true;
    await this.#authoredRuntime.settlePendingPublications();
    this.#hostCapabilities.dispose();
    this.#invalidateActiveImport("worker-first root session terminated");
    this.#authoredRuntime.invalidate("worker-first root session terminated");
    this.#activeImportContext = null;
    await this.#observations.clearContext(this.#bridge);
    await this.#observations.clearObservers(this.#bridge);
    await this.#bridge.terminate();
  }

  async refreshActiveImportContext() {
    if (this.#activeImportController === null || this.#activeImportContext === null) {
      return;
    }
    const activeImportController = this.#activeImportController;
    const activeImportContext = this.#activeImportContext;
    this.#requireActive("refreshActiveImportContext");
    this.#activeImportContext = await buildActiveImportContext(
      this.#bridge,
      activeImportContext.definition,
      activeImportContext.snapshot,
    );
    await this.#observations.replaceContext(this.#bridge, this.#activeImportContext);
    if (typeof activeImportController.refreshFromRootRuntime === "function") {
      await activeImportController.refreshFromRootRuntime();
    }
    this.#requireControllerActive(activeImportController, "refreshActiveImportContext");
  }

  async applyImportMutation(controller, transactionOps, outputIds) {
    return this.#mutation.applyImportMutation(controller, transactionOps, outputIds);
  }

  async applyActiveTransaction(transactionOps) {
    return this.#mutation.applyActiveTransaction(transactionOps);
  }

  async applyActiveInputMutation(id, mutation) {
    return this.#mutation.applyActiveInputMutation(id, mutation);
  }

  applyAuthoredInputMutation(id, mutation) {
    return this.#mutation.applyAuthoredInputMutation(id, mutation);
  }

  #invalidateActiveImport(message) {
    if (this.#activeImportController === null) {
      for (const dependent of this.#activeImportDependents) {
        dependent.invalidate(message);
      }
      this.#activeImportDependents.clear();
      return;
    }
    this.#activeImportController.invalidate(message);
    for (const dependent of this.#activeImportDependents) {
      dependent.invalidate(message);
    }
    this.#activeImportController = null;
    this.#activeImportDependents.clear();
    this.#activeImportContext = null;
  }

  async #bootstrapBridge() {
    await this.#bridge.bootstrapRecord();
    await this.#bridge.workerRuntimeShellLock();
    await this.#hostCapabilities.bootstrap();
    await this.#refreshBranchCache();
  }

  #requireActive(operation) {
    if (!this.#terminated) {
      return;
    }
    throw new TypeError(
      `worker-first root session ${operation}() cannot be used after free()`,
    );
  }

  #requireControllerActive(controller, operation) {
    if (!controller.isInvalidated()) {
      return;
    }
    throw controller.invalidatedError(operation);
  }

  async #refreshAfterHistoryMutation(operation, activeImportContext) {
    await refreshWorkerFirstRootAfterHistoryMutation(
      {
        bridge: this.#bridge,
        observations: this.#observations,
        authoredRuntime: this.#authoredRuntime,
        activeImportContext: () => this.#activeImportContext,
        activeImportController: () => this.#activeImportController,
        setActiveImportContext: (context) => {
          this.#activeImportContext = context;
        },
        refreshBranchCache: () => this.#refreshBranchCache(),
        requireControllerActive: (controller, action) => this.#requireControllerActive(controller, action),
      },
      operation,
      activeImportContext,
    );
  }

  async #refreshBranchCache() {
    let currentBranch = await this.#bridge.currentBranch();
    if (currentBranch !== null && currentBranch.head_snapshot_id === null) {
      try {
        currentBranch = {
          ...currentBranch,
          head_snapshot_id: await this.#bridge.branchSnapshotId(currentBranch.id),
        };
      } catch {}
    }
    this.#cachedCurrentBranch = currentBranch;
    this.#cachedBranches = await this.#bridge.branches();
  }
}
