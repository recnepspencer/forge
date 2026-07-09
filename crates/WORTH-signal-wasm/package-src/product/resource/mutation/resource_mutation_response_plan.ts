import { requireMutationResponseLensProof } from "./resource_mutation_response_lens_proof.js";
import { createMutationResponsePayloadDigest } from "./resource_mutation_response_payload_digest.js";
import {
  createMutationResponseDeclaredFieldDigest,
  createMutationResponseUnknownFieldPosture,
} from "./resource_mutation_response_response_mapping.js";
import {
  createMutationResponseTargetExecution,
} from "./resource_mutation_response_target_execution.js";
import { createMutationResponseDiagnosticFacts } from "./resource_mutation_response_diagnostic_facts.js";
import {
  createMutationResponseConfirmationClassification,
} from "./resource_mutation_response_confirmation_classification.js";
import {
  createMutationResponseLifecycleProof,
} from "./resource_mutation_response_lifecycle_proof.js";
import {
  createMutationResponseIdentityMigrationPlan,
} from "./identity/resource_mutation_response_identity_migration.js";
import {
  createExecutedMutationResponseExecutionDigest,
  createMutationResponseExecutionDigest,
  createMutationResponseFallbackDigest,
  createMutationResponseTargetDigest,
  createPlannedMutationResponseTargetDigest,
  readAppliedTargetBreadthCounter,
  readFallbackTargetBreadth,
  readStaleTargetDenialBreadth,
} from "./plan/resource_mutation_response_plan_digests.js";
import {
  applyMutationResponsePartialAdmission,
  createPublicMutationResponseTarget,
  readMutationResponsePartialAdmission,
} from "./plan/resource_mutation_response_target_planning.js";
import {
  readPayloadFieldExtractionBreadth,
  readReconstructionBreadth,
  readTopologyTraversalBreadth,
} from "./plan/resource_mutation_response_cost_counters.js";
import { readLineBindingState } from "../lines/state/line_binding_state.js";

const RESOURCE_MUTATION_RESPONSE_DECLARATION = Symbol(
  "WORTHSignal.resourceMutationResponseDeclaration",
);
const RESOURCE_MUTATION_RESPONSE_PLAN = Symbol(
  "WORTHSignal.resourceMutationResponsePlan",
);
const RESOURCE_MUTATION_RESPONSE_PREPARED_EXECUTIONS = Symbol(
  "WORTHSignal.resourceMutationResponsePreparedExecutions",
);

const RESOURCE_MUTATION_RESPONSE_DECLARATION_VERSION =
  "resource-mutation-response-declaration-v1";
const RESOURCE_MUTATION_RESPONSE_PLAN_VERSION =
  "resource-mutation-response-plan-v1";

function createMutationResponseDeclaration(options) {
  const lensProof = requireMutationResponseLensProof(
    options.lensProof,
    "mutation response declaration",
  );
  const targets = Object.freeze([...(options.targets ?? [])]);
  const diagnostics = Object.freeze([...(options.diagnostics ?? [])]);
  return Object.freeze({
    version: RESOURCE_MUTATION_RESPONSE_DECLARATION_VERSION,
    source: options.source,
    route: lensProof.route,
    method: lensProof.method,
    lensProof,
    targets,
    diagnostics,
    identityMigration: options.identityMigration ?? null,
    responseMappedFieldNames: options.responseMappedFieldNames ?? null,
    reconciliationAtomicity: options.reconciliationAtomicity ?? "allOrNone",
    targetCount: targets.length,
    diagnosticCount: diagnostics.length,
    atomicity: readMutationResponseAtomicity(targets.length),
    targetDigest: createMutationResponseTargetDigest(targets),
    [RESOURCE_MUTATION_RESPONSE_DECLARATION]: "resourceMutationResponseDeclaration",
  });
}

function readMutationResponseDeclaration(value) {
  if (
    !value
    || typeof value !== "object"
    || value[RESOURCE_MUTATION_RESPONSE_DECLARATION] !== "resourceMutationResponseDeclaration"
  ) {
    return null;
  }
  return value;
}

function createMutationResponsePlan(materialization, declaration, responseValue) {
  return createPreparedMutationResponsePlan({
    lineIdentity: materialization.lineIdentity,
    requestDescriptor: materialization.requestState.readDescriptor(),
    diagnostics: readLineBindingState(materialization.binding).diagnostics,
    declaration,
    responseValue,
    submittedTargets: null,
    submittedIdentityMigration: null,
  });
}

