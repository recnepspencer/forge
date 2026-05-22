import { freezeObject } from "../graph_support.js";
import { createResourceNamespace } from "../resource/facade.js";

export function createWorkerFirstResourceNamespace(
  signalNamespace,
  rootSession,
) {
  const rawSignals = freezeObject({
    history() {
      return freezeObject({
        current_branch() {
          const branch = rootSession.currentBranchSummary();
          if (branch === null) {
            throw new TypeError(
              "worker-first resource branch history requires current_branch(...) to be available on the worker-owned root",
            );
          }
          return branch;
        },
        branches() {
          return rootSession.branchesSummary();
        },
        plan_merge_policy_preview_with_proof(request) {
          return rootSession.bridge().planMergePolicyPreviewWithProof(request);
        },
        merge_branches_policy_preview_with_proof(request) {
          return rootSession.mergeHistoryBranchesPolicyPreviewWithProof(request);
        },
      });
    },
  });
  return createResourceNamespace(signalNamespace, rawSignals);
}
