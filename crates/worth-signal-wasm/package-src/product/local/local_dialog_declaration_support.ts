export const BUILT_IN_ACTION_IDS = ["close", "confirm", "discard"];

export function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

export function requireDialogOptions(options) {
  if (!isPlainObject(options)) {
    throw new TypeError("signals.local.dialogState(...) requires an options object");
  }
  if (typeof options.identity !== "string" || options.identity.length === 0) {
    throw new TypeError("signals.local.dialogState(...) requires a non-empty string identity");
  }
  if (options.actions !== undefined && typeof options.actions !== "function") {
    throw new TypeError("signals.local.dialogState(...) actions must be declared with an actions(...) builder");
  }
  return options;
}

export function normalizeInitialState(options) {
  const initial = isPlainObject(options.initial) ? options.initial : {};
  return Object.freeze({
    isOpen: Boolean(initial.isOpen),
    mode: initial.mode ?? null,
    data: initial.data ?? null,
    context: initial.context ?? null,
    loading: Boolean(initial.loading),
  });
}

export function normalizeCustomActions(options, scopeId) {
  if (!options.actions) {
    return Object.freeze({});
  }
  const declared = options.actions({
    custom(definition) {
      if (!isPlainObject(definition) || typeof definition.execute !== "function") {
        throw new TypeError(`signals.local.dialogState(...) custom actions in "${scopeId}" require an execute(...) function`);
      }
      return Object.freeze({ ...definition });
    },
  });
  if (!isPlainObject(declared)) {
    throw new TypeError(`signals.local.dialogState(...) actions(...) for "${scopeId}" must return a plain object`);
  }
  for (const [key, value] of Object.entries(declared)) {
    if (BUILT_IN_ACTION_IDS.includes(key)) {
      throw new TypeError(`signals.local.dialogState(...) custom action "${key}" in "${scopeId}" conflicts with a built-in action`);
    }
    if (!isPlainObject(value) || typeof value.execute !== "function") {
      throw new TypeError(`signals.local.dialogState(...) action "${key}" in "${scopeId}" must be authored with custom({...})`);
    }
  }
  return Object.freeze({ ...declared });
}

export function createActionRuntimeMap(actionIds) {
  return new Map(actionIds.map((actionId) => [actionId, { pending: false, latestExecution: null }]));
}
