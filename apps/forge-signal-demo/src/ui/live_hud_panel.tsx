import type { WorkerSnapshot } from "../gear-scene/worker/protocol";
import { formatNanos } from "../state/timeline_view";
import { HudStat } from "./hud_panels";

export function LiveHudPanel({
  latestSummary,
  suppression,
  branchesCount,
  frameIndex,
}: {
  latestSummary: WorkerSnapshot["latestSummary"];
  suppression: string;
  branchesCount: number;
  frameIndex: number;
}) {
  return (
    <section className="panel">
      <span className="panel__eyebrow">Live HUD</span>
      <dl className="hud-grid">
        <HudStat label="Evaluated" value={String(latestSummary?.nodesEvaluated ?? 0)} />
        <HudStat label="Suppressed" value={`${suppression}%`} />
        <HudStat label="Touched" value={String(latestSummary?.touchedNodes ?? 0)} />
        <HudStat label="Last run" value={formatNanos(Number(latestSummary?.totalNanos ?? 0))} />
        <HudStat label="Branches" value={String(branchesCount)} />
        <HudStat label="Frame" value={String(frameIndex)} />
      </dl>
    </section>
  );
}
