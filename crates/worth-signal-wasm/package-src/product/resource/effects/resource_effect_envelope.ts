import { requireResourceEffectPlan } from "./resource_effect_plan.js";
import { createResourceEffectBranchLifecycle } from "./resource_effect_branch_lifecycle.js";
import { createResourceEffectOptimisticLifecycle } from "./resource_effect_optimistic_lifecycle.js";
import { createResourceEffectPatchCounters } from "./resource_effect_patch_counters.js";
import { createResourceEffectPatchDigest } from "./resource_effect_patch_digest.js";
import { lowerResponseLensProofToEffectLocusWithOptions } from "../response/resource_response_effect_locus_lowering.js";
import { createEnvelopeBranchPosture } from "./resource_effect_branch_posture_digest.js";
import { createResourceEffectProfileDigest } from "./resource_effect_profile_digest.js";

const RESOURCE_EFFECT_ENVELOPE_VERSION = "resource-effect-envelope-v1";
const RESOURCE_EFFECT_AUTHORITY_VERSION = "resource-effect-authority-v1";
const resourceEffectAuthorityGlobal = globalThis as typeof globalThis & { __WorthResourceEffectAuthorityRegistry?: Map<string, string>; };
const RESOURCE_EFFECT_AUTHORITY_REGISTRY =
  resourceEffectAuthorityGlobal.__WorthResourceEffectAuthorityRegistry ?? new Map();
resourceEffectAuthorityGlobal.__WorthResourceEffectAuthorityRegistry = RESOURCE_EFFECT_AUTHORITY_REGISTRY;
let nextResourceEffectAuthoritySequence = 1;

function resetResourceEffectEnvelopeAuthorityForTesting() {
  RESOURCE_EFFECT_AUTHORITY_REGISTRY.clear();
  nextResourceEffectAuthoritySequence = 1;
}

function createLocalPatchEffectEnvelope(effectPlan, patch, result, branchAcquisition = null) {
  return createResourceEffectEnvelope({
    effectPlan,
    branchAcquisition,
    delivery: null,
    patch: Object.freeze({
      kind: patch.kind,
      scope: result.scope,
      itemId: result.itemId,
      field: result.field,
      region: result.region,
      path: result.path,
      aspect: result.aspect,
      summary: result.summary,
      valueChanged: result.valueChanged,
      fieldProof: result.fieldProof,
      regionProof: result.regionProof,
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
      field: delivery.patchedField,
      region: delivery.patchedRegion,
      path: delivery.patchedPath,
      aspect: delivery.patchedAspect,
      summary: delivery.patchedSummary,
      valueChanged: delivery.valueChanged,
      fieldProof: delivery.fieldProof,
      regionProof: delivery.regionProof,
      jsonPathProof: delivery.jsonPathProof,
    }),
  });
}

function createResourceEffectEnvelope(options) {
  const effectPlan = requireResourceEffectPlan(options.effectPlan);
  const requestDescriptor = effectPlan.requestDescriptor;
  const lineIdentity = effectPlan.lineIdentity;
  const patch = options.patch;
  const branchPosture = createEnvelopeBranchPosture(
    effectPlan.branchPosture,
    options.branchAcquisition ?? null,
  );
  const rawLocus = createEffectLocus(patch, options.delivery, effectPlan);
  const patchDigest = createResourceEffectPatchDigest(patch);
  const locusProof = lowerResponseLensProofToEffectLocusWithOptions(
    effectPlan.responseLensProof,
    rawLocus,
    patch.kind,
  );
  const locus = alignLocusWithResponseLensProof(rawLocus, locusProof);
  const envelope = {
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
      branch: branchPosture,
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
    profile: createResourceEffectProfileDigest(requestDescriptor.effects),
    branchLifecycle: createResourceEffectBranchLifecycle(effectPlan, branchPosture),
    optimistic: createResourceEffectOptimisticLifecycle(
      effectPlan,
      locus,
      patch,
      branchPosture,
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
      ...createResourceEffectPatchCounters(patchDigest),
    }),
  };
  const authority = createResourceEffectAuthority(envelope);
  Object.defineProperty(envelope, "authority", { value: authority, enumerable: true });
  registerResourceEffectEnvelopeAuthority(envelope);
  return Object.freeze(envelope);
}

