import React from "react";
import { DemoShell } from "../DemoShell";
import { useSignal } from "../Demos";

interface DemoOneProps {
  signals: any;
  demo: any;
  onNavigate: any;
}

export const DemoOne: React.FC<DemoOneProps> = ({ signals, demo, onNavigate }) => {
  const countSignal = React.useMemo(() => signals.input(0), [signals]);
  const doubledSignal = React.useMemo(
    () => signals.computed(() => countSignal() * 2),
    [signals, countSignal],
  );
  const statusSignal = React.useMemo(
    () =>
      signals.computed(() =>
        countSignal() >= 10 ? "OPTIMAL SYSTEM DENSITY" : "BOOTSTRAP INITIALIZATION",
      ),
    [signals, countSignal],
  );
  const panelSignal = React.useMemo(
    () =>
      signals.output(() => ({
        count: countSignal(),
        doubled: doubledSignal(),
        status: statusSignal(),
      })),
    [signals, countSignal, doubledSignal, statusSignal],
  );

  const count = useSignal<number>(signals, countSignal);
  const doubled = useSignal<number>(signals, doubledSignal);
  const status = useSignal<string>(signals, statusSignal);
  const panel = useSignal<{ count: number; doubled: number; status: string }>(signals, panelSignal);

  return (
    <DemoShell
      demo={demo}
      onNavigate={onNavigate}
      inspectorContent={
        <div style={{ display: "flex", flexDirection: "column", gap: "0.25rem" }}>
          <div><span style={{ color: "#71717a" }}>[reactivity]</span> count = {count}</div>
          <div><span style={{ color: "#71717a" }}>[reactivity]</span> doubled = {doubled}</div>
          <div><span style={{ color: "#71717a" }}>[reactivity]</span> status = "{status}"</div>
          <div><span style={{ color: "#71717a" }}>[output]</span> panel = {JSON.stringify(panel)}</div>
          <div>
            <span style={{ color: "#71717a" }}>[note]</span> this demo stays on the stable local graph lane:
            writable input, derived computed truth, and a published output projection.
          </div>
        </div>
      }
    >
      <div style={{ display: "flex", flexDirection: "column", alignItems: "center", gap: "1.5rem" }}>
        <div style={{ fontSize: "1rem", fontWeight: 600, color: "var(--text-primary)" }}>Counter Reactive State</div>
        <div style={{ fontSize: "3.5rem", fontWeight: 700, color: "var(--accent-signals)", textShadow: "0 0 15px rgba(6,182,212,0.3)" }}>
          {count}
        </div>
        <div style={{ display: "flex", gap: "1rem" }}>
          <button className="btn" onClick={() => countSignal.set(count - 1)}>-</button>
          <button className="btn" onClick={() => countSignal.set(count + 1)}>+</button>
          <button className="btn" onClick={() => countSignal.set(0)}>Reset</button>
        </div>
        <div style={{ width: "100%", borderTop: "1px solid var(--border-light)", paddingTop: "1rem", marginTop: "0.5rem", display: "flex", flexDirection: "column", gap: "0.5rem", fontSize: "0.85rem" }}>
          <div>doubled value: <strong style={{ color: "var(--text-primary)" }}>{doubled}</strong></div>
          <div>computed status: <strong style={{ color: "var(--accent-signals)" }}>{status}</strong></div>
          <div>published panel count: <strong style={{ color: "var(--text-primary)" }}>{panel.count}</strong></div>
        </div>
      </div>
    </DemoShell>
  );
};
