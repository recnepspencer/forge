function readLineDiagnostics(materialization) {
  const diagnostics = materialization.binding.diagnosticsSignal();
  const publicDiagnostics = {
    ...diagnostics,
  };
  Object.defineProperty(publicDiagnostics, "visibleSelection", {
    value: diagnostics.visibleSelection,
    enumerable: false,
    configurable: false,
    writable: false,
  });
  return Object.freeze(publicDiagnostics);
}

export { readLineDiagnostics };
