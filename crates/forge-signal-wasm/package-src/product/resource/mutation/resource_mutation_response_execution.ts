import { recordLineHistoryEntry } from "../lines/history/record_line_history_entry.js";
import { executeLineDelivery } from "../lines/actions/line_delivery_execution.js";
import {
  createMutationResponsePlannedDiagnostics,
} from "../lines/state/line_diagnostics_value.js";
import {
  RESOURCE_MUTATION_RESPONSE_PREPARED_EXECUTIONS,
  createExecutedMutationResponsePlan,
  createPreparedMutationResponsePlan,
  readMutationResponseDeclaration,
} from "./resource_mutation_response_plan.js";

function prepareMutationResponsePlanIfDeclared(
  lineIdentity,
  requestDescriptor,
  diagnostics,
  declaration,
  responseValue,
) {
  const readDeclaration = readMutationResponseDeclaration(declaration);
  if (readDeclaration === null) {
    return Object.freeze({
      plan: null,
      diagnostics,
    });
  }
  const plan = createPreparedMutationResponsePlan({
    lineIdentity,
    requestDescriptor,
    diagnostics,
    declaration: readDeclaration,
    responseValue,
  });
  if (plan === null) {
    return Object.freeze({
      plan: null,
      diagnostics,
    });
  }
  const executedPlan = executePreparedMutationResponsePlan(plan);
  return Object.freeze({
    plan: executedPlan,
    diagnostics: createMutationResponsePlannedDiagnostics(diagnostics, executedPlan),
  });
}

function recordMutationResponsePlanIfPresent(lifecycleHistory, binding, plan) {
  if (plan === null) {
    return null;
  }
  recordLineHistoryEntry(
    lifecycleHistory,
    binding,
    "mutationResponsePlanned",
  );
  return plan;
}

function executePreparedMutationResponsePlan(plan) {
  if (plan === null) {
    return null;
  }
  const preparedExecutions = plan[RESOURCE_MUTATION_RESPONSE_PREPARED_EXECUTIONS];
  const plannedArtifacts = plan.executionArtifacts;
  const executedArtifacts = [];
  for (let index = 0; index < preparedExecutions.length; index += 1) {
    const execution = preparedExecutions[index];
    const plannedArtifact = plannedArtifacts[index];
    if (execution.kind !== "exactDetail") {
      executedArtifacts.push(plannedArtifact);
      continue;
    }
    const result = executeLineDelivery(
      execution.targetMaterialization,
      execution.delivery,
    );
    if (result.kind !== "applied") {
      throw new TypeError(
        `mutation response exact detail reconciliation for ${execution.targetId} did not apply: ${result.kind}`,
      );
    }
    const diagnostics = execution.targetMaterialization.binding.diagnosticsSignal();
    executedArtifacts.push(Object.freeze({
      ...plannedArtifact,
      outcomeKind: "applied",
      deliveryKind: diagnostics.lastDeliveryKind,
      deliveryScope: diagnostics.lastDeliveryScope,
      effectId: diagnostics.lastEffect?.effectId ?? null,
      targetVisibleValueVersion: diagnostics.visibleValueVersion,
    }));
  }
  return createExecutedMutationResponsePlan(plan, executedArtifacts);
}

export {
  executePreparedMutationResponsePlan,
  prepareMutationResponsePlanIfDeclared,
  recordMutationResponsePlanIfPresent,
};
