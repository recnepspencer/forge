/// <reference lib="webworker" />

import {
  buildReplayScenarioProofArtifacts,
  buildScenarioProofArtifacts,
  createSceneRuntime,
  executeMerge,
  executeMergePolicyPreview,
  planMerge,
  planMergePolicyPreview,
  readBranchInspect,
  readBranchInspectForNode,
  renderBranch,
  totalGraphNodes,
  updateScene,
} from "../core/runtime";
import { RENDER_HEIGHT, RENDER_WIDTH } from "../core/types";
import type {
  BranchId,
  BranchInspect,
  DiagnosticsTier,
  MergePlan,
  ScenePatch,
  ScenarioMode,
  ScenarioProofArtifacts,
} from "../core/types";
import type {
  BranchFrame,
  BranchSummary,
  MergeReviewSnapshot,
  ReviewFrame,
  WorkerCommand,
  WorkerEvent,
  WorkerSnapshot,
} from "./protocol";
import {
  branchFrameIds,
  createScenarioState,
  withReplayScenarioProof,
  withScenarioProof,
  withScenarioStatus,
  DEFAULT_SCENARIO_INSPECT_NODES,
  emptyScenarioProof,
  findLastTimelineIndex,
  withScenarioTier,
} from "./scenario";
import {
  buildWorkerSnapshot,
  captureTimeline,
  createSessionFromInitialRender,
  getActiveBranch,
  isEmptyScenePatch,
  rebuildCommitState,
  type CachedBranch,
  type SessionState,
  summarizeBranchState,
  type TimelineState,
  updateBranchCache,
} from "./session";

self.postMessage({ type: "debug", phase: "worker:module-loaded", detail: "worker module evaluated" } satisfies WorkerEvent);

let session: SessionState | null = null;
let booting = false;
let renderLock = false;
let bootStartedAt = 0;

self.onmessage = (event: MessageEvent<WorkerCommand>) => {
  postDebug("worker:message-received", event.data?.type ?? "unknown");
  void handleCommand(event.data);
};

postDebug("worker:handler-attached", "message handler ready");

function postEvent(message: WorkerEvent, transfer: Transferable[] = []) {
  self.postMessage(message, transfer);
}

function postDebug(phase: string, detail?: string) {
  const elapsedMs = bootStartedAt > 0 ? performance.now() - bootStartedAt : undefined;
  postEvent({ type: "debug", phase, detail, elapsedMs });
}

async function handleCommand(command: WorkerCommand) {
  try {
    switch (command.type) {
      case "init":
        await initializeSession();
        break;
      case "branch":
        await withSession(createBranch);
        break;
      case "merge":
        await withSession(mergeActiveBranch);
        break;
      case "runAdversarialMergeScenario":
        await runAdversarialMergeScenario();
        break;
      case "planScenarioMerge":
        await withSession(planScenarioMerge);
        break;
      case "executeScenarioMerge":
        await withSession(executeScenarioMerge);
        break;
      case "replayScenarioMerge":
        await withSession(replayScenarioMerge);
        break;
      case "setScenarioMode":
        await withSession((current) => setScenarioMode(current, command.mode));
        break;
      case "setDiagnosticsTier":
        await withSession((current) => setDiagnosticsTier(current, command.tier));
        break;
      case "activateBranch":
        await withSession((current) => activateBranch(current, command.branchId));
        break;
      case "inspectNode":
        await withSession((current) => inspectNode(current, command.branchId, command.nodeId));
        break;
      case "scrub":
        await withSession((current) => scrubTimeline(current, command.index));
        break;
      case "setScenePatch":
        await withSession((current) => applyScenePatch(current, command.branchId, command.patch, command.label));
        break;
    }
  } catch (error) {
    postEvent({ type: "error", error: formatError(error) });
  }
}

async function initializeSession() {
  if (booting || session) {
    return;
  }

  booting = true;
  bootStartedAt = performance.now();
  try {
    postDebug("boot:start", "worker init received");
    const { runtime } = await createSceneRuntime((phase, detail) => {
      postDebug(phase, detail);
    });
    const initial = await renderBranch(runtime, runtime.history().currentBranch().id, (phase, detail) => {
      postDebug(phase, detail);
    });
    session = createSessionFromInitialRender(runtime, initial);
    session.graphNodes = totalGraphNodes(runtime);
    session.inspect = readBranchInspect(runtime, initial.branchId);
    session.scenario = createScenarioState("manual-gear", "idle", "manual gear mode ready", "webDevelopment");
    applyDiagnosticsTierToRuntime(
      session.runtime,
      session.scenario?.diagnosticsTier ?? "webDevelopment",
    );

    captureTimeline(session, "boot", true);
    emitSnapshot([initial.branchId]);
    postDebug("boot:ready", "worker session ready");
  } finally {
    booting = false;
  }
}

