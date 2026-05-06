export function normalizeRouteLineArtifact(line) {
  return {
    kind: line.descriptor().family.kind,
    canonicalKey: line.descriptor().canonicalParams.canonicalKey,
    canonicalParams: JSON.parse(
      JSON.stringify(line.descriptor().canonicalParams.params),
    ),
    value: JSON.parse(JSON.stringify(line.value())),
    request: normalizeRouteRequestArtifact(line.request()),
  };
}

function normalizeRouteRequestArtifact(request) {
  const snapshot = JSON.parse(JSON.stringify(request));
  delete snapshot.family.familyId;
  delete snapshot.sources;
  delete snapshot.target;
  delete snapshot.baseUrl;
  return snapshot;
}
