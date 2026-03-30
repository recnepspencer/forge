export type GearParams = {
  teeth: number;
  outerRadius: number;
  innerRadius: number;
  rotation: number;
};

export function buildGearPath({
  teeth,
  outerRadius,
  innerRadius,
  rotation,
}: GearParams): string {
  const toothCount = Math.max(6, Math.round(teeth));
  const steps = toothCount * 2;
  const angleStep = (Math.PI * 2) / steps;
  const rotationRad = (rotation * Math.PI) / 180;
  const points: string[] = [];

  for (let index = 0; index < steps; index += 1) {
    const angle = rotationRad + index * angleStep;
    const radius = index % 2 === 0 ? outerRadius : innerRadius;
    const x = 150 + Math.cos(angle) * radius;
    const y = 150 + Math.sin(angle) * radius;
    points.push(`${index === 0 ? "M" : "L"} ${x.toFixed(2)} ${y.toFixed(2)}`);
  }

  return `${points.join(" ")} Z`;
}

export function gearHoleRadius(innerRadius: number): number {
  return Math.max(14, innerRadius * 0.42);
}
