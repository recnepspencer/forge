import type {
  DisposableHandleLike,
  ReactPerformanceSummary,
  ReactSignalsStore,
  SignalHandleLike,
  SignalsDiagnosticsSnapshot,
  SignalsLike,
} from "./model.js";

const STORE_PATCH_STATE = new WeakMap<
  SignalsLike,
  {
    stores: Set<{ scheduleDiagnosticsRefresh: () => void }>;
    originalTransaction: SignalsLike["transaction"];
    originalBatch: SignalsLike["batch"];
  }
>();

type SignalEntry = {
  id: string;
  target: SignalHandleLike | string;
  version: number;
  snapshotVersion: number;
  snapshot: unknown;
  runtimeHandle: DisposableHandleLike | null;
  notifyQueued: boolean;
  establishingSubscription: boolean;
  listeners: Set<() => void>;
};

function resolveSignalId(target: SignalHandleLike | string): string {
  if (typeof target === "string") {
    return target;
  }
  if (target && typeof target.id === "string") {
    return target.id;
  }
  throw new TypeError("signal target must be a string id or signal handle");
}

function readSignalValue(
  signals: SignalsLike,
  compatibilityApp: ReturnType<SignalsLike["compatibilityApp"]>,
  target: SignalHandleLike | string,
): unknown {
  if (typeof target === "string") {
    return compatibilityApp.read(target);
  }
  if (target && typeof target.get === "function") {
    return target.get();
  }
  throw new TypeError("signal target must expose a get() method or a string id");
}

function createDiagnosticsSnapshot(signals: SignalsLike): SignalsDiagnosticsSnapshot {
  const diagnostics = signals.diagnostics();
  return Object.freeze({
    latestObservation: diagnostics.latestObservation(),
    latestFlow: diagnostics.latestFlow(),
    performanceSummary: diagnostics.performanceSummary(),
  });
}

function createReactPerformanceSummary(
  signalEntries: Map<string, SignalEntry>,
  diagnosticsListeners: Set<() => void>,
): ReactPerformanceSummary {
  let activeReactSubscriberCount = 0;
  let activeRuntimeWatchHandleCount = 0;

  for (const entry of signalEntries.values()) {
    activeReactSubscriberCount += entry.listeners.size;
    if (entry.runtimeHandle) {
      activeRuntimeWatchHandleCount += 1;
    }
  }

  const activeSignalSubscriptionCount = signalEntries.size;
  const sharedFanoutRatio =
    activeRuntimeWatchHandleCount === 0
      ? 0
      : activeReactSubscriberCount / activeRuntimeWatchHandleCount;

  return Object.freeze({
    activeSignalSubscriptionCount,
    activeReactSubscriberCount,
    activeRuntimeWatchHandleCount,
    diagnosticsSubscriberCount: diagnosticsListeners.size,
    sharedFanoutRatio,
  });
}

function notifyAll(listeners: Set<() => void>): void {
  for (const listener of Array.from(listeners)) {
    listener();
  }
}

function enqueueMicrotask(callback: () => void): void {
  if (typeof queueMicrotask === "function") {
    queueMicrotask(callback);
    return;
  }
  Promise.resolve().then(callback);
}

