import {
  applyCommittedWorkerFirstAuthoredInputs,
  buildAuthoredInputMutationOperation,
  createAuthoredInputPublication,
  createWorkerFirstAuthoredInputState,
  hasMutableWorkerFirstAuthoredInputId,
  readWorkerFirstAuthoredInputBaseline,
  invalidateWorkerFirstAuthoredInputs,
  nextGeneratedStandaloneSignalId,
  readWorkerFirstAuthoredInputValue,
  writeWorkerFirstAuthoredInputBaseline,
} from "./worker_first_authored_input_state.js";
import {
  createAuthoredReadablePublication,
  createWorkerFirstAuthoredReadableState,
  hasWorkerFirstAuthoredReadableId,
  invalidateWorkerFirstAuthoredReadables,
  readWorkerFirstAuthoredReadableValue,
  updateWorkerFirstAuthoredReadables,
} from "./worker_first_authored_readable_state.js";
import {
  captureWorkerFirstAuthoredCallback,
  createWorkerFirstAuthoredCallbackState,
  nextWorkerFirstCallbackBackingInputId,
} from "./worker_first_authored_callback_authoring.js";
import { buildWorkerFirstHostDependencyReport } from "./worker_first_host_dependency_report.js";
import {
  refreshWorkerFirstAuthoredCallbackReadables,
  refreshWorkerFirstAuthoredReadableSignals,
} from "./worker_first_authored_readable_refresh.js";
import { materializeWorkerCachedValue } from "../worker_cached_value.js";
import { outputProjectionSpec } from "../../../../output_projection_ids.js";

export function createWorkerFirstRootAuthoredRuntime(
  bridge,
  activeImportContext,
  requireActive,
  runtimeMarker,
) {
  return new WorkerFirstRootAuthoredRuntime(
    bridge,
    activeImportContext,
    requireActive,
    runtimeMarker,
  );
}

class WorkerFirstRootAuthoredRuntime {
  #bridge;
  #activeImportContext;
  #requireActive;
  #runtimeMarker;
  #authoredInputs;
  #authoredReadables;
  #authoredCallbacks;
  #generatedStandaloneSignalCounters;
  #pendingPublications;

  constructor(bridge, activeImportContext, requireActive, runtimeMarker) {
    this.#bridge = bridge;
    this.#activeImportContext = activeImportContext;
    this.#requireActive = requireActive;
    this.#runtimeMarker = runtimeMarker;
    this.#authoredInputs = new Map();
    this.#authoredReadables = new Map();
    this.#authoredCallbacks = new Map();
    this.#generatedStandaloneSignalCounters = new Map();
    this.#pendingPublications = new Set();
  }

