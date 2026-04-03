import type { ScenePatch } from "../gear-scene/core/types";
import type { WorkerSnapshot } from "../gear-scene/worker/protocol";
import { Slider } from "./slider_control";

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
