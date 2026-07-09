function requireListOptions(options) {
  if (
    !options
    || typeof options !== "object"
    || Array.isArray(options)
    || typeof options.identity !== "string"
    || options.identity.length === 0
  ) {
    throw new TypeError(
      "signals.local.listState(...) requires a non-empty string identity",
    );
  }
  if (!Array.isArray(options.initial)) {
    throw new TypeError(
      "signals.local.listState(...) requires an array initial value",
    );
  }
  return options;
}

export function createLocalListState(namespace, options) {
  const normalized = requireListOptions(options);
  const scope = namespace.scope(normalized.identity);
  const items = scope.spec.input("items", [...normalized.initial], {
    debugName: normalized.debugName ?? `${scope.scopeId}.items`,
    ...(normalized.aspects === undefined
      ? {}
      : { producesAspects: normalized.aspects }),
  });
  return Object.freeze({
    scope,
    scopeId: scope.scopeId,
    items,
    reset() {
      return items.reset();
    },
    free() {
      items.free();
    },
    [Symbol.dispose]() {
      items[Symbol.dispose]();
    },
  });
}
