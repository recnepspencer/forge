import type { WorkerSnapshot } from "../gear-scene/worker/protocol";
import { SHELL_SIGNAL_BINDINGS, type DemoShellSignalKey } from "./shell_signal_schema";
import { normalizeWorkerSnapshot } from "./shell_signal_normalization";
import type { ShellStoreApp } from "./shell_signal_store";

function computeSuppressionPercent(summary: { nodesEvaluated: number } | null, graphNodes: number): string {
  if (!summary || graphNodes === 0) return "0.0";
  const untouched = Math.max(graphNodes - summary.nodesEvaluated, 0);
  return ((untouched / graphNodes) * 100).toFixed(1);
}

function sameStringArray(left: string[], right: string[]) {
  return left.length === right.length && left.every((value, index) => value === right[index]);
}

function sameBranchState(left: WorkerSnapshot["branches"][number]["state"], right: WorkerSnapshot["branches"][number]["state"]) {
  return left.camera.x === right.camera.x
    && left.camera.y === right.camera.y
    && left.camera.z === right.camera.z
    && left.camera.yaw === right.camera.yaw
    && left.camera.pitch === right.camera.pitch
    && left.light.x === right.light.x
    && left.light.y === right.light.y
    && left.light.z === right.light.z
    && left.light.intensity === right.light.intensity
    && left.gear.teeth === right.gear.teeth
    && left.gear.outerRadius === right.gear.outerRadius
    && left.gear.innerRadius === right.gear.innerRadius
    && left.gear.thickness === right.gear.thickness
    && left.gear.rotation === right.gear.rotation;
}

function sameHud(left: WorkerSnapshot["branches"][number]["hud"], right: WorkerSnapshot["branches"][number]["hud"]) {
  return left.frameIndex === right.frameIndex
    && left.raysMarched === right.raysMarched
    && left.averageSteps === right.averageSteps
    && left.hits === right.hits
    && left.misses === right.misses
    && left.renderMs === right.renderMs
    && left.touchedNodes === right.touchedNodes
    && left.nodesEvaluated === right.nodesEvaluated
    && left.nodesSuppressed === right.nodesSuppressed
    && left.totalNanos === right.totalNanos
    && left.cameraX === right.cameraX
    && left.cameraY === right.cameraY
    && left.cameraZ === right.cameraZ
    && left.lightX === right.lightX
    && left.lightY === right.lightY
    && left.lightZ === right.lightZ;
}

function sameBranches(left: WorkerSnapshot["branches"], right: WorkerSnapshot["branches"]) {
  return left.length === right.length && left.every((branch, index) => {
    const other = right[index];
    return other
      && branch.id === other.id
      && branch.name === other.name
      && sameHud(branch.hud, other.hud)
      && sameBranchState(branch.state, other.state);
  });
}

function sameTimeline(left: WorkerSnapshot["timeline"], right: WorkerSnapshot["timeline"]) {
  return left.length === right.length && left.every((entry, index) => {
    const other = right[index];
    return other
      && entry.id === other.id
      && entry.branchName === other.branchName
      && entry.kind === other.kind
      && entry.label === other.label
      && entry.frameIndex === other.frameIndex
      && entry.activeBranchName === other.activeBranchName
      && entry.branchCount === other.branchCount
      && entry.snapshotId === other.snapshotId
      && entry.primaryNode === other.primaryNode
      && sameStringArray(entry.parentIds, other.parentIds)
      && sameStringArray(entry.touchedNodes, other.touchedNodes);
  });
}

function sameLatestSummary(
  left: WorkerSnapshot["latestSummary"],
  right: WorkerSnapshot["latestSummary"],
) {
  return left?.nodesEvaluated === right?.nodesEvaluated
    && left?.touchedNodes === right?.touchedNodes
    && left?.totalNanos === right?.totalNanos;
}

function samePlan(left: WorkerSnapshot["mergePlan"], right: WorkerSnapshot["mergePlan"]) {
  return left?.proof?.planDigest === right?.proof?.planDigest
    && left?.proof?.semanticsDigest === right?.proof?.semanticsDigest
    && left?.candidateCount === right?.candidateCount
    && left?.nodePlanCount === right?.nodePlanCount;
}

function sameResult(left: WorkerSnapshot["mergeResult"], right: WorkerSnapshot["mergeResult"]) {
  return left?.proof?.resultDigest === right?.proof?.resultDigest
    && left?.proof?.semanticsDigest === right?.proof?.semanticsDigest
    && left?.recordCount === right?.recordCount
    && left?.conflictCount === right?.conflictCount;
}

