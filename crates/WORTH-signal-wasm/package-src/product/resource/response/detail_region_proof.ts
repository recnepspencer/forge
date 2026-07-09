const RESOURCE_DETAIL_REGION_PROOF = Symbol(
  "WORTHSignal.resourceDetailRegionProof",
);

const RESOURCE_DETAIL_REGION_PROOF_VERSION =
  "resource-detail-region-proof-v1";

function createResourceDetailRegionProof(regionName, definition) {
  const traversalBreadth = definition.cost.traversalBreadth;
  const reconstructionBreadth = definition.cost.reconstructionBreadth;
  return Object.freeze({
    [RESOURCE_DETAIL_REGION_PROOF]: "resourceDetailRegionProof",
    version: RESOURCE_DETAIL_REGION_PROOF_VERSION,
    regionName,
    identityBoundary: definition.identityBoundary,
    mergeGranularity: definition.mergeGranularity,
    cost: Object.freeze({
      traversalBreadth,
      reconstructionBreadth,
      cloneBreadth: reconstructionBreadth,
    }),
    proofDigest: [
      "resource-detail-region-proof",
      regionName,
      `identity:${definition.identityBoundary}`,
      `merge:${definition.mergeGranularity}`,
      `traverse:${traversalBreadth}`,
      `reconstruct:${reconstructionBreadth}`,
    ].join("|"),
  });
}

function requireResourceDetailRegionProof(value, regionName) {
  if (value === undefined) {
    return undefined;
  }
  if (
    !value ||
    typeof value !== "object" ||
    value[RESOURCE_DETAIL_REGION_PROOF] !== "resourceDetailRegionProof" ||
    value.version !== RESOURCE_DETAIL_REGION_PROOF_VERSION ||
    value.regionName !== regionName
  ) {
    throw new TypeError(
      `resourceDetailRegions(...) region "${regionName}" has invalid region proof`,
    );
  }
  return value;
}

export {
  createResourceDetailRegionProof,
  requireResourceDetailRegionProof,
};
