import {
  branchWithHead,
  cloneAuthorityState,
  createCommit,
  createSnapshot,
  locusKey,
  boundLocalTruthReviews,
  requireCurrentBasis,
} from "../authority/authority_state.js";
import { aspectsEquivalent, materializeAspect } from "../schema/schema_declaration.js";
import { canonicalDigest, deepFreeze, immutableClone } from "../support/canonical.js";
import { advisory, denied, failed, success } from "../support/outcomes.js";
import { sourceIntegrationKey } from "./merge_basis.js";
import { admitReview } from "./merge_review.js";

const LOWERED_PLANS = new WeakSet();
const STAGED_MERGES = new WeakSet();

export function resolveAndCommitLocalTruthMerge(state, schema, request, faultInjector) {
  try {
    const duplicate = findDuplicateMergeRequest(state, request);
    if (duplicate) return { state, outcome: duplicate };
    const reviewOutcome = admitReview(state, request?.reviewId);
    if (reviewOutcome.posture !== "success") {
      return { state, outcome: reviewOutcome };
    }
    const plan = lowerMergePlan(state, schema, reviewOutcome.value, request);
    inject(faultInjector, "mergeReconstruction");
    const staged = stageMerge(state, schema, plan, faultInjector);
    inject(faultInjector, "mergePublication");
    const next = publishMerge(state, staged);
    return { state: next, outcome: success(deepFreeze({
      artifactFamily: "CommittedLocalTruthMerge",
      commit: staged.commit,
      decisions: staged.commit.decisions,
      targetBasis: next.branches.get(plan.targetBranchId).basis,
      counters: staged.commit.counters,
      retiredResolutionBranchIds: plan.resolutionBranchIds,
    })) };
  } catch (error) {
    if (error?.localTruthDenial) {
      return { state, outcome: denied(error.code, error.message) };
    }
    return {
      state,
      outcome: failed("localTruthMergeFailed", error instanceof Error ? error.message : String(error)),
    };
  }
}

function lowerMergePlan(state, schema, review, request) {
  const selections = normalizeSelections(review, request?.selections ?? []);
  const decisions = review.classifications.map((classification) => resolveDecision(
    state,
    schema,
    review,
    classification,
    selections.get(classification.conflict?.id),
  ));
  const targetSnapshot = state.snapshots.get(review.targetBasis.snapshotId);
  const operations = decisions
    .filter((decision) => decision.selection !== "target" && decision.selection !== "unchanged")
    .filter((decision) => !aspectsEquivalent(
      schema,
      decision.aspectId,
      targetSnapshot.values[decision.entityId][schema.aspects.find((aspect) => aspect.id === decision.aspectId).field],
      decision.value,
    ))
    .map((decision) => deepFreeze({
      entityId: decision.entityId,
      aspectId: decision.aspectId,
      after: decision.value,
      evidenceDigest: canonicalDigest(decision),
    }));
  const lineageUpdates = decisions.map((decision) => deepFreeze({
    sourceBranchId: review.sourceBasis.branchId,
    entityId: decision.entityId,
    aspectId: decision.aspectId,
    sourceCommitId: decision.sourceLocusCommitId,
    sourceValue: decision.sourceValue,
  }));
  const plan = deepFreeze({
    artifactFamily: "LoweredLocalTruthMergePlan",
    authorityId: state.authorityId,
    schemaIdentity: state.schemaIdentity,
    reviewId: review.id,
    sourceBranchId: review.sourceBasis.branchId,
    targetBranchId: review.targetBasis.branchId,
    expectedSourceBasis: review.sourceBasis,
    expectedTargetBasis: review.targetBasis,
    policyIdentity: review.policy.identity,
    requestId: requireRequestId(request?.requestId),
    requestDigest: mergeRequestDigest(request),
    operations,
    decisions,
    lineageUpdates,
    resolutionBranchIds: [...state.resolutionAdmissions.values()]
      .filter((admission) => admission.reviewId === review.id)
      .map((admission) => admission.branch.id)
      .sort(),
    counters: {
      lociPlanned: decisions.length,
      targetReplacements: operations.length,
      targetPreserved: decisions.length - operations.length,
      lineageUpdates: lineageUpdates.length,
    },
  });
  LOWERED_PLANS.add(plan);
  return plan;
}

