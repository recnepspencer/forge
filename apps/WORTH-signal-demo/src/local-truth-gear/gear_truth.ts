export interface GearTruth {
  innerRadius: number;
  outerRadius: number;
  thickness: number;
  teeth: number;
  material: "steel" | "titanium" | "ceramic";
  rotation: number;
  label: string;
}

export type GearDesignAspect = "thickness" | "teeth" | "innerRadius";

export const initialGearTruth: GearTruth = Object.freeze({
  innerRadius: 0.62,
  outerRadius: 1.78,
  thickness: 0.58,
  teeth: 18,
  material: "steel",
  rotation: 0,
  label: "Drive gear",
});

export const gearAspectMap = Object.freeze({
  innerRadius: 0,
  outerRadius: 1,
  thickness: 2,
  teeth: 3,
  material: 4,
  rotation: 5,
  label: 6,
});
