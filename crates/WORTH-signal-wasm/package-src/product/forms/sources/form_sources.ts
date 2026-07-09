import { isPublicGraphInputEntry } from "../../public_inputs.js";
import { PRODUCT_SIGNAL_KIND } from "../../symbols.js";
import { FormDeclarationError } from "../form_errors.js";
import { cloneFormValue, stableValueDigest } from "../values/value_paths.js";

const FORM_SOURCE_BRAND = Symbol("WORTH.form.sourceAuthorityDeclaration");
const SOURCE_KINDS = new Set([
  "signal",
  "graphPublicInput",
  "resourceLine",
  "externalBoundary",
]);

export function createFormSourceFactory() {
  return Object.freeze({
    signal(handle, options = {}) {
      requireSignalHandle(handle, "form.source.signal(...)");
      return sourceDeclaration("signal", handle, options);
    },
    graphPublicInput(entry, options = {}) {
      if (!isPublicGraphInputEntry(entry)) {
        throw new FormDeclarationError(
          "form.source.graphPublicInput(...) expects signals.publicInput(...) output",
        );
      }
      return sourceDeclaration("graphPublicInput", entry, options);
    },
    resourceLine(line, options = {}) {
      requireResourceLine(line);
      return sourceDeclaration("resourceLine", line, options);
    },
    external(readable, options = {}) {
      return sourceDeclaration("externalBoundary", readable, options);
    },
  });
}

export function materializeFormSourceAuthority(source) {
  const declaration = isFormSourceDeclaration(source)
    ? source
    : inferLegacySourceDeclaration(source);
  const read = sourceReader(declaration.kind, declaration.target);
  const sourceId = declaration.options.id ?? defaultSourceId(declaration);
  const authority = Object.freeze({
    kind: declaration.kind,
    sourceId,
    explicit: declaration.explicit,
    contract: declaration.options.contract ?? "phase1-source-authority-v1",
    read,
    diagnostics() {
      const value = read();
      const sourceValueDigest = stableValueDigest(value);
      return Object.freeze({
        kind: declaration.kind,
        sourceId,
        explicit: declaration.explicit,
        contract: authority.contract,
        sourceValueDigest,
        sourceAuthorityDigest: stableValueDigest({
          kind: declaration.kind,
          sourceId,
          explicit: declaration.explicit,
          contract: authority.contract,
          sourceValueDigest,
        }),
        identity: sourceIdentity(declaration.kind, declaration.target),
      });
    },
  });
  return authority;
}

export function readSource(sourceOrAuthority) {
  if (sourceOrAuthority && typeof sourceOrAuthority.read === "function") {
    return sourceOrAuthority.read();
  }
  return materializeFormSourceAuthority(sourceOrAuthority).read();
}

export function readSourceSchemaVersion(source) {
  if (!isSourceDescriptor(source) || source.schemaVersion === undefined) {
    return null;
  }
  return normalizeSchemaVersion(readSourceValue(source.schemaVersion));
}

export function readSourceDraftMigration(source) {
  return isSourceDescriptor(source) && typeof source.migrateDraft === "function"
    ? source.migrateDraft
    : null;
}

export function readSourceBootstrapArtifact(source, dependency) {
  if (!isSourceDescriptor(source) || source[dependency] === undefined) {
    return null;
  }
  return normalizeSourceBootstrapArtifact(readSourceValue(source[dependency]), dependency);
}

export function readResourceLineHandle(source) {
  const declaration = isFormSourceDeclaration(source)
    ? source
    : inferLegacySourceDeclaration(source);
  if (declaration.kind !== "resourceLine") {
    return null;
  }
  return sourceRuntimeTarget(declaration.target);
}

function sourceDeclaration(kind, target, options = {}) {
  requireSourceKind(kind);
  requireSourceOptions(options);
  return Object.freeze({
    [FORM_SOURCE_BRAND]: true,
    kind,
    target,
    options: Object.freeze({ ...options }),
    explicit: true,
  });
}

function inferLegacySourceDeclaration(source) {
  if (isSourceDescriptor(source)) {
    const descriptorValue = source.value;
    if (isFormSourceDeclaration(descriptorValue)) {
      return descriptorWrappedSourceDeclaration(descriptorValue, source);
    }
    if (isPublicGraphInputEntry(descriptorValue)) {
      return descriptorWrappedLegacySourceDeclaration("graphPublicInput", source);
    }
    if (isSignalHandle(descriptorValue)) {
      return descriptorWrappedLegacySourceDeclaration("signal", source);
    }
    if (isResourceLine(descriptorValue)) {
      return descriptorWrappedLegacySourceDeclaration("resourceLine", source);
    }
  }
  if (isPublicGraphInputEntry(source)) {
    return legacySourceDeclaration("graphPublicInput", source);
  }
  if (isSignalHandle(source)) {
    return legacySourceDeclaration("signal", source);
  }
  if (isResourceLine(source)) {
    return legacySourceDeclaration("resourceLine", source);
  }
  return legacySourceDeclaration("externalBoundary", source);
}

function legacySourceDeclaration(kind, target) {
  return Object.freeze({
    [FORM_SOURCE_BRAND]: true,
    kind,
    target,
    options: Object.freeze({}),
    explicit: false,
  });
}

function descriptorWrappedSourceDeclaration(declaration, descriptor) {
  return Object.freeze({
    [FORM_SOURCE_BRAND]: true,
    kind: declaration.kind,
    target: descriptor,
    options: declaration.options,
    explicit: declaration.explicit,
  });
}

function descriptorWrappedLegacySourceDeclaration(kind, descriptor) {
  return Object.freeze({
    [FORM_SOURCE_BRAND]: true,
    kind,
    target: descriptor,
    options: Object.freeze({}),
    explicit: false,
  });
}

