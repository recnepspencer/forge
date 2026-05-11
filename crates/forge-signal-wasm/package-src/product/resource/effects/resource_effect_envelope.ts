import { requireResourceEffectPlan } from "./resource_effect_plan.js";
import { createResourceEffectBranchLifecycle } from "./resource_effect_branch_lifecycle.js";
import { createResourceEffectOptimisticLifecycle } from "./resource_effect_optimistic_lifecycle.js";
import { lowerResponseLensProofToEffectLocus } from "../response/resource_response_effect_locus_lowering.js";

const RESOURCE_EFFECT_ENVELOPE_VERSION = "resource-effect-envelope-v1";

function createLocalPatchEffectEnvelope(effectPlan, patch, result) {
  return createResourceEffectEnvelope({
    effectPlan,
    delivery: null,
    patch: Object.freeze({
      kind: patch.kind,
      scope: result.scope,
      itemId: result.itemId,
      aspect: result.aspect,
      summary: result.summary,
      valueChanged: result.valueChanged,
      jsonPathProof: result.jsonPathProof,
    }),
  });
}

function createDeliveryEffectEnvelope(effectPlan, delivery) {
  return createResourceEffectEnvelope({
    effectPlan,
    delivery: Object.freeze({
      kind: delivery.deliveryKind,
      scope: delivery.deliveryScope,
      packetId: delivery.packetId,
      basisId: delivery.basisId,
      nextBasisId: delivery.nextBasisId,
    }),
    patch: Object.freeze({
      kind: delivery.patchKind,
      scope: delivery.patchScope,
      itemId: delivery.patchedItemId,
      aspect: delivery.patchedAspect,
      summary: delivery.patchedSummary,
      valueChanged: delivery.valueChanged,
      jsonPathProof: delivery.jsonPathProof,
    }),
  });
}

function createResourceEffectEnvelope(options) {
  const effectPlan = requireResourceEffectPlan(options.effectPlan);
  const requestDescriptor = effectPlan.requestDescriptor;
  const lineIdentity = effectPlan.lineIdentity;
  const patch = options.patch;
  const rawLocus = createEffectLocus(patch, options.delivery, effectPlan);
  const patchDigest = createPatchDigest(patch);
  const locusProof = lowerResponseLensProofToEffectLocus(
    effectPlan.responseLensProof,
    rawLocus,
  );
  const locus = alignLocusWithResponseLensProof(rawLocus, locusProof);
  return Object.freeze({
    version: RESOURCE_EFFECT_ENVELOPE_VERSION,
    effectId: createEffectId(effectPlan, options.delivery),
    provenance: effectPlan.provenance,
    idempotencyKey: effectPlan.idempotencyKey,
    serverCorrelationId: effectPlan.serverCorrelationId,
    plan: Object.freeze({
      planId: effectPlan.planId,
      admissionKind: effectPlan.admissionKind,
      causalSequence: effectPlan.causalSequence,
      retryLineageId: effectPlan.retryLineageId,
      branch: effectPlan.branchPosture,
    }),
    family: Object.freeze({
      kind: lineIdentity.family.kind,
      familyId: lineIdentity.family.familyId,
    }),
    line: Object.freeze({
      runtimeLineId: lineIdentity.runtimeLineId,
      scopeId: lineIdentity.scopeId,
      canonicalKey: lineIdentity.canonicalParams.canonicalKey,
    }),
    profile: createProfileDigest(requestDescriptor.effects),
    branchLifecycle: createResourceEffectBranchLifecycle(effectPlan),
    optimistic: createResourceEffectOptimisticLifecycle(
      effectPlan,
      locus,
      patch,
    ),
    request: Object.freeze({
      correlationId: requestDescriptor.context.correlationId,
      branchId: requestDescriptor.context.branchId,
      basisId: requestDescriptor.context.basisId,
    }),
    delivery: options.delivery,
    locus,
    locusProof,
    patch: patchDigest,
    counters: Object.freeze({
      ...effectPlan.counters,
      ...createJsonPathCounters(patchDigest.jsonPath),
    }),
  });
}

function createPatchDigest(patch) {
  return Object.freeze({
    kind: patch.kind,
    scope: patch.scope,
    itemId: patch.itemId,
    aspect: patch.aspect,
    summary: patch.summary,
    valueChanged: patch.valueChanged,
    jsonPath: createJsonPathPatchProof(patch.jsonPathProof),
  });
}