async function createFreshSession(
  progressLabel?: string,
  diagnosticsTier: DiagnosticsTier = "webDevelopment",
): Promise<SessionState> {
  const { runtime } = await createSceneRuntime((phase, detail) => {
    if (progressLabel) {
      postDebug(`${progressLabel}:${phase}`, detail);
    }
  });
  const initial = await renderBranch(runtime, runtime.history().currentBranch().id, (phase, detail) => {
    if (progressLabel) {
      postDebug(`${progressLabel}:${phase}`, detail);
    }
  });
  const fresh: SessionState = createSessionFromInitialRender(runtime, initial);
  fresh.graphNodes = totalGraphNodes(runtime);
  fresh.inspect = readBranchInspect(runtime, initial.branchId);
  fresh.scenario = createScenarioState("manual-gear", "idle", "manual gear mode ready", diagnosticsTier);
  fresh.scenario = withScenarioTier(fresh.scenario, diagnosticsTier, `diagnostics tier set to ${diagnosticsTier}`);
  applyDiagnosticsTierToRuntime(fresh.runtime, diagnosticsTier);
  captureTimeline(fresh, "boot", true);
  return fresh;
}

async function setScenarioMode(
  current: SessionState,
  mode: ScenarioMode,
) {
  current.scenario = {
    ...createScenarioState(
      mode,
      "idle",
      mode === "manual-gear" ? "manual gear mode ready" : "adversarial arena armed",
      current.scenario?.diagnosticsTier ?? "webDevelopment",
    ),
    proof: emptyScenarioProof(),
  };
  current.scenario = withScenarioProof(current.scenario, await buildScenarioProof(current));
  emitSnapshot(current.activeBranchId != null ? [current.activeBranchId] : []);
}

async function setDiagnosticsTier(
  current: SessionState,
  tier: DiagnosticsTier,
) {
  current.runtime.setRuntimePolicy({ preset: tier });
  if (current.activeBranchId != null) {
    const update = await renderBranch(current.runtime, current.activeBranchId, (phase, detail) => {
      postDebug(phase, detail);
    });
    updateBranchCache(
      current,
      update.branchId,
      {
        id: update.branchId,
        name: update.branchName,
        state: update.state,
        hud: update.hud,
      },
      update.frame,
    );
    current.latestSummary = update.summary;
    current.inspect = safeInspect(current.runtime, current.activeBranchId, current.inspectNodeId);
  }
  current.scenario = withScenarioTier(current.scenario, tier, `diagnostics tier set to ${tier}`);
  current.scenario = withScenarioProof(current.scenario, await buildScenarioProof(current));
  emitSnapshot(current.activeBranchId != null ? [current.activeBranchId] : []);
}

async function withSession(fn: (current: SessionState) => Promise<void>) {
  if (!session || renderLock) {
    return;
  }

  renderLock = true;
  try {
    await fn(session);
  } finally {
    renderLock = false;
  }
}

