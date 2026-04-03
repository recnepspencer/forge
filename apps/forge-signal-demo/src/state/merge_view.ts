import type { SceneState } from "../gear-scene/core/types";
import type { MergeReviewSnapshot, WorkerSnapshot } from "../gear-scene/worker/protocol";

export type MergeDecisionLane = {
  id: string;
  label: string;
  accent: string;
  frameId: string | null;
  visualMode: "rendered" | "manual-review";
  resultState: SceneState | null;
  policyFamily: string;
  policyLabel: string;
  statusLabel: string;
  actionLabel: string;
  manual: boolean;
};

export type MergeDecisionStep = {
  id: string;
  title: string;
  trailLabel: string;
  focusLabel: string;
  aspectKind: "topology" | "lighting" | "motion" | "mixed";
  sourceHighlights: string[];
  targetHighlights: string[];
  sourceMetrics: string[];
  targetMetrics: string[];
  lanes: MergeDecisionLane[];
};

export function mergeConflictNodeIds(
  mergePlan: WorkerSnapshot["mergePlan"],
  mergeResult: WorkerSnapshot["mergeResult"],
): Set<string> {
  const nodes = new Set<string>();
  for (const record of mergePlan?.conflictIsolation.records ?? mergeResult?.conflictIsolation.records ?? []) {
    nodes.add(record.sourceNode);
    if (record.targetNode) nodes.add(record.targetNode);
  }
  return nodes;
}

export function mergeResolvedNodeIds(
  _mergePlan: WorkerSnapshot["mergePlan"],
  mergeResult: WorkerSnapshot["mergeResult"],
): Set<string> {
  const nodes = new Set<string>();
  for (const record of mergeResult?.records ?? []) {
    if ((record.resolvedConflictKinds?.length ?? 0) > 0) {
      nodes.add(record.sourceNode);
      if (record.targetNode) nodes.add(record.targetNode);
    }
  }
  return nodes;
}

export function friendlyPolicy(name: string | null, basis: string | null): string {
  if (!name) {
    return "Pending";
  }
  const shortName = name.replace(/^signal\./, "");
  return basis ? `${shortName} (${basis})` : shortName;
}

export function describeMergeOutcome(
  mergePlan: WorkerSnapshot["mergePlan"],
  mergeResult: WorkerSnapshot["mergeResult"],
): string {
  if (mergeResult) {
    return `${mergeResult.adoptedCount} adopted, ${mergeResult.conflictCount} conflict regions, ${mergeResult.replacedCount} replacements.`;
  }
  if (mergePlan) {
    return `${mergePlan.candidateCount} artifacts are queued for review.`;
  }
  return "No merge data yet.";
}

export function buildMergeDecisionSteps(
  mergeReview: MergeReviewSnapshot | null,
  mergePlan: WorkerSnapshot["mergePlan"],
  mergeResult: WorkerSnapshot["mergeResult"],
): MergeDecisionStep[] {
  if (!mergeReview) {
    return [];
  }

  const topologyHighlights = topologyHighlightsFor(mergeReview.source.state, mergeReview.target.state);
  const lightingHighlights = lightingHighlightsFor(mergeReview.source.state, mergeReview.target.state);
  const motionHighlights = motionHighlightsFor(mergeReview.source.state, mergeReview.target.state);

  const steps: MergeDecisionStep[] = [];
  if (topologyHighlights) {
    steps.push(buildGroupedStep("topology", topologyHighlights, mergeReview, mergePlan, mergeResult));
  }
  if (lightingHighlights) {
    steps.push(buildGroupedStep("lighting", lightingHighlights, mergeReview, mergePlan, mergeResult));
  }
  if (motionHighlights) {
    steps.push(buildGroupedStep("motion", motionHighlights, mergeReview, mergePlan, mergeResult));
  }

  if (steps.length === 0) {
    steps.push(
      buildGroupedStep(
        "mixed",
        {
          focusLabel: "Merged world",
          sourceHighlights: ["Source and target remained visually aligned."],
          targetHighlights: ["No visible branch divergence remained."],
        },
        mergeReview,
        mergePlan,
        mergeResult,
      ),
    );
  }

  return steps;
}

