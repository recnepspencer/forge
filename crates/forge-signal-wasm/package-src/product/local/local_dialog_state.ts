function requireDialogIdentity(options) {
  if (
    !options
    || typeof options !== "object"
    || Array.isArray(options)
    || typeof options.identity !== "string"
    || options.identity.length === 0
  ) {
    throw new TypeError(
      "signals.local.dialogState(...) requires a non-empty string identity",
    );
  }
  return options.identity;
}

export function createLocalDialogState(namespace, options) {
  const identity = requireDialogIdentity(options);
  const scope = namespace.scope(identity);
  const signal = scope.spec.input("open", Boolean(options.initialOpen), {
    debugName: options.debugName ?? `${scope.scopeId}.open`,
  });
  return Object.freeze({
    scope,
    scopeId: scope.scopeId,
    signal,
    open() {
      return signal.set(true);
    },
    close() {
      return signal.set(false);
    },
    toggle() {
      return signal.set(!signal());
    },
    free() {
      signal.free();
    },
    [Symbol.dispose]() {
      signal[Symbol.dispose]();
    },
  });
}
