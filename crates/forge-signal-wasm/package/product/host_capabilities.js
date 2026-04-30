import { recordHostCapabilityRead } from "./callback_frames.js";
import { wrapReadableSignal } from "./handles.js";
import { RAW_SIGNAL_HANDLE } from "./symbols.js";

const HOST_CAPABILITY_PLAN_BRAND = Symbol("forgeSignal.hostCapabilityPlan");
const HOST_VIEWPORT_REGISTRATION_BRAND = Symbol("forgeSignal.hostViewportRegistration");
const HOST_VIEWPORT_HANDLE_BRAND = Symbol("forgeSignal.hostViewportHandle");
const HOST_VISIBILITY_REGISTRATION_BRAND = Symbol("forgeSignal.hostVisibilityRegistration");
const HOST_VISIBILITY_HANDLE_BRAND = Symbol("forgeSignal.hostVisibilityHandle");
const HOST_ONLINE_REGISTRATION_BRAND = Symbol("forgeSignal.hostOnlineRegistration");
const HOST_ONLINE_HANDLE_BRAND = Symbol("forgeSignal.hostOnlineHandle");
const HOST_CLOCK_REGISTRATION_BRAND = Symbol("forgeSignal.hostClockRegistration");
const HOST_CLOCK_HANDLE_BRAND = Symbol("forgeSignal.hostClockHandle");
const HOST_PERSISTENCE_REGISTRATION_BRAND = Symbol("forgeSignal.hostPersistenceRegistration");
const HOST_PERSISTENCE_HANDLE_BRAND = Symbol("forgeSignal.hostPersistenceHandle");
const HOST_CAPABILITY_EVENT_LIMIT = 32;

const HOST_COMPATIBILITY_VALUES = new Set([
  "LiveOnly",
  "Reattachable",
  "SnapshotPortable",
  "ImportDenied",
]);

const HIDDEN_HOST_SIGNAL_COUNTERS = new WeakMap();

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function requirePlainObject(value, message) {
  if (!isPlainObject(value)) {
    throw new TypeError(message);
  }
  return value;
}

function requireFunction(value, message) {
  if (typeof value !== "function") {
    throw new TypeError(message);
  }
  return value;
}

function normalizeCompatibility(family, compatibility, fallback) {
  if (compatibility === undefined) {
    return fallback;
  }
  if (!HOST_COMPATIBILITY_VALUES.has(compatibility)) {
    throw new TypeError(
      `${family} compatibility must be one of ${[...HOST_COMPATIBILITY_VALUES].join(", ")}`,
    );
  }
  return compatibility;
}

function normalizeBinaryCapabilityState(family, value, positiveState, negativeState) {
  if (value === true || value === positiveState) {
    return positiveState;
  }
  if (value === false || value === negativeState) {
    return negativeState;
  }
  throw new TypeError(
    `${family} capability source.current() must return \`${positiveState}\`, \`${negativeState}\`, true, or false`,
  );
}

function normalizeSubscription(value, family) {
  if (value === undefined || value === null) {
    return () => {};
  }
  if (typeof value === "function") {
    return value;
  }
  if (typeof value.dispose === "function") {
    return () => value.dispose();
  }
  if (typeof value.free === "function") {
    return () => value.free();
  }
  throw new TypeError(
    `${family} capability source.subscribe() must return nothing, a function, or an object with dispose()/free()`,
  );
}

function nextHiddenHostSignalId(rawSignals, family) {
  const next = (HIDDEN_HOST_SIGNAL_COUNTERS.get(rawSignals) ?? 0) + 1;
  HIDDEN_HOST_SIGNAL_COUNTERS.set(rawSignals, next);
  return `__forgeSignal.host.${family}.${next}`;
}

function emptyPerformanceSummary() {
  return {
    hostCapabilityRegistrationCount: 0,
    hostCapabilityDisposalCount: 0,
    hostCapabilityReadCount: 0,
    hostCapabilityPollCount: 0,
    hostCapabilityNoOpPollCount: 0,
    hostCapabilityManualCommitCount: 0,
    hostCapabilityNoOpManualCommitCount: 0,
    hostCapabilityInvalidationCount: 0,
    hostCapabilityInvalidationBatchFlushCount: 0,
    hostCapabilityReevaluationCount: 0,
    hostCapabilityInvalidationTouchedNodeCount: 0,
    hostCapabilityNoOpInvalidationSuppressedCount: 0,
    hostCapabilityStaleInvalidationIgnoredCount: 0,
    hostCapabilityCompatibilityDenialCount: 0,
    hostCapabilityUnavailabilityArtifactCount: 0,
    hostCapabilityBroadFanoutDenialCount: 0,
  };
}

