import { isPublicGraphInputEntry } from "../../public_inputs.js";
import { PRODUCT_SIGNAL_KIND } from "../../symbols.js";
import { FormDeclarationError } from "../form_errors.js";
import { cloneFormValue, stableValueDigest } from "../values/value_paths.js";

const FORM_SOURCE_BRAND = Symbol("forge.form.sourceAuthorityDeclaration");
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
      const valueDigest = stableValueDigest(value);
      return Object.freeze({
        kind: declaration.kind,
        sourceId,
        explicit: declaration.explicit,
        contract: authority.contract,
        sourceValueDigest: valueDigest,
        sourceAuthorityDigest: stableValueDigest({
          kind: declaration.kind,
          sourceId,
          explicit: declaration.explicit,
          contract: authority.contract,
          sourceValueDigest: valueDigest,
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

function isFormSourceDeclaration(value) {
  return !!value && value[FORM_SOURCE_BRAND] === true;
}

function sourceReader(kind, target) {
  if (kind === "graphPublicInput") {
    return () => cloneFormValue(target.handle());
  }
  if (kind === "resourceLine") {
    return () => cloneFormValue(target.value());
  }
  if (kind === "signal") {
    return () => cloneFormValue(target());
  }
  if (typeof target === "function") {
    return () => cloneFormValue(target());
  }
  if (target && typeof target.get === "function") {
    return () => cloneFormValue(target.get());
  }
  return () => cloneFormValue(target);
}

function defaultSourceId(declaration) {
  if (declaration.kind === "graphPublicInput") {
    return `graphPublicInput:${declaration.target.handle.id ?? "<anonymous>"}`;
  }
  if (declaration.kind === "signal") {
    return `signal:${declaration.target.id ?? "<anonymous>"}`;
  }
  if (declaration.kind === "resourceLine") {
    const descriptor = safeCall(() => declaration.target.descriptor());
    return `resourceLine:${stableValueDigest(descriptor)}`;
  }
  return "externalBoundary:<anonymous>";
}

function sourceIdentity(kind, target) {
  if (kind === "graphPublicInput") {
    return Object.freeze({
      handleId: target.handle.id ?? null,
      authority: target.authority,
      requiredness: target.requiredness,
    });
  }
  if (kind === "signal") {
    return Object.freeze({
      signalId: target.id ?? null,
      signalKind: target[PRODUCT_SIGNAL_KIND] ?? null,
    });
  }
  if (kind === "resourceLine") {
    return Object.freeze({
      descriptorDigest: stableValueDigest(safeCall(() => target.descriptor())),
      requestDigest: stableValueDigest(safeCall(() => target.request())),
    });
  }
  return Object.freeze({
    readable: typeof target === "function" ? "function" : typeof target,
  });
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
  return typeof handle === "function" && typeof handle.get === "function"
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
