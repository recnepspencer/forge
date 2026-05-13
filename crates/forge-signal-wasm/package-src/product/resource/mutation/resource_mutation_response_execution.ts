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
import {
  createMutationResponseTargetEffectProof,
} from "./resource_mutation_response_lifecycle_proof.js";
import {
  executePreparedMutationResponseIdentityMigration,
} from "./identity/resource_mutation_response_identity_migration.js";

function prepareMutationResponsePlanIfDeclared(
  lineIdentity,
  requestDescriptor,
  diagnostics,
  declaration,
  responseValue,
  submittedTargets = null,
  submittedIdentityMigration = null,
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
    submittedTargets,
    submittedIdentityMigration,
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
    if (execution.kind === "fallback") {
      executedArtifacts.push(plannedArtifact);
      continue;
    }
    const result = executeLineDelivery(
      execution.targetMaterialization,
      execution.delivery,
    );
    if (
      result.kind !== "applied"
      && !(
        result.kind === "duplicateIgnored"
        && execution.targetMaterialization.binding.diagnosticsSignal().lastDeliveryPacketId
          === execution.delivery.packetId
      )
    ) {
      throw new TypeError(
        `mutation response exact reconciliation for ${execution.targetId} did not apply: ${result.kind}`,
      );
    }
    const diagnostics = execution.targetMaterialization.binding.diagnosticsSignal();
    const effect = diagnostics.lastEffect ?? null;
    executedArtifacts.push(Object.freeze({
      ...plannedArtifact,
      outcomeKind: "applied",
      deliveryKind: diagnostics.lastDeliveryKind,
      deliveryScope: diagnostics.lastDeliveryScope,
      effectId: effect?.effectId ?? null,
      effectProof: createMutationResponseTargetEffectProof(effect, plannedArtifact),
      targetVisibleValueVersion: diagnostics.visibleValueVersion,
    }));
  }
  const executedIdentityMigration = executePreparedMutationResponseIdentityMigration(
    plan.identityMigration,
  );
  return createExecutedMutationResponsePlan(
    {
      ...plan,
      identityMigration: executedIdentityMigration,
    },
    executedArtifacts,
  );
}

export {
  executePreparedMutationResponsePlan,
  prepareMutationResponsePlanIfDeclared,
  recordMutationResponsePlanIfPresent,
};
