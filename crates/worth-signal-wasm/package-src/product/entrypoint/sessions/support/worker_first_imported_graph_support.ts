import {
  freezeObject,
  requireGraphExportDefinition,
  requireGraphExportSnapshot,
  requireMatchingGraphExportPair,
} from "../../../graph_support.js";
import { PRODUCT_SIGNAL_KIND } from "../../../symbols.js";
import {
  denyWorkerFirstMutationDuringCallbackAuthoring,
  readWorkerFirstTrackedSignal,
} from "../../worker_first_callback_tracking.js";
import { cloneWorkerCachedValue } from "./worker_cached_value.js";

export function normalizeImportedGraphSessionOptions(options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError("createWorkerFirstImportedGraphSession(...) expects an options object");
  }
  const { definition, snapshot, workerUrl, ...unknownOptions } = options;
  const unknownKeys = Object.keys(unknownOptions);
  if (unknownKeys.length > 0) {
    throw new TypeError(
      `createWorkerFirstImportedGraphSession(...) does not support: ${unknownKeys.join(", ")}`,
    );
  }
  const normalizedDefinition = requireGraphExportDefinition(definition);
  const normalizedSnapshot = requireGraphExportSnapshot(snapshot, normalizedDefinition.id);
  requireMatchingGraphExportPair(normalizedDefinition, normalizedSnapshot);
  if (!normalizedSnapshot.runtimeEnvelope || typeof normalizedSnapshot.runtimeEnvelope !== "object") {
    throw new TypeError(
      `createWorkerFirstImportedGraphSession(...) requires snapshot.runtimeEnvelope for graph \`${normalizedDefinition.id}\``,
    );
  }
  return freezeObject({
    definition: normalizedDefinition,
    snapshot: normalizedSnapshot,
    trackedInputIds: freezeObject(normalizedDefinition.inputDescriptors.map((entry) => entry.sourceId)),
    trackedOutputIds: freezeObject(normalizedDefinition.descriptors.map((entry) => entry.publishedId)),
    workerUrl: workerUrl === undefined ? null : workerUrl,
  });
}

export function buildImportedSignalRecord(runtimeMarker, descriptors, nameField, idFor, read) {
  const record = Object.create(null);
  for (const descriptor of descriptors) {
    const id = idFor(descriptor);
    record[descriptor[nameField]] = createWorkerImportedReadableSignal(
      runtimeMarker,
      id,
      descriptor.sourceKind,
      () => read(descriptor),
    );
  }
  return freezeObject(record);
}

export function buildImportedInputSignalRecord(
  runtimeMarker,
  descriptors,
  nameField,
  idFor,
  read,
  mutate,
) {
  const record = Object.create(null);
  for (const descriptor of descriptors) {
    const inputName = descriptor[nameField];
    const id = idFor(descriptor);
    record[inputName] = createWorkerImportedInputSignal(
      runtimeMarker,
      id,
      () => read(descriptor),
      {
        set: (value) => mutate({ writes: { [inputName]: value } }),
        reset: () => mutate({ reset: [inputName] }),
        patch: (value) => mutate({ patches: { [inputName]: value } }),
        assign: (fields) => mutate({ patches: { [inputName]: fields } }),
      },
    );
  }
  return freezeObject(record);
}

export function buildImportedGraphHydrationTransaction(snapshot, trackedInputIds, graphId) {
  const sourceValues = new Map(
    (snapshot?.snapshotEnvelope?.state?.sources ?? []).map((source) => [source?.id, source?.value]),
  );
  const transactionOps = [];
  for (const sourceId of trackedInputIds) {
    if (!sourceValues.has(sourceId)) {
      throw new TypeError(
        `createWorkerFirstImportedGraphSession(...) requires snapshot source state for tracked input \`${sourceId}\` in graph \`${graphId}\``,
      );
    }
    transactionOps.push({
      kind: "set",
      id: sourceId,
      value: cloneWorkerCachedValue(sourceValues.get(sourceId)),
    });
  }
  return freezeObject(transactionOps);
}

export function buildWorkerHydratedImportPosture(snapshot, graphId) {
  return freezeObject({
    ...snapshot.importPosture,
    graphId,
    hydrate: "Applied",
    hydrateReason:
      "worker-first imported graph hydrated tracked public inputs from exported snapshot state",
  });
}

export function buildWorkerPendingImportPosture(snapshot, graphId) {
  return freezeObject({
    ...snapshot.importPosture,
    graphId,
    hydrate: "Deferred",
    hydrateReason:
      "worker-first imported graph requires await importedGraph.ready() before tracked public inputs are hydrated into worker-owned truth",
  });
}

function createWorkerImportedReadableSignal(runtimeMarker, id, kind, read) {
  const trackedRead = () => readWorkerFirstTrackedSignal(runtimeMarker, id, read);
  const handle = function workerImportedSignal() {
    return trackedRead();
  };
  handle.get = trackedRead;
  handle.value = trackedRead;
  handle.free = () => {};
  handle[Symbol.dispose] = () => {};
  handle.id = id;
  handle.debugName = null;
  handle[PRODUCT_SIGNAL_KIND] = kind === "input" || kind === "computed" || kind === "output"
    ? kind
    : "signal";
  return freezeObject(handle);
}

function createWorkerImportedInputSignal(runtimeMarker, id, read, mutationMethods) {
  const trackedRead = () => readWorkerFirstTrackedSignal(runtimeMarker, id, read);
  const handle = function workerImportedInputSignal() {
    return trackedRead();
  };
  handle.get = trackedRead;
  handle.value = trackedRead;
  handle.free = () => {};
  handle[Symbol.dispose] = () => {};
  handle.id = id;
  handle.debugName = null;
  handle[PRODUCT_SIGNAL_KIND] = "input";
  handle.set = (value) => {
    denyWorkerFirstMutationDuringCallbackAuthoring();
    return mutationMethods.set(value);
  };
  handle.reset = () => {
    denyWorkerFirstMutationDuringCallbackAuthoring();
    return mutationMethods.reset();
  };
  handle.patch = (value) => {
    denyWorkerFirstMutationDuringCallbackAuthoring();
    return mutationMethods.patch(value);
  };
  handle.assign = (fields) => {
    denyWorkerFirstMutationDuringCallbackAuthoring();
    return mutationMethods.assign(fields);
  };
  return freezeObject(handle);
}
