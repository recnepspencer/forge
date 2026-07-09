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
  createWorkerFirstHostSurface,
  recordWorkerFirstAdmittedHostCapabilityRead,
} from "./worker_first_denied_host_capabilities.js";
import {
  recordFlushedEvent,
  recordIgnoredStaleEvent,
  recordNoOpEvent,
} from "./worker_first_host_capability_events.js";
import { scheduleWorkerFirstHostDependencyRefresh } from "./worker_first_host_dependency_refresh.js";
import { createWorkerFirstPersistenceCapability } from "./worker_first_persistence_host_capability.js";

export function createWorkerFirstHostCapabilities(rootSession, hostCapabilities) {
  const performanceSummary = emptyPerformanceSummary();
  const diagnosticsRecorder = createDiagnosticsRecorder();
  if (hostCapabilities === null) {
    return createWorkerFirstHostCapabilityManager(
      createWorkerFirstHostSurface([], performanceSummary, diagnosticsRecorder),
      [],
      performanceSummary,
      diagnosticsRecorder,
    );
  }

  const registrations = [
    hostCapabilities.viewport && createViewportCapability(
      hostCapabilities.viewport,
      rootSession,
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
      rootSession,
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
      rootSession,
      performanceSummary,
      diagnosticsRecorder,
    ),
    hostCapabilities.clock && createClockCapability(
      hostCapabilities.clock,
      rootSession,
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
    createWorkerFirstHostSurface(registrations, performanceSummary, diagnosticsRecorder),
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
    report(callbackHostDependencies = null) {
      return buildHostCapabilityDiagnosticsReport(
        performanceSummary,
        diagnosticsRecorder.recent(),
        callbackHostDependencies,
      );
    },
  };
}

function createViewportCapability(registration, rootSession, performanceSummary, diagnosticsRecorder) {
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
      scheduleHostDependencyRefresh(rootSession, descriptor, performanceSummary, diagnosticsRecorder, "push-driven");
    }),
    "viewport",
  );

  return {
    handle: freezeObject({
      size() {
        recordWorkerFirstAdmittedHostCapabilityRead(rootSession, descriptor, performanceSummary, diagnosticsRecorder, disposed);
        return currentState;
      },
      width() {
        recordWorkerFirstAdmittedHostCapabilityRead(rootSession, descriptor, performanceSummary, diagnosticsRecorder, disposed);
        return currentState.width;
      },
      height() {
        recordWorkerFirstAdmittedHostCapabilityRead(rootSession, descriptor, performanceSummary, diagnosticsRecorder, disposed);
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

function createBinaryCapability(
  registration,
  config,
  rootSession,
  performanceSummary,
  diagnosticsRecorder,
) {
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
      scheduleHostDependencyRefresh(rootSession, descriptor, performanceSummary, diagnosticsRecorder, "push-driven");
    }),
    config.family,
  );

  return {
    handle: freezeObject({
      state() {
        recordWorkerFirstAdmittedHostCapabilityRead(rootSession, descriptor, performanceSummary, diagnosticsRecorder, disposed);
        return currentState;
      },
      [config.booleanMethodName]() {
        recordWorkerFirstAdmittedHostCapabilityRead(rootSession, descriptor, performanceSummary, diagnosticsRecorder, disposed);
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

function createClockCapability(registration, rootSession, performanceSummary, diagnosticsRecorder) {
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
    scheduleHostDependencyRefresh(rootSession, descriptor, performanceSummary, diagnosticsRecorder, "polled");
  }, registration.pollMs);
  intervalHandle.unref?.();

  return {
    handle: freezeObject({
      now() {
        recordWorkerFirstAdmittedHostCapabilityRead(rootSession, descriptor, performanceSummary, diagnosticsRecorder, disposed);
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

function scheduleHostDependencyRefresh(
  rootSession,
  descriptor,
  performanceSummary,
  diagnosticsRecorder,
  invalidationMode,
) {
  scheduleWorkerFirstHostDependencyRefresh({
    rootSession,
    descriptor,
    performanceSummary,
    diagnosticsRecorder,
    invalidationMode,
  });
}
