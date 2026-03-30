import { useEffect, useRef } from "react";

import { RENDER_HEIGHT, RENDER_WIDTH } from "../core/types";

export function SceneCanvas({
  pixels,
  label,
}: {
  pixels: Uint8ClampedArray | null;
  label: string;
}) {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  useEffect(() => {
    if (!pixels || !canvasRef.current) {
      return;
    }

    const context = canvasRef.current.getContext("2d");
    if (!context) {
      return;
    }

    const image = new ImageData(new Uint8ClampedArray(pixels), RENDER_WIDTH, RENDER_HEIGHT);
    context.putImageData(image, 0, 0);
  }, [pixels]);

  return (
    <canvas
      ref={canvasRef}
      className="scene-canvas"
      width={RENDER_WIDTH}
      height={RENDER_HEIGHT}
      aria-label={label}
    />
  );
}