function createPreparedMutationResponsePlan(options) {
  const readDeclaration = readMutationResponseDeclaration(options.declaration);
  if (readDeclaration === null) {
    return null;
  }
  const requestDescriptor = options.requestDescriptor;
  const diagnostics = options.diagnostics;
  const planId = [
    options.lineIdentity.family.familyId,
    options.lineIdentity.canonicalParams.canonicalKey,
    requestDescriptor.method,
    String(diagnostics.refreshCount),
    String(diagnostics.revalidateCount),
    String(diagnostics.deliveryCount),
    String(diagnostics.patchCount),
    String(diagnostics.visibleValueVersion),
  ].join(":");
  const responsePayloadDigest = createMutationResponsePayloadDigest(
    options.responseValue,
  );
  const diagnosticFacts = createMutationResponseDiagnosticFacts(
    readDeclaration.diagnostics,
    options.responseValue,
  );
  const identityMigration = createMutationResponseIdentityMigrationPlan(
    readDeclaration.identityMigration,
    options.requestDescriptor.canonicalParams.params,
    options.responseValue,
    options.submittedIdentityMigration ?? null,
  );
  const plannedTargets = Object.freeze(
    readDeclaration.targets.map((target, index) =>
      createPlannedMutationResponseTarget(
        target,
        options,
        options.responseValue,
        planId,
        options.submittedTargets?.[index] ?? null,
      )),
  );
  const partialAdmission = readMutationResponsePartialAdmission(
    plannedTargets,
    readDeclaration.reconciliationAtomicity,
  );
  const admittedTargets = applyMutationResponsePartialAdmission(
    plannedTargets,
    readDeclaration.reconciliationAtomicity,
    partialAdmission,
  );
  const publicTargets = Object.freeze(
    admittedTargets.map((target) =>
      createPublicMutationResponseTarget(
        target,
        createPublicMutationResponseReconciliationDigest,
      )),
  );
  const preparedExecutions = Object.freeze(
    admittedTargets.map((target) => target.execution.preparedExecution),
  );
  const executionArtifacts = Object.freeze(
    admittedTargets.map((target) =>
      target.execution.artifact),
  );
  const confirmation = createMutationResponseConfirmationClassification(
    executionArtifacts,
    diagnosticFacts,
    identityMigration?.fallbackKinds ?? [],
    identityMigration?.exactTargetCount ?? 0,
  );
  const lifecycleProof = createMutationResponseLifecycleProof(
    executionArtifacts,
    identityMigration,
  );
  const plan = {
    version: RESOURCE_MUTATION_RESPONSE_PLAN_VERSION,
    source: readDeclaration.source,
    planId,
    route: readDeclaration.route,
    method: readDeclaration.method,
    line: Object.freeze({
      familyId: options.lineIdentity.family.familyId,
      runtimeLineId: options.lineIdentity.runtimeLineId,
      canonicalKey: options.lineIdentity.canonicalParams.canonicalKey,
    }),
    request: Object.freeze({
      correlationId: requestDescriptor.context.correlationId,
      branchId: requestDescriptor.context.branchId,
      basisId: requestDescriptor.context.basisId,
      requestPath: requestDescriptor.target.requestPath,
      url: requestDescriptor.target.url,
    }),
    submittedTargets: Object.freeze([...(options.submittedTargets ?? [])]),
    response: Object.freeze({
      topology: readDeclaration.lensProof.topology,
      readResponseLensSource: readDeclaration.lensProof.readResponseLensSource,
      readResponseLensDigest: readDeclaration.lensProof.readResponseLensDigest,
      mutationResponseLensDigest: readDeclaration.lensProof.compiledDigest,
      declaredFieldDigest: createMutationResponseDeclaredFieldDigest(
        readDeclaration.responseMappedFieldNames,
      ),
      unknownFieldPosture: createMutationResponseUnknownFieldPosture(
        readDeclaration.responseMappedFieldNames,
        options.responseValue,
      ),
      payloadDigest: responsePayloadDigest,
    }),
    confirmation,
    lifecycleProof,
    diagnostics: diagnosticFacts,
    identityMigration,
    targets: publicTargets,
    targetCount: readDeclaration.targetCount,
    atomicity: readDeclaration.atomicity,
    reconciliationAtomicity: readDeclaration.reconciliationAtomicity,
    partialAdmission,
    targetDigest: createPlannedMutationResponseTargetDigest(
      publicTargets,
    ),
    fallbackDigest: createMutationResponseFallbackDigest(publicTargets),
    executionArtifacts,
    executionDigest: createMutationResponseExecutionDigest(executionArtifacts),
    counters: Object.freeze({
      planningBreadth: 1,
      responseExtractionBreadth: 1,
      payloadFieldExtractionBreadth: readPayloadFieldExtractionBreadth(
        readDeclaration.responseMappedFieldNames,
      ),
      targetLookupBreadth: plannedTargets.length,
      targetFanoutBreadth: plannedTargets.length,
      topologyTraversalBreadth: readTopologyTraversalBreadth(admittedTargets),
      reconstructionBreadth: readReconstructionBreadth(admittedTargets),
      fallbackBreadth: readFallbackTargetBreadth(executionArtifacts),
      executionBreadth: executionArtifacts.length,
      diagnosticExtractionBreadth: diagnosticFacts.count,
      targetBasisSnapshotBreadth: (options.submittedTargets ?? []).length,
      staleTargetDenialBreadth:
        readStaleTargetDenialBreadth(executionArtifacts),
      partialPolicyBreadth: partialAdmission === "notNeeded" ? 0 : 1,
      identityResponseExtractionBreadth:
        identityMigration?.counters.responseIdentityExtractionBreadth ?? 0,
      identityMigrationTargetFanoutBreadth:
        identityMigration?.counters.targetFanoutBreadth ?? 0,
      identityMigrationStaleDenialBreadth:
        identityMigration?.counters.staleTargetDenialBreadth ?? 0,
      identityMigrationExecutionBreadth:
        identityMigration?.exactTargetCount ?? 0,
      identityMigrationLifecycleProofBreadth:
        identityMigration?.counters.lifecycleProofBreadth ?? 0,
      confirmationClassificationBreadth:
        executionArtifacts.length
        + diagnosticFacts.count
        + (identityMigration?.fallbackKinds.length ?? 0)
        + (identityMigration?.exactTargetCount ?? 0),
      lifecycleProofBreadth: lifecycleProof.count,
    }),
    [RESOURCE_MUTATION_RESPONSE_PLAN]: "resourceMutationResponsePlan",
  };
  Object.defineProperty(plan, RESOURCE_MUTATION_RESPONSE_PREPARED_EXECUTIONS, {
    value: preparedExecutions,
    enumerable: false,
  });
  return Object.freeze(plan);
}

