import {
  createSignals,
  type ComputedSpec,
  type InputSignalHandle,
  type OutputSpec,
  type Signal,
} from "./index.js";

const signals = createSignals();

const count: InputSignalHandle<number> = signals.input(1, { id: "count" });
const next: number = count();
const alsoNext: number = count.get();
const commit = count.set(next + alsoNext);

const doubledSpec: ComputedSpec = {
  reads: ["count"],
  expr: {
    kind: "multiply",
    args: [
      { kind: "read", id: "count" },
      { kind: "value", value: 2 },
    ],
  },
};

const doubled: Signal<number> = signals.computed<number>(doubledSpec, { id: "doubled" });
const doubledFromCallback: Signal<number> = signals.computed<number>(
  "doubledCallback",
  () => count() * 2,
);
const constantFromCallback: Signal<number> = signals.computed<number>(
  "constantCallback",
  () => 2,
);
const generatedFromCallback: Signal<number> = signals.computed<number>(() => 3, { id: "three" });
const legacyDoubledFromSpecAlias: Signal<number> = signals.computedSpec<number>(
  "legacyDoubled",
  doubledSpec,
);

const panelSpec: OutputSpec = {
  reads: ["count", "doubled"],
  expr: {
    kind: "object",
    fields: [
      ["count", { kind: "read", id: "count" }],
      ["doubled", { kind: "read", id: "doubled" }],
    ],
  },
};

const panel = signals.output<{ count: number; doubled: number }>(panelSpec, { id: "panel" });
const legacyPanelFromSpecAlias = signals.outputSpec<{ count: number; doubled: number }>(
  "legacyPanel",
  panelSpec,
);
const snapshot = panel();
const panelSnapshotFromRead = signals.read<{ count: number; doubled: number }>(panel);
const countSnapshotFromRead = signals.read<number>(count);
const callbackPanel = signals.output<{ count: number; doubled: number }>(() => ({
  count: count(),
  doubled: doubled(),
}), { id: "callbackPanel" });
const callbackPanelSnapshot = callbackPanel();
const explicitCallbackPanel = signals.outputCallback<{ count: number; doubled: number }>(
  "callbackPanelExplicit",
  () => snapshot,
);
const adapters = signals.adapters();
const definitions = adapters.exportDefinitions();
const runtimeEnvelope = adapters.exportRuntimeEnvelope();
adapters.replaceRuntimeEnvelope(runtimeEnvelope);
const proof = adapters.runtimeProofReport();
const restoredBranchId = runtimeEnvelope.snapshot.snapshot.meta.branch_id;
const snapshotExplanationRetention =
  runtimeEnvelope.snapshot.snapshot.meta.artifact_retention.explanation_retention;
const checkpointImage = runtimeEnvelope.snapshot.snapshot.checkpoint_image;
const diagnosticGraph = runtimeEnvelope.snapshot.snapshot.diagnostic_graph;
const proofVersion = proof.proofSchemaVersion;
const proofDigest = proof.registryBundleDigest;
const maybeUnavailable = definitions.unavailableCallbacks.map(
  (artifact) => artifact.signalKind,
);
const diagnostics = signals.diagnostics();
const history = signals.history();
const specialist = signals.specialist();
const currentBranch = history.current_branch();
const previewBranch = history.create_branch("preview");
const branchReplay = history.replay_for_branch(currentBranch.id);
const branchSnapshot = history.branch_snapshot(currentBranch.id);
const branchEnvelope = history.branch_snapshot_envelope(currentBranch.id);
history.restore_snapshot(branchEnvelope);
history.restore_branch_snapshot(currentBranch.id, branchSnapshot);
const branchProof = history.branch_state_proof(currentBranch.id);
const parityProof = history.replay_parity_proof(currentBranch.id, currentBranch.id);
const artifactProof = history.replay_artifact_proof({
  proofSchemaVersion: proof.proofSchemaVersion,
  registryBundleDigest: proof.registryBundleDigest,
  loweredStrategyBundleDigest: null,
  mergePlanDigest: null,
  mergeResultDigest: null,
  lineageDigest: null,
  branchStateDigest: branchProof.stateDigest,
}, currentBranch.id);
const previewPlan = history.plan_merge_policy_preview({
  source_branch_id: previewBranch.id,
  target_branch_id: currentBranch.id,
});
const previewPlanProof = history.plan_merge_policy_preview_with_proof({
  source_branch_id: previewBranch.id,
  target_branch_id: currentBranch.id,
});
const previewResult = history.merge_branches_policy_preview({
  source_branch_id: previewBranch.id,
  target_branch_id: currentBranch.id,
});
const previewResultProof = history.merge_branches_policy_preview_with_proof({
  source_branch_id: previewBranch.id,
  target_branch_id: currentBranch.id,
});
const graphSummary = diagnostics.summaryNow();
const specialistGraphSummary = specialist.graphSummary();
const specialistEvaluateDirty = specialist.evaluateDirty();
const latestFlow = diagnostics.latestFlow();
const latestObservation = diagnostics.latestObservation();
const latestFailure = diagnostics.latestFailure();
const latestFrontierExecution = diagnostics.latestFrontierExecution();
const recentHistory = diagnostics.recentHistory();

