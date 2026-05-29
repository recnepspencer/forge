function requireFormSourceOptions(options) {
  if (
    !options
    || typeof options !== "object"
    || Array.isArray(options)
    || typeof options.identity !== "string"
    || options.identity.length === 0
  ) {
    throw new TypeError(
      "signals.local.formSource(...) requires a non-empty string identity",
    );
  }
  return options;
}

export function createLocalFormSourceState(namespace, options) {
  const normalized = requireFormSourceOptions(options);
  const scope = namespace.scope(normalized.identity);
  const signal = scope.spec.input("source", normalized.initial, {
    debugName: normalized.debugName ?? `${scope.scopeId}.source`,
  });
  const source = namespace.form.source.signal(signal, {
    id: normalized.sourceId ?? `${scope.scopeId}.source`,
    ...(normalized.contract === undefined
      ? {}
      : { contract: normalized.contract }),
  });
  return Object.freeze({
    scope,
    scopeId: scope.scopeId,
    signal,
    source,
    reset() {
      return signal.reset();
    },
    free() {
      signal.free();
    },
    [Symbol.dispose]() {
      signal[Symbol.dispose]();
    },
  });
}
