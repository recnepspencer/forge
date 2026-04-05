import type { WorkerSnapshot } from "../gear-scene/worker/protocol";
import { formatNanos } from "../state/timeline_view";

export function HudOverlay({
  graphNodes,
  summary,
  suppression,
  tileCount,
  dirtyTiles,
  uploadSpans,
}: {
  graphNodes: number;
  summary: WorkerSnapshot["latestSummary"];
  suppression: string;
  tileCount: number;
  dirtyTiles: number;
  uploadSpans: number;
}) {
  return (
    <div className="hud-overlay">
      <span className="hud-badge">{graphNodes} nodes</span>
      <span className="hud-badge">{tileCount} tiles</span>
      <span className="hud-badge">{dirtyTiles} dirty</span>
      <span className="hud-badge">{uploadSpans} upload spans</span>
      <span className="hud-badge">{suppression}% suppressed</span>
      <span className="hud-badge">{summary?.nodesEvaluated ?? 0} evaluated</span>
      <span className="hud-badge">{formatNanos(Number(summary?.totalNanos ?? 0))}</span>
    </div>
  );
}

export function HudStat({ label, value }: { label: string; value: string }) {
  return (
    <div className="hud-cell">
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}