const callbackNodeIds =
  latestFlow?.callbackNodes.map((node) => node.id) ??
  latestObservation?.callbackNodes.map((node) => node.id) ??
  [];
const latestHistoryNode = recentHistory[0]?.nodes[0]?.node ?? null;
const graphProfile = graphSummary.profile;
const specialistGraphProfile = specialistGraphSummary.profile;
const specialistTouchedNodes = specialistEvaluateDirty.touchedNodes;
const latestFailureMessage = latestFailure?.message ?? null;
const latestFrontierSeedCount = latestFrontierExecution?.seed_count ?? 0;
const branchReplayCallback = branchReplay.frames[0]?.callback?.registered ?? null;
const branchSnapshotBranchId = branchSnapshot.meta.branch_id;
const branchEnvelopeSnapshotId = branchEnvelope.snapshot.meta.snapshot_id;
const parityMismatchCount = parityProof.mismatchClasses.length;
const artifactParity = artifactProof.parity;
const previewPlanSource = previewPlan.source_branch_id;
const previewPlanStrategy = previewPlan.selected_semantics.strategy_name;
const previewPlanResolution = previewPlan.resolution_plan?.divergence ?? null;
const previewPlanNodeMapEntry = previewPlan.node_map[0]?.source_node ?? null;
const previewPlanDecision = previewPlan.node_plan[0]?.decision ?? null;
const previewPlanAdoptionSource = previewPlan.adoption_core[0]?.source_node ?? null;
const previewPlanCarryPolicy = previewPlan.adoption_policy[0]?.runtime_artifact ?? null;
const previewPlanDigest = previewPlanProof.proof.planDigest;
const previewResultTarget = previewResult.target_branch;
const previewResultRecordNode = previewResult.records[0]?.source_node ?? null;
const previewResultCounter = previewResult.counters.replay_event_count;
const previewResultDigest = previewResultProof.proof.resultDigest;

signals.transaction((tx) => {
  tx.set(count, snapshot.count + commit.touchedNodes);
  // @ts-expect-error computed handles must stay read-only inside transactions
  tx.set(doubled, 4);
});

// @ts-expect-error branded callable handles must not accept structural forgeries
const forgedSignal: InputSignalHandle<number> = {
  id: "forged",
  get() {
    return 1;
  },
  set() {
    return commit;
  },
};

void constantFromCallback;
void doubledFromCallback;
void generatedFromCallback;
void legacyDoubledFromSpecAlias;
void legacyPanelFromSpecAlias;
void callbackPanelSnapshot;
void explicitCallbackPanel;
void panelSnapshotFromRead;
void countSnapshotFromRead;
void definitions;
void runtimeEnvelope;
void restoredBranchId;
void snapshotExplanationRetention;
void checkpointImage;
void diagnosticGraph;
void maybeUnavailable;
void proof;
void proofVersion;
void proofDigest;
void diagnostics;
void history;
void specialist;
void currentBranch;
void previewBranch;
void branchReplay;
void branchSnapshot;
void branchEnvelope;
void branchProof;
void parityProof;
void artifactProof;
void previewPlan;
void previewPlanProof;
void previewResult;
void previewResultProof;
void graphProfile;
void specialistGraphProfile;
void specialistTouchedNodes;
void callbackNodeIds;
void latestHistoryNode;
void latestFailureMessage;
void latestFrontierSeedCount;
void branchReplayCallback;
void branchSnapshotBranchId;
void branchEnvelopeSnapshotId;
void parityMismatchCount;
void artifactParity;
void previewPlanSource;
void previewPlanStrategy;
void previewPlanResolution;
void previewPlanNodeMapEntry;
void previewPlanDecision;
void previewPlanAdoptionSource;
void previewPlanCarryPolicy;
void previewPlanDigest;
void previewResultTarget;
void previewResultRecordNode;
void previewResultCounter;
void previewResultDigest;
void forgedSignal;