function unavailableArtifactKey(artifact) {
  const id = typeof artifact?.id === "string" ? artifact.id : "unknown";
  const transports = Array.isArray(artifact?.hostCapabilityTransports)
    ? artifact.hostCapabilityTransports
    : [];
  const transportKey = transports
    .map((transport) => [
      transport?.family ?? "unknown",
      transport?.registrationId ?? "unknown",
      transport?.compatibility ?? "unknown",
      transport?.portableImportOutcome ?? "unknown",
    ].join(":"))
    .sort()
    .join("|");
  return `${id}::${transportKey}`;
}

function createDiagnosticsRecorder() {
  let nextSequence = 0;
  const events = [];

  return {
    push(event) {
      nextSequence += 1;
      const recorded = Object.freeze({
        sequence: nextSequence,
        ...event,
      });
      events.push(recorded);
      if (events.length > HOST_CAPABILITY_EVENT_LIMIT) {
        events.shift();
      }
      return recorded;
    },
    latest() {
      return events.at(-1) ?? null;
    },
    recent() {
      return events.slice();
    },
  };
}

function requireVisibilityRegistration(registration) {
  if (!registration || registration[HOST_VISIBILITY_REGISTRATION_BRAND] !== true) {
    throw new TypeError(
      "hostCapabilityPlan visibility entries must be created with visibilityCapability(...)",
    );
  }
  return registration;
}

function requireViewportRegistration(registration) {
  if (!registration || registration[HOST_VIEWPORT_REGISTRATION_BRAND] !== true) {
    throw new TypeError(
      "hostCapabilityPlan viewport entries must be created with viewportCapability(...)",
    );
  }
  return registration;
}

function requireOnlineRegistration(registration) {
  if (!registration || registration[HOST_ONLINE_REGISTRATION_BRAND] !== true) {
    throw new TypeError(
      "hostCapabilityPlan online entries must be created with onlineCapability(...)",
    );
  }
  return registration;
}

function requireClockRegistration(registration) {
  if (!registration || registration[HOST_CLOCK_REGISTRATION_BRAND] !== true) {
    throw new TypeError(
      "hostCapabilityPlan clock entries must be created with clockCapability(...)",
    );
  }
  return registration;
}

function requirePersistenceRegistration(registration) {
  if (!registration || registration[HOST_PERSISTENCE_REGISTRATION_BRAND] !== true) {
    throw new TypeError(
      "hostCapabilityPlan persistence entries must be created with persistenceCapability(...)",
    );
  }
  return registration;
}

function normalizeClockValue(value) {
  if (typeof value !== "number" || !Number.isFinite(value)) {
    throw new TypeError(
      "clock capability source.current() must return a finite number",
    );
  }
  return value;
}

function normalizeViewportState(value) {
  requirePlainObject(
    value,
    "viewport capability source.current() must return an object with width and height",
  );
  if (typeof value.width !== "number" || !Number.isFinite(value.width)) {
    throw new TypeError(
      "viewport capability source.current().width must be a finite number",
    );
  }
  if (typeof value.height !== "number" || !Number.isFinite(value.height)) {
    throw new TypeError(
      "viewport capability source.current().height must be a finite number",
    );
  }
  return Object.freeze({
    width: value.width,
    height: value.height,
  });
}

function cloneSignalValue(value) {
  return structuredClone(value);
}

function normalizePollMs(value, family) {
  if (value === undefined) {
    return 1000;
  }
  if (!Number.isInteger(value) || value <= 0) {
    throw new TypeError(`${family} capability pollMs must be a positive integer`);
  }
  return value;
}

