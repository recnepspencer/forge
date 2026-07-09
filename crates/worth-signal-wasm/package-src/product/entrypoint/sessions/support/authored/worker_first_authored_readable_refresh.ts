import {
  applyCommittedWorkerFirstAuthoredInputs,
} from "./worker_first_authored_input_state.js";
import {
  invalidateWorkerFirstAuthoredReadables,
  updateWorkerFirstAuthoredReadables,
} from "./worker_first_authored_readable_state.js";
import { hostDependenciesIntersect } from "./worker_first_host_dependency_records.js";
import { materializeWorkerCachedValue } from "../worker_cached_value.js";

export async function refreshWorkerFirstAuthoredCallbackReadables(deps) {
  const {
    bridge,
    authoredInputs,
    authoredReadables,
    authoredCallbacks,
    changedIds,
    changedHostDependencyIds = [],
    captureCallback,
    skipDirectReadableRefresh = false,
  } = deps;
  const refreshAll = !Array.isArray(changedIds) || !Array.isArray(changedHostDependencyIds);
  if (
    !refreshAll
    && changedIds.length === 0
    && changedHostDependencyIds.length === 0
  ) {
    return;
  }
  const changedIdSet = refreshAll ? new Set() : new Set(changedIds);
  const changedHostDependencyIdSet = refreshAll ? new Set() : new Set(changedHostDependencyIds);
  const directlyChangedReadableIds = [];
  for (const [id, authoredReadable] of authoredReadables) {
    if (authoredReadable.invalidatedMessage !== null) {
      continue;
    }
    if (
      refreshAll
      || authoredReadable.dependencyIds.some((dependencyId) => changedIdSet.has(dependencyId))
    ) {
      directlyChangedReadableIds.push(id);
      changedIdSet.add(id);
    }
  }
  if (!skipDirectReadableRefresh && directlyChangedReadableIds.length > 0) {
    await refreshWorkerFirstAuthoredReadableIds({
      bridge,
      authoredReadables,
      readableIds: directlyChangedReadableIds,
      invalidationMessage: "worker-first authored readable no longer exists after worker runtime history mutation",
    });
  }
  const hiddenTransactionOps = [];
  for (const [id, authoredCallback] of authoredCallbacks) {
    const authoredReadable = authoredReadables.get(id);
    if (!authoredReadable || authoredReadable.invalidatedMessage !== null) {
      continue;
    }
    if (
      !refreshAll
      && !authoredCallback.dependencyIds.some((dependencyId) => changedIdSet.has(dependencyId))
      && !hostDependenciesIntersect(
        authoredCallback.hostDependencyIds,
        changedHostDependencyIdSet,
      )
    ) {
      continue;
    }
    const capture = captureCallback(authoredCallback.callback, authoredCallback.family);
    authoredCallback.dependencyIds = capture.reads;
    authoredCallback.hostDependencyIds = capture.hostDependencyIds;
    authoredCallback.hostDependencies = capture.hostDependencies;
    const nextValue = materializeWorkerCachedValue(capture.value);
    authoredReadable.currentValue = nextValue;
    authoredReadable.dependencyIds = [...capture.reads];
    authoredReadable.hostDependencyIds = [...capture.hostDependencyIds];
    authoredReadable.hostDependencies = [...capture.hostDependencies];
    hiddenTransactionOps.push({
      kind: "set",
      id: authoredCallback.hiddenInputId,
      value: nextValue,
    });
    changedIdSet.add(id);
    changedIdSet.add(authoredCallback.hiddenInputId);
  }
  if (hiddenTransactionOps.length === 0) {
    return;
  }
  await bridge.applyTransaction(hiddenTransactionOps);
  applyCommittedWorkerFirstAuthoredInputs(authoredInputs, hiddenTransactionOps);
}

export async function refreshWorkerFirstAuthoredReadableSignals(deps) {
  const activeReadableIds = [...deps.authoredReadables.entries()]
    .filter(([, authoredReadable]) => authoredReadable.invalidatedMessage === null)
    .map(([id]) => id);
  if (activeReadableIds.length === 0) {
    return;
  }
  await refreshWorkerFirstAuthoredReadableIds({
    bridge: deps.bridge,
    authoredReadables: deps.authoredReadables,
    readableIds: activeReadableIds,
    invalidationMessage: "worker-first authored readable no longer exists after worker runtime mutation",
  });
}

async function refreshWorkerFirstAuthoredReadableIds(deps) {
  const refreshedSignals = [];
  for (const readableId of deps.readableIds) {
    try {
      const signalPacket = await deps.bridge.readSignals({
        signalIds: [readableId],
      });
      refreshedSignals.push(...signalPacket.signals);
    } catch (error) {
      if (isUnknownSignalIdError(error, readableId)) {
        invalidateAuthoredReadable(deps.authoredReadables, readableId, deps.invalidationMessage);
        continue;
      }
      throw error;
    }
  }
  if (refreshedSignals.length > 0) {
    updateWorkerFirstAuthoredReadables(deps.authoredReadables, refreshedSignals);
  }
}

function invalidateAuthoredReadable(authoredReadables, id, message) {
  const authoredReadable = authoredReadables.get(id);
  if (authoredReadable) {
    authoredReadable.invalidatedMessage = message;
  }
}

function isUnknownSignalIdError(error, signalId) {
  const detail = error instanceof Error ? error.message : String(error);
  return detail.includes("unknown signal id") && detail.includes(signalId);
}
