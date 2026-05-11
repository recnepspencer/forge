function createLinePatchRecord(familyKind, itemIdentity, reconcile) {
  const summaryPatchScope = reconcile?.summaries?.patchScope ?? null;
  const admitsSummaryNarrowing =
    reconcile?.summaries !== null
    && reconcile?.summaries !== undefined
    && familyKind !== "detail"
    && (
      familyKind === "paged"
        ? summaryPatchScope === "pageWindow"
        : summaryPatchScope === "line"
    );
  const aspectNames =
    reconcile?.aspects === null || reconcile?.aspects === undefined
      ? Object.freeze([])
      : Object.freeze(Object.keys(reconcile.aspects.definitions).sort());
  const summaryNames =
    !admitsSummaryNarrowing
      ? Object.freeze([])
      : Object.freeze(Object.keys(reconcile.summaries.definitions).sort());
  return Object.freeze({
    familyKind,
    itemIdentity,
    reconcile,
    responseLensProof: reconcile?.responseLensProof ?? null,
    broadReplace:
      familyKind !== "detail"
      || reconcile?.responseLensProof?.topology === "detail",
    narrowItem:
      familyKind !== "detail"
      && typeof itemIdentity === "function"
      && reconcile !== null,
    narrowSummary: summaryNames.length > 0,
    aspectNames,
    summaryNames,
  });
}

export { createLinePatchRecord };