async function runAdversarialMergeScenario() {
  if (renderLock) {
    return;
  }

  renderLock = true;
  try {
    const selectedTier = session?.scenario?.diagnosticsTier ?? "webDevelopment";
    const fresh = await createFreshSession("scenario", selectedTier);
    session = fresh;
    const current = fresh;

    current.scenario = createScenarioState(
      "adversarial-gear-merge",
      "scripted",
      "scenario initialized",
      selectedTier,
    );

    const mainId = current.runtime.history().currentBranch().id;
    await createBranch(current);
    const feature = Array.from(current.branches.values()).find((branch) => branch.summary.name === "what-if");
    if (!feature) {
      throw new Error("Adversarial scenario failed to create feature branch");
    }
    const featureId = feature.summary.id;
    current.scenario.steps.push("forked feature branch from main");

    await applyScenePatch(
      current,
      mainId,
      { gear: { teeth: 20, outerRadius: 0.98, innerRadius: 0.34 } },
      "scenario-main-topology",
    );
    current.scenario.steps.push("main: teeth=20, outerRadius=0.98, innerRadius=0.34");

    await applyScenePatch(
      current,
      mainId,
      { light: { intensity: 1.43 }, gear: { rotation: 0.2 } },
      "scenario-main-render",
    );
    current.scenario.steps.push("main: light=1.43, rotation=0.2");

    await applyScenePatch(
      current,
      featureId,
      { gear: { teeth: 14, outerRadius: 0.9, innerRadius: 0.39 } },
      "scenario-feature-topology",
    );
    current.scenario.steps.push("what-if: teeth=14, outerRadius=0.9, innerRadius=0.39");

    await applyScenePatch(
      current,
      featureId,
      { light: { intensity: 0.93 }, gear: { rotation: -0.35 } },
      "scenario-feature-render",
    );
    current.scenario.steps.push("what-if: light=0.93, rotation=-0.35");

    current.activeBranchId = featureId;
    current.runtime.history().switchBranch(featureId);
    current.inspectNodeId = "gearTopologyModel";
    current.inspect = safeInspect(current.runtime, featureId, current.inspectNodeId);
    current.mergePlan = computeMergePlan(current);
    current.mergeResult = null;
    current.mergeReview = null;
    current.reviewFrames.clear();
    current.scenario = {
      ...(withScenarioStatus(
        current.scenario,
        "scripted",
        "scenario scripted and merge plan prepared",
        await buildScenarioProof(current),
      )),
      inspectedNodes: DEFAULT_SCENARIO_INSPECT_NODES,
    };
    emitSnapshot([]);
  } finally {
    renderLock = false;
  }
}

async function planScenarioMerge(current: SessionState) {
  current.mergePlan = computeMergePlan(current);
  current.mergeResult = null;
  current.mergeReview = null;
  current.reviewFrames.clear();
  current.inspectNodeId = "gearTopologyModel";
  if (current.activeBranchId !== null) {
    current.inspect = safeInspect(current.runtime, current.activeBranchId, current.inspectNodeId);
  }
  current.scenario = withScenarioStatus(
    current.scenario,
    "planned",
    "merge plan refreshed",
    await buildScenarioProof(current),
  );
  emitSnapshot([]);
}

async function executeScenarioMerge(current: SessionState) {
  await mergeActiveBranch(current);
  current.scenario = withScenarioStatus(
    current.scenario,
    "merged",
    "scenario merge executed",
    await buildScenarioProof(current),
  );
  current.inspectNodeId = "hudModel";
  if (current.activeBranchId !== null) {
    current.inspect = safeInspect(current.runtime, current.activeBranchId, current.inspectNodeId);
  }
  emitSnapshot([]);
}

async function replayScenarioMerge(current: SessionState) {
  const mergeIndex = findLastTimelineIndex(current.timeline, "merge");
  if (mergeIndex < 0) {
    return;
  }
  await scrubTimeline(current, mergeIndex);
  if (!session) {
    return;
  }
  const previousProof = session.scenario?.proof ?? current.scenario?.proof ?? null;
  const replayProof = buildReplayScenarioProofArtifacts({
    runtime: session.runtime,
    replayedBranchId: session.activeBranchId,
    previousProof,
  });
  session.scenario = withReplayScenarioProof(
    session.scenario,
    "timeline rebuilt from merge commit",
    previousProof,
    replayProof
      ? {
          proofSchemaVersion: replayProof.proofSchemaVersion ?? previousProof?.proofSchemaVersion ?? null,
          replayedLoweredStrategyBundleDigest:
            replayProof.replayedLoweredStrategyBundleDigest
            ?? previousProof?.replayedLoweredStrategyBundleDigest
            ?? null,
          replayedMergePlanDigest:
            replayProof.replayedMergePlanDigest
            ?? previousProof?.replayedMergePlanDigest
            ?? null,
          replayedMergeResultDigest:
            replayProof.replayedMergeResultDigest
            ?? previousProof?.replayedMergeResultDigest
            ?? null,
          replayedLineageDigest:
            replayProof.replayedLineageDigest
            ?? previousProof?.replayedLineageDigest
            ?? null,
          replayBranchStateDigest: replayProof.replayedBranchStateDigest ?? null,
          replayParity: replayProof.parity ?? null,
          replayMismatchClasses: replayProof.mismatchClasses ?? [],
        }
      : null,
  );
  session.inspectNodeId = "gearToothModel::tooth-0";
  if (session.activeBranchId !== null) {
    session.inspect = safeInspect(session.runtime, session.activeBranchId, session.inspectNodeId);
  }
  emitSnapshot([]);
}



