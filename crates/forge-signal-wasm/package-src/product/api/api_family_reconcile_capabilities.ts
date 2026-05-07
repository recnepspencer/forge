function readApiFamilyReconcileCapabilities(familyKind, declaration) {
  const reconcile = declaration.reconcile ?? null;
  if (reconcile === null) {
    return Object.freeze({
      hasReconcile: false,
      hasAspects: false,
      admitsSummary: false,
    });
  }
  const aspectNames = Object.keys(reconcile.aspects?.definitions ?? {});
  const summaries = reconcile.summaries ?? null;
  const summaryNames = Object.keys(summaries?.definitions ?? {});
  const admitsSummary =
    summaryNames.length > 0
    && (familyKind === "collection" || summaries.patchScope === "pageWindow");
  return Object.freeze({
    hasReconcile: true,
    hasAspects: aspectNames.length > 0,
    admitsSummary,
  });
}

export { readApiFamilyReconcileCapabilities };
