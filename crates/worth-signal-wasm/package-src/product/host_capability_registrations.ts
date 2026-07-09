import { recordHostCapabilityRead } from "./callback_frames.js";
import {
  cloneSignalValue,
  HOST_CLOCK_HANDLE_BRAND,
  HOST_ONLINE_HANDLE_BRAND,
  HOST_PERSISTENCE_HANDLE_BRAND,
  HOST_VIEWPORT_HANDLE_BRAND,
  HOST_VISIBILITY_HANDLE_BRAND,
  nextHiddenHostSignalId,
  normalizeBinaryCapabilityState,
  normalizeClockValue,
  normalizeSubscription,
  normalizeViewportState,
} from "./host_capability_declarations.js";
import { wrapReadableSignal } from "./handles.js";
import { RAW_SIGNAL_HANDLE } from "./symbols.js";

function summarizePushMetrics(metrics) {
  return {
    hostCapabilityRegistrationCount: metrics.registrationCount,
    hostCapabilityDisposalCount: metrics.disposalCount,
    hostCapabilityReadCount: metrics.readCount,
    hostCapabilityPollCount: metrics.pollCount ?? 0,
    hostCapabilityNoOpPollCount: metrics.noOpPollCount ?? 0,
    hostCapabilityManualCommitCount: metrics.manualCommitCount ?? 0,
    hostCapabilityNoOpManualCommitCount: metrics.noOpManualCommitCount ?? 0,
    hostCapabilityInvalidationCount: metrics.invalidationCount,
    hostCapabilityInvalidationBatchFlushCount: metrics.invalidationBatchFlushCount,
    hostCapabilityReevaluationCount: metrics.reevaluationCount,
    hostCapabilityInvalidationTouchedNodeCount: metrics.invalidationTouchedNodeCount,
    hostCapabilityNoOpInvalidationSuppressedCount: metrics.noOpInvalidationSuppressedCount,
    hostCapabilityStaleInvalidationIgnoredCount: metrics.staleInvalidationIgnoredCount,
    hostCapabilityCompatibilityDenialCount: metrics.compatibilityDenialCount,
  };
}

