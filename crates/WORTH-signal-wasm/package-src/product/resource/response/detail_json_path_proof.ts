const RESOURCE_DETAIL_JSON_PATH_PROOF = Symbol(
  "WORTHSignal.resourceDetailJsonPathProof",
);

const RESOURCE_DETAIL_JSON_PATH_PROOF_VERSION =
  "resource-detail-json-path-proof-v1";

function createResourceDetailJsonPathProof(pathName, path) {
  const pathSegments = Object.freeze([...path.segments]);
  const traversalBreadth = pathSegments.length + 1;
  const reconstructionBreadth = pathSegments.length + 1;
  const parsedPathDigest = createParsedPathDigest(pathSegments);
  return Object.freeze({
    [RESOURCE_DETAIL_JSON_PATH_PROOF]: "resourceDetailJsonPathProof",
    version: RESOURCE_DETAIL_JSON_PATH_PROOF_VERSION,
    pathName,
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
      "resource-detail-json-path-proof",
      pathName,
      parsedPathDigest,
      path.presence,
      `traverse:${traversalBreadth}`,
      `reconstruct:${reconstructionBreadth}`,
      "immutable-copy",
      "prototype-reconstruction:plain-or-null-copy",
    ].join("|"),
  });
}

function requireResourceDetailJsonPathProof(value, pathName) {
  if (value === undefined) {
    return undefined;
  }
  if (
    !value ||
    typeof value !== "object" ||
    value[RESOURCE_DETAIL_JSON_PATH_PROOF] !== "resourceDetailJsonPathProof" ||
    value.version !== RESOURCE_DETAIL_JSON_PATH_PROOF_VERSION ||
    value.pathName !== pathName
  ) {
    throw new TypeError(
      `resourceDetailJsonPaths(...) path "${pathName}" has invalid JSON path proof`,
    );
  }
  return value;
}

function createParsedPathDigest(segments) {
  return [
    "detail-json-path",
    ...segments.map((segment) => formatJsonPathDigestSegment(segment)),
  ].join("/");
}

function formatJsonPathDigestSegment(segment) {
  return typeof segment === "number"
    ? `#${segment}`
    : encodeURIComponent(segment);
}

export {
  createResourceDetailJsonPathProof,
  requireResourceDetailJsonPathProof,
};
