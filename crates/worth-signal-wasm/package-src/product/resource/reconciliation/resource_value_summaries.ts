const RESOURCE_VALUE_SUMMARIES = Symbol("WorthSignal.resourceValueSummaries");

function createResourceValueSummaries(definitions, patchScope) {
  if (!definitions || typeof definitions !== "object" || Array.isArray(definitions)) {
    throw new TypeError("resourceValueSummaries(...) requires a definition object");
  }
  const normalizedDefinitions = {};
  for (const [summary, definition] of Object.entries(definitions)) {
    if (!definition || typeof definition !== "object" || Array.isArray(definition)) {
      throw new TypeError(`resourceValueSummaries(...) summary "${summary}" must be an object`);
    }
    if (typeof definition.read !== "function") {
      throw new TypeError(`resourceValueSummaries(...) summary "${summary}" requires read(...)`);
    }
    if (typeof definition.write !== "function") {
      throw new TypeError(`resourceValueSummaries(...) summary "${summary}" requires write(...)`);
    }
    normalizedDefinitions[summary] = Object.freeze({
      read: definition.read,
      write: definition.write,
    });
  }
  return Object.freeze({
    definitions: Object.freeze(normalizedDefinitions),
    patchScope,
    [RESOURCE_VALUE_SUMMARIES]: "resourceValueSummaries",
  });
}

function resourceValueSummaries(definitions) {
  return createResourceValueSummaries(definitions, "line");
}

resourceValueSummaries.pageWindow = function pageWindow(definitions) {
  return createResourceValueSummaries(definitions, "pageWindow");
};

function requireResourceValueSummaries(value, kind) {
  if (
    !value ||
    typeof value !== "object" ||
    value[RESOURCE_VALUE_SUMMARIES] !== "resourceValueSummaries"
  ) {
    const label = kind.includes("(")
      ? `${kind} requires summaries created with resourceValueSummaries(...)`
      : `${kind} resources require summaries created with resourceValueSummaries(...)`;
    throw new TypeError(label);
  }
  return value;
}

export { requireResourceValueSummaries, resourceValueSummaries };
