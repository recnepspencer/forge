/// <reference lib="webworker" />

import {
  buildReplayScenarioProofArtifacts,
  buildScenarioProofArtifacts,
  createSceneRuntime,
  executeMerge,
  planMerge,
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
  current.mergePlan = null;
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
    activeBranchId,
    latestSummary: mainUpdate.summary,
    mergePlan: null,
    mergeResult: null,
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

function emitSnapshot(frameBranchIds: BranchId[]) {
  const current = session;
  if (!current) {
    return;
  }

  const frames: BranchFrame[] = [];
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

  const snapshot: WorkerSnapshot = buildWorkerSnapshot(current);

  postEvent({ type: "snapshot", snapshot, frames }, transfer);
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
