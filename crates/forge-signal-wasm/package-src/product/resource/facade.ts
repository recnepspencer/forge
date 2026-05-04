import { createCollectionFamily } from "./families/collection_family.js";
import { createDetailFamily } from "./families/detail_family.js";
import { nextResourceFamilyId } from "./families/family_id_sequence.js";
import { createPagedFamily } from "./families/paged_family.js";
import { resourceParamIdentity } from "./params/param_identity_factory.js";
import { resourceParams } from "./params/declared_resource_params.js";
import { resourcePolicyProfiles } from "./policies/resource_policy_profiles.js";
import { resourceProcessingJob } from "./processing/resource_processing_job.js";
import { resourceProcessingResult } from "./processing/processing_result.js";
import { resourceCollectionShape } from "./reconciliation/resource_collection_shape.js";
import { resourceItemAspects } from "./reconciliation/resource_item_aspects.js";
import { resourcePatch } from "./reconciliation/resource_patch.js";
import { resourceValueSummaries } from "./reconciliation/resource_value_summaries.js";
import { resourceUploadResult } from "./uploads/upload_result.js";
import { resourceUploadTransport } from "./uploads/resource_upload_transport.js";
import { resourceAuth } from "./requests/resource_auth.js";
import { resourceContinuation } from "./requests/resource_continuation.js";
import { resourceRequestContext } from "./requests/request_context.js";

function createResourceNamespace(signalNamespace, rawSignals) {
  return Object.freeze({
    detail(declaration) {
      return createDetailFamily(
        signalNamespace,
        nextResourceFamilyId(rawSignals, "detail"),
        declaration,
      );
    },
    collection(declaration) {
      return createCollectionFamily(
        signalNamespace,
        nextResourceFamilyId(rawSignals, "collection"),
        declaration,
      );
    },
    paged(declaration) {
      return createPagedFamily(
        signalNamespace,
        nextResourceFamilyId(rawSignals, "paged"),
        declaration,
      );
    },
  });
}

export {
  createResourceNamespace,
  resourceAuth,
  resourceCollectionShape,
  resourceContinuation,
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
};
