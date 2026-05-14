function createResourceEffectPatchCounters(patchDigest) {
  return Object.freeze({
    ...createFieldCounters(patchDigest.fieldProof),
    ...createRegionCounters(patchDigest.region),
    ...createJsonPathCounters(patchDigest.jsonPath),
  });
}

function createFieldCounters(fieldProof) {
  if (fieldProof === null) {
    return Object.freeze({
      detailFieldTraversalBreadth: 0,
      detailFieldReconstructionBreadth: 0,
    });
  }
  return Object.freeze({
    detailFieldTraversalBreadth: fieldProof.cost.traversalBreadth,
    detailFieldReconstructionBreadth: fieldProof.cost.reconstructionBreadth,
  });
}

function createRegionCounters(regionProof) {
  if (regionProof === null) {
    return Object.freeze({
      detailRegionTraversalBreadth: 0,
      detailRegionReconstructionBreadth: 0,
    });
  }
  return Object.freeze({
    detailRegionTraversalBreadth: regionProof.cost.traversalBreadth,
    detailRegionReconstructionBreadth: regionProof.cost.reconstructionBreadth,
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

export { createResourceEffectPatchCounters };
