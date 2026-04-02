import type { ScenePatch } from "../gear-scene/core/types";
import type { WorkerSnapshot } from "../gear-scene/worker/protocol";
import { humanLabel, formatNanos } from "../state/timeline_view";
import { CommitGraph } from "./commit_graph_panel";
import { HudStat } from "./hud_panels";
import { NodeTree } from "./signal_graph_tree";
import { Slider } from "./slider_control";

export function TimelinePanel({
  timeline,
  timelineIndex,
  onScrub,
}: {
  timeline: WorkerSnapshot["timeline"];
  timelineIndex: number;
  onScrub: (index: number) => void;
}) {
  return (
    <section className="timeline">
      <div className="timeline__head">
        <span className="timeline__label">Timeline</span>
        <span className="timeline__badge">
          {timeline.length > 0 ? `${timelineIndex + 1}/${timeline.length}` : "-"}
        </span>
      </div>
      <input
        className="timeline__slider"
        type="range"
        min={0}
        max={Math.max(timeline.length - 1, 0)}
        step={1}
        value={Math.min(timelineIndex, Math.max(timeline.length - 1, 0))}
        disabled={timeline.length === 0}
        onChange={(e) => onScrub(Number(e.target.value))}
      />
      <div className="timeline__scroll">
        <CommitGraph timeline={timeline} activeIndex={timelineIndex} onScrub={onScrub} />
        <div className="timeline__rail" role="list">
          {timeline.map((entry, i) => {
            const prev = timeline[i - 1];
            const hasFork = entry.branchCount > 1 || (prev?.branchCount ?? 1) > 1;
            return (
              <button
                key={`${entry.frameIndex}-${i}`}
                className={`tl-dot ${i === timelineIndex ? "tl-dot--active" : ""} ${hasFork ? "tl-dot--fork" : ""}`}
                onClick={() => onScrub(i)}
                type="button"
                role="listitem"
                title={`${humanLabel(entry.label)} - frame ${entry.frameIndex}`}
              >
                <span className="tl-dot__pip" />
                {hasFork && <span className="tl-dot__branch" />}
                <span className="tl-dot__text">{humanLabel(entry.label)}</span>
              </button>
            );
          })}
        </div>
      </div>
    </section>
  );
}

export function ControlsPanel({
  controlsOpen,
  activeBranch,
  onToggle,
  onPatch,
}: {
  controlsOpen: boolean;
  activeBranch: WorkerSnapshot["branches"][number] | null;
  onToggle: () => void;
  onPatch: (patch: ScenePatch, label: string) => void;
}) {
  return (
    <section className="panel">
      <button className="panel__toggle" type="button" onClick={onToggle}>
        <span className="panel__eyebrow">Controls</span>
        <span className="panel__chevron">{controlsOpen ? "v" : ">"}</span>
      </button>
      {controlsOpen && activeBranch && (
        <div key={activeBranch.id} className="sliders">
          <Slider label="Teeth" value={activeBranch.state.gear.teeth} min={8} max={32} step={1} fmt={(v) => `${v}`} onChange={(v) => onPatch({ gear: { teeth: Math.round(v) } }, "teeth")} />
          <Slider label="Outer radius" value={activeBranch.state.gear.outerRadius} min={0.8} max={1.9} step={0.01} onChange={(v) => onPatch({ gear: { outerRadius: v } }, "outer")} />
          <Slider label="Inner radius" value={activeBranch.state.gear.innerRadius} min={0.18} max={Math.max(activeBranch.state.gear.outerRadius - 0.12, 0.19)} step={0.01} onChange={(v) => onPatch({ gear: { innerRadius: v } }, "inner")} />
          <Slider label="Thickness" value={activeBranch.state.gear.thickness} min={0.1} max={0.5} step={0.01} onChange={(v) => onPatch({ gear: { thickness: v } }, "thickness")} />
          <Slider label="Rotation" value={activeBranch.state.gear.rotation} min={-Math.PI} max={Math.PI} step={0.01} onChange={(v) => onPatch({ gear: { rotation: v } }, "rotation")} />
          <Slider label="Light" value={activeBranch.state.light.intensity} min={0.4} max={2.2} step={0.01} onChange={(v) => onPatch({ light: { intensity: v } }, "light")} />
        </div>
      )}
    </section>
  );
}

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

export function SignalGraphPanel({
  teethCount,
  tracedNode,
  conflictedNodes,
  resolvedNodes,
  onInspect,
}: {
  teethCount: number;
  tracedNode: string | null;
  conflictedNodes: Set<string>;
  resolvedNodes: Set<string>;
  onInspect: (id: string) => void;
}) {
  return (
    <section className="panel">
      <span className="panel__eyebrow">Signal Graph</span>
      <p className="panel__hint">Tap a node to explore its lineage.</p>
      <NodeTree
        teethCount={teethCount}
        tracedNode={tracedNode}
        conflictedNodes={conflictedNodes}
        resolvedNodes={resolvedNodes}
        onInspect={onInspect}
      />
    </section>
  );
}
