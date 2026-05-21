import { createResourceBranchNamespace } from "../resource/branch/resource_branch_capabilities.js";
import { resourceExternalDelivery } from "../resource/compatibility/resource_external_delivery.js";
import { resourceEffects } from "../resource/effects/resource_effect_profile.js";
import { resourceMutationResponses } from "../resource/mutation/resource_mutation_response_closeout_matrix.js";
import { resourceDetailFields } from "../resource/reconciliation/resource_detail_fields.js";
import { resourceDetailJsonPaths } from "../resource/reconciliation/resource_detail_json_paths.js";
import { resourceDetailRegions } from "../resource/reconciliation/resource_detail_regions.js";
import { resourceResponse } from "../resource/response/resource_response_contract.js";
import { freezeObject } from "../graph_support.js";
import { createRootHistoryFacade } from "./worker_first_root_history.js";

export function createWorkerFirstResourceNamespace(rootSession) {
  const rawSignals = freezeObject({
    history() {
      return createRootHistoryFacade(rootSession);
    },
  });

  const compatibility = freezeObject({
    delivery: resourceExternalDelivery,
    detail() {
      throwWorkerFirstResourceUnavailable("signals.resource.compatibility.detail");
    },
    collection() {
      throwWorkerFirstResourceUnavailable("signals.resource.compatibility.collection");
    },
    paged() {
      throwWorkerFirstResourceUnavailable("signals.resource.compatibility.paged");
    },
  });

  return freezeObject({
    branch: createResourceBranchNamespace(rawSignals),
    compatibility,
    effects: resourceEffects,
    mutationResponses: resourceMutationResponses,
    detailFields: resourceDetailFields,
    detailRegions: resourceDetailRegions,
    detailJsonPaths: resourceDetailJsonPaths,
    response: resourceResponse,
    detail() {
      throwWorkerFirstResourceUnavailable("signals.resource.detail");
    },
    collection() {
      throwWorkerFirstResourceUnavailable("signals.resource.collection");
    },
    paged() {
      throwWorkerFirstResourceUnavailable("signals.resource.paged");
    },
  });
}

function throwWorkerFirstResourceUnavailable(operation) {
  const error = new Error(
    `${operation} is unavailable on the current worker-first resource surface because resource family materialization still depends on synchronous root signal creation; use deployment: "mainThreadCompatibility" for resource family construction`,
  );
  error.name = "WorkerFirstResourceSurfaceUnavailable";
  error.code = "workerFirstResourceSurfaceUnavailable";
  error.compatibilityRecovery = freezeObject({
    deployment: "mainThreadCompatibility",
    message:
      'Retry with deployment: "mainThreadCompatibility" to use resource family constructors.',
  });
  throw error;
}
