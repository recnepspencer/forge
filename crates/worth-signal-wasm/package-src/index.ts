export default async function init(...args) {
  const rawSurface = await import("./raw_surface.js");
  return rawSurface.default(...args);
}

export {
  clockCapability,
  hostCapabilityPlan,
  onlineCapability,
  persistenceCapability,
  resourceBinaryDescriptor,
  resourceBinaryValue,
  resourceAuth,
  resourceCollectionShape,
  resourceContinuation,
  resourceDetailFields,
  resourceDetailJsonPaths,
  resourceDelivery,
  resourceDownload,
  resourceItemAspects,
  resourceParamIdentity,
  resourcePatch,
  resourceValueSummaries,
  resourceParams,
  resourcePolicyProfiles,
  resourceProcessingJob,
  resourceProcessingResult,
  resourceUploadResult,
  resourceUploadTransport,
  resourceRequestContext,
  resourceMutationResponses,
  viewportCapability,
  visibilityCapability,
  wrapSignals,
} from "./product/signals.js";
export {
  createCallableSignals,
  createSignals,
  explainCreateSignalsConstruction,
  planCreateSignalsDeployment,
} from "./product/entrypoint/construction/entrypoint_construction.js";
export { resourceEffects } from "./product/resource/facade.js";
export {
  declareLocalTruthSchema,
  localTruthSchema,
} from "./product/local_truth/facade.js";