  nextGeneratedStandaloneSignalId(family, scopeId = null) {
    return nextGeneratedStandaloneSignalId(this.#generatedStandaloneSignalCounters, family, scopeId);
  }

  hasKnownSignalId(id) {
    return this.#authoredInputs.has(id)
      || hasWorkerFirstAuthoredReadableId(this.#authoredReadables, id)
      || this.#activeImportContext()?.signalValueById.has(id) === true;
  }

  hasMutableInputId(id) {
    return this.#activeImportContext()?.inputDescriptorBySourceId.has(id) === true
      || hasMutableWorkerFirstAuthoredInputId(this.#authoredInputs, id);
  }

  readSignalValue(id) {
    if (this.#authoredInputs.has(id)) {
      return readWorkerFirstAuthoredInputValue(this.#authoredInputs, id);
    }
    if (this.#authoredReadables.has(id)) {
      return readWorkerFirstAuthoredReadableValue(this.#authoredReadables, id);
    }
    const context = this.#activeImportContext();
    if (context?.signalValueById.has(id)) {
      return context.signalValueById.get(id);
    }
    throw new TypeError(
      `worker-first root read(${JSON.stringify(id)}) requires a currently available worker-first signal`,
    );
  }

  readAuthoredInputBaseline(id) { return readWorkerFirstAuthoredInputBaseline(this.#authoredInputs, id); }

  async createStandaloneInput(id, initial, options = {}) {
    this.#requireActive("inputAsync");
    if (typeof id !== "string" || id.length === 0) {
      throw new TypeError("worker-first inputAsync(...) requires a non-empty authored input id");
    }
    this.#assertUnusedId(id, "inputAsync");
    await this.#publishAuthoredInput(id, initial, options);
  }

  createEagerStandaloneInput(id, initial, options = {}) {
    this.#requireActive("input");
    if (typeof id !== "string" || id.length === 0) {
      throw new TypeError("worker-first input(...) requires a non-empty authored input id");
    }
    this.#assertUnusedId(id, "input");
    this.#authoredInputs.set(id, createWorkerFirstAuthoredInputState(initial));
    this.#trackPendingPublication(
      this.#bridge.publishPortableGraph(createAuthoredInputPublication(id, initial, options)),
      () => invalidateAuthoredInput(this.#authoredInputs, id, "worker-first input(...) background publication failed"),
    );
  }

  async createStandaloneReadable(id, family, spec) {
    this.#requireActive(`${family}Async`);
    if (typeof id !== "string" || id.length === 0) {
      throw new TypeError(`worker-first ${family}Async(...) requires a non-empty authored ${family} id`);
    }
    this.#assertUnusedId(id, `${family}Async`);
    this.#assertSupportedReadableSpec(family, spec);
    await this.#bridge.publishPortableGraph(createAuthoredReadablePublication(id, family, spec));
    const signalPacket = await this.#bridge.readSignals({ signalIds: [id] });
    const signal = signalPacket.signals[0];
    if (!signal || signal.id !== id) {
      throw new TypeError(
        `worker-first ${family}Async(...) could not read committed worker truth for \`${id}\` after authoring`,
      );
    }
    this.#authoredReadables.set(
      id,
      createWorkerFirstAuthoredReadableState(family, signal.value, spec.reads ?? []),
    );
  }

  createEagerStandaloneReadable(id, family, spec, initialValue, dependencyIds) {
    this.#requireActive(`${family}`);
    if (typeof id !== "string" || id.length === 0) {
      throw new TypeError(`worker-first ${family}(...) requires a non-empty authored ${family} id`);
    }
    this.#assertUnusedId(id, `${family}`);
    this.#assertSupportedReadableSpec(family, spec);
    this.#authoredReadables.set(
      id,
      createWorkerFirstAuthoredReadableState(family, initialValue, dependencyIds),
    );
    const initializePublishedReadable = spec?.when === undefined || spec?.when === null
      ? this.#bridge.publishPortableGraph(createAuthoredReadablePublication(id, family, spec))
      : this.#bridge.publishPortableGraph(createAuthoredReadablePublication(id, family, spec))
        .then(() => this.#bridge.readSignals({ signalIds: [id] }))
        .then((signalPacket) => {
          updateWorkerFirstAuthoredReadables(this.#authoredReadables, signalPacket.signals);
        });
    this.#trackPendingPublication(
      initializePublishedReadable,
      () => invalidateAuthoredReadable(this.#authoredReadables, id, `worker-first ${family}(...) background publication failed`),
    );
  }

  async createStandaloneCallbackReadable(id, family, callback) {
    this.#requireActive(`${family}Async`);
    if (typeof id !== "string" || id.length === 0) {
      throw new TypeError(`worker-first ${family}Async(...) requires a non-empty authored ${family} id`);
    }
    if (typeof callback !== "function") {
      throw new TypeError(`worker-first ${family}Async(...) callback form requires a function`);
    }
    this.#assertUnusedId(id, `${family}Async`);
    const capture = this.#captureCallback(callback, family);
    const hiddenInputId = nextWorkerFirstCallbackBackingInputId(
      this.#generatedStandaloneSignalCounters,
      family,
      id,
    );
    await this.#publishAuthoredInput(hiddenInputId, capture.value, {});
    await this.#bridge.publishPortableGraph(
      createAuthoredReadablePublication(id, family, outputProjectionSpec(hiddenInputId)),
    );
    const signalPacket = await this.#bridge.readSignals({ signalIds: [id] });
    const signal = signalPacket.signals[0];
    if (!signal || signal.id !== id) {
      throw new TypeError(
        `worker-first ${family}Async(...) could not read committed worker truth for \`${id}\` after callback authoring`,
      );
    }
    this.#authoredReadables.set(
      id,
      createWorkerFirstAuthoredReadableState(
        family,
        signal.value,
        capture.reads,
        capture.hostDependencyIds,
        capture.hostDependencies,
      ),
    );
    this.#authoredCallbacks.set(
      id,
      createWorkerFirstAuthoredCallbackState(family, callback, hiddenInputId, capture),
    );
  }

  createEagerStandaloneCallbackReadable(id, family, callback) {
    this.#requireActive(`${family}`);
    if (typeof id !== "string" || id.length === 0) {
      throw new TypeError(`worker-first ${family}(...) requires a non-empty authored ${family} id`);
    }
    if (typeof callback !== "function") {
      throw new TypeError(`worker-first ${family}(...) callback form requires a function`);
    }
    this.#assertUnusedId(id, `${family}`);
    const capture = this.#captureCallback(callback, family);
    const hiddenInputId = nextWorkerFirstCallbackBackingInputId(
      this.#generatedStandaloneSignalCounters,
      family,
      id,
    );
    this.#authoredInputs.set(hiddenInputId, createWorkerFirstAuthoredInputState(capture.value));
    this.#authoredReadables.set(
      id,
      createWorkerFirstAuthoredReadableState(
        family,
        capture.value,
        capture.reads,
        capture.hostDependencyIds,
        capture.hostDependencies,
      ),
    );
    this.#authoredCallbacks.set(
      id,
      createWorkerFirstAuthoredCallbackState(family, callback, hiddenInputId, capture),
    );
    this.#trackPendingPublication(
      this.#publishCallbackReadableGraph(id, family, hiddenInputId, capture.value),
      () => {
        invalidateAuthoredInput(this.#authoredInputs, hiddenInputId, `worker-first ${family}(...) background publication failed`);
        invalidateAuthoredReadable(this.#authoredReadables, id, `worker-first ${family}(...) background publication failed`);
      },
    );
  }

  beginAuthoredInputMutation(id, mutation) {
    const authoredInput = this.#authoredInputs.get(id);
    if (!authoredInput || authoredInput.invalidatedMessage !== null) {
      throw new TypeError(
        `worker-first inputAsync(...) can mutate only currently available worker-first authored inputs; \`${id}\` is not currently available`,
      );
    }
    const previousValue = authoredInput.currentValue;
    const transactionOp = buildAuthoredInputMutationOperation(id, mutation, authoredInput);
    authoredInput.currentValue = materializeWorkerCachedValue(transactionOp.value);
    return Object.freeze({
      transactionOps: [transactionOp],
      rollback() {
        authoredInput.currentValue = previousValue;
      },
    });
  }

  applyCommittedInputs(transactionOps) { applyCommittedWorkerFirstAuthoredInputs(this.#authoredInputs, transactionOps); }

  writeAuthoredInputBaseline(id, value) { writeWorkerFirstAuthoredInputBaseline(this.#authoredInputs, id, value); }

  async refreshReadables(changedIds = [], changedHostDependencyIds = []) {
    await refreshWorkerFirstAuthoredCallbackReadables({
      bridge: this.#bridge,
      authoredInputs: this.#authoredInputs,
      authoredReadables: this.#authoredReadables,
      authoredCallbacks: this.#authoredCallbacks,
      changedIds,
      changedHostDependencyIds,
      captureCallback: (callback, family) => this.#captureCallback(callback, family),
    });
    await refreshWorkerFirstAuthoredReadableSignals({
      bridge: this.#bridge,
      authoredReadables: this.#authoredReadables,
    });
  }

  hostDependencyReport() { return buildWorkerFirstHostDependencyReport(this.#authoredCallbacks); }

  invalidate(message) { invalidateWorkerFirstAuthoredInputs(this.#authoredInputs, message); invalidateWorkerFirstAuthoredReadables(this.#authoredReadables, message); }

  async settlePendingPublications() {
    while (this.#pendingPublications.size > 0) {
      await Promise.all([...this.#pendingPublications]);
    }
  }

  hasAuthoredSignalId(id) {
    return this.#authoredInputs.has(id)
      || hasWorkerFirstAuthoredReadableId(this.#authoredReadables, id);
  }

  async refreshAllReadables() {
    await refreshWorkerFirstAuthoredReadableSignals({
      bridge: this.#bridge,
      authoredReadables: this.#authoredReadables,
    });
    await refreshWorkerFirstAuthoredCallbackReadables({
      bridge: this.#bridge,
      authoredInputs: this.#authoredInputs,
      authoredReadables: this.#authoredReadables,
      authoredCallbacks: this.#authoredCallbacks,
      changedIds: null,
      changedHostDependencyIds: null,
      captureCallback: (callback, family) => this.#captureCallback(callback, family),
      skipDirectReadableRefresh: true,
    });
    await refreshWorkerFirstAuthoredReadableSignals({
      bridge: this.#bridge,
      authoredReadables: this.#authoredReadables,
    });
  }

  #assertUnusedId(id, operation) {
    if (
      this.#authoredInputs.has(id)
      || this.#authoredReadables.has(id)
      || this.#activeImportContext()?.signalValueById.has(id)
    ) {
      throw new TypeError(
        `worker-first ${operation}(...) cannot reuse canonical id \`${id}\` in the same worker-owned runtime`,
      );
    }
  }

  async #publishAuthoredInput(id, initial, options) {
    await this.#bridge.publishPortableGraph(createAuthoredInputPublication(id, initial, options));
    this.#authoredInputs.set(id, createWorkerFirstAuthoredInputState(initial));
  }

  async #publishCallbackReadableGraph(id, family, hiddenInputId, initialValue) {
    await this.#bridge.publishPortableGraph(createAuthoredInputPublication(hiddenInputId, initialValue, {}));
    await this.#bridge.publishPortableGraph(
      createAuthoredReadablePublication(id, family, outputProjectionSpec(hiddenInputId)),
    );
  }

  #assertSupportedReadableSpec(family, spec) {
    const reads = spec?.reads;
    if (reads === undefined) {
      return;
    }
    if (!Array.isArray(reads)) {
      throw new TypeError(`worker-first ${family}Async(...) requires spec.reads as an array when provided`);
    }
    for (const readId of reads) {
      if (typeof readId !== "string" || readId.length === 0) {
        throw new TypeError(
          `worker-first ${family}Async(...) requires every spec.reads entry to be a non-empty signal id`,
        );
      }
      if (!this.hasKnownSignalId(readId)) {
        throw new TypeError(
          `worker-first ${family}Async(...) can read only currently available worker-first signals; \`${readId}\` is not currently available`,
        );
      }
    }
  }

  #captureCallback(callback, family) {
    return captureWorkerFirstAuthoredCallback(
      this.#runtimeMarker,
      callback,
      family,
      (id) => this.hasKnownSignalId(id),
    );
  }

  #trackPendingPublication(publication, onFailure) {
    const tracked = Promise.resolve(publication).catch((error) => {
      onFailure(error);
    }).finally(() => {
      this.#pendingPublications.delete(tracked);
    });
    this.#pendingPublications.add(tracked);
  }
}

function invalidateAuthoredInput(authoredInputs, id, message) {
  const authoredInput = authoredInputs.get(id);
  if (authoredInput) {
    authoredInput.invalidatedMessage = message;
  }
}
