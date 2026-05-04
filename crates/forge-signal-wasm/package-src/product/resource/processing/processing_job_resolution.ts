import { requireResourceProcessingJobPosture } from "./processing_job_posture.js";
import { resourceProcessingJob } from "./resource_processing_job.js";

function resolveResourceProcessingJobPosture(input, params, family) {
  if (input === undefined) {
    return resourceProcessingJob.none();
  }
  const value = typeof input === "function" ? input(params) : input;
  return requireResourceProcessingJobPosture(value, family);
}

export { resolveResourceProcessingJobPosture };
