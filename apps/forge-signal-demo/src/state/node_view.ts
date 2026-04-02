export type TreeNode = { id: string; label: string; children: string[] };

export const NODES: Record<string, TreeNode> = {
  gearTeeth: { id: "gearTeeth", label: "Teeth", children: ["gearDimensionsModel"] },
  gearOuterRadius: { id: "gearOuterRadius", label: "Outer radius", children: ["gearDimensionsModel"] },
  gearInnerRadius: { id: "gearInnerRadius", label: "Inner radius", children: ["gearDimensionsModel"] },
  gearThickness: { id: "gearThickness", label: "Thickness", children: ["gearDimensionsModel"] },
  gearRotation: { id: "gearRotation", label: "Rotation", children: ["gearDimensionsModel"] },
  lightIntensity: { id: "lightIntensity", label: "Light", children: ["lightingModel"] },
  gearDimensionsModel: { id: "gearDimensionsModel", label: "Dimensions", children: ["gearProfileModel", "gearMeshModel"] },
  gearProfileModel: { id: "gearProfileModel", label: "Profile", children: ["gearTopologyModel"] },
  gearTopologyModel: { id: "gearTopologyModel", label: "Topology", children: ["gearMeshModel"] },
  gearMeshModel: { id: "gearMeshModel", label: "Mesh", children: ["viewportProjectionModel"] },
  lightingModel: { id: "lightingModel", label: "Lighting", children: ["viewportShadingModel"] },
  viewportProjectionModel: { id: "viewportProjectionModel", label: "Projection", children: ["viewportShadingModel"] },
  viewportShadingModel: { id: "viewportShadingModel", label: "Shading", children: ["hudModel"] },
  hudModel: { id: "hudModel", label: "HUD", children: [] },
};

export const STATIC_LAYERS = [
  { label: "Sources", ids: ["gearTeeth", "gearOuterRadius", "gearInnerRadius", "gearThickness", "gearRotation", "lightIntensity"] },
  { label: "Derived", ids: ["gearDimensionsModel", "gearProfileModel", "gearTopologyModel"] },
  { label: "Render", ids: ["gearMeshModel", "lightingModel", "viewportProjectionModel", "viewportShadingModel"] },
  { label: "Output", ids: ["hudModel"] },
];

export function buildToothNodes(count: number): Array<{ id: string; label: string }> {
  return Array.from({ length: count }, (_, i) => ({
    id: `gearToothModel::tooth-${i}`,
    label: `Tooth ${i}`,
  }));
}

export function nodeLabel(id: string): string {
  if (NODES[id]) return NODES[id].label;
  const match = id.match(/^gearToothModel::tooth-(\d+)$/);
  if (match) return `Tooth ${match[1]}`;
  return id;
}

export function getAncestors(id: string): string[] {
  const ancestors: string[] = [];
  for (const [nodeId, node] of Object.entries(NODES)) {
    if (node.children.includes(id)) ancestors.push(nodeId);
  }
  if (id.startsWith("gearToothModel::")) {
    return ["gearDimensionsModel", "gearProfileModel"];
  }
  return ancestors;
}

export function getDescendants(id: string): string[] {
  return NODES[id]?.children ?? [];
}

export function nodeZone(id: string): "topology" | "render" | "output" | "source" {
  if (
    id === "gearDimensionsModel"
    || id === "gearProfileModel"
    || id === "gearTopologyModel"
    || id === "gearMeshModel"
    || id.startsWith("gearToothModel::")
  ) {
    return "topology";
  }
  if (
    id === "lightingModel"
    || id === "viewportProjectionModel"
    || id === "viewportShadingModel"
  ) {
    return "render";
  }
  if (id === "hudModel") {
    return "output";
  }
  return "source";
}

export function nodeChipClassName(
  id: string,
  isSelected: boolean,
  conflictedNodes: Set<string>,
  resolvedNodes: Set<string>,
) {
  const classes = ["node-chip"];
  const zone = nodeZone(id);
  if (zone === "topology") classes.push("node-chip--topology");
  if (zone === "render") classes.push("node-chip--render");
  if (zone === "output") classes.push("node-chip--output");
  if (conflictedNodes.has(id)) classes.push("node-chip--conflict");
  if (resolvedNodes.has(id)) classes.push("node-chip--resolved");
  if (isSelected) classes.push("node-chip--active");
  return classes.join(" ");
}