async function applyScenePatch(current: SessionState, branchId: BranchId, patch: ScenePatch, label?: string) {
  const target = current.branches.get(branchId) ?? getActiveBranch(current);
  if (!target || isEmptyScenePatch(patch)) {
    return;
  }
  current.runtime.history().switchBranch(target.summary.id);

  postDebug(
    "scene-patch:start",
    JSON.stringify({
      branchId: target.summary.id,
      label: label ?? "edit",
      patch,
      before: summarizeBranchState(target.summary),
    }),
  );


  const update = await updateScene(current.runtime, target.summary.id, patch, (phase, detail) => {
    postDebug(phase, detail);
  });

  updateBranchCache(
    current,
    update.branchId,
    {
      id: update.branchId,
      name: update.branchName,
      state: update.state,
      hud: update.hud,
    },
    update.frame,
  );
  current.activeBranchId = update.branchId;
  current.latestSummary = update.summary;
  current.mergePlan = null;
  current.inspect = safeInspect(current.runtime, update.branchId, current.inspectNodeId);
  postDebug(
    "scene-patch:done",
    JSON.stringify({
      branchId: update.branchId,
      label: label ?? "edit",
      after: summarizeBranchState({
        id: update.branchId,
        name: update.branchName,
        state: update.state,
        hud: update.hud,
      }),
    }),
  );
  captureTimeline(current, label ?? "edit", false);
  emitSnapshot([update.branchId]);
}

async function createBranch(current: SessionState) {
  const existing = Array.from(current.branches.values()).find((branch) => branch.summary.name === "what-if");
  const branchId = existing?.summary.id ?? current.runtime.history().createBranch("what-if").id;
  const update = await renderBranch(current.runtime, branchId, (phase, detail) => {
    postDebug(phase, detail);
  });
  updateBranchCache(
    current,
    update.branchId,
    {
      id: update.branchId,
      name: update.branchName,
      state: update.state,
      hud: update.hud,
    },
    update.frame,
  );
  current.activeBranchId = branchId;
  current.latestSummary = update.summary;
  current.mergePlan = null;
  current.inspect = safeInspect(current.runtime, branchId, current.inspectNodeId);
  captureTimeline(current, "branch", true);
  emitSnapshot([branchId]);
}

async function mergeActiveBranch(current: SessionState) {
  const feature = Array.from(current.branches.values()).find((branch) => branch.summary.name === "what-if");
  const main = Array.from(current.branches.values()).find((branch) => branch.summary.name !== "what-if");
  if (!feature || !main) {
    return;
  }
  current.runtime.history().switchBranch(feature.summary.id);

  postDebug(
    "merge:start",
    JSON.stringify({
      source: summarizeBranchState(feature.summary),
      target: summarizeBranchState(main.summary),
      plan: current.mergePlan ?? computeMergePlan(current),
    }),
  );

  current.mergePlan = current.mergePlan ?? computeMergePlan(current);
  const mergeResult = await executeMerge(current.runtime, feature.summary.id, main.summary.id);
  const mainUpdate = await renderBranch(current.runtime, main.summary.id, (phase, detail) => {
    postDebug(phase, detail);
  });
  updateBranchCache(
    current,
    mainUpdate.branchId,
    {
      id: mainUpdate.branchId,
      name: mainUpdate.branchName,
      state: mainUpdate.state,
      hud: mainUpdate.hud,
    },
    mainUpdate.frame,
  );
  current.branches.get(feature.summary.id)?.frame?.close();
  current.branches.delete(feature.summary.id);

  current.activeBranchId = main.summary.id;
  current.latestSummary = mainUpdate.summary;
  current.mergeResult = mergeResult;
  current.mergePlan = current.mergePlan;
  current.mergeReview = await buildMergeReviewSnapshot(
    current,
    feature.summary,
    main.summary,
    {
      id: mainUpdate.branchId,
      name: mainUpdate.branchName,
      state: mainUpdate.state,
      hud: mainUpdate.hud,
    },
    current.mergePlan,
  );
  current.inspect = safeInspect(current.runtime, main.summary.id, current.inspectNodeId);
  postDebug(
    "merge:done",
    JSON.stringify({
      result: mergeResult,
      targetAfter: summarizeBranchState({
        id: mainUpdate.branchId,
        name: mainUpdate.branchName,
        state: mainUpdate.state,
        hud: mainUpdate.hud,
      }),
    }),
  );
  captureTimeline(current, "merge", true);
  emitSnapshot([main.summary.id]);
}

