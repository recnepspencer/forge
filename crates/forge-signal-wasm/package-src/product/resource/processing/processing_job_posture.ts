const RESOURCE_PROCESSING_JOB_POSTURE_BRAND = Symbol(
  "forgeSignal.resourceProcessingJobPosture",
);

function createResourceProcessingJobPosture(kind, fields) {
  return Object.freeze({
    kind,
    ...fields,
    [RESOURCE_PROCESSING_JOB_POSTURE_BRAND]: "resourceProcessingJobPosture",
  });
}

function requireResourceProcessingJobPosture(value, family) {
  if (
    !value ||
    value[RESOURCE_PROCESSING_JOB_POSTURE_BRAND] !==
      "resourceProcessingJobPosture"
  ) {
    throw new TypeError(
      `${family} resources require processingJob created with resourceProcessingJob.*()`,
    );
  }
  return value;
}

export {
  createResourceProcessingJobPosture,
  requireResourceProcessingJobPosture,
};
