import { freezeObject } from "../graph_support.js";
import {
  createDiagnosticsRecorder,
  emptyPerformanceSummary,
  HOST_CLOCK_HANDLE_BRAND,
  HOST_ONLINE_HANDLE_BRAND,
  HOST_VIEWPORT_HANDLE_BRAND,
  HOST_VISIBILITY_HANDLE_BRAND,
  normalizeBinaryCapabilityState,
  normalizeClockValue,
  normalizeSubscription,
  normalizeViewportState,
} from "../host_capability_declarations.js";
import { buildHostCapabilityDiagnosticsReport } from "../host_capability_reports.js";
import {
  recordFlushedEvent,
  recordIgnoredStaleEvent,
  recordNoOpEvent,
} from "./worker_first_host_capability_events.js";
import { createWorkerFirstPersistenceCapability } from "./worker_first_persistence_host_capability.js";

export function createWorkerFirstHostCapabilities(rootSession, hostCapabilities) {
  const performanceSummary = emptyPerformanceSummary();
  const diagnosticsRecorder = createDiagnosticsRecorder();
  if (hostCapabilities === null) {
    return createWorkerFirstHostCapabilityManager(
      Object.freeze({}),
      [],
      performanceSummary,
      diagnosticsRecorder,
    );
  }

  const registrations = [
    hostCapabilities.viewport && createViewportCapability(
      hostCapabilities.viewport,
      performanceSummary,
      diagnosticsRecorder,
    ),
    hostCapabilities.visibility && createBinaryCapability(
      hostCapabilities.visibility,
      {
        family: "visibility",
        positiveState: "visible",
        negativeState: "hidden",
        booleanMethodName: "isVisible",
        brand: HOST_VISIBILITY_HANDLE_BRAND,
      },
      performanceSummary,
      diagnosticsRecorder,
    ),
    hostCapabilities.online && createBinaryCapability(
      hostCapabilities.online,
      {
        family: "online",
        positiveState: "online",
        negativeState: "offline",
        booleanMethodName: "isOnline",
        brand: HOST_ONLINE_HANDLE_BRAND,
      },
      performanceSummary,
      diagnosticsRecorder,
    ),
    hostCapabilities.clock && createClockCapability(
      hostCapabilities.clock,
      performanceSummary,
      diagnosticsRecorder,
    ),
    hostCapabilities.persistence && createWorkerFirstPersistenceCapability(
      hostCapabilities.persistence,
      rootSession,
      performanceSummary,
      diagnosticsRecorder,
    ),
  ].filter(Boolean);
  performanceSummary.hostCapabilityRegistrationCount = registrations.length;

  return createWorkerFirstHostCapabilityManager(
    freezeObject(
      Object.fromEntries(
        registrations.map((registration) => [
          registration.handle.descriptor().family,
          registration.handle,
        ]),
      ),
    ),
    registrations,
    performanceSummary,
    diagnosticsRecorder,
  );
}

function createWorkerFirstHostCapabilityManager(
  host,
  registrations,
  performanceSummary,
  diagnosticsRecorder,
) {
  return {
    host,
    async bootstrap() {
      await Promise.all(registrations.map((registration) => registration.bootstrap()));
    },
    async replayCurrentIngress() {
      await Promise.all(registrations.map((registration) => registration.replayCurrentIngress()));
    },
    dispose() {
      for (const registration of registrations) {
        registration.dispose();
      }
    },
    latestEvent() {
      return diagnosticsRecorder.latest();
    },
    recentEvents() {
      return Object.freeze(diagnosticsRecorder.recent());
    },
    report() {
      return buildHostCapabilityDiagnosticsReport(
        performanceSummary,
        diagnosticsRecorder.recent(),
      );
    },
  };
}

