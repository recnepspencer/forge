import {
  readMutationResponsePlanRecord,
} from "../../mutation/resource_mutation_response_diagnostics_projection.js";

function readLineDiagnostics(materialization) {
  const diagnostics = materialization.binding.diagnosticsSignal();
  const publicDiagnostics = {
    ...diagnostics,
  };
  const mutationResponsePlanRecord = readMutationResponsePlanRecord(diagnostics);
  if (mutationResponsePlanRecord !== null) {
    publicDiagnostics.lastMutationResponsePlan =
      mutationResponsePlanRecord.plan;
    publicDiagnostics.mutationResponsePlanCount =
      mutationResponsePlanRecord.planCount;
  }
  Object.defineProperty(publicDiagnostics, "visibleSelection", {
    value: diagnostics.visibleSelection,
    enumerable: false,
    configurable: false,
    writable: false,
  });
  return Object.freeze(publicDiagnostics);
}

export { readLineDiagnostics };
