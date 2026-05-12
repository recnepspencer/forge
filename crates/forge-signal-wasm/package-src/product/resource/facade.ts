import { createCollectionFamily } from "./families/collection_family.js";
import { createResourceCompatibilityNamespace } from "./compatibility/resource_compatibility_namespace.js";
import { createResourceBranchNamespace } from "./branch/resource_branch_capabilities.js";
import { createDetailFamily } from "./families/detail_family.js";
import { resourceDelivery } from "./delivery/resource_delivery.js";
import { nextResourceFamilyId } from "./families/family_id_sequence.js";
import { createPagedFamily } from "./families/paged_family.js";
import { resourceBinaryDescriptor } from "./downloads/resource_binary_descriptor.js";
import { resourceBinaryValue } from "./downloads/resource_binary_value.js";
import { resourceDownload } from "./downloads/resource_download.js";
import { resourceParamIdentity } from "./params/param_identity_factory.js";
import { resourceParams } from "./params/declared_resource_params.js";
import { resourcePolicyProfiles } from "./policies/resource_policy_profiles.js";
import { resourceProcessingJob } from "./processing/resource_processing_job.js";
import { resourceProcessingResult } from "./processing/processing_result.js";
import { resourceResponse } from "./response/resource_response_contract.js";
import { resourceEffects } from "./effects/resource_effect_profile.js";
import { resourceCollectionShape } from "./reconciliation/resource_collection_shape.js";
import { resourceDetailFields } from "./reconciliation/resource_detail_fields.js";
import { resourceDetailRegions } from "./reconciliation/resource_detail_regions.js";
import { resourceDetailJsonPaths } from "./reconciliation/resource_detail_json_paths.js";
import { resourceItemAspects } from "./reconciliation/resource_item_aspects.js";
import { resourcePatch } from "./reconciliation/resource_patch.js";
import { resourceValueSummaries } from "./reconciliation/resource_value_summaries.js";
import { resourceUploadResult } from "./uploads/upload_result.js";
import { resourceUploadTransport } from "./uploads/resource_upload_transport.js";
import { resourceAuth } from "./requests/resource_auth.js";
import { resourceContinuation } from "./requests/resource_continuation.js";
import { resourceRequestContext } from "./requests/request_context.js";
import { createResourceLineEpoch } from "./lines/state/resource_line_epoch.js";

function createResourceNamespace(signalNamespace, rawSignals) {
  const resourceLineEpoch = createResourceLineEpoch();
  return Object.freeze({
    compatibility: createResourceCompatibilityNamespace(
      signalNamespace,
      rawSignals,
      resourceLineEpoch,
    ),
    branch: createResourceBranchNamespace(rawSignals),
    detail(declaration) {
      return createDetailFamily(
        signalNamespace,
        resourceLineEpoch,
        nextResourceFamilyId(rawSignals, "detail"),
        declaration,
      );
    },
    collection(declaration) {
      return createCollectionFamily(
        signalNamespace,
        resourceLineEpoch,
        nextResourceFamilyId(rawSignals, "collection"),
        declaration,
      );
    },
    paged(declaration) {
      return createPagedFamily(
        signalNamespace,
        resourceLineEpoch,
        nextResourceFamilyId(rawSignals, "paged"),
        declaration,
      );
    },
    effects: resourceEffects,
    detailFields: resourceDetailFields,
    detailRegions: resourceDetailRegions,
    detailJsonPaths: resourceDetailJsonPaths,
    response: resourceResponse,
  });
}

export {
  createResourceNamespace,
  resourceBinaryDescriptor,
  resourceBinaryValue,
  resourceAuth,
  resourceCollectionShape,
  resourceDetailFields,
  resourceDetailRegions,
  resourceDetailJsonPaths,
  resourceContinuation,
  resourceEffects,
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
  resourceResponse,
};
