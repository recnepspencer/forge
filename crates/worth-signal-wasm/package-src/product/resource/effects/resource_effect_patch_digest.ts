function createResourceEffectPatchDigest(patch) {
  return Object.freeze({
    kind: patch.kind,
    scope: patch.scope,
    itemId: patch.itemId,
    field: patch.field,
    regionName: patch.region ?? null,
    path: patch.path ?? null,
    aspect: patch.aspect,
    summary: patch.summary,
    valueChanged: patch.valueChanged,
    fieldProof: createFieldPatchProof(patch.fieldProof),
    region: createRegionPatchProof(patch.regionProof),
    jsonPath: createJsonPathPatchProof(patch.jsonPathProof),
  });
}

function createFieldPatchProof(fieldProof) {
  if (fieldProof === null || fieldProof === undefined) {
    return null;
  }
  return Object.freeze({
    version: fieldProof.version,
    fieldName: fieldProof.fieldName,
    cost: fieldProof.cost,
    proofDigest: fieldProof.proofDigest,
  });
}

function createRegionPatchProof(regionProof) {
  if (regionProof === null || regionProof === undefined) {
    return null;
  }
  return Object.freeze({
    version: regionProof.version,
    regionName: regionProof.regionName,
    identityBoundary: regionProof.identityBoundary,
    mergeGranularity: regionProof.mergeGranularity,
    cost: regionProof.cost,
    proofDigest: regionProof.proofDigest,
  });
}

function createJsonPathPatchProof(jsonPathProof) {
  if (jsonPathProof === null || jsonPathProof === undefined) {
    return null;
  }
  return Object.freeze({
    version: jsonPathProof.version,
    aspect: "aspect" in jsonPathProof ? jsonPathProof.aspect : null,
    field: "field" in jsonPathProof ? jsonPathProof.field : null,
    pathName: "pathName" in jsonPathProof ? jsonPathProof.pathName : null,
    path: jsonPathProof.path,
    parsedPathDigest: jsonPathProof.parsedPathDigest,
    policy: jsonPathProof.policy,
    cost: jsonPathProof.cost,
    proofDigest: jsonPathProof.proofDigest,
  });
}

export { createResourceEffectPatchDigest };
