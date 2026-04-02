import type { BranchInspect, MergePlan, MergeResult, ScenarioState } from "../gear-scene/core/types";
import type { WorkerSnapshot } from "../gear-scene/worker/protocol";

function sanitizeForSignal<T>(value: T): T {
  if (value === undefined) {
    return null as T;
  }
  if (Array.isArray(value)) {
    return value.map((entry) => sanitizeForSignal(entry)) as T;
  }
  if (value && typeof value === "object") {
    return Object.fromEntries(
      Object.entries(value).map(([key, entry]) => [key, sanitizeForSignal(entry)]),
    ) as T;
  }
  return value;
}

function cloneMergePlan(plan: MergePlan): MergePlan {
  return structuredClone(plan);
}

function cloneMergeResult(result: MergeResult): MergeResult {
  return structuredClone(result);
}

function cloneBranchInspect(inspect: BranchInspect): BranchInspect {
  return structuredClone(inspect);
}

function cloneScenarioState(scenario: ScenarioState): ScenarioState {
  return structuredClone(scenario);
}

export function normalizeWorkerSnapshot(snapshot: WorkerSnapshot): WorkerSnapshot {
  return sanitizeForSignal({
    ready: snapshot.ready,
    graphNodes: snapshot.graphNodes,
    branches: snapshot.branches.map((branch) => ({
      ...branch,
      hud: { ...branch.hud },
      state: {
        camera: { ...branch.state.camera },
        light: { ...branch.state.light },
        gear: { ...branch.state.gear },
      },
    })),
    activeBranchId: snapshot.activeBranchId ?? null,
    latestSummary: snapshot.latestSummary ? { ...snapshot.latestSummary } : null,
    mergePlan: snapshot.mergePlan ? cloneMergePlan(snapshot.mergePlan) : null,
    mergeResult: snapshot.mergeResult ? cloneMergeResult(snapshot.mergeResult) : null,
    timeline: snapshot.timeline.map((entry) => ({
      ...entry,
      snapshotId: entry.snapshotId ?? null,
      parentIds: [...entry.parentIds],
      touchedNodes: [...entry.touchedNodes],
    })),
    timelineIndex: snapshot.timelineIndex,
    inspect: snapshot.inspect ? cloneBranchInspect(snapshot.inspect) : null,
    scenario: snapshot.scenario ? cloneScenarioState(snapshot.scenario) : null,
    error: snapshot.error ?? null,
    debugStatus: snapshot.debugStatus ?? null,
  });
}
