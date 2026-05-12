import {
  createSignals,
  resourceEffects,
  resourceParamIdentity,
  resourceParams,
  type ResourceEffectCloseoutMatrix,
  type ResourceEffectCloseoutMatrixRow,
  type ResourceEffectProfile,
  type ResourceEffectMergeExecutionResult,
  type ResourceEffectMergePlanResult,
} from "../../../index.js";

const signals = createSignals();

const scopedApi = signals.api({
  effects: resourceEffects.branchNative(),
}).scope({
  effects: ({ workspaceId }: { workspaceId: string }) =>
    workspaceId === "sensitive"
      ? resourceEffects.sensitive()
      : resourceEffects.serverCanonical(),
});

const directResourceDetail = signals.resource.detail({
  params: resourceParams<{ id: string }>(),
  effects: resourceEffects.serverCanonical(),
  normalizeParams: ({ id }) => resourceParamIdentity({ id }, id),
  load: ({ id }) => ({ id }),
});

const taskDetail = scopedApi.url("/workspaces/:workspaceId/tasks/:taskId")
  .effects(resourceEffects.pessimistic())
  .detail({
    load: ({ workspaceId, taskId }) => ({
      id: taskId,
      workspaceId,
    }),
  });

const taskLine = taskDetail.line({
  workspaceId: "demo",
  taskId: "t1",
});

const lineEffects: ResourceEffectProfile | null = taskLine.request().effects;
const diagnosticsEffects: ResourceEffectProfile | null =
  taskLine.diagnostics().request.effects;

void lineEffects;
void diagnosticsEffects;
void signals.resource.effects.deliveryAuthoritative();
void signals.resource.branch.planMerge({
  source_branch_id: 0,
  target_branch_id: 0,
});
void directResourceDetail.line({ id: "direct" }).request().effects;

const branchNativeCloseout: ResourceEffectCloseoutMatrix =
  signals.resource.effects.closeoutMatrix(resourceEffects.branchNative());
const mergeRebaseRow:
  | ResourceEffectCloseoutMatrixRow
  | undefined = branchNativeCloseout.rows.find(
    (row) => row.effectFamily === "mergeRebase",
  );

void branchNativeCloseout.proofLanes.includes("branchMerge");
void mergeRebaseRow?.performanceProof;
void mergeRebaseRow?.evidence.branchMerge.includes(
  "resource_branch_effect_merge_execution.test.mjs",
);

function consumeResourceMergePlan(result: ResourceEffectMergePlanResult): string {
  if (result.kind === "denied") {
    return result.reason;
  }
  const artifact = result.resourceEffect.rebaseArtifact;
  if (artifact.kind === "mappingUnavailable") {
    return artifact.native.records[0]?.sourceNode ?? artifact.reason;
  }
  if (artifact.kind === "conflict") {
    return artifact.conflicts[0]?.resource.effectLocus ?? artifact.kind;
  }
  return artifact.proof.nativeMergePlanDigest;
}

function consumeResourceMergeExecution(
  result: ResourceEffectMergeExecutionResult,
): string {
  if (result.kind === "denied") {
    return result.reason;
  }
  const artifact = result.resourceEffect.mergeArtifact;
  if (artifact.kind === "mappingUnavailable") {
    return artifact.resource.effectId;
  }
  if (artifact.kind === "mergedWithConflictRecords") {
    return artifact.conflicts[0]?.proof.nativeMergeResultDigest ?? artifact.kind;
  }
  return artifact.proof.nativeMergeLineageDigest;
}

void consumeResourceMergePlan;
void consumeResourceMergeExecution;
