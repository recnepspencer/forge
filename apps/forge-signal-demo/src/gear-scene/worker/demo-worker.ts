/// <reference lib="webworker" />

import type { RunSummary } from "@forge/signal";

import {
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
  MergePlan,
  MergeResult,
  ScenePatch,
  SceneState,
} from "../core/types";
import type { BranchFrame, BranchSummary, WorkerCommand, WorkerEvent, WorkerSnapshot } from "./protocol";

self.postMessage({ type: "debug", phase: "worker:module-loaded", detail: "worker module evaluated" } satisfies WorkerEvent);

type RuntimeHandle = Awaited<ReturnType<typeof createSceneRuntime>>["runtime"];

type CachedBranch = {
  summary: BranchSummary;
  frame: ImageBitmap;
};

type TimelineState = {
  id: string;
  parentIds: string[];
  branchName: string | null;
  kind: "normal" | "branch" | "merge";
  label: string;
  frameIndex: number;
  activeBranchName: string | null;
  branchCount: number;
  snapshotId: number | null;
  primaryNode: string;
  touchedNodes: string[];
  branches: Array<{
    name: string;
    state: SceneState;
  }>;
};

type SessionState = {
  runtime: RuntimeHandle;
  graphNodes: number;
  branches: Map<BranchId, CachedBranch>;
  activeBranchId: BranchId | null;
  latestSummary: RunSummary | null;
  mergePlan: MergePlan | null;
  mergeResult: MergeResult | null;
  timeline: TimelineState[];
  timelineIndex: number;
  commitCounter: number;
  branchHeads: Map<string, string>;
  inspect: BranchInspect | null;
  inspectNodeId: string;
};

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

    const branches = new Map<BranchId, CachedBranch>();
    branches.set(initial.branchId, {
      summary: {
        id: initial.branchId,
        name: initial.branchName,
        state: initial.state,
        hud: initial.hud,
      },
      frame: initial.frame,
    });

    session = {
      runtime,
      graphNodes: totalGraphNodes(runtime),
      branches,
      activeBranchId: initial.branchId,
      latestSummary: initial.summary,
      mergePlan: null,
      mergeResult: null,
      timeline: [],
      timelineIndex: 0,
      commitCounter: 0,
      branchHeads: new Map(),
      inspect: readBranchInspect(runtime, initial.branchId),
      inspectNodeId: "hudModel",
    };

    captureTimeline(session, "boot", true);
    emitSnapshot([initial.branchId]);
    postDebug("boot:ready", "worker session ready");
  } finally {
    booting = false;
  }
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
  current.branches.get(feature.summary.id)?.frame.close();
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

  const rebuilt = await rebuildFromTimeline(nextEntry);
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

  emitSnapshot(Array.from(session.branches.keys()));
}

async function rebuildFromTimeline(entry: TimelineState) {
  const { runtime } = await createSceneRuntime();
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

function getActiveBranch(current: SessionState) {
  if (current.activeBranchId === null) {
    return null;
  }
  return current.branches.get(current.activeBranchId) ?? null;
}

function updateBranchCache(
  current: SessionState,
  branchId: BranchId,
  summary: BranchSummary,
  frame: ImageBitmap,
) {
  const existing = current.branches.get(branchId);
  existing?.frame.close();
  current.branches.set(branchId, {
    summary,
    frame,
  });
}

function captureTimeline(current: SessionState, label: string, force: boolean) {
  const active = getActiveBranch(current);
  const frameIndex = active?.summary.hud.frameIndex ?? 0;
  const last = current.timeline[current.timeline.length - 1];
  if (!force && last && last.frameIndex === frameIndex && last.label === label) {
    current.timelineIndex = current.timeline.length - 1;
    return;
  }

  const branchName = active?.summary.name ?? null;
  const commitId = `c${current.commitCounter + 1}`;
  const kind = timelineKindForLabel(label);
  const parentIds = parentCommitIdsFor(current, kind, branchName);

  current.timeline.push({
    id: commitId,
    parentIds,
    branchName,
    kind,
    label,
    frameIndex,
    activeBranchName: active?.summary.name ?? null,
    branchCount: current.branches.size,
    snapshotId:
      active != null
        ? current.runtime.history().branches().find((branch) => branch.id === active.summary.id)?.headSnapshotId ?? null
        : null,
    primaryNode: primaryNodeForLabel(label),
    touchedNodes: touchedNodesForLabel(label, active?.summary.state.gear.teeth ?? 1),
    branches: Array.from(current.branches.values()).map((branch) => ({
      name: branch.summary.name,
      state: structuredClone(branch.summary.state),
    })),
  });
  current.commitCounter += 1;
  updateBranchHeads(current.branchHeads, branchName, commitId, kind);
  current.timeline = current.timeline.slice(-80);
  current.timelineIndex = current.timeline.length - 1;
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
    if (!branch) {
      continue;
    }
    frames.push({
      branchId,
      width: RENDER_WIDTH,
      height: RENDER_HEIGHT,
      bitmap: branch.frame,
    });
    transfer.push(branch.frame);
  }

  const snapshot: WorkerSnapshot = {
    ready: true,
    graphNodes: current.graphNodes,
    branches: Array.from(current.branches.values()).map((branch) => branch.summary),
    activeBranchId: current.activeBranchId,
    latestSummary: current.latestSummary,
    mergePlan: current.mergePlan,
    mergeResult: current.mergeResult,
    timeline: current.timeline.map((entry) => ({
      id: entry.id,
      parentIds: entry.parentIds,
      branchName: entry.branchName,
      kind: entry.kind,
      label: entry.label,
      frameIndex: entry.frameIndex,
      activeBranchName: entry.activeBranchName,
      branchCount: entry.branchCount,
      snapshotId: entry.snapshotId,
      primaryNode: entry.primaryNode,
      touchedNodes: entry.touchedNodes,
    })),
    timelineIndex: current.timelineIndex,
    inspect: current.inspect,
    error: null,
  };

  postEvent({ type: "snapshot", snapshot, frames }, transfer);
}