function parseHostCapabilityPlan(options) {
  if (options === undefined) {
    return null;
  }
  const normalizedOptions = requirePlainObject(
    options,
    "createSignals options must be an object when provided",
  );
  const { hostCapabilities, ...unknownOptions } = normalizedOptions;
  const unknownKeys = Object.keys(unknownOptions);
  if (unknownKeys.length > 0) {
    throw new TypeError(
      `createSignals options do not support: ${unknownKeys.join(", ")}`,
    );
  }
  if (hostCapabilities === undefined) {
    return null;
  }
  if (!hostCapabilities || hostCapabilities[HOST_CAPABILITY_PLAN_BRAND] !== true) {
    throw new TypeError(
      "createSignals hostCapabilities must be created with hostCapabilityPlan(...)",
    );
  }
  return hostCapabilities;
}

export function visibilityCapability(options) {
  const normalized = requirePlainObject(
    options,
    "visibilityCapability options must be an object",
  );
  const source = requirePlainObject(
    normalized.source,
    "visibilityCapability source must be an object",
  );
  requireFunction(
    source.current,
    "visibility capability source.current must be a function",
  );
  requireFunction(
    source.subscribe,
    "visibility capability source.subscribe must be a function",
  );

  return Object.freeze({
    family: "visibility",
    compatibility: normalizeCompatibility("visibility", normalized.compatibility, "LiveOnly"),
    source,
    [HOST_VISIBILITY_REGISTRATION_BRAND]: true,
  });
}

export function viewportCapability(options) {
  const normalized = requirePlainObject(
    options,
    "viewportCapability options must be an object",
  );
  const source = requirePlainObject(
    normalized.source,
    "viewportCapability source must be an object",
  );
  requireFunction(
    source.current,
    "viewport capability source.current must be a function",
  );
  requireFunction(
    source.subscribe,
    "viewport capability source.subscribe must be a function",
  );

  return Object.freeze({
    family: "viewport",
    compatibility: normalizeCompatibility("viewport", normalized.compatibility, "Reattachable"),
    source,
    [HOST_VIEWPORT_REGISTRATION_BRAND]: true,
  });
}

export function onlineCapability(options) {
  const normalized = requirePlainObject(
    options,
    "onlineCapability options must be an object",
  );
  const source = requirePlainObject(
    normalized.source,
    "onlineCapability source must be an object",
  );
  requireFunction(
    source.current,
    "online capability source.current must be a function",
  );
  requireFunction(
    source.subscribe,
    "online capability source.subscribe must be a function",
  );

  return Object.freeze({
    family: "online",
    compatibility: normalizeCompatibility("online", normalized.compatibility, "Reattachable"),
    source,
    [HOST_ONLINE_REGISTRATION_BRAND]: true,
  });
}

export function clockCapability(options) {
  const normalized = requirePlainObject(
    options,
    "clockCapability options must be an object",
  );
  const source = requirePlainObject(
    normalized.source,
    "clockCapability source must be an object",
  );
  requireFunction(
    source.current,
    "clock capability source.current must be a function",
  );

  return Object.freeze({
    family: "clock",
    compatibility: normalizeCompatibility("clock", normalized.compatibility, "SnapshotPortable"),
    pollMs: normalizePollMs(normalized.pollMs, "clock"),
    source,
    [HOST_CLOCK_REGISTRATION_BRAND]: true,
  });
}

export function persistenceCapability(options) {
  const normalized = requirePlainObject(
    options,
    "persistenceCapability options must be an object",
  );
  const source = requirePlainObject(
    normalized.source,
    "persistenceCapability source must be an object",
  );
  requireFunction(
    source.current,
    "persistence capability source.current must be a function",
  );

  return Object.freeze({
    family: "persistence",
    compatibility: normalizeCompatibility("persistence", normalized.compatibility, "ImportDenied"),
    source,
    [HOST_PERSISTENCE_REGISTRATION_BRAND]: true,
  });
}

export function hostCapabilityPlan(options) {
  const normalized = requirePlainObject(
    options,
    "hostCapabilityPlan options must be an object",
  );
  const allowedKeys = new Set(["viewport", "visibility", "online", "clock", "persistence"]);
  const unknownKeys = Object.keys(normalized).filter((key) => !allowedKeys.has(key));
  if (unknownKeys.length > 0) {
    throw new TypeError(
      `hostCapabilityPlan does not support capability families: ${unknownKeys.join(", ")}`,
    );
  }
  const viewport = normalized.viewport === undefined
    ? undefined
    : requireViewportRegistration(normalized.viewport);
  const visibility = normalized.visibility === undefined
    ? undefined
    : requireVisibilityRegistration(normalized.visibility);
  const online = normalized.online === undefined
    ? undefined
    : requireOnlineRegistration(normalized.online);
  const clock = normalized.clock === undefined
    ? undefined
    : requireClockRegistration(normalized.clock);
  const persistence = normalized.persistence === undefined
    ? undefined
    : requirePersistenceRegistration(normalized.persistence);

  return Object.freeze({
    viewport,
    visibility,
    online,
    clock,
    persistence,
    [HOST_CAPABILITY_PLAN_BRAND]: true,
  });
}