function createResourceEffectAuthority(envelope) {
  const token = [
    RESOURCE_EFFECT_AUTHORITY_VERSION,
    envelope.effectId,
    nextResourceEffectAuthoritySequence++,
  ].join(":");
  return Object.freeze({
    version: RESOURCE_EFFECT_AUTHORITY_VERSION,
    runtimeEffectToken: token,
    envelopeDigest: createResourceEffectAuthorityDigest(envelope),
  });
}

function registerResourceEffectEnvelopeAuthority(envelope) {
  RESOURCE_EFFECT_AUTHORITY_REGISTRY.set(
    envelope.authority.runtimeEffectToken,
    envelope.authority.envelopeDigest,
  );
}

function requireRuntimeIssuedResourceEffectEnvelope(effect) {
  if (!effect || typeof effect !== "object" || Array.isArray(effect)) {
    return false;
  }
  const authority = effect.authority;
  if (
    !authority ||
    authority.version !== RESOURCE_EFFECT_AUTHORITY_VERSION ||
    typeof authority.runtimeEffectToken !== "string" ||
    typeof authority.envelopeDigest !== "string"
  ) {
    return false;
  }
  const registeredDigest = RESOURCE_EFFECT_AUTHORITY_REGISTRY.get(
    authority.runtimeEffectToken,
  );
  if (registeredDigest === undefined) {
    return false;
  }
  return registeredDigest === authority.envelopeDigest &&
    registeredDigest === createResourceEffectAuthorityDigest(effect);
}

function createResourceEffectAuthorityDigest(effect) {
  return canonicalStringify({
    version: effect.version,
    effectId: effect.effectId,
    provenance: effect.provenance,
    idempotencyKey: effect.idempotencyKey,
    serverCorrelationId: effect.serverCorrelationId,
    plan: effect.plan,
    family: effect.family,
    line: effect.line,
    profile: effect.profile,
    branchLifecycle: effect.branchLifecycle,
    optimistic: effect.optimistic,
    request: effect.request,
    delivery: effect.delivery,
    locus: effect.locus,
    locusProof: effect.locusProof,
    patch: effect.patch,
    counters: effect.counters,
  });
}

function canonicalStringify(value) {
  return JSON.stringify(canonicalizeAuthorityValue(value));
}

function canonicalizeAuthorityValue(value) {
  if (Array.isArray(value)) {
    return value.map(canonicalizeAuthorityValue);
  }
  if (!value || typeof value !== "object") {
    return value;
  }
  const canonical = {};
  for (const key of Object.keys(value).sort()) {
    canonical[key] = canonicalizeAuthorityValue(value[key]);
  }
  return canonical;
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
    case "field":
      return Object.freeze({
        kind: "detailField",
        field: patch.field,
      });
    case "region":
      return Object.freeze({
        kind: "detailRegion",
        region: patch.region,
      });
    case "jsonPath":
      return Object.freeze({
        kind: "detailJsonPath",
        path: patch.path,
      });
    case "item":
      return Object.freeze({
        kind: createResponseLensItemLocus(responseLensProof.topology),
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

function createResponseLensItemLocus(topology) {
  if (topology === "entityStore") {
    return "entityStore";
  }
  if (topology === "connection") {
    return "connection";
  }
  if (topology === "discriminatedTuple") {
    return "discriminatedTuple";
  }
  if (topology === "groupedCollection") {
    return "groupedCollection";
  }
  if (topology === "mapCollection") {
    return "mapCollection";
  }
  if (topology === "namedCollection") {
    return "namedCollection";
  }
  if (topology === "recursiveTree") {
    return "recursiveTree";
  }
  if (topology === "sparsePage") {
    return "sparsePage";
  }
  return "membership";
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
    case "field":
      return Object.freeze({ kind: "detailField", field: patch.field });
    case "region":
      return Object.freeze({ kind: "detailRegion", region: patch.region });
    case "jsonPath":
      return Object.freeze({ kind: "detailJsonPath", path: patch.path });
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
    case "field":
    case "region":
    case "jsonPath":
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
  requireRuntimeIssuedResourceEffectEnvelope,
  resetResourceEffectEnvelopeAuthorityForTesting,
};