function isEmptyScenePatch(patch: ScenePatch) {
  return !patch.camera && !patch.light && !patch.gear;
}



function safeInspect(runtime: RuntimeHandle, branchId: BranchId, nodeId: string): BranchInspect | null {
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

function summarizeBranchState(summary: BranchSummary) {
  return {
    id: summary.id,
    name: summary.name,
    gear: {
      teeth: summary.state.gear.teeth,
      outerRadius: summary.state.gear.outerRadius,
      innerRadius: summary.state.gear.innerRadius,
      thickness: summary.state.gear.thickness,
      rotation: summary.state.gear.rotation,
    },
    lightIntensity: summary.state.light.intensity,
  };
}


function timelineKindForLabel(label: string): TimelineState["kind"] {
  if (label === "branch") return "branch";
  if (label === "merge") return "merge";
  return "normal";
}

function parentCommitIdsFor(
  current: SessionState,
  kind: TimelineState["kind"],
  branchName: string | null,
): string[] {
  const mainHead = current.branchHeads.get("main");
  const whatIfHead = current.branchHeads.get("what-if");

  if (kind === "branch") {
    return mainHead ? [mainHead] : [];
  }

  if (kind === "merge") {
    return [mainHead, whatIfHead].filter((value): value is string => Boolean(value));
  }

  if (branchName && current.branchHeads.has(branchName)) {
    return [current.branchHeads.get(branchName)!];
  }

  return mainHead ? [mainHead] : [];
}

function updateBranchHeads(
  heads: Map<string, string>,
  branchName: string | null,
  commitId: string,
  kind: TimelineState["kind"],
) {
  if (kind === "branch") {
    heads.set("what-if", commitId);
    return;
  }

  if (kind === "merge") {
    heads.set("main", commitId);
    heads.delete("what-if");
    return;
  }

  if (branchName) {
    heads.set(branchName, commitId);
  }
}

function rebuildCommitState(timeline: TimelineState[], index: number) {
  const branchHeads = new Map<string, string>();
  const slice = timeline.slice(0, index + 1);
  for (const entry of slice) {
    updateBranchHeads(branchHeads, entry.branchName, entry.id, entry.kind);
  }
  return {
    commitCounter: slice.reduce((max, entry) => {
      const n = Number.parseInt(entry.id.slice(1), 10);
      return Number.isFinite(n) ? Math.max(max, n) : max;
    }, 0),
    branchHeads,
  };
}

function primaryNodeForLabel(label: string): string {
  switch (label) {
    case "teeth":
      return "gearToothModel::tooth-0";
    case "outer":
    case "inner":
    case "thickness":
    case "rotation":
      return "gearMeshModel";
    case "light":
      return "lightingModel";
    case "boot":
      return "gearMeshModel";
    default:
      return "hudModel";
  }
}

function touchedNodesForLabel(label: string, teeth: number): string[] {
  const toothNodes = Array.from({ length: Math.min(teeth, 6) }, (_, index) => `gearToothModel::tooth-${index}`);
  switch (label) {
    case "boot":
      return [
        "gearDimensionsModel",
        "gearProfileModel",
        "gearTopologyModel",
        "gearMeshModel",
        ...toothNodes,
        "lightingModel",
        "viewportProjectionModel",
        "viewportShadingModel",
        "hudModel",
      ];
    case "branch":
    case "merge":
      return ["hudModel", "viewportProjectionModel", "viewportShadingModel"];
    case "teeth":
      return ["gearTeeth", ...toothNodes, "gearDimensionsModel", "gearProfileModel", "gearTopologyModel", "gearMeshModel", "viewportProjectionModel", "viewportShadingModel", "hudModel"];
    case "outer":
      return ["gearOuterRadius", "gearDimensionsModel", "gearProfileModel", "gearTopologyModel", "gearMeshModel", "viewportProjectionModel", "viewportShadingModel", "hudModel"];
    case "inner":
      return ["gearInnerRadius", "gearDimensionsModel", "gearProfileModel", "gearTopologyModel", "gearMeshModel", "viewportProjectionModel", "viewportShadingModel", "hudModel"];
    case "thickness":
      return ["gearThickness", "gearDimensionsModel", "gearTopologyModel", "gearMeshModel", "viewportProjectionModel", "viewportShadingModel", "hudModel"];
    case "rotation":
      return ["gearRotation", "gearDimensionsModel", "viewportProjectionModel", "viewportShadingModel", "hudModel"];
    case "light":
      return ["lightIntensity", "lightingModel", "viewportProjectionModel", "viewportShadingModel", "hudModel"];
    default:
      return ["hudModel"];
  }
}
