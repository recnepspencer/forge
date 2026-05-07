function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function requireAuthoringOptions(family, options) {
  if (!isPlainObject(options)) {
    throw new TypeError(`${family} options must be an object when provided`);
  }
  return options;
}

function requireOptionalDebugName(family, options) {
  if (options.debugName === undefined) {
    return null;
  }
  if (typeof options.debugName !== "string" || options.debugName.length === 0) {
    throw new TypeError(
      `${family} debugName must be a non-empty string when provided`,
    );
  }
  return options.debugName;
}

function forbidOpaqueIdOption(family, options) {
  if (Object.prototype.hasOwnProperty.call(options, "id")) {
    throw new TypeError(
      `${family} app authoring does not accept id; use ${family === "input" ? "signals.spec.input" : `signals.spec.${family}`} when you need an explicit structural name`,
    );
  }
}

function looksLikeInputMetadataOptions(value) {
  if (
    !isPlainObject(value) ||
    typeof value.id !== "string" ||
    value.id.length === 0
  ) {
    return false;
  }
  return Object.keys(value).every(
    (key) => key === "id" || key === "producesAspects",
  );
}

function looksLikeOpaqueAuthoringOptions(value) {
  if (!isPlainObject(value)) {
    return false;
  }
  return Object.keys(value).every(
    (key) => key === "debugName" || key === "producesAspects",
  );
}

export {
  forbidOpaqueIdOption,
  isPlainObject,
  looksLikeInputMetadataOptions,
  looksLikeOpaqueAuthoringOptions,
  requireAuthoringOptions,
  requireOptionalDebugName,
};
