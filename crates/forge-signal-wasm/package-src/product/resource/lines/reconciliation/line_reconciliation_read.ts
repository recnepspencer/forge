function readLineReconciliation(materialization) {
  return Object.freeze({
    broadReplace: materialization.patch.broadReplace,
    narrowItem: materialization.patch.narrowItem,
    narrowSummary: materialization.patch.narrowSummary,
    aspectNames: materialization.patch.aspectNames,
    summaryNames: materialization.patch.summaryNames,
  });
}

export { readLineReconciliation };
