export function materializeGraphDiagnosticsSurface(surface) {
  return {
    graph: surface.graph,
    contract: surface.contract,
    dependencies: { ...surface.dependencies },
    inputDescriptors: surface.inputDescriptors,
    descriptors: surface.descriptors,
    inputVersions: surface.inputVersions,
    outputVersions: surface.outputVersions,
    inputs: { ...surface.inputs },
    outputs: { ...surface.outputs },
    runtimeGraph: surface.runtimeGraph,
    executionHistory: surface.executionHistory,
    latestFlow: surface.latestFlow,
    latestObservation: surface.latestObservation,
  };
}

export function materializeGraphHistorySurface(surface) {
  return {
    graph: surface.graph,
    contract: surface.contract,
    dependencies: { ...surface.dependencies },
    inputDescriptors: surface.inputDescriptors,
    descriptors: surface.descriptors,
    inputs: { ...surface.inputs },
    outputs: { ...surface.outputs },
    executionHistory: surface.executionHistory,
    recentHistory: surface.recentHistory,
  };
}
