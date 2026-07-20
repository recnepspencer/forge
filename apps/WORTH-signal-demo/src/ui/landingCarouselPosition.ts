export type LandingCarouselPosition = {
  filter: string;
  opacity: number;
  pointerEvents: "auto" | "none";
  rotateY: number;
  scale: number;
  x: string;
  z: number;
  zIndex: number;
};

type LandingCarouselPositionInput = {
  narrow: boolean;
  reducedMotion: boolean;
  relative: number;
};

const activePosition: LandingCarouselPosition = {
  filter: "blur(0px) saturate(1)",
  opacity: 1,
  pointerEvents: "auto",
  rotateY: 0,
  scale: 1,
  x: "0%",
  z: 0,
  zIndex: 5,
};

export function getLandingCarouselPosition({
  narrow,
  reducedMotion,
  relative,
}: LandingCarouselPositionInput): LandingCarouselPosition {
  if (reducedMotion) {
    return relative === 0
      ? activePosition
      : { ...activePosition, opacity: 0, pointerEvents: "none", x: `${relative * 100}%`, zIndex: 1 };
  }

  if (relative === 0) return activePosition;

  if (relative === -1 || relative === 1) {
    const direction = relative < 0 ? -1 : 1;
    return {
      filter: narrow ? "blur(2px) saturate(0.9)" : "blur(5px) saturate(0.78)",
      opacity: narrow ? 0.68 : 0.42,
      pointerEvents: "none",
      rotateY: direction * (narrow ? -58 : -68),
      scale: narrow ? 0.84 : 0.82,
      x: narrow ? `${direction * 72}%` : `${direction * 42}rem`,
      z: narrow ? -160 : -240,
      zIndex: 3,
    };
  }

  const direction = relative < 0 ? -1 : 1;
  return {
    filter: narrow ? "blur(8px) saturate(0.62)" : "blur(12px) saturate(0.55)",
    opacity: 0.08,
    pointerEvents: "none",
    rotateY: direction * (narrow ? -72 : -78),
    scale: narrow ? 0.72 : 0.68,
    x: narrow ? `${direction * 96}%` : `${direction * 54}rem`,
    z: narrow ? -320 : -420,
    zIndex: 1,
  };
}