async function activateBranch(current: SessionState, branchId: BranchId) {
  const target = current.branches.get(branchId);
  if (!target) {
    return;
  }

  current.runtime.history().switchBranch(target.summary.id);
  current.activeBranchId = branchId;
  current.latestSummary = null;
  current.mergePlan = null;
  current.mergeReview = null;
  current.reviewFrames.clear();
  current.inspect = safeInspect(current.runtime, branchId, current.inspectNodeId);
  postDebug(
    "activate:branch-selected",
    JSON.stringify({
      branchId,
      selected: summarizeBranchState(target.summary),
    }),
  );
  emitSnapshot([]);
}

async function inspectNode(current: SessionState, branchId: BranchId, nodeId: string) {
  current.activeBranchId = branchId;
  current.inspectNodeId = nodeId;
  current.inspect = safeInspect(current.runtime, branchId, nodeId);
  emitSnapshot([]);
}

async function scrubTimeline(current: SessionState, index: number) {
  const nextEntry = current.timeline[index];
  if (!nextEntry) {
    return;
  }

  const rebuilt = await rebuildFromTimeline(nextEntry, current.scenario?.diagnosticsTier ?? "webDevelopment");
  session = {
    ...rebuilt,
    graphNodes: current.graphNodes,
    timeline: current.timeline,
    timelineIndex: index,
    ...rebuildCommitState(current.timeline, index),
  };
  session.inspectNodeId = nextEntry.primaryNode;
  if (session.activeBranchId !== null) {
    session.inspect = safeInspect(session.runtime, session.activeBranchId, session.inspectNodeId);
  }

  emitSnapshot(branchFrameIds(session.branches));
}

async function rebuildFromTimeline(
  entry: TimelineState,
  diagnosticsTier: DiagnosticsTier,
) {
  const { runtime } = await createSceneRuntime();
  applyDiagnosticsTierToRuntime(runtime, diagnosticsTier);
  const branches = new Map<BranchId, CachedBranch>();

  const mainTarget = entry.branches.find((branch) => branch.name !== "what-if") ?? entry.branches[0];
  const mainId = runtime.history().currentBranch().id;
  const mainUpdate = await updateScene(runtime, mainId, {
    camera: mainTarget.state.camera,
    light: mainTarget.state.light,
    gear: mainTarget.state.gear,
  });
  branches.set(mainUpdate.branchId, {
      summary: {
        id: mainUpdate.branchId,
        name: mainUpdate.branchName,
        state: mainUpdate.state,
        hud: mainUpdate.hud,
      },
      frame: mainUpdate.frame,
  });

  let activeBranchId = mainUpdate.branchId;

  const featureTarget = entry.branches.find((branch) => branch.name === "what-if");
  if (featureTarget) {
    const featureId = runtime.history().createBranch("what-if").id;
    const featureUpdate = await updateScene(runtime, featureId, {
      camera: featureTarget.state.camera,
      light: featureTarget.state.light,
      gear: featureTarget.state.gear,
    });
    branches.set(featureUpdate.branchId, {
      summary: {
        id: featureUpdate.branchId,
        name: featureUpdate.branchName,
        state: featureUpdate.state,
        hud: featureUpdate.hud,
      },
      frame: featureUpdate.frame,
    });
    if (entry.activeBranchName === "what-if") {
      activeBranchId = featureUpdate.branchId;
    }
  }

  const provisional: SessionState = {
    runtime,
    graphNodes: totalGraphNodes(runtime),
    branches,
    reviewFrames: new Map(),
    activeBranchId,
    latestSummary: mainUpdate.summary,
    mergePlan: null,
    mergeResult: null,
    mergeReview: null,
    timeline: [],
    timelineIndex: 0,
    commitCounter: 0,
    branchHeads: new Map(),
    inspect: null,
    inspectNodeId: entry.primaryNode,
    scenario: null,
  };

  provisional.mergePlan = null;
  provisional.inspect = readBranchInspectForNode(runtime, activeBranchId, provisional.inspectNodeId);
  return provisional;
}

function computeMergePlan(current: SessionState): MergePlan | null {
  const feature = Array.from(current.branches.values()).find((branch) => branch.summary.name === "what-if");
  const main = Array.from(current.branches.values()).find((branch) => branch.summary.name !== "what-if");
  if (!feature || !main) {
    return null;
  }
  return planMerge(current.runtime, feature.summary.id, main.summary.id);
}

