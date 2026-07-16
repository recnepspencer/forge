import { buildActiveImportContext } from "./sessions/support/worker_first_root_import_context.js";

export function createWorkerFirstRootMutation(deps) {
  const pendingMutations = new Set();
  let operationTail = Promise.resolve();
  const trackMutation = (operation) => {
    const mutation = operationTail.then(operation, operation);
    operationTail = mutation.catch(() => {});
    const tracked = Promise.resolve(mutation).finally(() => {
      pendingMutations.delete(tracked);
    });
    pendingMutations.add(tracked);
    return tracked;
  };
  return Object.freeze({
    applyImportMutation(controller, transactionOps, outputIds) {
      return trackMutation(() => applyImportMutation(deps, controller, transactionOps, outputIds));
    },
    applyActiveTransaction(transactionOps) {
      return trackMutation(() => applyActiveTransaction(deps, transactionOps));
    },
    applyActiveInputMutation(id, mutation) {
      return trackMutation(() => applyActiveInputMutation(deps, id, mutation));
    },
    applyAuthoredInputMutation(id, mutation) {
      return trackMutation(() => applyAuthoredInputMutation(deps, id, mutation));
    },
    async settlePendingMutations() {
      while (pendingMutations.size > 0) {
        await Promise.all([...pendingMutations]);
      }
    },
  });
}

async function applyImportMutation(deps, controller, transactionOps, outputIds) {
  await deps.ready();
  deps.requireActive("importedGraph.apply");
  deps.requireControllerActive(controller, "importedGraph.apply");
  await deps.observations.syncLifecycle(deps.bridge);
  const projectionPacket = await deps.bridge.applyTransactionProjection({
    transactionOps,
    outputIds,
  });
  deps.requireControllerActive(controller, "importedGraph.apply");
  const activeImportController = deps.activeImportController();
  const activeImportContext = deps.activeImportContext();
  if (activeImportController === null || activeImportContext === null) {
    throw new TypeError(
      "worker-first root importedGraph.apply() requires an active imported graph context",
    );
  }
  deps.setActiveImportContext(
    await buildActiveImportContext(
      deps.bridge,
      activeImportContext.definition,
      activeImportContext.snapshot,
    ),
  );
  await deps.refreshBranchCache();
  const deliveryPacket = await deps.observations.syncLifecycle(deps.bridge)
    .then(() => deps.bridge.deliverLatestObservation())
    .catch(() => null);
  await deps.observations.replaceContext(
    deps.bridge,
    deps.activeImportContext(),
    deliveryPacket,
  );
  if (typeof activeImportController.refreshFromRootRuntime === "function") {
    await activeImportController.refreshFromRootRuntime();
  }
  deps.authoredRuntime.applyCommittedInputs(transactionOps);
  await deps.authoredRuntime.refreshReadables(extractChangedSignalIds(transactionOps));
  deps.requireControllerActive(activeImportController, "importedGraph.apply");
  return projectionPacket.transaction.runSummary;
}

async function applyActiveTransaction(deps, transactionOps) {
  await deps.ready();
  deps.requireActive("transactionAsync");
  if (!Array.isArray(transactionOps)) {
    throw new TypeError("worker-first root transactionAsync(...) requires transactionOps as an array");
  }
  for (const op of transactionOps) {
    if (!op || typeof op !== "object" || typeof op.id !== "string") {
      throw new TypeError(
        "worker-first root transactionAsync(...) encountered an invalid input mutation operation",
      );
    }
    if (!deps.hasMutableInputId(op.id)) {
      throw new TypeError(
        `worker-first root transactionAsync(...) can mutate only currently available worker-first inputs; \`${op.id}\` is not currently available`,
      );
    }
  }
  return applyWorkerOwnedTransaction(deps, transactionOps);
}

async function applyActiveInputMutation(deps, id, mutation) {
  await deps.ready();
  deps.requireActive("signals.spec.input");
  const activeImportContext = deps.currentImportContext();
  const inputDescriptor = activeImportContext.definition.inputDescriptors.find(
    (entry) => entry.sourceId === id,
  );
  if (!inputDescriptor) {
    throw new TypeError(
      `worker-first signals.spec.input(...) binds only to input ids from the active imported graph; \`${id}\` is not currently available`,
    );
  }
  const transactionOps = [buildActiveInputMutationOperation(id, mutation)];
  return applyImportMutation(
    deps,
    deps.activeImportController(),
    transactionOps,
    activeImportContext.definition.descriptors.map((entry) => entry.publishedId),
  );
}

function applyAuthoredInputMutation(deps, id, mutation) {
  deps.requireActive("signals.inputAsync");
  const pendingMutation = deps.authoredRuntime.beginAuthoredInputMutation(id, mutation);
  return (async () => {
    try {
      await deps.ready();
      return await applyWorkerOwnedTransaction(
        deps,
        pendingMutation.transactionOps,
      );
    } catch (error) {
      pendingMutation.rollback();
      throw error;
    }
  })();
}

async function applyWorkerOwnedTransaction(deps, transactionOps) {
  await deps.authoredRuntime.settlePendingPublications();
  const activeImportController = deps.activeImportController();
  const activeImportContext = deps.activeImportContext();
  if (activeImportController === null || activeImportContext === null) {
    await deps.observations.syncLifecycle(deps.bridge);
    const transaction = await deps.bridge.applyTransaction(transactionOps);
    await deps.refreshBranchCache();
    deps.authoredRuntime.applyCommittedInputs(transactionOps);
    await deps.authoredRuntime.refreshReadables(extractChangedSignalIds(transactionOps));
    const deliveryPacket = await deps.bridge.deliverLatestObservation().catch(() => null);
    deps.observations.deliverCurrent(deliveryPacket);
    return transaction.runSummary;
  }
  return applyImportMutation(
    deps,
    activeImportController,
    transactionOps,
    activeImportContext.definition.descriptors.map((entry) => entry.publishedId),
  );
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

function extractChangedSignalIds(transactionOps) {
  if (!Array.isArray(transactionOps)) {
    return [];
  }
  return transactionOps
    .filter((op) => op && typeof op.id === "string")
    .map((op) => op.id);
}
