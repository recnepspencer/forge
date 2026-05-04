function createLinePatchRecord(familyKind, itemIdentity, reconcile) {
  const aspectNames =
    reconcile?.aspects === null || reconcile?.aspects === undefined
      ? Object.freeze([])
      : Object.freeze(Object.keys(reconcile.aspects.definitions).sort());
  const summaryNames =
    reconcile?.summaries === null || reconcile?.summaries === undefined
      ? Object.freeze([])
      : Object.freeze(Object.keys(reconcile.summaries.definitions).sort());
  return Object.freeze({
    familyKind,
    itemIdentity,
    reconcile,
    broadReplace: familyKind !== "detail",
    narrowItem:
      familyKind !== "detail"
      && typeof itemIdentity === "function"
      && reconcile !== null,
    narrowSummary:
      familyKind !== "detail"
      && reconcile !== null
      && summaryNames.length > 0,
    aspectNames,
    summaryNames,
  });
}

export { createLinePatchRecord };