export function registerViewportCapability(rawSignals, registration, diagnosticsRecorder) {
  const hiddenSignalId = nextHiddenHostSignalId(rawSignals, "viewport");
  let committedState = normalizeViewportState(registration.source.current());
  const rawHiddenSignal = rawSignals.input(hiddenSignalId, committedState);
  const hiddenSignal = wrapReadableSignal(rawHiddenSignal, rawSignals, "hostCapability");
  const descriptor = Object.freeze({
    family: "viewport",
    compatibility: registration.compatibility,
    registrationId: "viewport",
  });
  const metrics = {
    registrationCount: 1,
    disposalCount: 0,
    readCount: 0,
    invalidationCount: 0,
    invalidationBatchFlushCount: 0,
    reevaluationCount: 0,
    invalidationTouchedNodeCount: 0,
    noOpInvalidationSuppressedCount: 0,
    staleInvalidationIgnoredCount: 0,
    compatibilityDenialCount: 0,
  };
  let scheduled = false;
  let queuedState = committedState;
  let queuedInvalidationCount = 0;
  let disposed = false;

  function readViewportSize() {
    recordHostCapabilityRead(rawSignals, descriptor);
    metrics.readCount += 1;
    return hiddenSignal();
  }

  function flushQueuedInvalidation() {
    scheduled = false;
    const flushedInvalidationCount = queuedInvalidationCount;
    queuedInvalidationCount = 0;
    if (disposed) {
      metrics.staleInvalidationIgnoredCount += 1;
      diagnosticsRecorder.push({
        kind: "InvalidationIgnoredStale",
        family: descriptor.family,
        registrationId: descriptor.registrationId,
        compatibility: descriptor.compatibility,
        invalidationMode: "push-driven",
        queuedInvalidationCount: flushedInvalidationCount,
        previousState: committedState,
        nextState: queuedState,
        touchedNodes: 0,
        reevaluatedNodes: 0,
      });
      return;
    }
    if (queuedState.width === committedState.width && queuedState.height === committedState.height) {
      metrics.noOpInvalidationSuppressedCount += 1;
      diagnosticsRecorder.push({
        kind: "InvalidationNoOpSuppressed",
        family: descriptor.family,
        registrationId: descriptor.registrationId,
        compatibility: descriptor.compatibility,
        invalidationMode: "push-driven",
        queuedInvalidationCount: flushedInvalidationCount,
        previousState: committedState,
        nextState: queuedState,
        touchedNodes: 0,
        reevaluatedNodes: 0,
      });
      return;
    }
    metrics.invalidationBatchFlushCount += 1;
    const previousState = committedState;
    const result = rawSignals.transaction((tx) => {
      tx.set(hiddenSignal[RAW_SIGNAL_HANDLE], queuedState);
    });
    committedState = queuedState;
    const touchedNodes = typeof result?.touchedNodes === "number"
      ? Math.max(0, result.touchedNodes)
      : 0;
    const reevaluatedNodes = typeof result?.nodesRecomputed === "number"
      ? Math.max(0, result.nodesRecomputed)
      : touchedNodes;
    metrics.reevaluationCount += reevaluatedNodes;
    metrics.invalidationTouchedNodeCount += touchedNodes;
    diagnosticsRecorder.push({
      kind: "InvalidationFlushed",
      family: descriptor.family,
      registrationId: descriptor.registrationId,
      compatibility: descriptor.compatibility,
      invalidationMode: "push-driven",
      queuedInvalidationCount: flushedInvalidationCount,
      previousState,
      nextState: queuedState,
      touchedNodes,
      reevaluatedNodes,
    });
  }

  function scheduleFlush() {
    if (!scheduled) {
      scheduled = true;
      queueMicrotask(flushQueuedInvalidation);
    }
  }

  const unsubscribe = normalizeSubscription(
    registration.source.subscribe(() => {
      if (disposed) {
        metrics.staleInvalidationIgnoredCount += 1;
        diagnosticsRecorder.push({
          kind: "InvalidationIgnoredStale",
          family: descriptor.family,
          registrationId: descriptor.registrationId,
          compatibility: descriptor.compatibility,
          invalidationMode: "push-driven",
          queuedInvalidationCount: 1,
          previousState: committedState,
          nextState: committedState,
          touchedNodes: 0,
          reevaluatedNodes: 0,
        });
        return;
      }
      metrics.invalidationCount += 1;
      queuedInvalidationCount += 1;
      queuedState = normalizeViewportState(registration.source.current());
      scheduleFlush();
    }),
    "viewport",
  );

  function dispose() {
    if (!disposed) {
      disposed = true;
      metrics.disposalCount += 1;
      unsubscribe();
    }
  }

  return {
    hostEntry: Object.freeze({
      size() {
        return readViewportSize();
      },
      width() {
        return readViewportSize().width;
      },
      height() {
        return readViewportSize().height;
      },
      descriptor() {
        return descriptor;
      },
      [HOST_VIEWPORT_HANDLE_BRAND]: true,
    }),
    dispose,
    performanceSummary() {
      return summarizePushMetrics(metrics);
    },
  };
}

