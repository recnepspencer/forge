/// <reference lib="webworker" />

import type { RunSummary } from "@forge/signal";

import {
  createSceneRuntime,
  ensureFeatureBranch,
  executeMerge,
  planMerge,
  readBranchInspect,
  readRenderAspects,
  readBranchSummary,
  renderBranch,
  totalGraphNodes,
  updateScene,
} from "../core/runtime";
import { applyLookDelta, movementStep, renderScene } from "../core/renderer";
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
  label: string;
  frameIndex: number;
  activeBranchName: string | null;
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
  inspect: BranchInspect | null;
  pressed: Set<string>;
  lookDelta: { x: number; y: number };
};

let session: SessionState | null = null;
let booting = false;
let renderLock = false;
let bootStartedAt = 0;
let tickHandle: number | null = null;
let lastTick = 0;

const FPS_KEYS = new Set(["w", "a", "s", "d", "space", "shift"]);

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
      case "setInputs":
        if (session) {
          session.pressed = new Set(command.pressed.filter((key) => FPS_KEYS.has(key)));
        }
        break;
      case "look":
        if (session) {
          session.lookDelta.x += command.deltaX;
          session.lookDelta.y += command.deltaY;
        }
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
      case "scrub":
        await withSession((current) => scrubTimeline(current, command.index));
        break;
      case "setScenePatch":
        await withSession((current) => applyScenePatch(current, command.patch, command.label));
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
      inspect: readBranchInspect(runtime, initial.branchId),
      pressed: new Set<string>(),
      lookDelta: { x: 0, y: 0 },
    };

    captureTimeline(session, "boot", true);
    emitSnapshot([initial.branchId]);
    ensureTickLoop();
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

function ensureTickLoop() {
  if (tickHandle !== null) {
    return;
  }
  tickHandle = self.setInterval(() => {
    void tick();
  }, 16);
}

async function tick() {
  const current = session;
  if (!current || renderLock) {
    return;
  }

  if (current.pressed.size === 0 && current.lookDelta.x === 0 && current.lookDelta.y === 0) {
    return;
  }

  const now = performance.now();
  if (now - lastTick < 16) {
    return;
  }
  lastTick = now;

  const active = getActiveBranch(current);
  if (!active) {
    return;
  }

  const looked = applyLookDelta(active.summary.state.camera, -current.lookDelta.x, current.lookDelta.y);
  current.lookDelta = { x: 0, y: 0 };
  const moved = movementStep(current.pressed, looked);
  const state = {
    ...active.summary.state,
    camera: moved,
  };
  const rendered = renderScene(state, readRenderAspects(current.runtime));
  updateBranchCache(
    current,
    active.summary.id,
    {
      ...active.summary,
      state,
      hud: {
        ...active.summary.hud,
        renderMs: rendered.stats.renderMs,
        frameIndex: active.summary.hud.frameIndex + 1,
      },
    },
    rendered.frame,
  );
  current.activeBranchId = active.summary.id;
  emitSnapshot([active.summary.id]);
}

async function applyScenePatch(current: SessionState, patch: ScenePatch, label?: string) {
  const active = getActiveBranch(current);
  if (!active || isEmptyScenePatch(patch)) {
    return;
  }

  if (isTransientCameraPatch(patch, label)) {
    const state = mergeSceneState(active.summary.state, patch);
    const rendered = renderScene(state, readRenderAspects(current.runtime));
    updateBranchCache(
      current,
      active.summary.id,
      {
        ...active.summary,
        state,
        hud: {
          ...active.summary.hud,
          renderMs: rendered.stats.renderMs,
          frameIndex: active.summary.hud.frameIndex + 1,
        },
      },
      rendered.frame,
    );
    current.activeBranchId = active.summary.id;
    emitSnapshot([active.summary.id]);
    return;
  }

  const runtimePatch: ScenePatch = {
    ...patch,
    // Keep the live FPS camera authoritative for subsequent signal-backed edits
    // so gear/light changes do not snap the viewport back to older runtime state.
    camera: {
      ...active.summary.state.camera,
      ...patch.camera,
    },
  };

  const update = await updateScene(current.runtime, active.summary.id, runtimePatch, (phase, detail) => {
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
  current.mergePlan = computeMergePlan(current);
  current.inspect = readBranchInspect(current.runtime, update.branchId);
  captureTimeline(current, label ?? "edit", false);
  emitSnapshot([update.branchId]);
}

async function createBranch(current: SessionState) {
  const branchId = ensureFeatureBranch(current.runtime);
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
  current.mergePlan = computeMergePlan(current);
  current.inspect = readBranchInspect(current.runtime, branchId);
  captureTimeline(current, "branch", true);
  emitSnapshot([branchId]);
}

async function mergeActiveBranch(current: SessionState) {
  const feature = Array.from(current.branches.values()).find((branch) => branch.summary.name === "what-if");
  const main = Array.from(current.branches.values()).find((branch) => branch.summary.name !== "what-if");
  if (!feature || !main) {
    return;
  }

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

  const featureSummary = readBranchSummary(current.runtime, feature.summary.id, (phase, detail) => {
    postDebug(phase, detail);
  });
  updateBranchCache(current, feature.summary.id, featureSummary, feature.frame);

  current.activeBranchId = main.summary.id;
  current.latestSummary = mainUpdate.summary;
  current.mergeResult = mergeResult;
  current.mergePlan = computeMergePlan(current);
  current.inspect = readBranchInspect(current.runtime, main.summary.id);
  captureTimeline(current, "merge", true);
  emitSnapshot([main.summary.id]);
}

async function activateBranch(current: SessionState, branchId: BranchId) {
  const target = current.branches.get(branchId);
  if (!target) {
    return;
  }

  current.activeBranchId = branchId;
  current.inspect = readBranchInspect(current.runtime, branchId);
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
  };

  emitSnapshot(Array.from(session.branches.keys()));
}

async function rebuildFromTimeline(entry: TimelineState) {
  const { runtime } = await createSceneRuntime();
  const branches = new Map<BranchId, CachedBranch>();

  const mainTarget = entry.branches.find((branch) => branch.name !== "what-if") ?? entry.branches[0];
  const mainId = runtime.history().currentBranch().id;
  const mainUpdate = await updateScene(runtime, mainId, mainTarget.state);
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
    const featureId = ensureFeatureBranch(runtime);
    const featureUpdate = await updateScene(runtime, featureId, featureTarget.state);
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
    inspect: null,
    pressed: new Set<string>(),
    lookDelta: { x: 0, y: 0 },
  };

  provisional.mergePlan = computeMergePlan(provisional);
  provisional.inspect = readBranchInspect(runtime, activeBranchId);
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

  current.timeline.push({
    label,
    frameIndex,
    activeBranchName: active?.summary.name ?? null,
    branches: Array.from(current.branches.values()).map((branch) => ({
      name: branch.summary.name,
      state: structuredClone(branch.summary.state),
    })),
  });
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
      label: entry.label,
      frameIndex: entry.frameIndex,
      activeBranchName: entry.activeBranchName,
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

function isTransientCameraPatch(patch: ScenePatch, label?: string) {
  return Boolean(patch.camera) && !patch.light && !patch.gear && (label === "orbit" || label === "zoom");
}

function mergeSceneState(base: SceneState, patch: ScenePatch): SceneState {
  return {
    camera: {
      ...base.camera,
      ...patch.camera,
    },
    light: {
      ...base.light,
      ...patch.light,
    },
    gear: {
      ...base.gear,
      ...patch.gear,
    },
  };
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
