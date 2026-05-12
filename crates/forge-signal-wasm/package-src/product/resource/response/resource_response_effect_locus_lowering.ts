import { requireResponseLensProof } from "./resource_response_lens_proof.js";
import { RESOURCE_RESPONSE_TOPOLOGY_COSTS } from "./resource_response_topology_costs.js";

const RESOURCE_EFFECT_LOCUS_PROOF_VERSION = "resource-effect-locus-proof-v1";
const RESOURCE_RESPONSE_LENS_DENIAL_PROOF_VERSION =
  "resource-response-lens-denial-proof-v1";

function lowerResponseLensProofToEffectLocus(lensProof, locus) {
  if (lensProof === null) {
    return null;
  }
  const proof = requireResponseLensProof(lensProof, "resource effect locus lowering");
  if (patchScopeForEffectLocus(locus) === null) {
    return null;
  }
  const effectiveLocus = normalizeJsonItemAspectLocus(proof, locus);
  const capability = readCapabilityForLocus(proof, effectiveLocus);
  if (capability === null || capability.admitted !== true) {
    throw createResponseLensDenialError(
      proof,
      effectiveLocus,
      "unsupportedCapability",
      `${proof.source} cannot lower effect locus "${locus.kind}" through its compiled response lens proof`,
    );
  }
  assertNamedLocusIsDeclared(proof, effectiveLocus);
  return Object.freeze({
    version: RESOURCE_EFFECT_LOCUS_PROOF_VERSION,
    lensVersion: proof.version,
    lensSource: proof.source,
    declarationDigest: proof.declarationDigest,
    capabilityDigest: proof.capabilityDigest,
    compiledLensDigest: proof.compiledLensDigest,
    parityDigest: proof.parityDigest,
    compileBoundaryDigest: proof.compileBoundaryDigest,
    capabilityRowDigest: createCapabilityRowDigest(capability),
    effectLocusDigest: createEffectLocusDigest(proof, capability, effectiveLocus),
    topology: proof.topology,
    itemField: proof.itemField,
    locus: capability.locus,
    patchScope: capability.patchScope,
    field: effectiveLocus.kind === "detailField" ? effectiveLocus.field : null,
    region: effectiveLocus.kind === "detailRegion" ? effectiveLocus.region : null,
    path: effectiveLocus.kind === "detailJsonPath" ? effectiveLocus.path : null,
    aspect: isAspectLocus(effectiveLocus) ? effectiveLocus.aspect : null,
    summary: effectiveLocus.kind === "summary" ? effectiveLocus.summary : null,
    summaryPatchScope:
      effectiveLocus.kind === "summary" ? proof.summaryPatchScope : null,
    cost: createEffectLocusCostCounters(proof, capability, effectiveLocus),
    proofBreadth: 1,
  });
}

function createEffectLocusCostCounters(proof, capability, locus) {
  const cost = readDeclaredEffectLocusCost(proof, capability, locus);
  if (cost !== null) {
    return createEffectLocusCostCounter(...cost);
  }
  return createEffectLocusCostCounter(
    `${capability.locus}-declaration`,
    1,
    `${capability.patchScope}-scope`,
    `${proof.topology}-lens`,
  );
}

function readDeclaredEffectLocusCost(proof, capability, locus) {
  const cost = RESOURCE_RESPONSE_TOPOLOGY_COSTS[proof.topology];
  if (cost === undefined) {
    return null;
  }
  if (capability.locus === "broadResponse" && cost.broad !== undefined) {
    return cost.broad;
  }
  if (capability.locus === "detailField" && cost.field !== undefined) {
    return cost.field;
  }
  if (capability.locus === "detailRegion" && cost.region !== undefined) {
    return cost.region;
  }
  if (capability.locus === "detailJsonPath" && cost.jsonPath !== undefined) {
    return cost.jsonPath;
  }
  if (
    (capability.locus === cost.itemLocus || isAspectLocus(locus)) &&
    cost.item !== undefined
  ) {
    return cost.item;
  }
  return null;
}

function createEffectLocusCostCounter(
  lookup,
  lookupBreadth,
  traversal,
  reconstruction,
) {
  return Object.freeze({
    lookup,
    lookupBreadth,
    traversal,
    traversalBreadth: 1,
    reconstruction,
    reconstructionBreadth: 1,
  });
}

