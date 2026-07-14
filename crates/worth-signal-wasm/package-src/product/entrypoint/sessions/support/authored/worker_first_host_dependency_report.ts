import {
  digestCanonicalDiagnosticValue,
} from "../../../../canonical_diagnostic_digest.js";

export function buildWorkerFirstHostDependencyReport(authoredCallbacks) {
  const dependenciesById = new Map();
  const callbackRows = [];
  for (const [id, authoredCallback] of authoredCallbacks) {
    const dependencies = authoredCallback.hostDependencies ?? [];
    if (dependencies.length === 0) {
      continue;
    }
    callbackRows.push({
      id,
      family: authoredCallback.family,
      dependencyIds: dependencies.map((dependency) => dependency.dependencyId).sort(),
    });
    for (const dependency of dependencies) {
      dependenciesById.set(dependency.dependencyId, dependency);
    }
  }
  const dependencies = [...dependenciesById.values()].sort((left, right) => (
    left.dependencyId.localeCompare(right.dependencyId)
  ));
  const report = {
    totals: {
      callbackCount: authoredCallbacks.size,
      dependentCallbackCount: callbackRows.length,
      dependencyEdgeCount: callbackRows.reduce((count, row) => count + row.dependencyIds.length, 0),
      distinctDependencyCount: dependencies.length,
    },
    dependencies,
    callbacks: callbackRows.sort((left, right) => left.id.localeCompare(right.id)),
  };
  return Object.freeze({
    ...report,
    dependencyDigest: digestCanonicalDiagnosticValue(dependencies),
    callbackDigest: digestCanonicalDiagnosticValue(report.callbacks),
    digest: digestCanonicalDiagnosticValue(report),
  });
}
