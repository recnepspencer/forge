import { createCollectionFamily } from "./families/collection_family.js";
import { createResourceCompatibilityNamespace } from "./compatibility/resource_compatibility_namespace.js";
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
import { resourceCollectionShape } from "./reconciliation/resource_collection_shape.js";
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
    response: resourceResponse,
  });
}

export {
  createResourceNamespace,
  resourceBinaryDescriptor,
  resourceBinaryValue,
  resourceAuth,
  resourceCollectionShape,
  resourceContinuation,
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
