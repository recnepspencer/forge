import { requireMutationResponseLensProof } from "./resource_mutation_response_lens_proof.js";
import { resourceDelivery } from "../delivery/resource_delivery.js";

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
  return Object.freeze({
    version: RESOURCE_MUTATION_RESPONSE_DECLARATION_VERSION,
    source: options.source,
    route: lensProof.route,
    method: lensProof.method,
    lensProof,
    targets,
    targetCount: targets.length,
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
  const plannedTargets = Object.freeze(
    readDeclaration.targets.map((target) =>
      createPlannedMutationResponseTarget(
        target,
        options,
        options.responseValue,
        planId,
      )),
  );
  const exactExecutionCount = plannedTargets.filter(
    (target) => target.execution.kind === "exactDetail",
  ).length;
  if (exactExecutionCount > 1) {
    throw new TypeError(
      "mutation response planning currently admits at most one exact detail reconciliation target before multi-target atomicity support lands",
    );
  }
  const preparedExecutions = Object.freeze(
    plannedTargets.map((target) => target.execution.preparedExecution),
  );
  const executionArtifacts = Object.freeze(
    plannedTargets.map((target) =>
      target.execution.artifact),
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
    response: Object.freeze({
      topology: readDeclaration.lensProof.topology,
      readResponseLensSource: readDeclaration.lensProof.readResponseLensSource,
      readResponseLensDigest: readDeclaration.lensProof.readResponseLensDigest,
      mutationResponseLensDigest: readDeclaration.lensProof.compiledDigest,
      payloadDigest: responsePayloadDigest,
    }),
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
      fallbackBreadth: plannedTargets.length,
      executionBreadth: executionArtifacts.length,
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
  const nextTargets = Object.freeze(
    plan.targets.map((target, index) =>
      Object.freeze({
        ...target,
        execution: executedArtifacts[index],
      })),
  );
  return Object.freeze({
    ...plan,
    targets: nextTargets,
    executionArtifacts: Object.freeze(executedArtifacts),
    executionDigest: createExecutedMutationResponseExecutionDigest(executedArtifacts),
    counters: Object.freeze({
      ...plan.counters,
      ...readAppliedTargetBreadthCounter(executedArtifacts),
    }),
  });
}

function createPlannedMutationResponseTarget(target, options, responseValue, planId) {
  const targetParams = target.params(options.requestDescriptor.canonicalParams.params);
  const lineIdentity = target.readTargetLineIdentity(targetParams);
  const execution = createMutationResponseTargetExecution(
    target,
    targetParams,
    lineIdentity,
    responseValue,
    planId,
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

function createMutationResponseTargetExecution(
  target,
  targetParams,
  lineIdentity,
  responseValue,
  planId,
) {
  if (
    target.reconciliation === null
    || lineIdentity.residency !== "resident"
  ) {
    return createFallbackMutationResponseExecutionArtifact(target, lineIdentity);
  }
  const targetMaterialization = target.lookupResidentTargetMaterialization(targetParams);
  if (targetMaterialization === null) {
    return createFallbackMutationResponseExecutionArtifact(target, lineIdentity);
  }
  const packetId = `${planId}:${target.targetId}`;
  const currentBasisId = targetMaterialization.requestState.currentBasisId();
  const delivery =
    target.reconciliation.kind === "replace"
      ? resourceDelivery.replace({
          packetId,
          basisId: null,
          nextBasisId: currentBasisId,
          nextValue: responseValue,
        })
      : resourceDelivery.patch({
          packetId,
          basisId: null,
          nextBasisId: currentBasisId,
          patch: target.reconciliation.createPatch(responseValue),
        });
  return Object.freeze({
    preparedExecution: Object.freeze({
      kind: "exactDetail",
      targetId: target.targetId,
      targetMaterialization,
      delivery,
    }),
    artifact: createExactDetailExecutionArtifact(
      target,
      lineIdentity,
      target.reconciliation,
      packetId,
    ),
  });
}

function createFallbackMutationResponseExecutionArtifact(target, lineIdentity) {
  return Object.freeze({
    preparedExecution: Object.freeze({
      kind: "fallback",
      targetId: target.targetId,
      fallback: target.fallback,
    }),
    artifact: Object.freeze({
      artifactId: `${target.targetId}:fallback`,
      targetId: target.targetId,
      kind: "fallback",
      fallback: target.fallback,
      familyKind: target.family.kind,
      familyId: target.family.familyId,
      canonicalKey: lineIdentity.canonicalKey,
      runtimeLineId: lineIdentity.runtimeLineId,
      residency: lineIdentity.residency,
      detail: createMutationResponseFallbackDetail(target, lineIdentity),
    }),
  });
}

function createExactDetailExecutionArtifact(
  target,
  lineIdentity,
  reconciliation,
  packetId,
) {
  return Object.freeze({
    artifactId: `${target.targetId}:exactDetail`,
    targetId: target.targetId,
    kind: "exactDetail",
    scope: reconciliation.kind === "replace" ? "line" : reconciliation.kind,
    familyKind: target.family.kind,
    familyId: target.family.familyId,
    canonicalKey: lineIdentity.canonicalKey,
    runtimeLineId: lineIdentity.runtimeLineId,
    residency: lineIdentity.residency,
    packetId,
    field: "field" in reconciliation ? reconciliation.field : null,
    region: "region" in reconciliation ? reconciliation.region : null,
    path: "path" in reconciliation ? reconciliation.path : null,
  });
}

function createPublicMutationResponseReconciliationDigest(reconciliation) {
  return Object.freeze({
    kind: reconciliation.kind,
    field: "field" in reconciliation ? reconciliation.field : null,
    region: "region" in reconciliation ? reconciliation.region : null,
    path: "path" in reconciliation ? reconciliation.path : null,
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
      : executionArtifact.kind === "exactDetail"
        ? `exact:${executionArtifact.scope}:${
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
      : `${artifact.targetId}:exactDetail:${artifact.scope}:${
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
      : `${artifact.targetId}:exactDetail:${artifact.scope}:${
        artifact.field ?? artifact.region ?? artifact.path ?? "line"
      }:${artifact.effectId ?? "none"}:${artifact.canonicalKey}`).join(",")}`;
}

function readAppliedTargetBreadthCounter(executedArtifacts) {
  const appliedTargetBreadth = executedArtifacts.filter(
    (artifact) => artifact.kind === "exactDetail",
  ).length;
  if (appliedTargetBreadth === 0) {
    return Object.freeze({});
  }
  return Object.freeze({
    appliedTargetBreadth,
  });
}

function createMutationResponseFallbackDetail(target, lineIdentity) {
  return [
    `${target.family.kind} ${target.family.familyId}`,
    `line ${lineIdentity.canonicalKey}`,
    `stays in ${target.fallback} posture until a later mutation-response phase admits exact reconciliation`,
  ].join(" ");
}

function createMutationResponsePayloadDigest(value) {
  return ["mutation-response-payload", canonicalStringify(value)].join("|");
}

function canonicalStringify(value) {
  return JSON.stringify(canonicalize(value, new Set(), "$response"));
}

function canonicalize(value, seen, path) {
  if (typeof value === "bigint" || typeof value === "function" || typeof value === "symbol") {
    throw new TypeError(
      `mutation response payload digest cannot classify ${typeof value} at ${path}`,
    );
  }
  if (Array.isArray(value)) {
    if (seen.has(value)) {
      throw new TypeError(
        `mutation response payload digest cannot classify a cyclic array at ${path}`,
      );
    }
    seen.add(value);
    try {
      const canonicalArray = [];
      for (let index = 0; index < value.length; index += 1) {
        const descriptor = Object.getOwnPropertyDescriptor(value, String(index));
        if (descriptor === undefined) {
          throw new TypeError(
            `mutation response payload digest cannot classify a sparse array slot at ${path}[${index}]`,
          );
        }
        if ("get" in descriptor || "set" in descriptor) {
          throw new TypeError(
            `mutation response payload digest cannot inspect accessor-backed array slot at ${path}[${index}]`,
          );
        }
        canonicalArray.push(canonicalize(descriptor.value, seen, `${path}[${index}]`));
      }
      return canonicalArray;
    } finally {
      seen.delete(value);
    }
  }
  if (!value || typeof value !== "object") {
    return value;
  }
  const prototype = Object.getPrototypeOf(value);
  if (prototype !== Object.prototype && prototype !== null) {
    throw new TypeError(
      `mutation response payload digest requires plain objects or arrays at ${path}`,
    );
  }
  if (seen.has(value)) {
    throw new TypeError(
      `mutation response payload digest cannot classify a cyclic object at ${path}`,
    );
  }
  seen.add(value);
  const result = {};
  try {
    for (const key of Object.keys(value).sort()) {
      const descriptor = Object.getOwnPropertyDescriptor(value, key);
      if (descriptor === undefined) {
        continue;
      }
      if ("get" in descriptor || "set" in descriptor) {
        throw new TypeError(
          `mutation response payload digest cannot inspect accessor-backed property "${key}" at ${path}`,
        );
      }
      result[key] = canonicalize(descriptor.value, seen, `${path}.${key}`);
    }
    return result;
  } finally {
    seen.delete(value);
  }
}

export {
  createExecutedMutationResponsePlan,
  createMutationResponseDeclaration,
  createMutationResponsePlan,
  createPreparedMutationResponsePlan,
  readMutationResponseDeclaration,
  RESOURCE_MUTATION_RESPONSE_PREPARED_EXECUTIONS,
};
