export interface GearParams {
  innerRadius: number;
  outerRadius: number;
  thickness: number;
  teeth: number;
}

export type GearParamKey = keyof GearParams;

export interface GearParamSpec {
  key: GearParamKey;
  label: string;
  min: number;
  max: number;
  step: number;
}

export interface RuntimeTimelineBookmark {
  id: string;
  parentIds: readonly string[];
  branchId: number;
  branchName: string;
  snapshotId: number | null;
  snapshot: unknown;
  label: string;
  params: GearParams;
}

export const initialGearParams: GearParams = {
  innerRadius: 0.62,
  outerRadius: 1.78,
  thickness: 0.58,
  teeth: 18,
};

export const gearParamSpecs: readonly GearParamSpec[] = [
  { key: "innerRadius", label: "Inner radius", min: 0.25, max: 1.2, step: 0.01 },
  { key: "outerRadius", label: "Outer radius", min: 1.25, max: 2.35, step: 0.01 },
  { key: "thickness", label: "Thickness", min: 0.18, max: 1.2, step: 0.01 },
  { key: "teeth", label: "Teeth", min: 8, max: 36, step: 1 },
];

export function normalizeGearParams(params: GearParams): GearParams {
  return {
    innerRadius: Math.min(params.outerRadius - 0.2, Math.max(0.18, params.innerRadius)),
    outerRadius: Math.max(params.innerRadius + 0.2, params.outerRadius),
    thickness: Math.max(0.1, params.thickness),
    teeth: Math.round(Math.max(4, params.teeth)),
  };
}