function registerViewportCapability(rawSignals, registration, diagnosticsRecorder) {
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
    if (scheduled) {
      return;
    }
    scheduled = true;
    queueMicrotask(flushQueuedInvalidation);
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
    if (disposed) {
      return;
    }
    disposed = true;
    metrics.disposalCount += 1;
    unsubscribe();
  }

  const handle = Object.freeze({
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
  });

  return {
    hostEntry: handle,
    dispose,
    performanceSummary() {
      return {
        hostCapabilityRegistrationCount: metrics.registrationCount,
        hostCapabilityDisposalCount: metrics.disposalCount,
        hostCapabilityReadCount: metrics.readCount,
        hostCapabilityPollCount: 0,
        hostCapabilityNoOpPollCount: 0,
        hostCapabilityManualCommitCount: 0,
        hostCapabilityNoOpManualCommitCount: 0,
        hostCapabilityInvalidationCount: metrics.invalidationCount,
        hostCapabilityInvalidationBatchFlushCount: metrics.invalidationBatchFlushCount,
        hostCapabilityReevaluationCount: metrics.reevaluationCount,
        hostCapabilityInvalidationTouchedNodeCount: metrics.invalidationTouchedNodeCount,
        hostCapabilityNoOpInvalidationSuppressedCount: metrics.noOpInvalidationSuppressedCount,
        hostCapabilityStaleInvalidationIgnoredCount: metrics.staleInvalidationIgnoredCount,
        hostCapabilityCompatibilityDenialCount: metrics.compatibilityDenialCount,
      };
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

  function readVisibilityState() {
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
    if (scheduled) {
      return;
    }
    scheduled = true;
    queueMicrotask(flushQueuedInvalidation);
  }

  let disposed = false;
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
    if (disposed) {
      return;
    }
    disposed = true;
    metrics.disposalCount += 1;
    unsubscribe();
  }

  const handle = Object.freeze({
    state() {
      return readVisibilityState();
    },
    [config.booleanMethodName]() {
      return readVisibilityState() === config.positiveState;
    },
    descriptor() {
      return descriptor;
    },
    [config.handleBrand]: true,
  });

  return {
    hostEntry: handle,
    dispose,
    performanceSummary() {
      return {
        hostCapabilityRegistrationCount: metrics.registrationCount,
        hostCapabilityDisposalCount: metrics.disposalCount,
        hostCapabilityReadCount: metrics.readCount,
        hostCapabilityPollCount: 0,
        hostCapabilityNoOpPollCount: 0,
        hostCapabilityManualCommitCount: 0,
        hostCapabilityNoOpManualCommitCount: 0,
        hostCapabilityInvalidationCount: metrics.invalidationCount,
        hostCapabilityInvalidationBatchFlushCount: metrics.invalidationBatchFlushCount,
        hostCapabilityReevaluationCount: metrics.reevaluationCount,
        hostCapabilityInvalidationTouchedNodeCount: metrics.invalidationTouchedNodeCount,
        hostCapabilityNoOpInvalidationSuppressedCount: metrics.noOpInvalidationSuppressedCount,
        hostCapabilityStaleInvalidationIgnoredCount: metrics.staleInvalidationIgnoredCount,
        hostCapabilityCompatibilityDenialCount: metrics.compatibilityDenialCount,
      };
    },
  };
}

