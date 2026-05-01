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

function visibilityCapability(options) {
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

function viewportCapability(options) {
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

function onlineCapability(options) {
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

function clockCapability(options) {
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

function persistenceCapability(options) {
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

function hostCapabilityPlan(options) {
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

export {
  HOST_CAPABILITY_PLAN_BRAND,
  HOST_VIEWPORT_HANDLE_BRAND,
  HOST_VISIBILITY_HANDLE_BRAND,
  HOST_ONLINE_HANDLE_BRAND,
  HOST_CLOCK_HANDLE_BRAND,
  HOST_PERSISTENCE_HANDLE_BRAND,
  cloneSignalValue,
  createDiagnosticsRecorder,
  emptyPerformanceSummary,
  hostCapabilityPlan,
  nextHiddenHostSignalId,
  normalizeBinaryCapabilityState,
  normalizeClockValue,
  normalizePollMs,
  normalizeSubscription,
  normalizeViewportState,
  parseHostCapabilityPlan,
  persistenceCapability,
  onlineCapability,
  unavailableArtifactKey,
  viewportCapability,
  visibilityCapability,
  clockCapability,
};
