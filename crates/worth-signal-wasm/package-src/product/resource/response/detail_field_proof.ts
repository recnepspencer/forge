const RESOURCE_DETAIL_FIELD_PROOF = Symbol(
  "WorthSignal.resourceDetailFieldProof",
);

const RESOURCE_DETAIL_FIELD_PROOF_VERSION =
  "resource-detail-field-proof-v1";

function createResourceDetailFieldProof(fieldName) {
  return Object.freeze({
    [RESOURCE_DETAIL_FIELD_PROOF]: "resourceDetailFieldProof",
    version: RESOURCE_DETAIL_FIELD_PROOF_VERSION,
    fieldName,
    cost: Object.freeze({
      traversalBreadth: 1,
      reconstructionBreadth: 1,
      cloneBreadth: 1,
    }),
    proofDigest: [
      "resource-detail-field-proof",
      fieldName,
      "traverse:1",
      "reconstruct:1",
    ].join("|"),
  });
}

function requireResourceDetailFieldProof(value, fieldName) {
  if (value === undefined) {
    return undefined;
  }
  if (
    !value ||
    typeof value !== "object" ||
    value[RESOURCE_DETAIL_FIELD_PROOF] !== "resourceDetailFieldProof" ||
    value.version !== RESOURCE_DETAIL_FIELD_PROOF_VERSION ||
    value.fieldName !== fieldName
  ) {
    throw new TypeError(
      `resourceDetailFields(...) field "${fieldName}" has invalid field proof`,
    );
  }
  return value;
}

export {
  createResourceDetailFieldProof,
  requireResourceDetailFieldProof,
};