function registerClockCapability(rawSignals, registration, diagnosticsRecorder) {
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
    if (scheduled) {
      return;
    }
    scheduled = true;
    queueMicrotask(flushQueuedInvalidation);
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
    if (disposed) {
      return;
    }
    disposed = true;
    metrics.disposalCount += 1;
    clearInterval(intervalHandle);
  }

  const handle = Object.freeze({
    now() {
      return readClockNow();
    },
    descriptor() {
      return descriptor;
    },
    [HOST_CLOCK_HANDLE_BRAND]: true,
  });

  return {
    hostEntry: handle,
    dispose,
    performanceSummary() {
      return {
        hostCapabilityRegistrationCount: metrics.registrationCount,
        hostCapabilityDisposalCount: metrics.disposalCount,
        hostCapabilityReadCount: metrics.readCount,
        hostCapabilityPollCount: metrics.pollCount,
        hostCapabilityNoOpPollCount: metrics.noOpPollCount,
        hostCapabilityManualCommitCount: 0,
        hostCapabilityNoOpManualCommitCount: 0,
        hostCapabilityInvalidationCount: metrics.invalidationCount,
        hostCapabilityInvalidationBatchFlushCount: metrics.invalidationBatchFlushCount,
        hostCapabilityReevaluationCount: metrics.reevaluationCount,
        hostCapabilityInvalidationTouchedNodeCount: metrics.invalidationTouchedNodeCount,
        hostCapabilityNoOpInvalidationSuppressedCount: metrics.noOpInvalidationSuppressedCount,
        hostCapabilityStaleInvalidationIgnoredCount: metrics.staleInvalidationIgnoredCount,
        hostCapabilityCompatibilityDenialCount: metrics.compatibilityDenialCount,
      };
    },
  };
}

function registerPersistenceCapability(rawSignals, registration, diagnosticsRecorder) {
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
    if (disposed) {
      return;
    }
    disposed = true;
    metrics.disposalCount += 1;
  }

  const handle = Object.freeze({
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
  });

  return {
    hostEntry: handle,
    dispose,
    performanceSummary() {
      return {
        hostCapabilityRegistrationCount: metrics.registrationCount,
        hostCapabilityDisposalCount: metrics.disposalCount,
        hostCapabilityReadCount: metrics.readCount,
        hostCapabilityPollCount: 0,
        hostCapabilityNoOpPollCount: 0,
        hostCapabilityManualCommitCount: metrics.manualCommitCount,
        hostCapabilityNoOpManualCommitCount: metrics.noOpManualCommitCount,
        hostCapabilityInvalidationCount: metrics.invalidationCount,
        hostCapabilityInvalidationBatchFlushCount: metrics.invalidationBatchFlushCount,
        hostCapabilityReevaluationCount: metrics.reevaluationCount,
        hostCapabilityInvalidationTouchedNodeCount: metrics.invalidationTouchedNodeCount,
        hostCapabilityNoOpInvalidationSuppressedCount: metrics.noOpInvalidationSuppressedCount,
        hostCapabilityStaleInvalidationIgnoredCount: metrics.staleInvalidationIgnoredCount,
        hostCapabilityCompatibilityDenialCount: metrics.compatibilityDenialCount,
      };
    },
  };
}

function registerVisibilityCapability(rawSignals, registration, diagnosticsRecorder) {
  return registerBinaryCapability(rawSignals, registration, diagnosticsRecorder, {
    family: "visibility",
    positiveState: "visible",
    negativeState: "hidden",
    booleanMethodName: "isVisible",
    handleBrand: HOST_VISIBILITY_HANDLE_BRAND,
  });
}

function registerOnlineCapability(rawSignals, registration, diagnosticsRecorder) {
  return registerBinaryCapability(rawSignals, registration, diagnosticsRecorder, {
    family: "online",
    positiveState: "online",
    negativeState: "offline",
    booleanMethodName: "isOnline",
    handleBrand: HOST_ONLINE_HANDLE_BRAND,
  });
}