type ReviewPreviewDefinition = {
  id: string;
  label: string;
  accent: string;
  description: string;
  frameId: string;
  request: {
    conflictPolicyName?: string | null;
    conflictIsolationPolicyName?: string | null;
    identityMatcherName?: string | null;
    deletionPolicyName?: string | null;
  } | null;
  fallbackPlan: MergePlan | null;
};

type ReviewPreviewRender = {
  id: string;
  label: string;
  accent: string;
  description: string;
  frameId: string;
  frame: ImageBitmap;
  plan: MergePlan | null;
  resultState: BranchSummary["state"] | null;
  visualMode: "rendered" | "manual-review";
  sourceFrame?: ImageBitmap;
  targetFrame?: ImageBitmap;
};

async function buildMergeReviewSnapshot(
  current: SessionState,
  source: BranchSummary,
  target: BranchSummary,
  merged: BranchSummary,
  currentPlan: MergePlan | null,
): Promise<MergeReviewSnapshot | null> {
  const sourceFrameId = buildReviewFrameId("source");
  const targetFrameId = buildReviewFrameId("target");
  const mergedFrameId = buildReviewFrameId("merged-current");
  const diagnosticsTier = current.scenario?.diagnosticsTier ?? "webDevelopment";

  const currentPreview = await renderReviewPreview({
    id: "current",
    label: "Active merge stack",
    accent: "#d1ff5a",
    description: "The exact policy stack used for the executed merge.",
    frameId: mergedFrameId,
    request: null,
    fallbackPlan: currentPlan,
  }, source, target, diagnosticsTier);
  const strictPreview = await renderReviewPreview({
    id: "strict",
    label: "Conflict override",
    accent: "#ff8e78",
    description: "Swap the conflict resolver to reject shared-state edits and stop for manual review.",
    frameId: buildReviewFrameId("strict"),
    request: {
      conflictPolicyName: "signal.conflict.reject-shared-state",
    },
    fallbackPlan: tryPreviewPlan(current, source.id, target.id, {
      conflictPolicyName: "signal.conflict.reject-shared-state",
    }),
  }, source, target, diagnosticsTier);
  const perAspectPreview = await renderReviewPreview({
    id: "perAspect",
    label: "Isolation override",
    accent: "#71d8ff",
    description: "Swap conflict isolation to per-aspect so only the moved decision surface is isolated.",
    frameId: buildReviewFrameId("per-aspect"),
    request: {
      conflictIsolationPolicyName: "signal.conflict-isolation.per-aspect",
    },
    fallbackPlan: tryPreviewPlan(current, source.id, target.id, {
      conflictIsolationPolicyName: "signal.conflict-isolation.per-aspect",
    }),
  }, source, target, diagnosticsTier);

  current.reviewFrames.clear();
  current.reviewFrames.set(sourceFrameId, currentPreview.sourceFrame ?? currentPreview.frame);
  current.reviewFrames.set(targetFrameId, currentPreview.targetFrame ?? currentPreview.frame);
  current.reviewFrames.set(mergedFrameId, currentPreview.frame);
  current.reviewFrames.set(strictPreview.frameId, strictPreview.frame);
  current.reviewFrames.set(perAspectPreview.frameId, perAspectPreview.frame);

  return {
    source: cloneBranchSummary(source),
    target: cloneBranchSummary(target),
    merged: cloneBranchSummary(merged),
    sourceFrameId,
    targetFrameId,
    mergedFrameId,
    previews: [
      {
        id: currentPreview.id,
        label: currentPreview.label,
        accent: currentPreview.accent,
        description: currentPreview.description,
        plan: currentPreview.plan,
        frameId: mergedFrameId,
        resultState: currentPreview.resultState,
        visualMode: currentPreview.visualMode,
      },
      {
        id: strictPreview.id,
        label: strictPreview.label,
        accent: strictPreview.accent,
        description: strictPreview.description,
        plan: strictPreview.plan,
        frameId: strictPreview.frameId,
        resultState: strictPreview.resultState,
        visualMode: strictPreview.visualMode,
      },
      {
        id: perAspectPreview.id,
        label: perAspectPreview.label,
        accent: perAspectPreview.accent,
        description: perAspectPreview.description,
        plan: perAspectPreview.plan,
        frameId: perAspectPreview.frameId,
        resultState: perAspectPreview.resultState,
        visualMode: perAspectPreview.visualMode,
      },
    ],
  };
}

