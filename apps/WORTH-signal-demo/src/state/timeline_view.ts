import type { WorkerSnapshot } from "../gear-scene/worker/protocol";
import { nodeLabel } from "./node_view";

export function humanLabel(label: string): string {
  switch (label) {
    case "boot": return "Boot";
    case "branch": return "Branch";
    case "merge": return "Merge";
    case "scenario-main-topology": return "Main topo";
    case "scenario-feature-topology": return "What-if topo";
    case "scenario-main-render": return "Main render";
    case "scenario-feature-render": return "What-if render";
    case "teeth": return "Teeth";
    case "outer": return "Outer";
    case "inner": return "Inner";
    case "thickness": return "Thick";
    case "rotation": return "Rot";
    case "light": return "Light";
    default: return label;
  }
}

export function humanLongLabel(label: string): string {
  switch (label) {
    case "boot": return "initial boot";
    case "branch": return "branch creation";
    case "merge": return "merge";
    case "scenario-main-topology": return "main-branch topology pressure";
    case "scenario-feature-topology": return "what-if topology pressure";
    case "scenario-main-render": return "main-branch render pressure";
    case "scenario-feature-render": return "what-if render pressure";
    case "teeth": return "Teeth";
    case "outer": return "Outer radius";
    case "inner": return "Inner radius";
    case "thickness": return "Thickness";
    case "rotation": return "Rotation";
    case "light": return "Light intensity";
    default: return label;
  }
}

export function formatNanos(value: number) {
  if (!Number.isFinite(value) || value <= 0) return "0 ms";
  if (value >= 1_000_000) return `${(value / 1_000_000).toFixed(2)} ms`;
  if (value >= 1_000) return `${(value / 1_000).toFixed(1)} us`;
  return `${value.toFixed(0)} ns`;
}

function isSourceNode(nodeId: string): boolean {
  return [
    "gearTeeth",
    "gearOuterRadius",
    "gearInnerRadius",
    "gearThickness",
    "gearRotation",
    "lightIntensity",
  ].includes(nodeId);
}

export function explainCommitForNode(nodeId: string, label: string): string {
  const node = nodeLabel(nodeId);
  const change = humanLongLabel(label);

  if (isSourceNode(nodeId)) {
    return `${node} changed directly via ${change}.`;
  }

  if (nodeId.startsWith("gearToothModel::")) {
    if (label === "teeth") {
      return `${node} was re-derived because Teeth changed.`;
    }
    return `${node} was re-derived after ${change}.`;
  }

  if (nodeId === "gearDimensionsModel") return `Gear dimensions recomputed after ${change}.`;
  if (nodeId === "gearProfileModel") return `Gear profile recomputed from updated dimensions after ${change}.`;
  if (nodeId === "gearTopologyModel") return `Gear topology updated after ${change}.`;
  if (nodeId === "gearMeshModel") return `Gear mesh regenerated after ${change}.`;
  if (nodeId === "lightingModel") return `Lighting updated after ${change}.`;
  if (nodeId === "viewportProjectionModel") return `Projection refreshed after ${change}.`;
  if (nodeId === "viewportShadingModel") return `Shading refreshed after ${change}.`;
  if (nodeId === "hudModel") return `HUD summary updated after ${change}.`;

  return `${node} was touched after ${change}.`;
}

export function buildTraceStory(
  nodeId: string,
  timeline: WorkerSnapshot["timeline"],
  touchedIndices: number[],
  lineageEventCount: number,
) {
  const recent = touchedIndices.slice(-4).reverse();
  const steps = recent.map((idx) => {
    const entry = timeline[idx];
    const title = explainCommitForNode(nodeId, entry.label);
    const detail = `${entry.branchName ?? "main"} · frame ${entry.frameIndex} · commit ${idx + 1}/${timeline.length}`;
    return { commitIndex: idx, title, detail };
  });

  if (steps.length === 0 && lineageEventCount > 0) {
    return [
      {
        commitIndex: -1,
        title: `${nodeLabel(nodeId)} has recorded lineage`,
        detail: `${lineageEventCount} runtime lineage events were captured for this node.`,
      },
    ];
  }

  return steps;
}

export function buildWhyProse(
  nodeName: string,
  state: string,
  suppressed: boolean,
  upstream: string[],
) {
  if (suppressed) {
    return `${nodeName} was checked, but propagation was suppressed because its output didn't change.`;
  }
  if (upstream.length === 0) {
    return `${nodeName} is a source node with no upstream dependencies.`;
  }
  const depNames = upstream
    .slice(0, 3)
    .map((id) => nodeLabel(id))
    .join(", ");
  const extra = upstream.length > 3 ? ` and ${upstream.length - 3} more` : "";
  return `${nodeName} is ${state} because ${depNames}${extra} changed upstream. ${suppressed ? "Output unchanged, so propagation stopped here." : "Its value was recalculated and propagated forward."}`;
}

export function humanWhyState(state: string): string {
  if (/clean/i.test(state)) return "clean";
  if (/dirty/i.test(state)) return "dirty";
  if (/stale/i.test(state)) return "stale";
  return state;
}
