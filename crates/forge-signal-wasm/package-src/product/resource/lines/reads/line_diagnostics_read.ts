function readLineDiagnostics(materialization) {
  return materialization.binding.diagnosticsSignal();
}

export { readLineDiagnostics };