function stageMerge(state, schema, plan, faultInjector) {
  if (!LOWERED_PLANS.has(plan)) {
    throw new TypeError("merge staging requires a sealed lowered plan");
  }
  validatePlanBases(state, plan);
  const target = state.branches.get(plan.targetBranchId);
  const before = state.snapshots.get(target.snapshotId);
  const values = { ...before.values };
  for (const [index, operation] of plan.operations.entries()) {
    inject(faultInjector, `mergeReconstruction:${index}`);
    values[operation.entityId] = materializeAspect(
      schema,
      values[operation.entityId],
      operation.aspectId,
      operation.after,
    );
  }
  const snapshot = createSnapshot(state.authorityId, schema.identity, values);
  inject(faultInjector, "mergeDigesting");
  const commit = createCommit({
    artifactFamily: "LocalTruthCommit",
    authorityId: state.authorityId,
    authorityKind: state.authorityKind,
    schemaIdentity: state.schemaIdentity,
    branchId: target.id,
    parentCommitId: target.headCommitId,
    beforeSnapshotId: before.id,
    afterSnapshotId: snapshot.id,
    kind: "merge",
    requestId: plan.requestId,
    sourceBranchId: plan.sourceBranchId,
    sourceHeadCommitId: plan.expectedSourceBasis.headCommitId,
    reviewId: plan.reviewId,
    policyIdentity: plan.policyIdentity,
    operations: plan.operations,
    lineageUpdates: plan.lineageUpdates,
    decisions: plan.decisions,
    counters: { committedLoci: plan.operations.length, ...plan.counters },
  });
  const staged = deepFreeze({ artifactFamily: "StagedLocalTruthMerge", plan, snapshot, commit });
  STAGED_MERGES.add(staged);
  return staged;
}

function publishMerge(state, staged) {
  if (!STAGED_MERGES.has(staged)) {
    throw new TypeError("merge publication requires authority-staged work");
  }
  validatePlanBases(state, staged.plan);
  const next = cloneAuthorityState(state);
  const revision = state.revision + 1;
  const target = state.branches.get(staged.plan.targetBranchId);
  next.revision = revision;
  next.snapshots.set(staged.snapshot.id, staged.snapshot);
  next.commits.set(staged.commit.id, staged.commit);
  next.branches.set(target.id, branchWithHead(target, staged.commit, staged.snapshot, revision));
  const lineage = next.lineageByBranch.get(target.id);
  const locusHeads = next.locusHeadByBranch.get(target.id);
  for (const update of staged.plan.lineageUpdates) {
    const locus = locusKey(update.entityId, update.aspectId);
    lineage.set(sourceIntegrationKey(update.sourceBranchId, locus), deepFreeze({
      sourceCommitId: update.sourceCommitId,
      sourceValue: immutableClone(update.sourceValue),
    }));
  }
  for (const operation of staged.plan.operations) {
    locusHeads.set(locusKey(operation.entityId, operation.aspectId), staged.commit.id);
  }
  for (const branchId of staged.plan.resolutionBranchIds) {
    const branch = next.branches.get(branchId);
    next.branches.set(branchId, deepFreeze({ ...branch, retired: true }));
  }
  const retiredResolutionBranchIds = new Set(staged.plan.resolutionBranchIds);
  next.resolutionAdmissions = new Map(
    [...next.resolutionAdmissions].filter(([branchId]) => !retiredResolutionBranchIds.has(branchId)),
  );
  next.customAlternatives = new Map(
    [...next.customAlternatives].filter(([, alternative]) => (
      !retiredResolutionBranchIds.has(alternative.resolutionBranchId)
    )),
  );
  next.decisionLog.push(...staged.plan.decisions);
  next.counters.commits += 1;
  next.counters.merges += 1;
  next.requestLog.set(staged.plan.requestId, deepFreeze({
    requestDigest: staged.plan.requestDigest,
    commit: staged.commit,
  }));
  return boundLocalTruthReviews(next);
}

function findDuplicateMergeRequest(state, request) {
  if (!request || typeof request.requestId !== "string") return null;
  const prior = state.requestLog.get(request.requestId);
  if (!prior) return null;
  if (prior.requestDigest !== mergeRequestDigest(request)) {
    return denied("requestIdentityReuse", `requestId ${request.requestId} was already used for different local truth work`);
  }
  return advisory(prior.commit, [{ code: "duplicateRequest", message: "existing commit returned" }]);
}