function isFormSourceDeclaration(value) {
  return !!value && value[FORM_SOURCE_BRAND] === true;
}

function sourceReader(kind, target) {
  const normalizedTarget = sourceRuntimeTarget(target);
  if (kind === "graphPublicInput") {
    return () => cloneFormValue(normalizedTarget.handle());
  }
  if (kind === "resourceLine") {
    return () => cloneFormValue(normalizedTarget.value());
  }
  if (kind === "signal") {
    return () => cloneFormValue(normalizedTarget());
  }
  if (isSourceDescriptor(target)) {
    return () => cloneFormValue(readSourceValue(normalizedTarget));
  }
  return () => cloneFormValue(readSourceValue(normalizedTarget));
}

function readSourceValue(source) {
  if (typeof source === "function") {
    return source();
  }
  if (source && typeof source.get === "function") {
    return source.get();
  }
  return source;
}

function defaultSourceId(declaration) {
  const normalizedTarget = sourceRuntimeTarget(declaration.target);
  if (declaration.kind === "graphPublicInput") {
    return `graphPublicInput:${normalizedTarget.handle.id ?? "<anonymous>"}`;
  }
  if (declaration.kind === "signal") {
    return `signal:${normalizedTarget.id ?? "<anonymous>"}`;
  }
  if (declaration.kind === "resourceLine") {
    const descriptor = safeCall(() => normalizedTarget.descriptor());
    return `resourceLine:${stableValueDigest(descriptor)}`;
  }
  return "externalBoundary:<anonymous>";
}

function sourceIdentity(kind, target) {
  const normalizedTarget = sourceRuntimeTarget(target);
  if (kind === "graphPublicInput") {
    return Object.freeze({
      handleId: normalizedTarget.handle.id ?? null,
      authority: normalizedTarget.authority,
      requiredness: normalizedTarget.requiredness,
    });
  }
  if (kind === "signal") {
    return Object.freeze({
      signalId: normalizedTarget.id ?? null,
      signalKind: normalizedTarget[PRODUCT_SIGNAL_KIND] ?? null,
    });
  }
  if (kind === "resourceLine") {
    return Object.freeze({
      descriptorDigest: stableValueDigest(safeCall(() => normalizedTarget.descriptor())),
      requestDigest: stableValueDigest(safeCall(() => normalizedTarget.request())),
    });
  }
  return Object.freeze({
    readable: typeof normalizedTarget === "function" ? "function" : typeof normalizedTarget,
  });
}

function sourceRuntimeTarget(target) {
  if (!isSourceDescriptor(target)) {
    return target;
  }
  return isFormSourceDeclaration(target.value)
    ? target.value.target
    : target.value;
}

function safeCall(callback) {
  try {
    return callback();
  } catch (error) {
    return {
      unavailable: true,
      reason: error instanceof Error ? error.message : String(error),
    };
  }
}

function requireSourceKind(kind) {
  if (!SOURCE_KINDS.has(kind)) {
    throw new FormDeclarationError("form source kind is not supported", { kind });
  }
}

function requireSourceOptions(options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new FormDeclarationError("form source options must be an object");
  }
  if (options.id !== undefined && typeof options.id !== "string") {
    throw new FormDeclarationError("form source id must be a string when provided");
  }
  if (options.contract !== undefined && typeof options.contract !== "string") {
    throw new FormDeclarationError("form source contract must be a string when provided");
  }
}

function requireSignalHandle(handle, label) {
  if (!isSignalHandle(handle)) {
    throw new FormDeclarationError(`${label} expects a signal handle`);
  }
}

function isSignalHandle(handle) {
  return typeof handle === "function"
    && typeof handle.get === "function"
    && typeof handle[PRODUCT_SIGNAL_KIND] === "string";
}

function requireResourceLine(line) {
  if (!isResourceLine(line)) {
    throw new FormDeclarationError(
      "form.source.resourceLine(...) expects a resource line handle",
    );
  }
}

function isResourceLine(line) {
  return !!line
    && typeof line.value === "function"
    && typeof line.descriptor === "function"
    && typeof line.request === "function"
    && typeof line.summary === "function";
}

function isSourceDescriptor(source) {
  return source !== null
    && typeof source === "object"
    && "value" in source
    && !isResourceLine(source);
}

function normalizeSchemaVersion(value) {
  if (value == null) {
    return null;
  }
  return String(value);
}

function normalizeSourceBootstrapArtifact(value, dependency) {
  if (value == null || typeof value !== "object" || Array.isArray(value)) {
    throw new FormDeclarationError(`form source ${dependency} must be an object`, {
      value,
    });
  }
  if (!SOURCE_BOOTSTRAP_STATUSES.has(value.status)) {
    throw new FormDeclarationError(`form source ${dependency} status is not supported`, {
      status: value.status,
    });
  }
  if (typeof value.reason !== "string" || value.reason.length === 0) {
    throw new FormDeclarationError(`form source ${dependency} reason must be a non-empty string`, {
      reason: value.reason,
    });
  }
  if (
    value.token !== undefined
    && value.token !== null
    && typeof value.token !== "string"
    && typeof value.token !== "number"
  ) {
    throw new FormDeclarationError(`form source ${dependency} token must be a string, number, or null`, {
      token: value.token,
    });
  }
  return Object.freeze({
    status: value.status,
    reason: value.reason,
    token: value.token ?? null,
  });
}

const SOURCE_BOOTSTRAP_STATUSES = new Set([
  "pending",
  "busy",
  "settling",
  "ready",
  "unavailable",
]);
