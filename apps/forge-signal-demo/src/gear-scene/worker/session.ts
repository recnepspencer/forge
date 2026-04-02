import type { RunSummary, SignalRuntime } from "@forge/signal";

import type {
  BranchId,
  BranchInspect,
  MergePlan,
  MergeResult,
  ScenePatch,
  ScenarioState,
  SceneState,
} from "../core/types";
import type { RenderUpdate } from "../core/types";
import type { BranchSummary, WorkerSnapshot } from "./protocol";
import {
  displayBranchSetSnapshot,
  primaryNodeForLabel,
  timelineKindForLabel,
  touchedNodesForLabel,
} from "./scenario";

export type CachedBranch = {
  summary: BranchSummary;
  frame: ImageBitmap | null;
};

export type TimelineState = {
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

export type SessionState = {
  runtime: SignalRuntime;
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
  scenario: ScenarioState | null;
};

const DEFAULT_TIMELINE_STATE: SceneState = {
  camera: { x: 0, y: 0, z: 0, yaw: 0, pitch: 0 },
  light: { x: 0, y: 0, z: 0, intensity: 0 },
  gear: { teeth: 1, outerRadius: 0, innerRadius: 0, thickness: 0, rotation: 0 },
};

export function getActiveBranch(current: SessionState) {
  if (current.activeBranchId === null) {
    return null;
  }
  return current.branches.get(current.activeBranchId) ?? null;
}

export function updateBranchCache(
  current: SessionState,
  branchId: BranchId,
  summary: BranchSummary,
  frame: ImageBitmap,
) {
  const existing = current.branches.get(branchId);
  existing?.frame?.close();
  current.branches.set(branchId, {
    summary,
    frame,
  });
}

export function captureTimeline(current: SessionState, label: string, force: boolean) {
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
    touchedNodes: touchedNodesForLabel(label, active?.summary.state ?? DEFAULT_TIMELINE_STATE),
    branches: displayBranchSetSnapshot(
      Array.from(current.branches.values()).map((branch) => ({
        name: branch.summary.name,
        state: branch.summary.state,
      })),
    ),
  });
  current.commitCounter += 1;
  updateBranchHeads(current.branchHeads, branchName, commitId, kind);
  current.timeline = current.timeline.slice(-80);
  current.timelineIndex = current.timeline.length - 1;
}

export function buildWorkerSnapshot(current: SessionState): WorkerSnapshot {
  return {
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
    scenario: current.scenario,
    error: null,
  };
}

export function isEmptyScenePatch(patch: ScenePatch) {
  return !patch.camera && !patch.light && !patch.gear;
}

export function summarizeBranchState(summary: BranchSummary) {
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

export function createSessionFromInitialRender(
  runtime: SignalRuntime,
  initial: RenderUpdate,
): SessionState {
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

  return {
    runtime,
    graphNodes: 0,
    branches,
    activeBranchId: initial.branchId,
    latestSummary: initial.summary,
    mergePlan: null,
    mergeResult: null,
    timeline: [],
    timelineIndex: 0,
    commitCounter: 0,
    branchHeads: new Map(),
    inspect: null,
    inspectNodeId: "hudModel",
    scenario: null,
  };
}

export function rebuildCommitState(timeline: TimelineState[], index: number) {
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