export function metricsForAspect(
  state: SceneState | null,
  aspectKind: MergeDecisionStep["aspectKind"],
): string[] {
  if (!state) {
    return ["Awaiting decision"];
  }
  switch (aspectKind) {
    case "topology":
      return [
        `${state.gear.teeth} teeth`,
        `R${state.gear.outerRadius.toFixed(2)} / ${state.gear.innerRadius.toFixed(2)}`,
      ];
    case "lighting":
      return [
        `Intensity ${state.light.intensity.toFixed(2)}`,
        `Light ${state.light.x.toFixed(1)}, ${state.light.y.toFixed(1)}, ${state.light.z.toFixed(1)}`,
      ];
    case "motion":
      return [
        `Rotation ${signed(state.gear.rotation)} rad`,
        `Camera ${state.camera.x.toFixed(1)}, ${state.camera.y.toFixed(1)}, ${state.camera.z.toFixed(1)}`,
      ];
    case "mixed":
    default:
      return [
        `${state.gear.teeth} teeth`,
        `Light ${state.light.intensity.toFixed(2)}`,
        `Rotation ${signed(state.gear.rotation)} rad`,
      ];
  }
}

export function shortDigest(value: string | null): string {
  if (!value) {
    return "pending";
  }
  return `${value.slice(0, 12)}...`;
}

function buildLane(
  preview: NonNullable<MergeReviewSnapshot["previews"]>[number],
  mergePlan: WorkerSnapshot["mergePlan"],
  mergeResult: WorkerSnapshot["mergeResult"],
  aspectKind: MergeDecisionStep["aspectKind"],
): MergeDecisionLane {
  const hasConflictShape = hasConflictForAspect(preview.plan ?? mergePlan, mergeResult, aspectKind);
  const manual = preview.visualMode === "manual-review";
  return {
    id: preview.id,
    label: preview.label,
    accent: preview.accent,
    frameId: preview.frameId,
    visualMode: preview.visualMode,
    resultState: preview.resultState,
    policyFamily: lanePolicyFamily(preview),
    policyLabel: lanePolicyLabel(preview),
    statusLabel: manual ? "Manual review" : hasConflictShape ? "Auto-merge" : "Safe merge",
    actionLabel: laneOutcomeLabel(preview.plan ?? mergePlan, aspectKind, manual),
    manual,
  };
}

function laneOutcomeLabel(
  plan: WorkerSnapshot["mergePlan"],
  aspectKind: MergeDecisionStep["aspectKind"],
  manual: boolean,
) {
  if (manual) {
    return "Decision stops here";
  }
  const decision = plan?.aspectDecisions.find((record) => classifyAspectKind([record.aspect]) === aspectKind);
  if (!decision) {
    return aspectKind === "lighting" || aspectKind === "motion"
      ? "Both edits stay admissible"
      : "Source shape wins cleanly";
  }
  return humanizeDecisionOutcome(decision.outcome);
}

function hasConflictForAspect(
  plan: WorkerSnapshot["mergePlan"],
  mergeResult: WorkerSnapshot["mergeResult"],
  aspectKind: MergeDecisionStep["aspectKind"],
) {
  const isolated = (plan?.conflictIsolation.records ?? mergeResult?.conflictIsolation.records ?? []).flatMap(
    (record) => record.isolatedAspects,
  );
  return isolated.some((aspect) => classifyAspectKind([aspect]) === aspectKind);
}

