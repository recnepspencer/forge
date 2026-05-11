import { requireResponseLensProof } from "./resource_response_lens_proof.js";

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
    aspect: isAspectLocus(effectiveLocus) ? effectiveLocus.aspect : null,
    summary: effectiveLocus.kind === "summary" ? effectiveLocus.summary : null,
    summaryPatchScope:
      effectiveLocus.kind === "summary" ? proof.summaryPatchScope : null,
    proofBreadth: 1,
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
    isAspectLocus(locus) ? locus.aspect : "none",
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
        kind: proof.topology === "detail" ? "detailResponse" : "broadResponse",
      });
    case "item":
      return Object.freeze({ kind: "membership", itemId: patch.itemId });
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
    case "line":
      return "line";
    case "membership":
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
    case "membership":
      return "membership";
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

export {
  assertResponseLensAdmitsPatch,
  createResponseLensDenialError,
  lowerResponseLensProofToEffectLocus,
};
