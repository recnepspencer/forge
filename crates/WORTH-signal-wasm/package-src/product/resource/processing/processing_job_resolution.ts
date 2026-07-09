import { requireResourceProcessingJobPosture } from "./processing_job_posture.js";
import { resourceProcessingJob } from "./resource_processing_job.js";
import {
  readTaggedRequestSourceResolution,
} from "../requests/request_source_metadata.js";

function resolveResourceProcessingJobPosture(input, params, family) {
  if (input === undefined) {
    return Object.freeze({
      value: resourceProcessingJob.none(),
      source: Object.freeze({
        source: "default.processingJob",
        overridden: false,
      }),
    });
  }
  const tagged = readTaggedRequestSourceResolution(input, params);
  if (tagged !== null) {
    return Object.freeze({
      value: requireResourceProcessingJobPosture(tagged.value, family),
      source: tagged.source,
    });
  }
  const value = typeof input === "function" ? input(params) : input;
  return Object.freeze({
    value: requireResourceProcessingJobPosture(value, family),
    source: Object.freeze({
      source: "endpoint.processingJob",
      overridden: false,
    }),
  });
}

export { resolveResourceProcessingJobPosture };