async function renderReviewPreview(
  definition: ReviewPreviewDefinition,
  source: BranchSummary,
  target: BranchSummary,
  diagnosticsTier: DiagnosticsTier,
): Promise<ReviewPreviewRender> {
  const { runtime } = await createSceneRuntime();
  applyDiagnosticsTierToRuntime(runtime, diagnosticsTier);

  const targetBranchId = runtime.history().currentBranch().id;
  const targetUpdate = await updateScene(runtime, targetBranchId, fullScenePatch(target.state));
  const sourceBranchId = runtime.history().createBranch("review-source").id;
  const sourceUpdate = await updateScene(runtime, sourceBranchId, fullScenePatch(source.state));

  if (!definition.request) {
    await executeMerge(runtime, sourceBranchId, targetBranchId);
    const mergedUpdate = await renderBranch(runtime, targetBranchId);
    return {
      id: definition.id,
      label: definition.label,
      accent: definition.accent,
      description: definition.description,
      frameId: definition.frameId,
      frame: mergedUpdate.frame,
      plan: definition.fallbackPlan,
      resultState: mergedUpdate.state,
      visualMode: "rendered",
      sourceFrame: sourceUpdate.frame,
      targetFrame: targetUpdate.frame,
    };
  }

  try {
    executeMergePolicyPreview(runtime, {
      sourceBranchId,
      targetBranchId,
      conflictPolicyName: definition.request.conflictPolicyName ?? null,
      conflictIsolationPolicyName: definition.request.conflictIsolationPolicyName ?? null,
      identityMatcherName: definition.request.identityMatcherName ?? null,
      deletionPolicyName: definition.request.deletionPolicyName ?? null,
    });
    const mergedUpdate = await renderBranch(runtime, targetBranchId);
    closeBitmapSafe(sourceUpdate.frame);
    closeBitmapSafe(targetUpdate.frame);
    return {
      id: definition.id,
      label: definition.label,
      accent: definition.accent,
      description: definition.description,
      frameId: definition.frameId,
      frame: mergedUpdate.frame,
      plan: definition.fallbackPlan,
      resultState: mergedUpdate.state,
      visualMode: "rendered",
    };
  } catch {
    const composite = composeManualReviewFrame(sourceUpdate.frame, targetUpdate.frame, definition.accent);
    closeBitmapSafe(sourceUpdate.frame);
    closeBitmapSafe(targetUpdate.frame);
    return {
      id: definition.id,
      label: definition.label,
      accent: definition.accent,
      description: definition.description,
      frameId: definition.frameId,
      frame: composite,
      plan: definition.fallbackPlan,
      resultState: null,
      visualMode: "manual-review",
    };
  }
}

function tryPreviewPlan(
  current: SessionState,
  sourceBranchId: BranchId,
  targetBranchId: BranchId,
  request: {
    conflictPolicyName?: string | null;
    conflictIsolationPolicyName?: string | null;
    identityMatcherName?: string | null;
    deletionPolicyName?: string | null;
  },
): MergePlan | null {
  try {
    return planMergePolicyPreview(current.runtime, {
      sourceBranchId,
      targetBranchId,
      conflictPolicyName: request.conflictPolicyName ?? null,
      conflictIsolationPolicyName: request.conflictIsolationPolicyName ?? null,
      identityMatcherName: request.identityMatcherName ?? null,
      deletionPolicyName: request.deletionPolicyName ?? null,
    });
  } catch (error) {
    postDebug("merge:preview-failed", formatError(error));
    return null;
  }
}

function cloneBranchSummary(summary: BranchSummary): BranchSummary {
  return {
    id: summary.id,
    name: summary.name,
    state: {
      camera: { ...summary.state.camera },
      light: { ...summary.state.light },
      gear: { ...summary.state.gear },
    },
    hud: { ...summary.hud },
  };
}

function fullScenePatch(state: BranchSummary["state"]) {
  return {
    camera: { ...state.camera },
    light: { ...state.light },
    gear: { ...state.gear },
  };
}

function buildReviewFrameId(label: string) {
  return `merge-review:${label}:${Math.random().toString(36).slice(2, 10)}`;
}

