const RESOURCE_CONTINUATION_POSTURE_BRAND = Symbol(
  "WORTHSignal.resourceContinuationPosture",
);

function createResourceContinuationPosture(kind, summary) {
  return Object.freeze({
    kind,
    ...summary,
    [RESOURCE_CONTINUATION_POSTURE_BRAND]: "resourceContinuationPosture",
  });
}

function requireResourceContinuationPosture(value, family) {
  if (
    !value ||
    value[RESOURCE_CONTINUATION_POSTURE_BRAND] !==
      "resourceContinuationPosture"
  ) {
    throw new TypeError(
      `${family} resources require continuation created with resourceContinuation.*()`,
    );
  }
  return value;
}

export {
  createResourceContinuationPosture,
  requireResourceContinuationPosture,
};
