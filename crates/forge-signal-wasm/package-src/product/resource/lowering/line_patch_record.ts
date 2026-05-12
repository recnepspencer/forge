function createLinePatchRecord(familyKind, itemIdentity, reconcile) {
  const detailDefinitions =
    familyKind !== "detail" || reconcile?.definitions === undefined
      ? Object.freeze([])
      : Object.freeze(Object.entries(reconcile.definitions));
  const detailFieldNames =
    familyKind !== "detail" || reconcile?.definitions === undefined
      ? Object.freeze([])
      : Object.freeze(
          detailDefinitions
            .filter(
              ([, definition]) =>
                definition?.jsonPathProof === undefined
                && definition?.regionProof === undefined,
            )
            .map(([name]) => name)
            .sort(),
        );
  const detailRegionNames =
    familyKind !== "detail" || reconcile?.definitions === undefined
      ? Object.freeze([])
      : Object.freeze(
          detailDefinitions
            .filter(([, definition]) => definition?.regionProof !== undefined)
            .map(([name]) => name)
            .sort(),
        );
  const detailJsonPathNames =
    familyKind !== "detail" || reconcile?.definitions === undefined
      ? Object.freeze([])
      : Object.freeze(
          detailDefinitions
            .filter(([, definition]) => definition?.jsonPathProof !== undefined)
            .map(([name]) => name)
            .sort(),
        );
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
    broadReplace: familyKind === "detail"
      ? true
      : reconcile?.responseLensProof?.topology !== null,
    narrowField: detailFieldNames.length > 0,
    narrowRegion: detailRegionNames.length > 0,
    narrowJsonPath: detailJsonPathNames.length > 0,
    narrowItem:
      familyKind !== "detail"
      && typeof itemIdentity === "function"
      && reconcile !== null,
    narrowSummary: summaryNames.length > 0,
    fieldNames: detailFieldNames,
    regionNames: detailRegionNames,
    jsonPathNames: detailJsonPathNames,
    aspectNames,
    summaryNames,
  });
}

export { createLinePatchRecord };