function classifyAspectKind(aspects: string[]): MergeDecisionStep["aspectKind"] {
  const normalized = aspects.map((aspect) => aspect.toLowerCase());
  const hasTopology = normalized.some((aspect) =>
    aspect.includes("gear") || aspect.includes("profile") || aspect.includes("topology") || aspect.includes("mesh"),
  );
  const hasLighting = normalized.some((aspect) => aspect.includes("light") || aspect.includes("shading"));
  const hasMotion = normalized.some((aspect) => aspect.includes("rotation") || aspect.includes("projection"));
  const count = Number(hasTopology) + Number(hasLighting) + Number(hasMotion);
  if (count > 1) return "mixed";
  if (hasLighting) return "lighting";
  if (hasMotion) return "motion";
  return "topology";
}

function titleForAspect(aspectKind: MergeDecisionStep["aspectKind"], index: number) {
  switch (aspectKind) {
    case "topology":
      return `Geometry ${index + 1}`;
    case "lighting":
      return `Light ${index + 1}`;
    case "motion":
      return `Motion ${index + 1}`;
    case "mixed":
    default:
      return `Shared surface ${index + 1}`;
  }
}

function buildDecisionTitle(
  aspectKind: MergeDecisionStep["aspectKind"],
  focusLabel: string,
) {
  const prefix = titleForAspect(aspectKind, 0).replace(/\s\d+$/, "");
  return `${prefix} / ${focusLabel}`;
}

function focusLabelForAspect(
  aspectKind: MergeDecisionStep["aspectKind"],
  focusLabel: string,
) {
  if (focusLabel) return focusLabel;
  switch (aspectKind) {
    case "topology":
      return "Gear geometry";
    case "lighting":
      return "Light field";
    case "motion":
      return "Motion and projection";
    case "mixed":
    default:
      return "Shared artifact";
  }
}

function humanizeDecisionOutcome(value: string) {
  return value
    .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
    .replace(/[_-]+/g, " ")
    .trim();
}

function signed(value: number) {
  return `${value >= 0 ? "+" : ""}${value.toFixed(2)}`;
}

function aspectTag(aspectKind: MergeDecisionStep["aspectKind"]) {
  switch (aspectKind) {
    case "topology":
      return "Geometry";
    case "lighting":
      return "Light";
    case "motion":
      return "Motion";
    case "mixed":
    default:
      return "Shared";
  }
}

function buildGroupedStep(
  aspectKind: MergeDecisionStep["aspectKind"],
  detail: {
    focusLabel: string;
    sourceHighlights: string[];
    targetHighlights: string[];
  },
  mergeReview: MergeReviewSnapshot,
  mergePlan: WorkerSnapshot["mergePlan"],
  mergeResult: WorkerSnapshot["mergeResult"],
): MergeDecisionStep {
  return {
    id: `decision-${aspectKind}`,
    title: buildDecisionTitle(aspectKind, detail.focusLabel),
    trailLabel: `${aspectTag(aspectKind)} / ${detail.focusLabel}`,
    focusLabel: focusLabelForAspect(aspectKind, detail.focusLabel),
    aspectKind,
    sourceHighlights: detail.sourceHighlights,
    targetHighlights: detail.targetHighlights,
    sourceMetrics: metricsForAspect(mergeReview.source.state, aspectKind),
    targetMetrics: metricsForAspect(mergeReview.target.state, aspectKind),
    lanes: mergeReview.previews.map((preview) => buildLane(preview, mergePlan, mergeResult, aspectKind)),
  };
}