function sameInspect(left: WorkerSnapshot["inspect"], right: WorkerSnapshot["inspect"]) {
  return left?.selectedNode === right?.selectedNode
    && left?.why.state === right?.why.state
    && left?.lineage.events.length === right?.lineage.events.length
    && left?.replay.length === right?.replay.length;
}

function sameMergeReview(left: WorkerSnapshot["mergeReview"], right: WorkerSnapshot["mergeReview"]) {
  if (!left || !right) {
    return left === right;
  }
  return left.source.id === right.source.id
    && left.target.id === right.target.id
    && left.merged.id === right.merged.id
    && sameBranchState(left.source.state, right.source.state)
    && sameBranchState(left.target.state, right.target.state)
    && sameBranchState(left.merged.state, right.merged.state)
    && sameStringArray(
      left.previews.map((preview) => preview.plan?.proof?.planDigest ?? `${preview.id}:pending`),
      right.previews.map((preview) => preview.plan?.proof?.planDigest ?? `${preview.id}:pending`),
    );
}

function sameScenario(left: WorkerSnapshot["scenario"], right: WorkerSnapshot["scenario"]) {
  return left?.mode === right?.mode
    && left?.status === right?.status
    && left?.lastAction === right?.lastAction
    && left?.proof?.mergeResultDigest === right?.proof?.mergeResultDigest
    && left?.proof?.replayBranchStateDigest === right?.proof?.replayBranchStateDigest
    && sameStringArray(left?.steps ?? [], right?.steps ?? [])
    && sameStringArray(left?.inspectedNodes ?? [], right?.inspectedNodes ?? []);
}

function queueSetIfChanged<T>(
  app: ShellStoreApp,
  ops: Array<{ kind: "set"; id: string; value: unknown }>,
  key: DemoShellSignalKey,
  next: T,
  equal: (left: T, right: T) => boolean = Object.is,
) {
  const signalId = SHELL_SIGNAL_BINDINGS[key].id;
  const current = app.read<T>(signalId);
  if (!equal(current, next)) {
    ops.push({ kind: "set", id: signalId, value: next });
  }
}

export function applySnapshotToShellSignals(app: ShellStoreApp, snapshot: WorkerSnapshot) {
  const normalized = normalizeWorkerSnapshot(snapshot);
  const activeBranch =
    normalized.branches.find((branch) => branch.id === normalized.activeBranchId)
    ?? normalized.branches[0]
    ?? null;
  const hasFeatureBranch = normalized.branches.some((branch) => branch.name === "what-if");
  const suppressionPercent = computeSuppressionPercent(normalized.latestSummary, normalized.graphNodes);
  const ops: Array<{ kind: "set"; id: string; value: unknown }> = [];

  queueSetIfChanged(app, ops, "graphNodes", normalized.graphNodes);
  queueSetIfChanged(app, ops, "branches", normalized.branches, sameBranches);
  queueSetIfChanged(app, ops, "activeBranch", activeBranch, (left, right) =>
    left?.id === right?.id
    && left?.hud.frameIndex === right?.hud.frameIndex
    && left?.state.gear.teeth === right?.state.gear.teeth,
  );
  queueSetIfChanged(app, ops, "hasFeatureBranch", hasFeatureBranch);
  queueSetIfChanged(app, ops, "activeBranchId", normalized.activeBranchId);
  queueSetIfChanged(app, ops, "latestSummary", normalized.latestSummary, sameLatestSummary);
  queueSetIfChanged(app, ops, "suppressionPercent", suppressionPercent);
  queueSetIfChanged(app, ops, "mergePlan", normalized.mergePlan, samePlan);
  queueSetIfChanged(app, ops, "mergeResult", normalized.mergeResult, sameResult);
  queueSetIfChanged(app, ops, "timeline", normalized.timeline, sameTimeline);
  queueSetIfChanged(app, ops, "timelineIndex", normalized.timelineIndex);
  queueSetIfChanged(app, ops, "inspect", normalized.inspect, sameInspect);
  queueSetIfChanged(app, ops, "mergeReview", normalized.mergeReview, sameMergeReview);
  queueSetIfChanged(app, ops, "scenario", normalized.scenario, sameScenario);
  queueSetIfChanged(app, ops, "error", normalized.error);
  queueSetIfChanged(app, ops, "debugStatus", normalized.debugStatus ?? null);

  if (ops.length > 0) {
    app.batch(ops);
  }
}
