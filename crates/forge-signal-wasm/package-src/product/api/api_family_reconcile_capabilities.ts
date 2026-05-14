function readApiFamilyReconcileCapabilities(familyKind, declaration) {
  const reconcile =
    declaration.reconcile
    ?? declaration.detailFields
    ?? declaration.detailRegions
    ?? declaration.detailJsonPaths
    ?? null;
  if (reconcile === null) {
    return Object.freeze({
      hasReconcile: false,
      hasFields: false,
      hasRegions: false,
      hasJsonPaths: false,
      hasAspects: false,
      admitsSummary: false,
    });
  }
  const definitions = Object.entries(reconcile.definitions ?? {});
  const fieldNames = definitions
    .filter(
      ([, definition]) =>
        definition?.jsonPathProof === undefined
        && definition?.regionProof === undefined,
    )
    .map(([name]) => name);
  const regionNames = definitions
    .filter(([, definition]) => definition?.regionProof !== undefined)
    .map(([name]) => name);
  const jsonPathNames = definitions
    .filter(([, definition]) => definition?.jsonPathProof !== undefined)
    .map(([name]) => name);
  const aspectNames = Object.keys(reconcile.aspects?.definitions ?? {});
  const summaries = reconcile.summaries ?? null;
  const summaryNames = Object.keys(summaries?.definitions ?? {});
  const admitsSummary =
    summaryNames.length > 0
    && (familyKind === "collection" || summaries.patchScope === "pageWindow");
  return Object.freeze({
    hasReconcile: true,
    hasFields: fieldNames.length > 0,
    hasRegions: regionNames.length > 0,
    hasJsonPaths: jsonPathNames.length > 0,
    hasAspects: aspectNames.length > 0,
    admitsSummary,
  });
}

export { readApiFamilyReconcileCapabilities };