export function createHostCapabilities(rawSignals, options) {
  const plan = parseHostCapabilityPlan(options);
  const diagnosticsRecorder = createDiagnosticsRecorder();
  const globalMetrics = emptyPerformanceSummary();
  const exportedUnavailableArtifactKeys = new Set();
  function recordPortableImportDenial(unavailableCallbacks) {
    const callbacksByFamily = new Map();
    for (const artifact of unavailableCallbacks) {
      if (!artifact || typeof artifact.id !== "string") {
        continue;
      }
      const transports = Array.isArray(artifact.hostCapabilityTransports)
        ? artifact.hostCapabilityTransports
        : [];
      for (const transport of transports) {
        if (!transport || typeof transport.family !== "string") {
          continue;
        }
        const ids = callbacksByFamily.get(transport.family) ?? [];
        ids.push(artifact.id);
        callbacksByFamily.set(transport.family, ids);
      }
    }
    const transports = unavailableCallbacks.flatMap((artifact) =>
      Array.isArray(artifact?.hostCapabilityTransports) ? artifact.hostCapabilityTransports : []);
    for (const transport of transports) {
      if (transport?.portableImportOutcome !== "Denied" && transport?.portableImportOutcome !== "Incompatible") {
        continue;
      }
      const callbackIds = callbacksByFamily.get(transport.family) ?? [];
      globalMetrics.hostCapabilityCompatibilityDenialCount += 1;
      diagnosticsRecorder.push({
        kind: "PortableImportDenied",
        family: transport.family,
        registrationId: transport.registrationId,
        compatibility: transport.compatibility,
        invalidationMode: null,
        queuedInvalidationCount: 0,
        previousState: null,
        nextState: null,
        touchedNodes: 0,
        reevaluatedNodes: 0,
        portableImportOutcome: transport.portableImportOutcome,
        portableImportReason: transport.portableImportReason,
        deniedCallbackIds: callbackIds,
      });
    }
  }
  function recordExportedUnavailableCallbacks(unavailableCallbacks) {
    const artifacts = Array.isArray(unavailableCallbacks)
      ? unavailableCallbacks.filter((artifact) => Array.isArray(artifact?.hostCapabilityTransports) && artifact.hostCapabilityTransports.length > 0)
      : [];
    for (const artifact of artifacts) {
      exportedUnavailableArtifactKeys.add(unavailableArtifactKey(artifact));
    }
    globalMetrics.hostCapabilityUnavailabilityArtifactCount = exportedUnavailableArtifactKeys.size;
  }
  if (!plan) {
    return {
      host: Object.freeze({}),
      dispose() {},
      performanceSummary() {
        return { ...globalMetrics };
      },
      latestDiagnosticsEvent() {
        return diagnosticsRecorder.latest();
      },
      recentDiagnosticsEvents() {
        return diagnosticsRecorder.recent();
      },
      recordExportedUnavailableCallbacks,
      recordPortableImportDenial,
    };
  }

  const disposers = [];
  const metricsReaders = [];
  const host = {};
  if (plan.viewport) {
    const registration = registerViewportCapability(rawSignals, plan.viewport, diagnosticsRecorder);
    host.viewport = registration.hostEntry;
    disposers.push(registration.dispose);
    metricsReaders.push(registration.performanceSummary);
  }
  if (plan.visibility) {
    const registration = registerVisibilityCapability(rawSignals, plan.visibility, diagnosticsRecorder);
    host.visibility = registration.hostEntry;
    disposers.push(registration.dispose);
    metricsReaders.push(registration.performanceSummary);
  }
  if (plan.online) {
    const registration = registerOnlineCapability(rawSignals, plan.online, diagnosticsRecorder);
    host.online = registration.hostEntry;
    disposers.push(registration.dispose);
    metricsReaders.push(registration.performanceSummary);
  }
  if (plan.clock) {
    const registration = registerClockCapability(rawSignals, plan.clock, diagnosticsRecorder);
    host.clock = registration.hostEntry;
    disposers.push(registration.dispose);
    metricsReaders.push(registration.performanceSummary);
  }
  if (plan.persistence) {
    const registration = registerPersistenceCapability(rawSignals, plan.persistence, diagnosticsRecorder);
    host.persistence = registration.hostEntry;
    disposers.push(registration.dispose);
    metricsReaders.push(registration.performanceSummary);
  }

  return {
    host: Object.freeze(host),
    dispose() {
      for (const disposer of disposers.splice(0)) {
        disposer();
      }
    },
    performanceSummary() {
      return metricsReaders.reduce((summary, readSummary) => {
        const metrics = readSummary();
        for (const [key, value] of Object.entries(metrics)) {
          summary[key] = (summary[key] ?? 0) + value;
        }
        return summary;
      }, { ...globalMetrics });
    },
    latestDiagnosticsEvent() {
      return diagnosticsRecorder.latest();
    },
    recentDiagnosticsEvents() {
      return diagnosticsRecorder.recent();
    },
    recordExportedUnavailableCallbacks,
    recordPortableImportDenial,
  };
}