function topologyHighlightsFor(source: SceneState, target: SceneState) {
  const changes: Array<{ label: string; source: string; target: string }> = [];
  if (source.gear.teeth !== target.gear.teeth) {
    changes.push({ label: "Teeth", source: `${source.gear.teeth}`, target: `${target.gear.teeth}` });
  }
  if (source.gear.outerRadius !== target.gear.outerRadius) {
    changes.push({
      label: "Outer radius",
      source: source.gear.outerRadius.toFixed(2),
      target: target.gear.outerRadius.toFixed(2),
    });
  }
  if (source.gear.innerRadius !== target.gear.innerRadius) {
    changes.push({
      label: "Inner radius",
      source: source.gear.innerRadius.toFixed(2),
      target: target.gear.innerRadius.toFixed(2),
    });
  }
  if (source.gear.thickness !== target.gear.thickness) {
    changes.push({
      label: "Thickness",
      source: source.gear.thickness.toFixed(2),
      target: target.gear.thickness.toFixed(2),
    });
  }
  if (changes.length === 0) return null;
  return {
    focusLabel: "Gear shape",
    sourceHighlights: changes.map((change) => `${change.label}: ${change.source}`),
    targetHighlights: changes.map((change) => `${change.label}: ${change.target}`),
  };
}

function lightingHighlightsFor(source: SceneState, target: SceneState) {
  const changes: Array<{ label: string; source: string; target: string }> = [];
  if (source.light.intensity !== target.light.intensity) {
    changes.push({
      label: "Intensity",
      source: source.light.intensity.toFixed(2),
      target: target.light.intensity.toFixed(2),
    });
  }
  if (source.light.x !== target.light.x || source.light.y !== target.light.y || source.light.z !== target.light.z) {
    changes.push({
      label: "Position",
      source: `${source.light.x.toFixed(1)}, ${source.light.y.toFixed(1)}, ${source.light.z.toFixed(1)}`,
      target: `${target.light.x.toFixed(1)}, ${target.light.y.toFixed(1)}, ${target.light.z.toFixed(1)}`,
    });
  }
  if (changes.length === 0) return null;
  return {
    focusLabel: "Lighting",
    sourceHighlights: changes.map((change) => `${change.label}: ${change.source}`),
    targetHighlights: changes.map((change) => `${change.label}: ${change.target}`),
  };
}

function motionHighlightsFor(source: SceneState, target: SceneState) {
  const changes: Array<{ label: string; source: string; target: string }> = [];
  if (source.gear.rotation !== target.gear.rotation) {
    changes.push({
      label: "Rotation",
      source: signed(source.gear.rotation),
      target: signed(target.gear.rotation),
    });
  }
  if (
    source.camera.x !== target.camera.x
    || source.camera.y !== target.camera.y
    || source.camera.z !== target.camera.z
    || source.camera.yaw !== target.camera.yaw
    || source.camera.pitch !== target.camera.pitch
  ) {
    changes.push({
      label: "Camera",
      source: `${source.camera.x.toFixed(1)}, ${source.camera.y.toFixed(1)}, ${source.camera.z.toFixed(1)}`,
      target: `${target.camera.x.toFixed(1)}, ${target.camera.y.toFixed(1)}, ${target.camera.z.toFixed(1)}`,
    });
  }
  if (changes.length === 0) return null;
  return {
    focusLabel: "Motion",
    sourceHighlights: changes.map((change) => `${change.label}: ${change.source}`),
    targetHighlights: changes.map((change) => `${change.label}: ${change.target}`),
  };
}

function lanePolicyFamily(preview: NonNullable<MergeReviewSnapshot["previews"]>[number]) {
  if (preview.id === "strict") {
    return "Conflict policy override";
  }
  if (preview.id === "perAspect") {
    return "Isolation policy override";
  }
  return "Executed merge stack";
}

function lanePolicyLabel(preview: NonNullable<MergeReviewSnapshot["previews"]>[number]) {
  if (!preview.plan) {
    return preview.label;
  }
  if (preview.id === "perAspect") {
    return preview.plan.semantics.conflictIsolationName ?? friendlyPolicy(
      preview.plan.semantics.conflictIsolationName,
      preview.plan.semantics.conflictIsolationBasis,
    );
  }
  return preview.plan.semantics.conflictPolicyName ?? friendlyPolicy(
    preview.plan.semantics.conflictPolicyName,
    preview.plan.semantics.conflictPolicyBasis,
  );
}