export function createReactSignalsStore(signals: SignalsLike): ReactSignalsStore {
  const compatibilityApp = signals.compatibilityApp();
  const signalEntries = new Map<string, SignalEntry>();
  const diagnosticsListeners = new Set<() => void>();
  let diagnosticsSnapshot = createDiagnosticsSnapshot(signals);
  let diagnosticsRefreshQueued = false;

  function scheduleDiagnosticsRefresh(): void {
    if (diagnosticsRefreshQueued) {
      return;
    }
    diagnosticsRefreshQueued = true;
    enqueueMicrotask(() => {
      diagnosticsRefreshQueued = false;
      diagnosticsSnapshot = createDiagnosticsSnapshot(signals);
      notifyAll(diagnosticsListeners);
    });
  }

  let patchState = STORE_PATCH_STATE.get(signals);
  if (!patchState) {
    patchState = {
      stores: new Set(),
      originalTransaction: signals.transaction.bind(signals),
      originalBatch: signals.batch.bind(signals),
    };
    STORE_PATCH_STATE.set(signals, patchState);
    const newPatchState = patchState;

    signals.transaction = ((callback) => {
      const summary = newPatchState.originalTransaction(callback);
      for (const store of newPatchState.stores) {
        store.scheduleDiagnosticsRefresh();
      }
      return summary;
    }) as SignalsLike["transaction"];

    signals.batch = ((callback) => {
      const summary = newPatchState.originalBatch(callback);
      for (const store of newPatchState.stores) {
        store.scheduleDiagnosticsRefresh();
      }
      return summary;
    }) as SignalsLike["batch"];
  }
  const ensuredPatchState = patchState;
  const storeState = { scheduleDiagnosticsRefresh };
  ensuredPatchState.stores.add(storeState);

  function ensureSignalEntry(target: SignalHandleLike | string): SignalEntry {
    const id = resolveSignalId(target);
    let entry = signalEntries.get(id);
    if (!entry) {
      entry = {
        id,
        target,
        version: 0,
        snapshotVersion: -1,
        snapshot: undefined,
        runtimeHandle: null,
        notifyQueued: false,
        establishingSubscription: false,
        listeners: new Set(),
      };
      signalEntries.set(id, entry);
    } else {
      entry.target = target;
    }
    return entry;
  }

  function subscribeSignal(target: SignalHandleLike | string, listener: () => void): () => void {
    const entry = ensureSignalEntry(target);

    if (!entry.runtimeHandle) {
      entry.establishingSubscription = true;
      entry.runtimeHandle = signals.watch(entry.target, (notice) => {
        if (entry.establishingSubscription) {
          return;
        }
        if (!notice.triggerMatched || !notice.meaningfulChange) {
          return;
        }
        entry.version += 1;
        entry.snapshotVersion = -1;
        if (!entry.notifyQueued) {
          entry.notifyQueued = true;
          enqueueMicrotask(() => {
            entry.notifyQueued = false;
            notifyAll(entry.listeners);
          });
        }
      });
      entry.establishingSubscription = false;
    }

    entry.listeners.add(listener);

    return () => {
      const current = signalEntries.get(entry.id);
      if (!current) {
        return;
      }

      current.listeners.delete(listener);

      if (current.listeners.size === 0) {
        if (current.runtimeHandle) {
          signals.nuke(current.runtimeHandle);
        }
        signalEntries.delete(current.id);
      }
    };
  }

  function getSignalSnapshot(target: SignalHandleLike | string): unknown {
    const entry = ensureSignalEntry(target);
    if (entry.snapshotVersion !== entry.version) {
      entry.snapshot = readSignalValue(signals, compatibilityApp, entry.target);
      entry.snapshotVersion = entry.version;
    }
    return entry.snapshot;
  }

  function subscribeDiagnostics(listener: () => void): () => void {
    diagnosticsListeners.add(listener);
    return () => {
      diagnosticsListeners.delete(listener);
    };
  }

  function getDiagnosticsSnapshot(): SignalsDiagnosticsSnapshot {
    return diagnosticsSnapshot;
  }

  function transaction(callback: Parameters<SignalsLike["transaction"]>[0]): unknown {
    return signals.transaction(callback);
  }

  function batch(callback: Parameters<SignalsLike["batch"]>[0]): unknown {
    return signals.batch(callback);
  }

  function refreshDiagnostics(): SignalsDiagnosticsSnapshot {
    diagnosticsSnapshot = createDiagnosticsSnapshot(signals);
    notifyAll(diagnosticsListeners);
    return diagnosticsSnapshot;
  }

  function performanceSummary(): ReactPerformanceSummary {
    return createReactPerformanceSummary(signalEntries, diagnosticsListeners);
  }

  function dispose(): void {
    for (const entry of signalEntries.values()) {
      if (entry.runtimeHandle) {
        signals.nuke(entry.runtimeHandle);
      }
    }
    signalEntries.clear();
    diagnosticsListeners.clear();

    ensuredPatchState.stores.delete(storeState);
    if (ensuredPatchState.stores.size === 0) {
      signals.transaction = ensuredPatchState.originalTransaction;
      signals.batch = ensuredPatchState.originalBatch;
      STORE_PATCH_STATE.delete(signals);
    }
  }

  return Object.freeze({
    signals,
    subscribeSignal,
    getSignalSnapshot,
    subscribeDiagnostics,
    getDiagnosticsSnapshot,
    transaction,
    batch,
    refreshDiagnostics,
    performanceSummary,
    dispose,
  });
}
