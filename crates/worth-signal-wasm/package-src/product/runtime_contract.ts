const SIGNALS_RUNTIME_SURFACE_VERSION = "1";

const KNOWN_RUNTIME_CAPABILITIES = Object.freeze([
  "callableSurface",
  "scopedAuthoring",
  "specNamespace",
  "workerRuntime",
]);

function freezeObject(value) {
  return Object.freeze(value);
}

function requirePlainObject(value, operation) {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(`${operation} requires an options object`);
  }
  return value;
}

function requireRuntimeCapabilities(capabilities, operation) {
  const candidate = requirePlainObject(capabilities, `${operation} capabilities`);
  return freezeObject({
    callableSurface: candidate.callableSurface === true,
    scopedAuthoring: candidate.scopedAuthoring === true,
    specNamespace: candidate.specNamespace === true,
    workerRuntime: candidate.workerRuntime === true,
  });
}

export function createSignalsRuntimeContract(definition) {
  const candidate = requirePlainObject(
    definition,
    "signals runtime contract construction",
  );
  if (typeof candidate.surfaceFamily !== "string" || candidate.surfaceFamily.length === 0) {
    throw new TypeError(
      "signals runtime contract construction requires a non-empty string surfaceFamily",
    );
  }
  if (
    candidate.deployment !== "mainThreadCompatibility" &&
    candidate.deployment !== "workerFirst"
  ) {
    throw new TypeError(
      "signals runtime contract construction requires deployment to be \"mainThreadCompatibility\" or \"workerFirst\"",
    );
  }
  if (
    candidate.scopeId !== null &&
    candidate.scopeId !== undefined &&
    (typeof candidate.scopeId !== "string" || candidate.scopeId.length === 0)
  ) {
    throw new TypeError(
      "signals runtime contract construction requires scopeId to be null, undefined, or a non-empty string",
    );
  }
  return freezeObject({
    surfaceFamily: candidate.surfaceFamily,
    surfaceVersion: SIGNALS_RUNTIME_SURFACE_VERSION,
    deployment: candidate.deployment,
    scopeId: candidate.scopeId ?? null,
    capabilities: requireRuntimeCapabilities(
      candidate.capabilities,
      "signals runtime contract construction",
    ),
  });
}

function requireKnownRuntimeCapabilityName(capability, operation) {
  if (!KNOWN_RUNTIME_CAPABILITIES.includes(capability)) {
    throw new TypeError(
      `${operation} received unknown runtime capability \`${String(capability)}\`; expected one of ${KNOWN_RUNTIME_CAPABILITIES.map((value) => `\`${value}\``).join(", ")}`,
    );
  }
  return capability;
}

function requireRequestedCapabilities(options, operation) {
  const candidate = requirePlainObject(options, operation);
  const requires = candidate.requires;
  if (requires === undefined) {
    return freezeObject([]);
  }
  if (!Array.isArray(requires)) {
    throw new TypeError(`${operation} requires \`requires\` to be an array when provided`);
  }
  const seen = new Set();
  const normalized = [];
  for (const capability of requires) {
    const known = requireKnownRuntimeCapabilityName(capability, operation);
    if (seen.has(known)) {
      continue;
    }
    seen.add(known);
    normalized.push(known);
  }
  return freezeObject(normalized);
}

export function assertSignalsRuntimeCompatibility(
  contract,
  options,
  operation = "signals.assertCompatibility",
) {
  const requires = requireRequestedCapabilities(options, operation);
  const missing = requires.filter((capability) => contract.capabilities[capability] !== true);
  if (missing.length === 0) {
    return contract;
  }
  const error = new Error(
    `${operation} requires capabilities ${requires.map((capability) => `\`${capability}\``).join(", ")}, but surface \`${contract.surfaceFamily}\` is missing ${missing.map((capability) => `\`${capability}\``).join(", ")}`,
  );
  error.name = "SignalsCompatibilityAssertionError";
  error.code = "signalsCompatibilityAssertionFailed";
  error.requiredCapabilities = requires;
  error.missingCapabilities = freezeObject(missing);
  error.contract = contract;
  throw error;
}