function createExecutedMutationResponsePlan(plan, executedArtifacts) {
  const identityMigration = plan.identityMigration;
  const confirmation = createMutationResponseConfirmationClassification(
    executedArtifacts,
    plan.diagnostics,
    identityMigration?.fallbackKinds ?? [],
    identityMigration?.exactTargetCount ?? 0,
  );
  const lifecycleProof = createMutationResponseLifecycleProof(
    executedArtifacts,
    identityMigration,
  );
  const nextTargets = Object.freeze(
    plan.targets.map((target, index) =>
      Object.freeze({
        ...target,
        execution: executedArtifacts[index],
      })),
  );
  return Object.freeze({
    ...plan,
    confirmation,
    lifecycleProof,
    identityMigration,
    targets: nextTargets,
    executionArtifacts: Object.freeze(executedArtifacts),
    executionDigest: createExecutedMutationResponseExecutionDigest(executedArtifacts),
    counters: Object.freeze({
      ...plan.counters,
      ...readAppliedTargetBreadthCounter(executedArtifacts),
      fallbackBreadth: readFallbackTargetBreadth(executedArtifacts),
      staleTargetDenialBreadth:
        readStaleTargetDenialBreadth(executedArtifacts),
      identityMigrationExecutionBreadth:
        identityMigration?.exactTargetCount ?? 0,
      identityMigrationLifecycleProofBreadth:
        identityMigration?.counters.lifecycleProofBreadth ?? 0,
      confirmationClassificationBreadth:
        executedArtifacts.length
        + plan.diagnostics.count
        + (identityMigration?.fallbackKinds.length ?? 0)
        + (identityMigration?.exactTargetCount ?? 0),
    }),
  });
}

function createPlannedMutationResponseTarget(
  target,
  options,
  responseValue,
  planId,
  submittedTarget,
) {
  const mutationParams = options.requestDescriptor.canonicalParams.params;
  const targetParams = target.params(options.requestDescriptor.canonicalParams.params);
  const lineIdentity = target.readTargetLineIdentity(targetParams);
  const execution = createMutationResponseTargetExecution(
    target,
    targetParams,
    mutationParams,
    lineIdentity,
    responseValue,
    planId,
    submittedTarget,
  );
  return Object.freeze({
    target,
    lineIdentity,
    submittedTarget,
    execution,
  });
}

function createPublicMutationResponseReconciliationDigest(reconciliation) {
  return Object.freeze({
    kind: reconciliation.kind,
    itemId: null,
    placement: "placement" in reconciliation ? reconciliation.placement : null,
    field: "field" in reconciliation ? reconciliation.field : null,
    region: "region" in reconciliation ? reconciliation.region : null,
    path: "path" in reconciliation ? reconciliation.path : null,
    summary: "summary" in reconciliation ? reconciliation.summary : null,
    targetDigest: reconciliation.targetDigest,
  });
}
function readMutationResponseAtomicity(targetCount) {
  if (targetCount === 0) {
    return "zeroTargets";
  }
  if (targetCount === 1) {
    return "singleTarget";
  }
  return "allOrNone";
}

export {
  createExecutedMutationResponsePlan,
  createMutationResponseDeclaration,
  createMutationResponsePlan,
  createPreparedMutationResponsePlan,
  readMutationResponseDeclaration,
  RESOURCE_MUTATION_RESPONSE_PREPARED_EXECUTIONS,
};