function createCapabilityRowDigest(capability) {
  return [
    "response-capability-row",
    capability.locus,
    capability.patchScope,
    capability.admitted ? "admitted" : "denied",
    capability.summaryPatchScope ?? "none",
  ].join("|");
}

function createEffectLocusDigest(proof, capability, locus) {
  return [
    "response-effect-locus",
    proof.compiledLensDigest,
    createCapabilityRowDigest(capability),
    locus.kind,
    locus.kind === "detailRegion" ? locus.region : "none",
    isAspectLocus(locus) ? locus.aspect : "none",
    locus.kind === "detailJsonPath" ? locus.path : "none",
    locus.kind === "summary" ? locus.summary : "none",
  ].join("|");
}

function createResponseLensDenialError(proof, locus, reason, message) {
  const error = new TypeError(message);
  error.name = "ResourceResponseLensDenialError";
  error.denialProof = createResponseLensDenialProof(proof, locus, reason);
  return error;
}

function createResponseLensDenialProof(proof, locus, reason) {
  const requestedPatchScope = patchScopeForEffectLocus(locus);
  return Object.freeze({
    version: RESOURCE_RESPONSE_LENS_DENIAL_PROOF_VERSION,
    lensVersion: proof.version,
    lensSource: proof.source,
    declarationDigest: proof.declarationDigest,
    capabilityDigest: proof.capabilityDigest,
    compiledLensDigest: proof.compiledLensDigest,
    parityDigest: proof.parityDigest,
    compileBoundaryDigest: proof.compileBoundaryDigest,
    requestedLocus: capabilityLocusForEffectLocus(locus) ?? locus.kind,
    requestedPatchScope,
    field: locus.kind === "detailField" ? locus.field : null,
    region: locus.kind === "detailRegion" ? locus.region : null,
    path: locus.kind === "detailJsonPath" ? locus.path : null,
    aspect: isAspectLocus(locus) ? locus.aspect : null,
    summary: locus.kind === "summary" ? locus.summary : null,
    reason,
    denialDigest: createResponseLensDenialDigest(
      proof,
      locus,
      reason,
      requestedPatchScope,
    ),
  });
}

function createResponseLensDenialDigest(proof, locus, reason, patchScope) {
  return [
    "response-lens-denial",
    proof.compiledLensDigest,
    proof.compileBoundaryDigest,
    reason,
    capabilityLocusForEffectLocus(locus) ?? locus.kind,
    patchScope ?? "none",
    locus.kind === "detailField" ? locus.field : "none",
    locus.kind === "detailRegion" ? locus.region : "none",
    locus.kind === "detailJsonPath" ? locus.path : "none",
    isAspectLocus(locus) ? locus.aspect : "none",
    locus.kind === "summary" ? locus.summary : "none",
  ].join("|");
}

function assertResponseLensAdmitsPatch(lensProof, patch) {
  if (lensProof === null) {
    return;
  }
  const proof = requireResponseLensProof(
    lensProof,
    "resource response lens patch admission",
  );
  const locus = createResponseLensPatchAdmissionLocus(proof, patch);
  if (locus === null) {
    return;
  }
  lowerResponseLensProofToEffectLocus(proof, locus);
}

function createResponseLensPatchAdmissionLocus(proof, patch) {
  switch (patch.kind) {
    case "replace":
      return Object.freeze({
        kind: lineResponseLocusForTopology(proof.topology),
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
        kind: itemLocusForCollectionTopology(proof.topology),
        itemId: patch.itemId,
      });
    case "itemAspect":
      return Object.freeze({
        kind: proof.jsonAspectNames.includes(patch.aspect)
          ? "jsonItemAspect"
          : "itemAspect",
        itemId: patch.itemId,
        aspect: patch.aspect,
      });
    case "summary":
      return Object.freeze({ kind: "summary", summary: patch.summary });
    default:
      return null;
  }
}

function readCapabilityForLocus(proof, locus) {
  const patchScope = patchScopeForEffectLocus(locus);
  const capabilityLocus = capabilityLocusForEffectLocus(locus);
  return proof.capabilityRows.find(
    (row) =>
      row.locus === capabilityLocus
      && row.patchScope === patchScope,
  ) ?? null;
}

