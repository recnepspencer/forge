import {
  boundLocalTruthReviews,
  requireCurrentBasis,
} from "../authority/authority_state.js";
import { forkLocalTruthBranch } from "../history/branch_history.js";
import { extractAspect } from "../schema/schema_declaration.js";
import { canonicalDigest, deepFreeze } from "../support/canonical.js";
import { denied, reviewRequired, success } from "../support/outcomes.js";
import { classifyLocalTruthDeltas } from "./conflict_classification.js";
import { resolveLocalTruthMergeBasis } from "./merge_basis.js";

export function previewLocalTruthMerge(state, schema, request) {
  const basisOutcome = resolveLocalTruthMergeBasis(state, schema, request);
  if (basisOutcome.posture !== "success") {
    return { state, outcome: basisOutcome };
  }
  let classification;
  try {
    classification = classifyLocalTruthDeltas(schema, basisOutcome.value, request?.policy);
  } catch (error) {
    return {
      state,
      outcome: denied("localTruthMergePolicyDenied", error instanceof Error ? error.message : String(error)),
    };
  }
  const reviewIdentity = {
    authorityId: state.authorityId,
    schemaIdentity: state.schemaIdentity,
    basis: basisOutcome.value.identityDigest,
    classification: classification.digest,
  };
  const id = `truth-review:${canonicalDigest(reviewIdentity)}`;
  const review = deepFreeze({
    artifactFamily: "LocalTruthMergeReview",
    id,
    authorityId: state.authorityId,
    schemaIdentity: state.schemaIdentity,
    sourceBasis: basisOutcome.value.sourceBasis,
    targetBasis: basisOutcome.value.targetBasis,
    structuralAncestorCommitId: basisOutcome.value.structuralAncestorCommitId,
    scope: basisOutcome.value.scope,
    policy: classification.policy,
    classifications: classification.classifications,
    conflicts: classification.conflicts,
    expiryPosture: "headsSchemaAndPolicyBound",
    counters: { ...basisOutcome.value.counters, ...classification.counters },
    digest: canonicalDigest({ id, reviewIdentity, classification }),
  });
  const next = { ...state, reviews: new Map(state.reviews) };
  next.reviews.set(review.id, review);
  boundLocalTruthReviews(next);
  const outcome = review.conflicts.length > 0 ? reviewRequired(review) : success(review);
  return { state: next, outcome };
}

export function createLocalTruthResolutionBranch(state, request) {
  const admitted = admitReview(state, request?.reviewId);
  if (admitted.posture !== "success") {
    return { state, outcome: admitted };
  }
  const review = admitted.value;
  const conflict = review.conflicts.find((candidate) => candidate.id === request?.conflictId);
  if (!conflict) {
    return { state, outcome: denied("unknownLocalTruthConflict", "review does not contain that conflict") };
  }
  const fork = forkLocalTruthBranch(state, {
    parentBranchId: review.targetBasis.branchId,
    expectedParentBasis: review.targetBasis,
    name: request.name ?? `resolution-${conflict.aspectId}`,
    kind: "resolution",
  });
  if (fork.outcome.posture !== "success") {
    return fork;
  }
  const next = { ...fork.state, resolutionAdmissions: new Map(fork.state.resolutionAdmissions) };
  const admission = deepFreeze({
    artifactFamily: "LocalTruthResolutionBranchReceipt",
    reviewId: review.id,
    conflictId: conflict.id,
    entityId: conflict.entityId,
    aspectId: conflict.aspectId,
    branch: fork.outcome.value,
    targetBasis: review.targetBasis,
  });
  next.resolutionAdmissions.set(admission.branch.id, admission);
  return { state: next, outcome: success(admission) };
}

export function issueCustomResolutionAlternative(state, schema, request) {
  const admission = state.resolutionAdmissions.get(request?.resolutionBranchId);
  if (!admission || admission.reviewId !== request?.reviewId || admission.conflictId !== request?.conflictId) {
    return { state, outcome: denied("ineligibleResolutionBranch", "resolution branch is not admitted for this review conflict") };
  }
  const branch = state.branches.get(admission.branch.id);
  if (!branch || branch.retired || branch.headCommitId === admission.branch.headCommitId) {
    return { state, outcome: denied("uncommittedResolutionBranch", "resolution branch must author one committed custom value") };
  }
  const snapshot = state.snapshots.get(branch.snapshotId);
  const value = extractAspect(schema, snapshot.values[admission.entityId], admission.aspectId);
  const alternative = deepFreeze({
    artifactFamily: "LocalTruthConflictAlternative",
    id: `truth-alternative:${canonicalDigest({
      conflictId: admission.conflictId,
      choice: "custom",
      resolutionBranchId: branch.id,
      resolutionHeadCommitId: branch.headCommitId,
      value,
    })}`,
    choice: "custom",
    conflictId: admission.conflictId,
    resolutionBranchId: branch.id,
    resolutionBasis: branch.basis,
    value,
  });
  const next = { ...state, customAlternatives: new Map(state.customAlternatives) };
  next.customAlternatives.set(alternative.id, alternative);
  return { state: next, outcome: success(alternative) };
}

export function validateResolutionBranchMutation(state, request) {
  const branch = state.branches.get(request?.branchId);
  if (!branch || branch.kind !== "resolution") {
    return success(null);
  }
  const admission = state.resolutionAdmissions.get(branch.id);
  if (!admission) {
    return denied("unregisteredResolutionBranch", "resolution branch has no review admission");
  }
  if (branch.headCommitId !== admission.branch.headCommitId) {
    return denied("resolutionBranchAlreadyAuthored", "resolution branch accepts exactly one custom aspect commit");
  }
  if (!Array.isArray(request.operations) || request.operations.length !== 1) {
    return denied("resolutionMutationScope", "resolution branch commit must touch exactly one reviewed locus");
  }
  const [operation] = request.operations;
  if (operation.entityId !== admission.entityId || operation.aspectId !== admission.aspectId) {
    return denied("resolutionMutationScope", "resolution branch commit touched an out-of-review locus");
  }
  return success(admission);
}

export function admitReview(state, reviewId) {
  const review = state.reviews.get(reviewId);
  if (!review) {
    return denied("unknownLocalTruthReview", `review ${String(reviewId)} is unavailable`);
  }
  const source = state.branches.get(review.sourceBasis.branchId);
  const target = state.branches.get(review.targetBasis.branchId);
  const sourceCheck = source && requireCurrentBasis(state, source, review.sourceBasis);
  const targetCheck = target && requireCurrentBasis(state, target, review.targetBasis);
  if (!sourceCheck?.ok || !targetCheck?.ok) {
    return denied("staleLocalTruthReview", "source or target advanced after merge preview");
  }
  return success(review);
}