function composeManualReviewFrame(
  sourceFrame: ImageBitmap,
  targetFrame: ImageBitmap,
  accent: string,
) {
  const canvas = new OffscreenCanvas(RENDER_WIDTH, RENDER_HEIGHT);
  const ctx = canvas.getContext("2d");
  if (!ctx) {
    throw new Error("2D OffscreenCanvas context is required for manual review composition.");
  }

  ctx.clearRect(0, 0, RENDER_WIDTH, RENDER_HEIGHT);
  ctx.drawImage(targetFrame, 0, 0, RENDER_WIDTH, RENDER_HEIGHT);
  ctx.save();
  ctx.beginPath();
  ctx.moveTo(0, 0);
  ctx.lineTo(RENDER_WIDTH * 0.58, 0);
  ctx.lineTo(RENDER_WIDTH * 0.42, RENDER_HEIGHT);
  ctx.lineTo(0, RENDER_HEIGHT);
  ctx.closePath();
  ctx.clip();
  ctx.drawImage(sourceFrame, 0, 0, RENDER_WIDTH, RENDER_HEIGHT);
  ctx.restore();

  ctx.fillStyle = "rgba(7, 11, 15, 0.18)";
  ctx.fillRect(0, 0, RENDER_WIDTH, RENDER_HEIGHT);
  ctx.strokeStyle = accent;
  ctx.lineWidth = 5;
  ctx.beginPath();
  ctx.moveTo(RENDER_WIDTH * 0.58, 0);
  ctx.lineTo(RENDER_WIDTH * 0.42, RENDER_HEIGHT);
  ctx.stroke();

  ctx.fillStyle = "rgba(9, 13, 18, 0.88)";
  ctx.fillRect(RENDER_WIDTH * 0.5 - 62, RENDER_HEIGHT * 0.5 - 22, 124, 44);
  ctx.strokeStyle = accent;
  ctx.lineWidth = 2;
  ctx.strokeRect(RENDER_WIDTH * 0.5 - 62, RENDER_HEIGHT * 0.5 - 22, 124, 44);
  ctx.fillStyle = "#f7fbfd";
  ctx.font = "700 14px Inter";
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.fillText("Manual Review", RENDER_WIDTH * 0.5, RENDER_HEIGHT * 0.5);

  return canvas.transferToImageBitmap();
}

function closeBitmapSafe(bitmap: ImageBitmap | null | undefined) {
  if (!bitmap) {
    return;
  }
  try {
    bitmap.close();
  } catch {
    // Ignore detached or already-closed bitmaps.
  }
}

function emitSnapshot(frameBranchIds: BranchId[]) {
  const current = session;
  if (!current) {
    return;
  }

  const frames: BranchFrame[] = [];
  const reviewFrames: ReviewFrame[] = [];
  const transfer: Transferable[] = [];
  for (const branchId of frameBranchIds) {
    const branch = current.branches.get(branchId);
    if (!branch?.frame) {
      continue;
    }
    frames.push({
      branchId,
      width: RENDER_WIDTH,
      height: RENDER_HEIGHT,
      bitmap: branch.frame,
    });
    transfer.push(branch.frame);
    branch.frame = null;
  }

  for (const [id, bitmap] of current.reviewFrames.entries()) {
    if (!bitmap) {
      continue;
    }
    reviewFrames.push({
      id,
      width: RENDER_WIDTH,
      height: RENDER_HEIGHT,
      bitmap,
    });
    transfer.push(bitmap);
    current.reviewFrames.set(id, null);
  }

  const snapshot: WorkerSnapshot = buildWorkerSnapshot(current);

  postEvent({ type: "snapshot", snapshot, frames, reviewFrames }, transfer);
}

function safeInspect(runtime: SessionState["runtime"], branchId: BranchId, nodeId: string): BranchInspect | null {
  try {
    return readBranchInspectForNode(runtime, branchId, nodeId);
  } catch {
    return null;
  }
}

function formatError(error: unknown): string {
  if (error instanceof Error) {
    return error.message;
  }
  if (typeof error === "string") {
    return error;
  }
  if (error && typeof error === "object" && "message" in error) {
    const message = (error as { message?: unknown }).message;
    if (typeof message === "string") {
      return message;
    }
  }
  try {
    return JSON.stringify(error);
  } catch {
    return String(error);
  }
}


async function buildScenarioProof(current: SessionState): Promise<ScenarioProofArtifacts> {
  return buildScenarioProofArtifacts({
    runtime: current.runtime,
    mergePlan: current.mergePlan,
    mergeResult: current.mergeResult,
    activeBranchId: current.activeBranchId,
    previousProof: current.scenario?.proof,
  });
}

function applyDiagnosticsTierToRuntime(runtime: SessionState["runtime"], tier: DiagnosticsTier) {
  runtime.setRuntimePolicy({ preset: tier });
}