function createViewportCapability(registration, performanceSummary, diagnosticsRecorder) {
  const descriptor = freezeObject({
    family: "viewport",
    compatibility: registration.compatibility,
    registrationId: "viewport",
  });
  let currentState = normalizeViewportState(registration.source.current());
  let disposed = false;

  const unsubscribe = normalizeSubscription(
    registration.source.subscribe(() => {
      if (disposed) {
        recordIgnoredStaleEvent(
          performanceSummary,
          diagnosticsRecorder,
          descriptor,
          "push-driven",
          currentState,
        );
        return;
      }
      performanceSummary.hostCapabilityInvalidationCount += 1;
      const nextState = normalizeViewportState(registration.source.current());
      if (nextState.width === currentState.width && nextState.height === currentState.height) {
        recordNoOpEvent(
          performanceSummary,
          diagnosticsRecorder,
          descriptor,
          "push-driven",
          currentState,
          nextState,
        );
        return;
      }
      const previousState = currentState;
      currentState = nextState;
      recordFlushedEvent(
        performanceSummary,
        diagnosticsRecorder,
        descriptor,
        "push-driven",
        previousState,
        nextState,
      );
    }),
    "viewport",
  );

  return {
    handle: freezeObject({
      size() {
        performanceSummary.hostCapabilityReadCount += 1;
        return currentState;
      },
      width() {
        performanceSummary.hostCapabilityReadCount += 1;
        return currentState.width;
      },
      height() {
        performanceSummary.hostCapabilityReadCount += 1;
        return currentState.height;
      },
      descriptor() {
        return descriptor;
      },
      [HOST_VIEWPORT_HANDLE_BRAND]: true,
    }),
    bootstrap() {
      return Promise.resolve();
    },
    replayCurrentIngress() {
      return Promise.resolve();
    },
    dispose() {
      if (!disposed) {
        disposed = true;
        performanceSummary.hostCapabilityDisposalCount += 1;
        unsubscribe();
      }
    },
  };
}

function createBinaryCapability(registration, config, performanceSummary, diagnosticsRecorder) {
  const descriptor = freezeObject({
    family: config.family,
    compatibility: registration.compatibility,
    registrationId: config.family,
  });
  let currentState = normalizeBinaryCapabilityState(
    config.family,
    registration.source.current(),
    config.positiveState,
    config.negativeState,
  );
  let disposed = false;

  const unsubscribe = normalizeSubscription(
    registration.source.subscribe(() => {
      if (disposed) {
        recordIgnoredStaleEvent(
          performanceSummary,
          diagnosticsRecorder,
          descriptor,
          "push-driven",
          currentState,
        );
        return;
      }
      performanceSummary.hostCapabilityInvalidationCount += 1;
      const nextState = normalizeBinaryCapabilityState(
        config.family,
        registration.source.current(),
        config.positiveState,
        config.negativeState,
      );
      if (nextState === currentState) {
        recordNoOpEvent(
          performanceSummary,
          diagnosticsRecorder,
          descriptor,
          "push-driven",
          currentState,
          nextState,
        );
        return;
      }
      const previousState = currentState;
      currentState = nextState;
      recordFlushedEvent(
        performanceSummary,
        diagnosticsRecorder,
        descriptor,
        "push-driven",
        previousState,
        nextState,
      );
    }),
    config.family,
  );

  return {
    handle: freezeObject({
      state() {
        performanceSummary.hostCapabilityReadCount += 1;
        return currentState;
      },
      [config.booleanMethodName]() {
        performanceSummary.hostCapabilityReadCount += 1;
        return currentState === config.positiveState;
      },
      descriptor() {
        return descriptor;
      },
      [config.brand]: true,
    }),
    bootstrap() {
      return Promise.resolve();
    },
    replayCurrentIngress() {
      return Promise.resolve();
    },
    dispose() {
      if (!disposed) {
        disposed = true;
        performanceSummary.hostCapabilityDisposalCount += 1;
        unsubscribe();
      }
    },
  };
}

function createClockCapability(registration, performanceSummary, diagnosticsRecorder) {
  const descriptor = freezeObject({
    family: "clock",
    compatibility: registration.compatibility,
    registrationId: "clock",
  });
  let currentState = normalizeClockValue(registration.source.current());
  let disposed = false;

  const intervalHandle = setInterval(() => {
    if (disposed) {
      return;
    }
    performanceSummary.hostCapabilityPollCount += 1;
    const nextState = normalizeClockValue(registration.source.current());
    if (nextState === currentState) {
      performanceSummary.hostCapabilityNoOpPollCount += 1;
      return;
    }
    performanceSummary.hostCapabilityInvalidationCount += 1;
    const previousState = currentState;
    currentState = nextState;
    recordFlushedEvent(
      performanceSummary,
      diagnosticsRecorder,
      descriptor,
      "polled",
      previousState,
      nextState,
    );
  }, registration.pollMs);
  intervalHandle.unref?.();

  return {
    handle: freezeObject({
      now() {
        performanceSummary.hostCapabilityReadCount += 1;
        return currentState;
      },
      descriptor() {
        return descriptor;
      },
      [HOST_CLOCK_HANDLE_BRAND]: true,
    }),
    bootstrap() {
      return Promise.resolve();
    },
    replayCurrentIngress() {
      return Promise.resolve();
    },
    dispose() {
      if (!disposed) {
        disposed = true;
        performanceSummary.hostCapabilityDisposalCount += 1;
        clearInterval(intervalHandle);
      }
    },
  };
}

export function workerFirstHostCapabilitiesUnsupportedReason(hostCapabilities) {
  void hostCapabilities;
  return null;
}
