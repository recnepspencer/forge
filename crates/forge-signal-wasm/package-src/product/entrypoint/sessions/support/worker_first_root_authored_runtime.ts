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

export function createWorkerFirstRootAuthoredRuntime(bridge, activeImportContext, requireActive) {
  return new WorkerFirstRootAuthoredRuntime(bridge, activeImportContext, requireActive);
}

class WorkerFirstRootAuthoredRuntime {
  #bridge;
  #activeImportContext;
  #requireActive;
  #authoredInputs;
  #authoredReadables;
  #generatedStandaloneSignalCounters;

  constructor(bridge, activeImportContext, requireActive) {
    this.#bridge = bridge;
    this.#activeImportContext = activeImportContext;
    this.#requireActive = requireActive;
    this.#authoredInputs = new Map();
    this.#authoredReadables = new Map();
    this.#generatedStandaloneSignalCounters = new Map();
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

  readAuthoredInputBaseline(id) {
    return readWorkerFirstAuthoredInputBaseline(this.#authoredInputs, id);
  }

  async createStandaloneInput(id, initial, options = {}) {
    this.#requireActive("inputAsync");
    if (typeof id !== "string" || id.length === 0) {
      throw new TypeError("worker-first inputAsync(...) requires a non-empty authored input id");
    }
    if (
      this.#authoredInputs.has(id)
      || this.#authoredReadables.has(id)
      || this.#activeImportContext()?.signalValueById.has(id)
    ) {
      throw new TypeError(
        `worker-first inputAsync(...) cannot reuse canonical id \`${id}\` in the same worker-owned runtime`,
      );
    }
    await this.#bridge.publishPortableGraph(createAuthoredInputPublication(id, initial, options));
    this.#authoredInputs.set(id, createWorkerFirstAuthoredInputState(initial));
  }

  async createStandaloneReadable(id, family, spec) {
    this.#requireActive(`${family}Async`);
    if (typeof id !== "string" || id.length === 0) {
      throw new TypeError(`worker-first ${family}Async(...) requires a non-empty authored ${family} id`);
    }
    if (
      this.#authoredInputs.has(id)
      || this.#authoredReadables.has(id)
      || this.#activeImportContext()?.signalValueById.has(id)
    ) {
      throw new TypeError(
        `worker-first ${family}Async(...) cannot reuse canonical id \`${id}\` in the same worker-owned runtime`,
      );
    }
    this.#assertSupportedReadableSpec(family, spec);
    await this.#bridge.publishPortableGraph(createAuthoredReadablePublication(id, family, spec));
    const signalPacket = await this.#bridge.readSignals({ signalIds: [id] });
    const signal = signalPacket.signals[0];
    if (!signal || signal.id !== id) {
      throw new TypeError(
        `worker-first ${family}Async(...) could not read committed worker truth for \`${id}\` after authoring`,
      );
    }
    this.#authoredReadables.set(id, createWorkerFirstAuthoredReadableState(family, signal.value));
  }

  authoredInputMutation(id, mutation) {
    const authoredInput = this.#authoredInputs.get(id);
    if (!authoredInput || authoredInput.invalidatedMessage !== null) {
      throw new TypeError(
        `worker-first inputAsync(...) can mutate only currently available worker-first authored inputs; \`${id}\` is not currently available`,
      );
    }
    return [buildAuthoredInputMutationOperation(id, mutation, authoredInput)];
  }

  applyCommittedInputs(transactionOps) {
    applyCommittedWorkerFirstAuthoredInputs(this.#authoredInputs, transactionOps);
  }

  writeAuthoredInputBaseline(id, value) {
    writeWorkerFirstAuthoredInputBaseline(this.#authoredInputs, id, value);
  }

  async refreshReadables() {
    const activeReadableIds = [...this.#authoredReadables.entries()]
      .filter(([, authoredReadable]) => authoredReadable.invalidatedMessage === null)
      .map(([id]) => id);
    if (activeReadableIds.length === 0) {
      return;
    }
    const signalPacket = await this.#bridge.readSignals({
      signalIds: activeReadableIds,
    });
    updateWorkerFirstAuthoredReadables(this.#authoredReadables, signalPacket.signals);
  }

  invalidate(message) {
    invalidateWorkerFirstAuthoredInputs(this.#authoredInputs, message);
    invalidateWorkerFirstAuthoredReadables(this.#authoredReadables, message);
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
}