function assertNamedLocusIsDeclared(proof, locus) {
  if (locus.kind === "itemAspect" && !proof.aspectNames.includes(locus.aspect)) {
    throw createResponseLensDenialError(
      proof,
      locus,
      "undeclaredAspect",
      `${proof.source} cannot lower undeclared aspect "${locus.aspect}" through its compiled response lens proof`,
    );
  }
  if (
    locus.kind === "jsonItemAspect" &&
    !proof.jsonAspectNames.includes(locus.aspect)
  ) {
    throw createResponseLensDenialError(
      proof,
      locus,
      "undeclaredJsonAspect",
      `${proof.source} cannot lower undeclared JSON aspect "${locus.aspect}" through its compiled response lens proof`,
    );
  }
  if (locus.kind === "detailField" && !proof.fieldNames.includes(locus.field)) {
    throw createResponseLensDenialError(
      proof,
      locus,
      "undeclaredField",
      `${proof.source} cannot lower undeclared detail field "${locus.field}" through its compiled response lens proof`,
    );
  }
  if (
    locus.kind === "detailRegion" &&
    !proof.regionNames.includes(locus.region)
  ) {
    throw createResponseLensDenialError(
      proof,
      locus,
      "undeclaredRegion",
      `${proof.source} cannot lower undeclared detail region "${locus.region}" through its compiled response lens proof`,
    );
  }
  if (
    locus.kind === "detailJsonPath" &&
    !proof.jsonPathNames.includes(locus.path)
  ) {
    throw createResponseLensDenialError(
      proof,
      locus,
      "undeclaredJsonPath",
      `${proof.source} cannot lower undeclared detail JSON path "${locus.path}" through its compiled response lens proof`,
    );
  }
  if (locus.kind === "summary" && !proof.summaryNames.includes(locus.summary)) {
    throw createResponseLensDenialError(
      proof,
      locus,
      "undeclaredSummary",
      `${proof.source} cannot lower undeclared summary "${locus.summary}" through its compiled response lens proof`,
    );
  }
}

function normalizeJsonItemAspectLocus(proof, locus) {
  if (
    locus.kind === "itemAspect" &&
    proof.jsonAspectNames.includes(locus.aspect)
  ) {
    return Object.freeze({
      kind: "jsonItemAspect",
      itemId: locus.itemId,
      aspect: locus.aspect,
    });
  }
  return locus;
}

function isAspectLocus(locus) {
  return locus.kind === "itemAspect" || locus.kind === "jsonItemAspect";
}

function patchScopeForEffectLocus(locus) {
  switch (locus.kind) {
    case "broadResponse":
    case "detailResponse":
    case "summaryResponse":
    case "line":
      return "line";
    case "detailField":
      return "field";
    case "detailRegion":
      return "region";
    case "detailJsonPath":
      return "jsonPath";
    case "membership":
    case "connection":
    case "discriminatedTuple":
    case "entityStore":
    case "groupedCollection":
    case "mapCollection":
    case "namedCollection":
    case "recursiveTree":
    case "sparsePage":
    case "item":
      return "item";
    case "itemAspect":
    case "jsonItemAspect":
      return "aspect";
    case "summary":
      return "summary";
    default:
      return null;
  }
}

function capabilityLocusForEffectLocus(locus) {
  switch (locus.kind) {
    case "broadResponse":
      return "broadResponse";
    case "detailResponse":
      return "detailResponse";
    case "detailField":
      return "detailField";
    case "detailRegion":
      return "detailRegion";
    case "detailJsonPath":
      return "detailJsonPath";
    case "summaryResponse":
      return "summaryResponse";
    case "membership":
      return "membership";
    case "connection":
      return "connection";
    case "discriminatedTuple":
      return "discriminatedTuple";
    case "entityStore":
      return "entityStore";
    case "groupedCollection":
      return "groupedCollection";
    case "mapCollection":
      return "mapCollection";
    case "namedCollection":
      return "namedCollection";
    case "recursiveTree":
      return "recursiveTree";
    case "sparsePage":
      return "sparsePage";
    case "itemAspect":
      return "itemAspect";
    case "jsonItemAspect":
      return "jsonItemAspect";
    case "summary":
      return "summary";
    default:
      return null;
  }
}

function itemLocusForCollectionTopology(topology) {
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

function lineResponseLocusForTopology(topology) {
  if (topology === "detail") {
    return "detailResponse";
  }
  if (topology === "summary") {
    return "summaryResponse";
  }
  return "broadResponse";
}

export {
  assertResponseLensAdmitsPatch,
  createResponseLensDenialError,
  lowerResponseLensProofToEffectLocus,
};
