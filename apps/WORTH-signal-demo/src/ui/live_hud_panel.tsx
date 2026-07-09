import type { WorkerSnapshot } from "../gear-scene/worker/protocol";
import { formatNanos } from "../state/timeline_view";
import { HudStat } from "./hud_panels";

export function LiveHudPanel({
  latestSummary,
  suppression,
  branchesCount,
  frameIndex,
  tileCount,
  tileGrid,
  dirtyTiles,
  uploadedTiles,
  uploadSpans,
  uploadBytes,
  changedDetails,
}: {
  latestSummary: WorkerSnapshot["latestSummary"];
  suppression: string;
  branchesCount: number;
  frameIndex: number;
  tileCount: number;
  tileGrid: string;
  dirtyTiles: number;
  uploadedTiles: number;
  uploadSpans: number;
  uploadBytes: number;
  changedDetails: number;
}) {
  return (
    <section className="panel">
      <span className="panel__eyebrow">Live HUD</span>
      <dl className="hud-grid">
        <HudStat label="Evaluated" value={String(latestSummary?.nodesEvaluated ?? 0)} />
        <HudStat label="Tiles" value={String(tileCount)} />
        <HudStat label="Grid" value={tileGrid} />
        <HudStat label="Suppressed" value={`${suppression}%`} />
        <HudStat label="Touched" value={String(latestSummary?.touchedNodes ?? 0)} />
        <HudStat label="Last run" value={formatNanos(Number(latestSummary?.totalNanos ?? 0))} />
        <HudStat label="Branches" value={String(branchesCount)} />
        <HudStat label="Frame" value={String(frameIndex)} />
        <HudStat label="Changed details" value={String(changedDetails)} />
        <HudStat label="Dirty tiles" value={String(dirtyTiles)} />
        <HudStat label="Uploaded tiles" value={String(uploadedTiles)} />
        <HudStat label="Upload spans" value={String(uploadSpans)} />
        <HudStat label="Upload bytes" value={`${(uploadBytes / 1024).toFixed(1)} KB`} />
      </dl>
    </section>
  );
}