function registerBinaryCapability(rawSignals, registration, diagnosticsRecorder, config) {
  const hiddenSignalId = nextHiddenHostSignalId(rawSignals, config.family);
  let committedState = normalizeBinaryCapabilityState(
    config.family,
    registration.source.current(),
    config.positiveState,
    config.negativeState,
  );
  const rawHiddenSignal = rawSignals.input(hiddenSignalId, committedState);
  const hiddenSignal = wrapReadableSignal(rawHiddenSignal, rawSignals, "hostCapability");
  const descriptor = Object.freeze({
    family: config.family,
    compatibility: registration.compatibility,
    registrationId: config.family,
  });
  const metrics = {
    registrationCount: 1,
    disposalCount: 0,
    readCount: 0,
    invalidationCount: 0,
    invalidationBatchFlushCount: 0,
    reevaluationCount: 0,
    invalidationTouchedNodeCount: 0,
    noOpInvalidationSuppressedCount: 0,
    staleInvalidationIgnoredCount: 0,
    compatibilityDenialCount: 0,
  };
  let scheduled = false;
  let queuedState = committedState;
  let queuedInvalidationCount = 0;
  let disposed = false;

  function readState() {
    recordHostCapabilityRead(rawSignals, descriptor);
    metrics.readCount += 1;
    return hiddenSignal();
  }

  function flushQueuedInvalidation() {
    scheduled = false;
    const flushedInvalidationCount = queuedInvalidationCount;
    queuedInvalidationCount = 0;
    if (disposed) {
      metrics.staleInvalidationIgnoredCount += 1;
      diagnosticsRecorder.push({
        kind: "InvalidationIgnoredStale",
        family: descriptor.family,
        registrationId: descriptor.registrationId,
        compatibility: descriptor.compatibility,
        invalidationMode: "push-driven",
        queuedInvalidationCount: flushedInvalidationCount,
        previousState: committedState,
        nextState: queuedState,
        touchedNodes: 0,
        reevaluatedNodes: 0,
      });
      return;
    }
    if (queuedState === committedState) {
      metrics.noOpInvalidationSuppressedCount += 1;
      diagnosticsRecorder.push({
        kind: "InvalidationNoOpSuppressed",
        family: descriptor.family,
        registrationId: descriptor.registrationId,
        compatibility: descriptor.compatibility,
        invalidationMode: "push-driven",
        queuedInvalidationCount: flushedInvalidationCount,
        previousState: committedState,
        nextState: queuedState,
        touchedNodes: 0,
        reevaluatedNodes: 0,
      });
      return;
    }
    metrics.invalidationBatchFlushCount += 1;
    const previousState = committedState;
    const result = rawSignals.transaction((tx) => {
      tx.set(hiddenSignal[RAW_SIGNAL_HANDLE], queuedState);
    });
    committedState = queuedState;
    const touchedNodes = typeof result?.touchedNodes === "number"
      ? Math.max(0, result.touchedNodes)
      : 0;
    const reevaluatedNodes = typeof result?.nodesRecomputed === "number"
      ? Math.max(0, result.nodesRecomputed)
      : touchedNodes;
    metrics.reevaluationCount += reevaluatedNodes;
    if (typeof result?.touchedNodes === "number") {
      metrics.invalidationTouchedNodeCount += touchedNodes;
    }
    diagnosticsRecorder.push({
      kind: "InvalidationFlushed",
      family: descriptor.family,
      registrationId: descriptor.registrationId,
      compatibility: descriptor.compatibility,
      invalidationMode: "push-driven",
      queuedInvalidationCount: flushedInvalidationCount,
      previousState,
      nextState: queuedState,
      touchedNodes,
      reevaluatedNodes,
    });
  }

  function scheduleFlush() {
    if (!scheduled) {
      scheduled = true;
      queueMicrotask(flushQueuedInvalidation);
    }
  }

  const unsubscribe = normalizeSubscription(
    registration.source.subscribe(() => {
      if (disposed) {
        metrics.staleInvalidationIgnoredCount += 1;
        diagnosticsRecorder.push({
          kind: "InvalidationIgnoredStale",
          family: descriptor.family,
          registrationId: descriptor.registrationId,
          compatibility: descriptor.compatibility,
          invalidationMode: "push-driven",
          queuedInvalidationCount: 1,
          previousState: committedState,
          nextState: committedState,
          touchedNodes: 0,
          reevaluatedNodes: 0,
        });
        return;
      }
      metrics.invalidationCount += 1;
      queuedInvalidationCount += 1;
      queuedState = normalizeBinaryCapabilityState(
        config.family,
        registration.source.current(),
        config.positiveState,
        config.negativeState,
      );
      scheduleFlush();
    }),
    config.family,
  );

  function dispose() {
    if (!disposed) {
      disposed = true;
      metrics.disposalCount += 1;
      unsubscribe();
    }
  }

  return {
    hostEntry: Object.freeze({
      state() {
        return readState();
      },
      [config.booleanMethodName]() {
        return readState() === config.positiveState;
      },
      descriptor() {
        return descriptor;
      },
      [config.handleBrand]: true,
    }),
    dispose,
    performanceSummary() {
      return summarizePushMetrics(metrics);
    },
  };
}