function createJsonPathPatchProof(jsonPathProof) {
  if (jsonPathProof === null || jsonPathProof === undefined) {
    return null;
  }
  return Object.freeze({
    version: jsonPathProof.version,
    aspect: jsonPathProof.aspect,
    field: jsonPathProof.field,
    path: jsonPathProof.path,
    parsedPathDigest: jsonPathProof.parsedPathDigest,
    policy: jsonPathProof.policy,
    cost: jsonPathProof.cost,
    proofDigest: jsonPathProof.proofDigest,
  });
}

function createJsonPathCounters(jsonPathProof) {
  if (jsonPathProof === null) {
    return Object.freeze({
      jsonPathTraversalBreadth: 0,
      jsonPathReconstructionBreadth: 0,
    });
  }
  return Object.freeze({
    jsonPathTraversalBreadth: jsonPathProof.cost.traversalBreadth,
    jsonPathReconstructionBreadth: jsonPathProof.cost.reconstructionBreadth,
  });
}

function alignLocusWithResponseLensProof(locus, locusProof) {
  if (locusProof?.locus !== "jsonItemAspect") {
    return locus;
  }
  return Object.freeze({
    kind: "jsonItemAspect",
    itemId: locus.itemId,
    aspect: locus.aspect,
  });
}

function createProfileDigest(profile) {
  if (profile === null) {
    return null;
  }
  return Object.freeze({
    name: profile.name,
    optimism: profile.optimism,
    confirmation: profile.confirmation,
    rollback: profile.rollback,
    rebase: profile.rebase,
    preimage: profile.preimage,
  });
}

function createEffectLocus(patch, delivery, effectPlan) {
  if (delivery !== null) {
    const deliveryLocus = createDeliveryOnlyEffectLocus(delivery.scope);
    if (deliveryLocus !== null) {
      return deliveryLocus;
    }
  }
  if (effectPlan.responseLensProof !== null) {
    return createResponseLensBackedEffectLocus(
      patch,
      effectPlan.responseLensProof,
    );
  }
  return createGenericResourceEffectLocus(patch);
}

function createResponseLensBackedEffectLocus(patch, responseLensProof) {
  switch (patch.scope) {
    case "line":
      return Object.freeze({
        kind: createResponseLensLineLocus(responseLensProof.topology),
      });
    case "item":
      return Object.freeze({
        kind: responseLensProof.topology === "entityStore"
          ? "entityStore"
          : "membership",
        itemId: patch.itemId,
      });
    case "aspect":
      return Object.freeze({
        kind: responseLensProof.jsonAspectNames.includes(patch.aspect)
          ? "jsonItemAspect"
          : "itemAspect",
        itemId: patch.itemId,
        aspect: patch.aspect,
      });
    case "summary":
      return Object.freeze({ kind: "summary", summary: patch.summary });
    default:
      throw new TypeError(
        `resource effect envelope cannot classify patch scope "${patch.scope}"`,
      );
  }
}

function createResponseLensLineLocus(topology) {
  if (topology === "detail") {
    return "detailResponse";
  }
  if (topology === "summary") {
    return "summaryResponse";
  }
  return "broadResponse";
}

function createGenericResourceEffectLocus(patch) {
  switch (patch.scope) {
    case "line":
      return Object.freeze({ kind: "line" });
    case "item":
      return Object.freeze({ kind: "item", itemId: patch.itemId });
    case "aspect":
      return Object.freeze({
        kind: "itemAspect",
        itemId: patch.itemId,
        aspect: patch.aspect,
      });
    case "summary":
      return Object.freeze({ kind: "summary", summary: patch.summary });
    default:
      throw new TypeError(
        `resource effect envelope cannot classify patch scope "${patch.scope}"`,
      );
  }
}

function createDeliveryOnlyEffectLocus(deliveryScope) {
  switch (deliveryScope) {
    case "basis":
      return Object.freeze({ kind: "basis" });
    case "invalidate":
      return Object.freeze({ kind: "invalidation" });
    case "line":
    case "item":
    case "aspect":
    case "summary":
      return null;
    default:
      throw new TypeError(
        `resource effect envelope cannot classify delivery scope "${deliveryScope}"`,
      );
  }
}

function createEffectId(effectPlan, delivery) {
  if (delivery !== null) {
    return [
      effectPlan.lineIdentity.family.familyId,
      effectPlan.lineIdentity.canonicalParams.canonicalKey,
      effectPlan.provenance,
      delivery.packetId,
    ].join(":");
  }
  return effectPlan.planId;
}

export {
  createDeliveryEffectEnvelope,
  createLocalPatchEffectEnvelope,
};
