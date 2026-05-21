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
          return rootSession.bridge().mergeBranchesPolicyPreviewWithProof(request);
        },
      });
    },
  });
  const namespace = createResourceNamespace(signalNamespace, rawSignals);
  return freezeObject({
    ...namespace,
    branch: createWorkerFirstResourceBranchNamespace(namespace.branch),
  });
}

function createWorkerFirstResourceBranchNamespace(branchNamespace) {
  return freezeObject({
    planMerge(request) {
      return branchNamespace.planMerge(request);
    },
    planEffectMerge(request) {
      return branchNamespace.planEffectMerge(request);
    },
    mergeEffect() {
      return freezeObject({
        kind: "denied",
        reason: "workerFirstResourceBranchEffectMergeUnavailable",
        detail:
          "worker-first resource.branch.mergeEffect(...) remains unavailable for root-authored resource lines; use deployment: \"mainThreadCompatibility\" for branch effect merge execution",
      });
    },
  });
}
