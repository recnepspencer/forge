const RESOURCE_RESPONSE_LENS_PROOF = Symbol(
  "forgeSignal.resourceResponseLensProof",
);

const RESOURCE_RESPONSE_LENS_PROOF_VERSION = "resource-response-lens-proof-v1";

function createResponseLensProof(options) {
  const topology = requireResponseLensTopology(options.topology);
  const aspectNames = Object.freeze([...(options.aspectNames ?? [])].sort());
  const jsonAspectNames = Object.freeze([...(options.jsonAspectNames ?? [])].sort());
  const summaryNames = Object.freeze([...(options.summaryNames ?? [])].sort());
  const summaryPatchScope = options.summaryPatchScope ?? null;
  const capabilityRows = createCapabilityRows({
    ...options,
    aspectNames,
    jsonAspectNames,
    summaryNames,
    summaryPatchScope,
  });
  const declarationDigest = createResponseDeclarationDigest({
    source: options.source,
    topology,
    itemField: options.itemField ?? null,
    aspectNames,
    jsonAspectNames,
    summaryNames,
    summaryPatchScope,
  });
  const capabilityDigest = createCapabilityDigest(capabilityRows);
  const compiledLensDigest = `${declarationDigest}|${capabilityDigest}`;
  return Object.freeze({
    version: RESOURCE_RESPONSE_LENS_PROOF_VERSION,
    source: options.source,
    topology,
    itemField: options.itemField ?? null,
    declarationDigest,
    capabilityDigest,
    compiledLensDigest,
    parityDigest: createResponseLensParityDigest({
      source: options.source,
      topology,
      itemField: options.itemField ?? null,
      capabilityRows,
      summaryPatchScope,
    }),
    compileBoundaryDigest: createResponseLensCompileBoundaryDigest({
      declarationDigest,
      capabilityDigest,
      compiledLensDigest,
    }),
    capabilityRows,
    aspectNames,
    jsonAspectNames,
    summaryNames,
    summaryPatchScope,
    [RESOURCE_RESPONSE_LENS_PROOF]: "resourceResponseLensProof",
  });
}

function requireResponseLensTopology(topology) {
  if (
    topology !== "directArray"
    && topology !== "objectItems"
    && topology !== "customCollection"
    && topology !== "detail"
  ) {
    throw new TypeError(
      `resource response lens proof cannot classify topology "${topology}"`,
    );
  }
  return topology;
}

function createCapabilityRows(options) {
  const rows = [];
  if (options.topology === "detail") {
    rows.push(createCapabilityRow("detailResponse", "line", true));
    return Object.freeze(rows);
  }
  rows.push(
    createCapabilityRow("broadResponse", "line", true),
    createCapabilityRow("membership", "item", true),
  );
  const jsonAspectNames = options.jsonAspectNames ?? [];
  const ordinaryAspectNames = (options.aspectNames ?? []).filter(
    (aspect) => !jsonAspectNames.includes(aspect),
  );
  if (ordinaryAspectNames.length > 0) {
    rows.push(createCapabilityRow("itemAspect", "aspect", true));
  }
  if (jsonAspectNames.length > 0) {
    rows.push(createCapabilityRow("jsonItemAspect", "aspect", true));
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

function createResponseDeclarationDigest(options) {
  return [
    "response-declaration",
    options.source,
    options.topology,
    options.itemField ?? "none",
    `aspects:${options.aspectNames.join(",")}`,
    `json:${options.jsonAspectNames.join(",")}`,
    `summaries:${options.summaryNames.join(",")}`,
    `summaryScope:${options.summaryPatchScope ?? "none"}`,
  ].join("|");
}

function createCapabilityDigest(capabilityRows) {
  return [
    "response-capabilities",
    ...capabilityRows.map((row) => [
      row.locus,
      row.patchScope,
      row.admitted ? "admitted" : "denied",
      row.summaryPatchScope ?? "none",
    ].join(":")),
  ].join("|");
}

function createResponseLensParityDigest(options) {
  return [
    "response-parity",
    options.source,
    options.topology,
    options.itemField ?? "none",
    `summaryScope:${options.summaryPatchScope ?? "none"}`,
    ...options.capabilityRows.map((row) => [
      row.locus,
      row.patchScope,
      row.summaryPatchScope ?? "none",
    ].join(":")),
  ].join("|");
}

function createResponseLensCompileBoundaryDigest(options) {
  return [
    "response-compile-boundary",
    RESOURCE_RESPONSE_LENS_PROOF_VERSION,
    options.declarationDigest,
    options.capabilityDigest,
    options.compiledLensDigest,
  ].join("|");
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
  createResponseLensProof,
  requireResponseLensProof,
};