export function registerClockCapability(rawSignals, registration, diagnosticsRecorder) {
  const hiddenSignalId = nextHiddenHostSignalId(rawSignals, "clock");
  let committedState = normalizeClockValue(registration.source.current());
  const rawHiddenSignal = rawSignals.input(hiddenSignalId, committedState);
  const hiddenSignal = wrapReadableSignal(rawHiddenSignal, rawSignals, "hostCapability");
  const descriptor = Object.freeze({
    family: "clock",
    compatibility: registration.compatibility,
    registrationId: "clock",
  });
  const metrics = {
    registrationCount: 1,
    disposalCount: 0,
    readCount: 0,
    pollCount: 0,
    noOpPollCount: 0,
    invalidationCount: 0,
    invalidationBatchFlushCount: 0,
    reevaluationCount: 0,
    invalidationTouchedNodeCount: 0,
    noOpInvalidationSuppressedCount: 0,
    staleInvalidationIgnoredCount: 0,
    compatibilityDenialCount: 0,
  };
  let scheduled = false;
  let queuedState = committedState;
  let queuedInvalidationCount = 0;
  let disposed = false;

  function readClockNow() {
    recordHostCapabilityRead(rawSignals, descriptor);
    metrics.readCount += 1;
    return hiddenSignal();
  }

  function flushQueuedInvalidation() {
    scheduled = false;
    const flushedInvalidationCount = queuedInvalidationCount;
    queuedInvalidationCount = 0;
    if (disposed) {
      metrics.staleInvalidationIgnoredCount += 1;
      diagnosticsRecorder.push({
        kind: "InvalidationIgnoredStale",
        family: descriptor.family,
        registrationId: descriptor.registrationId,
        compatibility: descriptor.compatibility,
        invalidationMode: "polled",
        queuedInvalidationCount: flushedInvalidationCount,
        previousState: committedState,
        nextState: queuedState,
        touchedNodes: 0,
        reevaluatedNodes: 0,
      });
      return;
    }
    if (queuedState === committedState) {
      metrics.noOpInvalidationSuppressedCount += 1;
      diagnosticsRecorder.push({
        kind: "InvalidationNoOpSuppressed",
        family: descriptor.family,
        registrationId: descriptor.registrationId,
        compatibility: descriptor.compatibility,
        invalidationMode: "polled",
        queuedInvalidationCount: flushedInvalidationCount,
        previousState: committedState,
        nextState: queuedState,
        touchedNodes: 0,
        reevaluatedNodes: 0,
      });
      return;
    }
    metrics.invalidationBatchFlushCount += 1;
    const previousState = committedState;
    const result = rawSignals.transaction((tx) => {
      tx.set(hiddenSignal[RAW_SIGNAL_HANDLE], queuedState);
    });
    committedState = queuedState;
    const touchedNodes = typeof result?.touchedNodes === "number"
      ? Math.max(0, result.touchedNodes)
      : 0;
    const reevaluatedNodes = typeof result?.nodesRecomputed === "number"
      ? Math.max(0, result.nodesRecomputed)
      : touchedNodes;
    metrics.reevaluationCount += reevaluatedNodes;
    if (typeof result?.touchedNodes === "number") {
      metrics.invalidationTouchedNodeCount += touchedNodes;
    }
    diagnosticsRecorder.push({
      kind: "InvalidationFlushed",
      family: descriptor.family,
      registrationId: descriptor.registrationId,
      compatibility: descriptor.compatibility,
      invalidationMode: "polled",
      queuedInvalidationCount: flushedInvalidationCount,
      previousState,
      nextState: queuedState,
      touchedNodes,
      reevaluatedNodes,
    });
  }

  function scheduleFlush() {
    if (!scheduled) {
      scheduled = true;
      queueMicrotask(flushQueuedInvalidation);
    }
  }

  const intervalHandle = setInterval(() => {
    if (disposed) {
      metrics.staleInvalidationIgnoredCount += 1;
      return;
    }
    metrics.pollCount += 1;
    const nextState = normalizeClockValue(registration.source.current());
    if (nextState === committedState) {
      metrics.noOpPollCount += 1;
      return;
    }
    metrics.invalidationCount += 1;
    queuedInvalidationCount += 1;
    queuedState = nextState;
    scheduleFlush();
  }, registration.pollMs);
  intervalHandle.unref?.();

  function dispose() {
    if (!disposed) {
      disposed = true;
      metrics.disposalCount += 1;
      clearInterval(intervalHandle);
    }
  }

  return {
    hostEntry: Object.freeze({
      now() {
        return readClockNow();
      },
      descriptor() {
        return descriptor;
      },
      [HOST_CLOCK_HANDLE_BRAND]: true,
    }),
    dispose,
    performanceSummary() {
      return summarizePushMetrics(metrics);
    },
  };
}

