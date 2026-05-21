import { createWorkerRuntimeBridge } from "./bridge/worker_runtime_bridge.js";
import { createWorkerFirstHostCapabilities } from "./worker_first_host_capabilities.js";
import { createWorkerFirstRootObservationManager } from "./worker_first_root_observations.js";
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
  #terminated;

  constructor(options) {
    this.#bridge = createWorkerRuntimeBridge(
      options.workerUrl === undefined ? {} : { workerUrl: options.workerUrl },
    );
    this.#hostCapabilities = createWorkerFirstHostCapabilities(options.hostCapabilities ?? null);
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
    );
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
      await this.ready();
      await this.#observations.clearContext(this.#bridge);
      this.#requireControllerActive(controller, "importGraph");
      const report = await this.#bridge.admitWorkerRuntimeEnvelopeImportPortableWire(portableWire);
      if (report?.importOutcome !== "Admitted") {
        throw new TypeError(
          `worker-first root importGraph(...) could not admit the portable runtime envelope: ${report?.importOutcome ?? "Unknown"}`,
        );
      }
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

  writeAuthoredInputBaseline(id, value) {
    this.#authoredRuntime.writeAuthoredInputBaseline(id, value);
  }

  async createStandaloneInput(id, initial, options = {}) {
    await this.ready();
    await this.#authoredRuntime.createStandaloneInput(id, initial, options);
  }

  async createStandaloneReadable(id, family, spec) {
    await this.ready();
    await this.#authoredRuntime.createStandaloneReadable(id, family, spec);
  }

  async replaceRuntimeEnvelope(envelope) {
    const portableWire = envelope?.runtimeEnvelopePortableWire;
    if (typeof portableWire !== "string") {
      throw new TypeError(
        "worker-first root adapters().replaceRuntimeEnvelope(...) requires an artifact returned by adapters.exportRuntimeEnvelope()",
      );
    }
    await this.ready();
    this.#requireActive("adapters.replaceRuntimeEnvelope");
    const report = await this.#bridge.admitWorkerRuntimeEnvelopeImportPortableWire(portableWire);
    if (isWorkerRuntimeEnvelopeImportAdmitted(report)) {
      this.#invalidateActiveImport(
        "worker-first root adapters().replaceRuntimeEnvelope(...) replaced the active imported graph runtime",
      );
      this.#authoredRuntime.invalidate(
        "worker-first root adapters().replaceRuntimeEnvelope(...) replaced the worker-owned runtime",
      );
      await this.#observations.clearContext(this.#bridge);
    }
    return report;
  }

  async restoreExactRuntimeEnvelope(envelope) {
    const restoreToken = envelope?.runtimeEnvelopeRestoreToken;
    if (typeof restoreToken !== "string") {
      throw new TypeError(
        "worker-first root adapters().restoreExactRuntimeEnvelope(...) requires an artifact returned by adapters.exportRuntimeEnvelope()",
      );
    }
    await this.ready();
    this.#requireActive("adapters.restoreExactRuntimeEnvelope");
    const report = await this.#bridge.admitWorkerRuntimeEnvelopeImportWire(restoreToken);
    if (isWorkerRuntimeEnvelopeImportAdmitted(report)) {
      this.#invalidateActiveImport(
        "worker-first root adapters().restoreExactRuntimeEnvelope(...) replaced the active imported graph runtime",
      );
      this.#authoredRuntime.invalidate(
        "worker-first root adapters().restoreExactRuntimeEnvelope(...) replaced the worker-owned runtime",
      );
      await this.#observations.clearContext(this.#bridge);
    }
    return report;
  }

  async terminate() {
    if (this.#terminated) {
      return;
    }
    this.#terminated = true;
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
    await this.ready();
    this.#requireActive("importedGraph.apply");
    this.#requireControllerActive(controller, "importedGraph.apply");
    await this.#observations.syncLifecycle(this.#bridge);
    const projectionPacket = await this.#bridge.applyTransactionProjection({
      transactionOps,
      outputIds,
    });
    this.#requireControllerActive(controller, "importedGraph.apply");
    const activeImportController = this.#activeImportController;
    const activeImportContext = this.#activeImportContext;
    if (activeImportController === null || activeImportContext === null) {
      throw new TypeError(
        "worker-first root importedGraph.apply() requires an active imported graph context",
      );
    }
    this.#activeImportContext = await buildActiveImportContext(
      this.#bridge,
      activeImportContext.definition,
      activeImportContext.snapshot,
    );
    const deliveryPacket = await this.#observations.syncLifecycle(this.#bridge)
      .then(() => this.#bridge.deliverLatestObservation())
      .catch(() => null);
    await this.#observations.replaceContext(
      this.#bridge,
      this.#activeImportContext,
      deliveryPacket,
    );
    if (typeof activeImportController?.refreshFromRootRuntime === "function") {
      await activeImportController.refreshFromRootRuntime();
    }
    this.#authoredRuntime.applyCommittedInputs(transactionOps);
    await this.#authoredRuntime.refreshReadables();
    this.#requireControllerActive(activeImportController, "importedGraph.apply");
    return projectionPacket.transaction.runSummary;
  }

  async applyActiveTransaction(transactionOps) {
    await this.ready();
    this.#requireActive("transactionAsync");
    if (!Array.isArray(transactionOps)) {
      throw new TypeError("worker-first root transactionAsync(...) requires transactionOps as an array");
    }
    for (const op of transactionOps) {
      if (!op || typeof op !== "object" || typeof op.id !== "string") {
        throw new TypeError(
          "worker-first root transactionAsync(...) encountered an invalid input mutation operation",
        );
      }
      if (!this.hasMutableInputId(op.id)) {
        throw new TypeError(
          `worker-first root transactionAsync(...) can mutate only currently available worker-first inputs; \`${op.id}\` is not currently available`,
        );
      }
    }
    return this.#applyWorkerOwnedTransaction(transactionOps);
  }

  async applyActiveInputMutation(id, mutation) {
    await this.ready();
    this.#requireActive("signals.spec.input");
    const activeImportContext = this.currentImportContext();
    const inputDescriptor = activeImportContext.definition.inputDescriptors.find(
      (entry) => entry.sourceId === id,
    );
    if (!inputDescriptor) {
      throw new TypeError(
        `worker-first signals.spec.input(...) binds only to input ids from the active imported graph; \`${id}\` is not currently available`,
      );
    }
    const transactionOps = [buildActiveInputMutationOperation(id, mutation)];
    return this.applyImportMutation(
      this.#activeImportController,
      transactionOps,
      activeImportContext.definition.descriptors.map((entry) => entry.publishedId),
    );
  }

  async applyAuthoredInputMutation(id, mutation) {
    await this.ready();
    this.#requireActive("signals.inputAsync");
    return this.#applyWorkerOwnedTransaction(
      this.#authoredRuntime.authoredInputMutation(id, mutation),
    );
  }

  async #applyWorkerOwnedTransaction(transactionOps) {
    const activeImportController = this.#activeImportController;
    const activeImportContext = this.#activeImportContext;
    if (activeImportController === null || activeImportContext === null) {
      const transaction = await this.#bridge.applyTransaction(transactionOps);
      this.#authoredRuntime.applyCommittedInputs(transactionOps);
      await this.#authoredRuntime.refreshReadables();
      return transaction.runSummary;
    }
    return this.applyImportMutation(
      activeImportController,
      transactionOps,
      activeImportContext.definition.descriptors.map((entry) => entry.publishedId),
    );
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
}

function buildActiveInputMutationOperation(id, mutation) {
  if (!mutation || typeof mutation !== "object") {
    throw new TypeError("worker-first signals.spec.input mutation requires an operation object");
  }
  switch (mutation.kind) {
    case "set":
      return { kind: "set", id, value: mutation.value };
    case "reset":
      return { kind: "reset", id };
    case "patch":
      return { kind: "patch", id, value: mutation.value };
    default:
      throw new TypeError("worker-first signals.spec.input mutation kind is unsupported");
  }
}

function isWorkerRuntimeEnvelopeImportAdmitted(report) {
  return report?.importOutcome === "Admitted" || report?.importOutcome === "AdmittedExact";
}
