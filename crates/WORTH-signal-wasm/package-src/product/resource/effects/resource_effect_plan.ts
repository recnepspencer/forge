import { resolveResourceEffectBranchPosture } from "./resource_effect_branch_posture.js";

const RESOURCE_EFFECT_PLAN_BRAND = Symbol("WORTHSignal.resourceEffectPlan");

function createLocalPatchEffectPlan(
  materialization,
  previousDiagnostics,
  inverseDescriptor,
) {
  return createResourceEffectPlan({
    materialization,
    requestDescriptor: materialization.requestState.readDescriptor(),
    previousDiagnostics,
    admissionKind: "localPatch",
    provenance: "localPatch",
    sequence: previousDiagnostics.patchCount + 1,
    idempotencyKey: null,
    serverCorrelationId: null,
    previousEffect: null,
    inverseDescriptor,
  });
}

function createDeliveryEffectPlan(
  materialization,
  requestDescriptor,
  previousDiagnostics,
  delivery,
) {
  const deliveryKind = delivery.deliveryKind ?? delivery.kind;
  return createResourceEffectPlan({
    materialization,
    requestDescriptor,
    previousDiagnostics,
    admissionKind: "delivery",
    provenance: deliveryProvenance(deliveryKind),
    sequence: previousDiagnostics.deliveryCount + 1,
    idempotencyKey: delivery.packetId,
    serverCorrelationId: delivery.packetId,
    previousEffect: previousDiagnostics.lastEffect,
  });
}

function requireResourceEffectPlan(value) {
  if (!value || value[RESOURCE_EFFECT_PLAN_BRAND] !== "resourceEffectPlan") {
    throw new TypeError(
      "resource effect envelopes require a lowered resource effect plan",
    );
  }
  return value;
}

function createResourceEffectPlan(options) {
  const lineIdentity = options.materialization.lineIdentity;
  const requestDescriptor = options.requestDescriptor;
  const branchPosture = resolveResourceEffectBranchPosture({
    materialization: options.materialization,
    requestDescriptor,
    admissionKind: options.admissionKind,
    inverseDescriptor: options.inverseDescriptor ?? null,
  });
  return Object.freeze({
    [RESOURCE_EFFECT_PLAN_BRAND]: "resourceEffectPlan",
    planId: createEffectPlanId(options.provenance, lineIdentity, options),
    admissionKind: options.admissionKind,
    provenance: options.provenance,
    causalSequence: createCausalSequence(options, lineIdentity),
    retryLineageId: createRetryLineageId(options, lineIdentity),
    idempotencyKey: options.idempotencyKey,
    serverCorrelationId: options.serverCorrelationId,
    previousEffect: options.previousEffect,
    lineIdentity,
    requestDescriptor,
    branchPosture,
    responseLensProof: options.materialization.patch.responseLensProof,
    inverseDescriptor: options.inverseDescriptor ?? null,
    counters: Object.freeze({
      patchCountBefore: options.previousDiagnostics.patchCount,
      deliveryCountBefore: options.previousDiagnostics.deliveryCount,
      basisAdvanceCountBefore: options.previousDiagnostics.basis.advanceCount,
      planningBreadth: 1,
      executionBreadth: 1,
      branchProofBreadth: branchPosture.proofBreadth,
      branchLifecycleBreadth: 1,
      optimisticLifecycleBreadth: 1,
      serverConfirmationBreadth: options.admissionKind === "delivery" ? 1 : 0,
      rollbackReadinessBreadth: 1,
      responseLensBreadth:
        options.materialization.patch.responseLensProof === null ? 0 : 1,
      effectLocusBreadth: 1,
    }),
  });
}

function createEffectPlanId(provenance, lineIdentity, options) {
  return [
    lineIdentity.family.familyId,
    lineIdentity.canonicalParams.canonicalKey,
    provenance,
    String(options.sequence),
  ].join(":");
}

function createCausalSequence(options, lineIdentity) {
  return [
    lineIdentity.runtimeLineId,
    options.provenance,
    String(options.sequence),
  ].join("#");
}

function createRetryLineageId(options, lineIdentity) {
  return options.idempotencyKey === null
    ? null
    : [
        lineIdentity.family.familyId,
        lineIdentity.canonicalParams.canonicalKey,
        options.idempotencyKey,
      ].join(":");
}

function deliveryProvenance(deliveryKind) {
  switch (deliveryKind) {
    case "patch":
      return "deliveredPatch";
    case "replace":
      return "deliveredReplace";
    case "invalidate":
      return "deliveryInvalidate";
    case "basisRefresh":
      return "deliveryBasisRefresh";
    default:
      throw new TypeError(
        `resource effect plan cannot classify delivery kind "${deliveryKind}"`,
      );
  }
}

export {
  createDeliveryEffectPlan,
  createLocalPatchEffectPlan,
  requireResourceEffectPlan,
};
