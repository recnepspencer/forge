function readLineReconciliation(materialization) {
  return Object.freeze({
    broadReplace: materialization.patch.broadReplace,
    narrowItem: materialization.patch.narrowItem,
    narrowField: materialization.patch.narrowField,
    narrowRegion: materialization.patch.narrowRegion,
    narrowJsonPath: materialization.patch.narrowJsonPath,
    narrowSummary: materialization.patch.narrowSummary,
    fieldNames: materialization.patch.fieldNames,
    regionNames: materialization.patch.regionNames,
    jsonPathNames: materialization.patch.jsonPathNames,
    aspectNames: materialization.patch.aspectNames,
    summaryNames: materialization.patch.summaryNames,
  });
}

export { readLineReconciliation };
