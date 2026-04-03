import type { WorkerSnapshot } from "../gear-scene/worker/protocol";

function sanitizeSignalValue<T>(value: T): T {
  if (value === undefined) {
    return null as T;
  }
  if (value === null || typeof value !== "object") {
    return value;
  }
  if (Array.isArray(value)) {
    return value.map((item) => sanitizeSignalValue(item)) as T;
  }

  const sanitized: Record<string, unknown> = {};
  for (const [key, entry] of Object.entries(value)) {
    sanitized[key] = sanitizeSignalValue(entry);
  }
  return sanitized as T;
}

function normalizeBranch(branch: WorkerSnapshot["branches"][number]) {
  return {
    ...branch,
    hud: { ...branch.hud },
    state: {
      camera: { ...branch.state.camera },
      light: { ...branch.state.light },
      gear: { ...branch.state.gear },
    },
  };
}

function normalizeTimelineEntry(entry: WorkerSnapshot["timeline"][number]) {
  return {
    ...entry,
    snapshotId: entry.snapshotId ?? null,
    parentIds: [...entry.parentIds],
    touchedNodes: [...entry.touchedNodes],
  };
}

export function normalizeWorkerSnapshot(snapshot: WorkerSnapshot): WorkerSnapshot {
  return {
    ready: snapshot.ready,
    graphNodes: snapshot.graphNodes,
    branches: snapshot.branches.map(normalizeBranch),
    activeBranchId: snapshot.activeBranchId ?? null,
    latestSummary: snapshot.latestSummary ? { ...snapshot.latestSummary } : null,
    mergePlan: snapshot.mergePlan ? sanitizeSignalValue(snapshot.mergePlan) : null,
    mergeResult: snapshot.mergeResult ? sanitizeSignalValue(snapshot.mergeResult) : null,
    timeline: snapshot.timeline.map(normalizeTimelineEntry),
    timelineIndex: snapshot.timelineIndex,
    inspect: snapshot.inspect ? sanitizeSignalValue(snapshot.inspect) : null,
    mergeReview: snapshot.mergeReview ? sanitizeSignalValue(snapshot.mergeReview) : null,
    scenario: snapshot.scenario ? sanitizeSignalValue(snapshot.scenario) : null,
    error: snapshot.error ?? null,
    debugStatus: snapshot.debugStatus ?? null,
  };
}
