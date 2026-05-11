const RESOURCE_RESPONSE_LENS_PROOF = Symbol(
  "forgeSignal.resourceResponseLensProof",
);

const RESOURCE_EFFECT_LOCUS_PROOF_VERSION = "resource-effect-locus-proof-v1";
const RESOURCE_RESPONSE_LENS_PROOF_VERSION = "resource-response-lens-proof-v1";

function createResponseLensProof(options) {
  const topology = requireResponseLensTopology(options.topology);
  return Object.freeze({
    version: RESOURCE_RESPONSE_LENS_PROOF_VERSION,
    source: options.source,
    topology,
    itemField: options.itemField ?? null,
    capabilityRows: createCapabilityRows(options),
    aspectNames: Object.freeze([...(options.aspectNames ?? [])].sort()),
    summaryNames: Object.freeze([...(options.summaryNames ?? [])].sort()),
    summaryPatchScope: options.summaryPatchScope ?? null,
    [RESOURCE_RESPONSE_LENS_PROOF]: "resourceResponseLensProof",
  });
}

function requireResponseLensTopology(topology) {
  if (
    topology !== "directArray"
    && topology !== "objectItems"
    && topology !== "customCollection"
  ) {
    throw new TypeError(
      `resource response lens proof cannot classify topology "${topology}"`,
    );
  }
  return topology;
}

function createCapabilityRows(options) {
  const rows = [
    createCapabilityRow("broadResponse", "line", true),
    createCapabilityRow("membership", "item", true),
  ];
  if ((options.aspectNames ?? []).length > 0) {
    rows.push(createCapabilityRow("itemAspect", "aspect", true));
  }
  if ((options.summaryNames ?? []).length > 0) {
    rows.push(
      createCapabilityRow(
        "summary",
        "summary",
        true,
        options.summaryPatchScope ?? null,
      ),
    );
  }
  return Object.freeze(rows);
}

function createCapabilityRow(locus, patchScope, admitted, summaryPatchScope = null) {
  return Object.freeze({
    locus,
    patchScope,
    admitted,
    summaryPatchScope,
  });
}

function lowerResponseLensProofToEffectLocus(lensProof, locus) {
  if (lensProof === null) {
    return null;
  }
  const proof = requireResponseLensProof(lensProof, "resource effect locus lowering");
  if (patchScopeForEffectLocus(locus) === null) {
    return null;
  }
  const capability = readCapabilityForLocus(proof, locus);
  if (capability === null || capability.admitted !== true) {
    throw new TypeError(
      `${proof.source} cannot lower effect locus "${locus.kind}" through its compiled response lens proof`,
    );
  }
  assertNamedLocusIsDeclared(proof, locus);
  return Object.freeze({
    version: RESOURCE_EFFECT_LOCUS_PROOF_VERSION,
    lensVersion: proof.version,
    lensSource: proof.source,
    topology: proof.topology,
    itemField: proof.itemField,
    locus: capability.locus,
    patchScope: capability.patchScope,
    aspect: locus.kind === "itemAspect" ? locus.aspect : null,
    summary: locus.kind === "summary" ? locus.summary : null,
    summaryPatchScope:
      locus.kind === "summary" ? proof.summaryPatchScope : null,
    proofBreadth: 1,
  });
}

function assertResponseLensAdmitsPatch(lensProof, patch) {
  if (lensProof === null) {
    return;
  }
  const locus = createPatchAdmissionLocus(patch);
  if (locus === null) {
    return;
  }
  lowerResponseLensProofToEffectLocus(lensProof, locus);
}

function createPatchAdmissionLocus(patch) {
  switch (patch.kind) {
    case "replace":
      return Object.freeze({ kind: "line" });
    case "item":
      return Object.freeze({ kind: "item", itemId: patch.itemId });
    case "itemAspect":
      return Object.freeze({
        kind: "itemAspect",
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
  return proof.capabilityRows.find((row) => row.patchScope === patchScope) ?? null;
}

function assertNamedLocusIsDeclared(proof, locus) {
  if (locus.kind === "itemAspect" && !proof.aspectNames.includes(locus.aspect)) {
    throw new TypeError(
      `${proof.source} cannot lower undeclared aspect "${locus.aspect}" through its compiled response lens proof`,
    );
  }
  if (locus.kind === "summary" && !proof.summaryNames.includes(locus.summary)) {
    throw new TypeError(
      `${proof.source} cannot lower undeclared summary "${locus.summary}" through its compiled response lens proof`,
    );
  }
}

function patchScopeForEffectLocus(locus) {
  switch (locus.kind) {
    case "line":
      return "line";
    case "item":
      return "item";
    case "itemAspect":
      return "aspect";
    case "summary":
      return "summary";
    default:
      return null;
  }
}

function requireResponseLensProof(value, source) {
  if (
    !value
    || typeof value !== "object"
    || value[RESOURCE_RESPONSE_LENS_PROOF] !== "resourceResponseLensProof"
  ) {
    throw new TypeError(`${source} requires a compiled response lens proof`);
  }
  return value;
}

export {
  assertResponseLensAdmitsPatch,
  createResponseLensProof,
  lowerResponseLensProofToEffectLocus,
  requireResponseLensProof,
};