export function registerPersistenceCapability(rawSignals, registration, diagnosticsRecorder) {
  const hiddenSignalId = nextHiddenHostSignalId(rawSignals, "persistence");
  let committedState = cloneSignalValue(registration.source.current());
  const rawHiddenSignal = rawSignals.input(hiddenSignalId, committedState);
  const hiddenSignal = wrapReadableSignal(rawHiddenSignal, rawSignals, "hostCapability");
  const descriptor = Object.freeze({
    family: "persistence",
    compatibility: registration.compatibility,
    registrationId: "persistence",
  });
  const metrics = {
    registrationCount: 1,
    disposalCount: 0,
    readCount: 0,
    manualCommitCount: 0,
    noOpManualCommitCount: 0,
    invalidationCount: 0,
    invalidationBatchFlushCount: 0,
    reevaluationCount: 0,
    invalidationTouchedNodeCount: 0,
    noOpInvalidationSuppressedCount: 0,
    staleInvalidationIgnoredCount: 0,
    compatibilityDenialCount: 0,
  };
  let disposed = false;

  function readPersistenceValue() {
    recordHostCapabilityRead(rawSignals, descriptor);
    metrics.readCount += 1;
    return hiddenSignal();
  }

  function commitPersistence() {
    if (disposed) {
      metrics.staleInvalidationIgnoredCount += 1;
      diagnosticsRecorder.push({
        kind: "InvalidationIgnoredStale",
        family: descriptor.family,
        registrationId: descriptor.registrationId,
        compatibility: descriptor.compatibility,
        invalidationMode: "manually-committed",
        queuedInvalidationCount: 1,
        previousState: committedState,
        nextState: committedState,
        touchedNodes: 0,
        reevaluatedNodes: 0,
      });
      return { touchedNodes: 0, nodesRecomputed: 0 };
    }
    metrics.manualCommitCount += 1;
    const nextState = cloneSignalValue(registration.source.current());
    if (Object.is(JSON.stringify(nextState), JSON.stringify(committedState))) {
      metrics.noOpManualCommitCount += 1;
      metrics.noOpInvalidationSuppressedCount += 1;
      diagnosticsRecorder.push({
        kind: "InvalidationNoOpSuppressed",
        family: descriptor.family,
        registrationId: descriptor.registrationId,
        compatibility: descriptor.compatibility,
        invalidationMode: "manually-committed",
        queuedInvalidationCount: 1,
        previousState: committedState,
        nextState,
        touchedNodes: 0,
        reevaluatedNodes: 0,
      });
      return { touchedNodes: 0, nodesRecomputed: 0 };
    }
    metrics.invalidationCount += 1;
    metrics.invalidationBatchFlushCount += 1;
    const previousState = committedState;
    const result = rawSignals.transaction((tx) => {
      tx.set(hiddenSignal[RAW_SIGNAL_HANDLE], nextState);
    });
    committedState = nextState;
    const touchedNodes = typeof result?.touchedNodes === "number"
      ? Math.max(0, result.touchedNodes)
      : 0;
    const reevaluatedNodes = typeof result?.nodesRecomputed === "number"
      ? Math.max(0, result.nodesRecomputed)
      : touchedNodes;
    metrics.reevaluationCount += reevaluatedNodes;
    metrics.invalidationTouchedNodeCount += touchedNodes;
    diagnosticsRecorder.push({
      kind: "InvalidationFlushed",
      family: descriptor.family,
      registrationId: descriptor.registrationId,
      compatibility: descriptor.compatibility,
      invalidationMode: "manually-committed",
      queuedInvalidationCount: 1,
      previousState,
      nextState,
      touchedNodes,
      reevaluatedNodes,
    });
    return result ?? { touchedNodes, nodesRecomputed: reevaluatedNodes };
  }

  function dispose() {
    if (!disposed) {
      disposed = true;
      metrics.disposalCount += 1;
    }
  }

  return {
    hostEntry: Object.freeze({
      value() {
        return readPersistenceValue();
      },
      commit() {
        return commitPersistence();
      },
      descriptor() {
        return descriptor;
      },
      [HOST_PERSISTENCE_HANDLE_BRAND]: true,
    }),
    dispose,
    performanceSummary() {
      return summarizePushMetrics(metrics);
    },
  };
}

export function registerVisibilityCapability(rawSignals, registration, diagnosticsRecorder) {
  return registerBinaryCapability(rawSignals, registration, diagnosticsRecorder, {
    family: "visibility",
    positiveState: "visible",
    negativeState: "hidden",
    booleanMethodName: "isVisible",
    handleBrand: HOST_VISIBILITY_HANDLE_BRAND,
  });
}

export function registerOnlineCapability(rawSignals, registration, diagnosticsRecorder) {
  return registerBinaryCapability(rawSignals, registration, diagnosticsRecorder, {
    family: "online",
    positiveState: "online",
    negativeState: "offline",
    booleanMethodName: "isOnline",
    handleBrand: HOST_ONLINE_HANDLE_BRAND,
  });
}
