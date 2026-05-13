import { requireMutationResponseLensProof } from "./resource_mutation_response_lens_proof.js";
import { createMutationResponsePayloadDigest } from "./resource_mutation_response_payload_digest.js";
import {
  createMutationResponseFallbackDetail,
  createMutationResponseTargetExecution,
  isExactMutationResponseExecutionKind,
} from "./resource_mutation_response_target_execution.js";
import { createMutationResponseDiagnosticFacts } from "./resource_mutation_response_diagnostic_facts.js";
import {
  createMutationResponseConfirmationClassification,
} from "./resource_mutation_response_confirmation_classification.js";
import {
  createMutationResponseLifecycleProof,
} from "./resource_mutation_response_lifecycle_proof.js";

const RESOURCE_MUTATION_RESPONSE_DECLARATION = Symbol(
  "forgeSignal.resourceMutationResponseDeclaration",
);
const RESOURCE_MUTATION_RESPONSE_PLAN = Symbol(
  "forgeSignal.resourceMutationResponsePlan",
);
const RESOURCE_MUTATION_RESPONSE_PREPARED_EXECUTIONS = Symbol(
  "forgeSignal.resourceMutationResponsePreparedExecutions",
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
    diagnostics: materialization.binding.diagnosticsSignal(),
    declaration,
    responseValue,
    submittedTargets: null,
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
  const exactExecutionCount = plannedTargets.filter((target) =>
    isExactMutationResponseExecutionKind(target.execution.kind)).length;
  if (exactExecutionCount > 1) {
    throw new TypeError(
      "mutation response planning currently admits at most one exact reconciliation target before multi-target atomicity support lands",
    );
  }
  const preparedExecutions = Object.freeze(
    plannedTargets.map((target) => target.execution.preparedExecution),
  );
  const executionArtifacts = Object.freeze(
    plannedTargets.map((target) =>
      target.execution.artifact),
  );
  const confirmation = createMutationResponseConfirmationClassification(
    executionArtifacts,
    diagnosticFacts,
  );
  const lifecycleProof = createMutationResponseLifecycleProof(executionArtifacts);
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
      payloadDigest: responsePayloadDigest,
    }),
    confirmation,
    lifecycleProof,
    diagnostics: diagnosticFacts,
    targets: plannedTargets.map((target) => target.publicTarget),
    targetCount: readDeclaration.targetCount,
    atomicity: readDeclaration.atomicity,
    targetDigest: createPlannedMutationResponseTargetDigest(
      plannedTargets.map((target) => target.publicTarget),
    ),
    fallbackDigest: createMutationResponseFallbackDigest(
      plannedTargets.map((target) => target.publicTarget),
    ),
    executionArtifacts,
    executionDigest: createMutationResponseExecutionDigest(executionArtifacts),
    counters: Object.freeze({
      planningBreadth: 1,
      responseExtractionBreadth: 1,
      targetLookupBreadth: plannedTargets.length,
      targetFanoutBreadth: plannedTargets.length,
      fallbackBreadth: readFallbackTargetBreadth(executionArtifacts),
      executionBreadth: executionArtifacts.length,
      diagnosticExtractionBreadth: diagnosticFacts.count,
      targetBasisSnapshotBreadth: (options.submittedTargets ?? []).length,
      staleTargetDenialBreadth:
        readStaleTargetDenialBreadth(executionArtifacts),
      confirmationClassificationBreadth:
        executionArtifacts.length + diagnosticFacts.count,
      lifecycleProofBreadth: executionArtifacts.length,
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
  const confirmation = createMutationResponseConfirmationClassification(
    executedArtifacts,
    plan.diagnostics,
  );
  const lifecycleProof = createMutationResponseLifecycleProof(executedArtifacts);
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
    targets: nextTargets,
    executionArtifacts: Object.freeze(executedArtifacts),
    executionDigest: createExecutedMutationResponseExecutionDigest(executedArtifacts),
    counters: Object.freeze({
      ...plan.counters,
      ...readAppliedTargetBreadthCounter(executedArtifacts),
      fallbackBreadth: readFallbackTargetBreadth(executedArtifacts),
      staleTargetDenialBreadth:
        readStaleTargetDenialBreadth(executedArtifacts),
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
  const targetParams = target.params(options.requestDescriptor.canonicalParams.params);
  const lineIdentity = target.readTargetLineIdentity(targetParams);
  const execution = createMutationResponseTargetExecution(
    target,
    targetParams,
    lineIdentity,
    responseValue,
    planId,
    submittedTarget,
  );
  const publicTarget = Object.freeze({
    targetId: target.targetId,
    family: target.family,
    line: Object.freeze({
      familyKind: target.family.kind,
      familyId: target.family.familyId,
      canonicalKey: lineIdentity.canonicalKey,
      runtimeLineId: lineIdentity.runtimeLineId,
      residency: lineIdentity.residency,
    }),
    fallback: Object.freeze({
      kind: target.fallback,
      detail: createMutationResponseFallbackDetail(target, lineIdentity),
    }),
    submittedTarget,
    reconciliation:
      target.reconciliation === null
        ? null
        : createPublicMutationResponseReconciliationDigest(target.reconciliation),
    execution: execution.artifact,
    targetDigest: createPlannedMutationResponseTargetIdentityDigest(
      target,
      lineIdentity,
      execution.artifact,
    ),
  });
  return Object.freeze({
    publicTarget,
    execution,
  });
}

function createPublicMutationResponseReconciliationDigest(reconciliation) {
  return Object.freeze({
    kind: reconciliation.kind,
    itemId: null,
    field: "field" in reconciliation ? reconciliation.field : null,
    region: "region" in reconciliation ? reconciliation.region : null,
    path: "path" in reconciliation ? reconciliation.path : null,
    summary: "summary" in reconciliation ? reconciliation.summary : null,
    targetDigest: reconciliation.targetDigest,
  });
}

function createPlannedMutationResponseTargetIdentityDigest(
  target,
  lineIdentity,
  executionArtifact,
) {
  return [
    target.targetId,
    target.family.kind,
    target.family.familyId,
    lineIdentity.canonicalKey,
    executionArtifact.kind === "fallback"
      ? target.fallback
      : isExactMutationResponseExecutionKind(executionArtifact.kind)
        ? `exact:${executionArtifact.scope}:${
          executionArtifact.itemId
          ?? executionArtifact.summary
          ??
          executionArtifact.field
          ?? executionArtifact.region
          ?? executionArtifact.path
          ?? "line"
        }`
        : target.fallback,
  ].join("|");
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

function createMutationResponseTargetDigest(targets) {
  if (targets.length === 0) {
    return "mutation-response-targets|none";
  }
  return `mutation-response-targets|${targets.map((target) =>
    `${target.targetId}:${target.family.kind}:${target.family.familyId}:${target.fallback}`).join(",")}`;
}

function createMutationResponseFallbackDigest(targets) {
  if (targets.length === 0) {
    return "mutation-response-fallbacks|none";
  }
  return `mutation-response-fallbacks|${targets.map((target) =>
    target.execution.kind === "fallback"
      ? `${target.targetId}:${target.fallback.kind}:${target.line.canonicalKey}`
      : `${target.targetId}:none:${target.line.canonicalKey}`).join(",")}`;
}

function createPlannedMutationResponseTargetDigest(targets) {
  if (targets.length === 0) {
    return "mutation-response-targets|none";
  }
  return `mutation-response-targets|${targets.map((target) =>
    `${target.targetId}:${target.family.kind}:${target.family.familyId}:${target.line.canonicalKey}:${target.fallback.kind}`).join(",")}`;
}

function createMutationResponseExecutionDigest(executionArtifacts) {
  if (executionArtifacts.length === 0) {
    return "mutation-response-execution|none";
  }
  return `mutation-response-execution|${executionArtifacts.map((artifact) =>
    artifact.kind === "fallback"
      ? `${artifact.targetId}:fallback:${artifact.fallback}:${artifact.canonicalKey}`
      : `${artifact.targetId}:${artifact.kind}:${artifact.scope}:${
        artifact.itemId
        ?? artifact.summary
        ??
        artifact.field ?? artifact.region ?? artifact.path ?? "line"
      }:${artifact.canonicalKey}`).join(",")}`;
}

function createExecutedMutationResponseExecutionDigest(executedArtifacts) {
  if (executedArtifacts.length === 0) {
    return "mutation-response-execution|none";
  }
  return `mutation-response-execution|${executedArtifacts.map((artifact) =>
    artifact.kind === "fallback"
      ? `${artifact.targetId}:fallback:${artifact.fallback}:${artifact.canonicalKey}`
      : `${artifact.targetId}:${artifact.kind}:${artifact.scope}:${
        artifact.itemId
        ?? artifact.summary
        ??
        artifact.field ?? artifact.region ?? artifact.path ?? "line"
      }:${artifact.effectId ?? "none"}:${artifact.canonicalKey}`).join(",")}`;
}

function readAppliedTargetBreadthCounter(executedArtifacts) {
  const appliedTargetBreadth = executedArtifacts.filter(
    (artifact) => isExactMutationResponseExecutionKind(artifact.kind),
  ).length;
  if (appliedTargetBreadth === 0) {
    return Object.freeze({});
  }
  return Object.freeze({
    appliedTargetBreadth,
  });
}

const readFallbackTargetBreadth = (executionArtifacts) =>
  executionArtifacts.filter((artifact) => artifact.kind === "fallback").length;
const readStaleTargetDenialBreadth = (executionArtifacts) =>
  executionArtifacts.filter((artifact) => artifact.staleness !== null).length;

export {
  createExecutedMutationResponsePlan,
  createMutationResponseDeclaration,
  createMutationResponsePlan,
  createPreparedMutationResponsePlan,
  readMutationResponseDeclaration,
  RESOURCE_MUTATION_RESPONSE_PREPARED_EXECUTIONS,
};