function mergeRequestDigest(request) {
  return canonicalDigest({
    reviewId: request?.reviewId,
    selections: Array.isArray(request?.selections)
      ? request.selections.map((selection) => ({
        conflictId: selection.conflictId,
        alternativeId: selection.alternativeId,
      })).sort((left, right) => left.conflictId.localeCompare(right.conflictId))
      : request?.selections,
  });
}

function resolveDecision(state, _schema, review, classification, selection) {
  const delta = classification;
  if (classification.kind === "ResolutionRequired") {
    return resolveConflictDecision(state, classification.conflict, delta, selection);
  }
  const sourceSelection = classification.kind === "AdoptSource";
  return deepFreeze({
    artifactFamily: "ResolvedLocalTruthMergeDecision",
    entityId: classification.entityId,
    aspectId: classification.aspectId,
    classification: classification.kind,
    selection: sourceSelection ? "source" : classification.kind === "Unchanged" ? "unchanged" : "target",
    selectionBasis: classification.selectionBasis,
    value: immutableClone(sourceSelection ? classification.sourceValue : classification.targetValue),
    sourceLocusCommitId: classification.sourceLocusCommitId,
    sourceValue: immutableClone(classification.sourceValue),
    resolutionBranchId: null,
  });
}

function resolveConflictDecision(state, conflict, delta, selection) {
  if (!selection) {
    throw denial("incompleteLocalTruthResolution", `conflict ${conflict.id} has no selection`);
  }
  const builtIn = conflict.alternatives.find((alternative) => alternative.id === selection.alternativeId);
  if (builtIn) {
    return decisionFromAlternative(conflict, delta, builtIn, null);
  }
  const custom = state.customAlternatives.get(selection.alternativeId);
  const admission = custom && state.resolutionAdmissions.get(custom.resolutionBranchId);
  const branch = admission && state.branches.get(admission.branch.id);
  if (
    !custom
    || custom.conflictId !== conflict.id
    || admission.reviewId !== selection.reviewId
    || !branch
    || branch.retired
    || branch.basis.identityDigest !== custom.resolutionBasis.identityDigest
  ) {
    throw denial("staleCustomResolution", `custom resolution for conflict ${conflict.id} is stale or ineligible`);
  }
  const alternative = {
    choice: "custom",
    value: custom.value,
  };
  return decisionFromAlternative(conflict, delta, alternative, branch.id);
}

function decisionFromAlternative(conflict, delta, alternative, resolutionBranchId) {
  return deepFreeze({
    artifactFamily: "ResolvedLocalTruthMergeDecision",
    entityId: conflict.entityId,
    aspectId: conflict.aspectId,
    classification: "ResolutionRequired",
    selection: alternative.choice,
    selectionBasis: alternative.choice === "custom" ? "resolutionBranchCommit" : "manualReview",
    value: immutableClone(alternative.value),
    sourceLocusCommitId: delta.sourceLocusCommitId,
    sourceValue: immutableClone(delta.sourceValue),
    resolutionBranchId,
  });
}

function normalizeSelections(review, selections) {
  if (!Array.isArray(selections)) {
    throw denial("invalidLocalTruthResolution", "merge selections must be an array");
  }
  const byConflict = new Map();
  for (const selection of selections) {
    if (!selection || selection.reviewId !== review.id || byConflict.has(selection.conflictId)) {
      throw denial("invalidLocalTruthResolution", "merge selections are duplicate or cross-review");
    }
    byConflict.set(selection.conflictId, selection);
  }
  const expected = new Set(review.conflicts.map((conflict) => conflict.id));
  if (byConflict.size !== expected.size || [...byConflict.keys()].some((id) => !expected.has(id))) {
    throw denial("incompleteLocalTruthResolution", "merge selections must cover every conflict exactly once");
  }
  return byConflict;
}

function validatePlanBases(state, plan) {
  const source = state.branches.get(plan.sourceBranchId);
  const target = state.branches.get(plan.targetBranchId);
  if (!source || !target
    || !requireCurrentBasis(state, source, plan.expectedSourceBasis).ok
    || !requireCurrentBasis(state, target, plan.expectedTargetBasis).ok) {
    throw denial("staleLocalTruthMergePlan", "source or target advanced before merge publication");
  }
}

function requireRequestId(value) {
  if (typeof value !== "string" || value.trim() === "") {
    throw new TypeError("local truth merge requestId must be a non-empty string");
  }
  return value;
}

function denial(code, message) {
  return Object.assign(new Error(message), {
    localTruthDenial: true,
    code,
  });
}

function inject(faultInjector, point) {
  faultInjector?.(point);
}
