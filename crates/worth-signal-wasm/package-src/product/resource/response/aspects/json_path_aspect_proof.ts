const RESOURCE_JSON_PATH_ASPECT_PROOF = Symbol("WorthSignal.resourceJsonPathAspectProof");

const RESOURCE_JSON_PATH_ASPECT_PROOF_VERSION = "resource-json-path-aspect-proof-v1";

function createResourceJsonPathAspectProof(aspect, path) {
  const pathSegments = Object.freeze([...path.segments]);
  const traversalBreadth = pathSegments.length + 1;
  const reconstructionBreadth = pathSegments.length + 1;
  const parsedPathDigest = createParsedPathDigest(path.field, pathSegments);
  return Object.freeze({
    [RESOURCE_JSON_PATH_ASPECT_PROOF]: "resourceJsonPathAspectProof",
    version: RESOURCE_JSON_PATH_ASPECT_PROOF_VERSION,
    aspect,
    field: path.field,
    path: pathSegments,
    parsedPathDigest,
    policy: Object.freeze({
      presence: path.presence,
      absence: path.presence === "optional" ? "readAsNull" : "deny",
      containerWrite: "immutableCopy",
      extensibility: "immutableCopy",
      objectPrototype: "plainOrNull",
      prototypeReconstruction: "plainOrNullCopy",
      arrayIndex: "explicitExistingIndex",
      accessor: "denyWithoutInvocation",
    }),
    cost: Object.freeze({
      traversalBreadth,
      reconstructionBreadth,
      cloneBreadth: reconstructionBreadth,
    }),
    proofDigest: [
      "resource-json-path-aspect-proof",
      aspect,
      parsedPathDigest,
      path.presence,
      `traverse:${traversalBreadth}`,
      `reconstruct:${reconstructionBreadth}`,
      "immutable-copy",
      "prototype-reconstruction:plain-or-null-copy",
    ].join("|"),
  });
}

function requireResourceJsonPathAspectProof(value, aspect) {
  if (value === undefined) {
    return undefined;
  }
  if (
    !value ||
    typeof value !== "object" ||
    value[RESOURCE_JSON_PATH_ASPECT_PROOF] !== "resourceJsonPathAspectProof" ||
    value.version !== RESOURCE_JSON_PATH_ASPECT_PROOF_VERSION ||
    value.aspect !== aspect
  ) {
    throw new TypeError(
      `resourceItemAspects(...) aspect "${aspect}" has invalid JSON path proof`,
    );
  }
  return value;
}

function createParsedPathDigest(field, segments) {
  return [
    "json-path",
    formatJsonPathDigestSegment(field),
    ...segments.map((segment) => formatJsonPathDigestSegment(segment)),
  ].join("/");
}

function formatJsonPathDigestSegment(segment) {
  return typeof segment === "number"
    ? `#${segment}`
    : encodeURIComponent(segment);
}

export {
  createResourceJsonPathAspectProof,
  requireResourceJsonPathAspectProof,
};
