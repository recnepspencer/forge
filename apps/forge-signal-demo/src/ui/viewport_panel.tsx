import { useCallback } from "react";

import { RENDER_HEIGHT, RENDER_WIDTH } from "../gear-scene/core/types";
import type { WorkerSnapshot } from "../gear-scene/worker/protocol";

export function Viewport({
  branch,
  active,
  bitmap,
  frameVersion,
  onActivate,
}: {
  branch: WorkerSnapshot["branches"][number];
  active: boolean;
  bitmap: ImageBitmap | null;
  frameVersion: number;
  onActivate: () => void;
}) {
  const drawCanvas = useCallback(
    (canvas: HTMLCanvasElement | null) => {
      if (!canvas) return;
      const ctx = canvas.getContext("2d");
      if (!ctx) return;
      ctx.clearRect(0, 0, RENDER_WIDTH, RENDER_HEIGHT);
      if (!bitmap) return;
      try {
        ctx.drawImage(bitmap, 0, 0, RENDER_WIDTH, RENDER_HEIGHT);
      } catch (error) {
        if (!(error instanceof DOMException) || error.name !== "InvalidStateError") {
          throw error;
        }
      }
    },
    [bitmap, frameVersion],
  );

  return (
    <button type="button" className={`vp ${active ? "vp--active" : ""}`} onClick={onActivate}>
      <div className="vp__badge-rail">
        <span className="vp__name">{branch.name}</span>
        <span className="vp__pill">{branch.state.gear.teeth} teeth</span>
        <span className="vp__pill">frame {branch.hud.frameIndex}</span>
      </div>
      <canvas ref={drawCanvas} className="vp__canvas" width={RENDER_WIDTH} height={RENDER_HEIGHT} />
      <div className="vp__footer">
        <div className="vp__footer-copy">
          <span className="vp__footer-label">{active ? "Active branch" : "Tap to inspect"}</span>
          <strong className="vp__footer-title">{branch.name === "what-if" ? "Counterfactual branch" : "Mainline branch"}</strong>
        </div>
        <span className={`vp__state ${active ? "vp__state--active" : ""}`}>
          {active ? "Live" : "Standby"}
        </span>
      </div>
    </button>
  );
}
