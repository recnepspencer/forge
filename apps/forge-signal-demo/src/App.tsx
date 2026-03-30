import { useEffect, useMemo, useRef, useState } from "react";

import "./App.css";

import type { BranchId, ScenePatch } from "./gear-scene/core/types";
import { RENDER_HEIGHT, RENDER_WIDTH } from "./gear-scene/core/types";
import type { WorkerCommand, WorkerEvent, WorkerSnapshot } from "./gear-scene/worker/protocol";

const FPS_KEYS = new Set(["w", "a", "s", "d", "space", "shift"]);

function createEmptySnapshot(): WorkerSnapshot {
  return {
    ready: false,
    graphNodes: 0,
    branches: [],
    activeBranchId: null,
    latestSummary: null,
    mergePlan: null,
    mergeResult: null,
    timeline: [],
    timelineIndex: 0,
    inspect: null,
    error: null,
    debugStatus: "worker idle",
  };
}

function App() {
  const workerRef = useRef<Worker | null>(null);
  const frameStoreRef = useRef<Map<BranchId, ImageBitmap>>(new Map());
  const initPostedRef = useRef(false);
  const activeCanvasRef = useRef<HTMLCanvasElement | null>(null);
  const pressedRef = useRef<Set<string>>(new Set());
  const lastPostedInputsRef = useRef("");

  const [snapshot, setSnapshot] = useState<WorkerSnapshot>(() => createEmptySnapshot());
  const [frameVersion, setFrameVersion] = useState(0);
  const [fpsView, setFpsView] = useState(false);

  useEffect(() => {
    const worker = new Worker(new URL("./gear-scene/worker/demo-worker.ts", import.meta.url), {
      type: "module",
    });
    workerRef.current = worker;

    worker.onerror = (event) => {
      const detail = [event.message, event.filename, event.lineno, event.colno].filter(Boolean).join(" | ");
      console.error("[forge-signal-demo] worker error", event);
      setSnapshot((current) => ({
        ...current,
        error: detail || "Worker failed to load",
        debugStatus: "worker:error",
      }));
    };

    worker.onmessageerror = (event) => {
      console.error("[forge-signal-demo] worker message error", event);
      setSnapshot((current) => ({
        ...current,
        error: "Worker message deserialization failed",
        debugStatus: "worker:message-error",
      }));
    };

    worker.onmessage = (event: MessageEvent<WorkerEvent>) => {
      const message = event.data;
      if (message.type === "debug") {
        const suffix = message.detail ? ` - ${message.detail}` : "";
        const timing = typeof message.elapsedMs === "number" ? ` (${message.elapsedMs.toFixed(1)} ms)` : "";
        const debugStatus = `${message.phase}${suffix}${timing}`;
        console.info("[forge-signal-demo]", debugStatus);
        if (message.phase === "worker:handler-attached" && !initPostedRef.current) {
          initPostedRef.current = true;
          post(worker, { type: "init" });
        }
        setSnapshot((current) => ({
          ...current,
          debugStatus,
        }));
        return;
      }

      if (message.type === "error") {
        setSnapshot((current) => ({
          ...current,
          error: message.error,
        }));
        return;
      }

      for (const frame of message.frames) {
        frameStoreRef.current.get(frame.branchId)?.close();
        frameStoreRef.current.set(frame.branchId, frame.bitmap);
      }
      if (message.frames.length > 0) {
        setFrameVersion((current) => current + 1);
      }
      setSnapshot(message.snapshot);
    };

    return () => {
      for (const bitmap of frameStoreRef.current.values()) {
        bitmap.close();
      }
      worker.terminate();
      workerRef.current = null;
      initPostedRef.current = false;
    };
  }, []);

  useEffect(() => {
    const onMouseMove = (event: MouseEvent) => {
      const worker = workerRef.current;
      const canvas = activeCanvasRef.current;
      if (!worker || !canvas || document.pointerLockElement !== canvas) {
        return;
      }
      post(worker, { type: "look", deltaX: event.movementX, deltaY: event.movementY });
    };

    const onPointerLockChange = () => {
      const locked = document.pointerLockElement === activeCanvasRef.current;
      setFpsView(locked);
      if (!locked) {
        pressedRef.current.clear();
        flushInputs(workerRef.current, pressedRef.current, lastPostedInputsRef.current, (value) => {
          lastPostedInputsRef.current = value;
        });
      }
    };

    document.addEventListener("mousemove", onMouseMove);
    document.addEventListener("pointerlockchange", onPointerLockChange);
    return () => {
      document.removeEventListener("mousemove", onMouseMove);
      document.removeEventListener("pointerlockchange", onPointerLockChange);
    };
  }, []);

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (!document.pointerLockElement) {
        return;
      }
      const key = mapFpsKey(event);
      if (!FPS_KEYS.has(key)) {
        return;
      }
      event.preventDefault();
      pressedRef.current.add(key);
      flushInputs(workerRef.current, pressedRef.current, lastPostedInputsRef.current, (value) => {
        lastPostedInputsRef.current = value;
      });
    };

    const onKeyUp = (event: KeyboardEvent) => {
      const key = mapFpsKey(event);
      if (!FPS_KEYS.has(key)) {
        return;
      }
      pressedRef.current.delete(key);
      flushInputs(workerRef.current, pressedRef.current, lastPostedInputsRef.current, (value) => {
        lastPostedInputsRef.current = value;
      });
    };

    const onBlur = () => {
      pressedRef.current.clear();
      flushInputs(workerRef.current, pressedRef.current, lastPostedInputsRef.current, (value) => {
        lastPostedInputsRef.current = value;
      });
    };

    window.addEventListener("keydown", onKeyDown, { passive: false });
    window.addEventListener("keyup", onKeyUp);
    window.addEventListener("blur", onBlur);
    return () => {
      window.removeEventListener("keydown", onKeyDown);
      window.removeEventListener("keyup", onKeyUp);
      window.removeEventListener("blur", onBlur);
    };
  }, []);

  const activeBranch = useMemo(
    () => snapshot.branches.find((branch) => branch.id === snapshot.activeBranchId) ?? snapshot.branches[0] ?? null,
    [snapshot.activeBranchId, snapshot.branches],
  );

  const suppression = suppressionPercent(snapshot.latestSummary, snapshot.graphNodes);
  const replayParity = snapshot.timeline.length > 0 ? "Replay parity: deterministic" : "Replay parity: pending";

  function handleCreateBranch() {
    const worker = workerRef.current;
    if (!worker) return;
    post(worker, { type: "branch" });
  }

  function handleMerge() {
    const worker = workerRef.current;
    if (!worker) return;
    post(worker, { type: "merge" });
  }

  function handleActivateBranch(branchId: BranchId) {
    const worker = workerRef.current;
    if (!worker) return;
    post(worker, { type: "activateBranch", branchId });
  }

  function handleTimelineScrub(index: number) {
    const worker = workerRef.current;
    if (!worker) return;
    post(worker, { type: "scrub", index });
  }

  function handleScenePatch(patch: ScenePatch, label: string) {
    const worker = workerRef.current;
    if (!worker) return;
    post(worker, { type: "setScenePatch", patch, label });
  }

  return (
    <main className="app-shell">
      <header className="hero">
        <div className="hero__copy">
          <p className="eyebrow">Forge Signal Demo</p>
          <h1>Parametric gear branch / merge / replay</h1>
          <p className="hero__lede">
            Gear edits live in Forge Signal. Click the viewport for FPS mode, move through the scene, branch the
            design, merge it back, and scrub the journal like a timeline.
          </p>
        </div>
        <div className="hero__actions">
          <button className="button button--primary" onClick={handleCreateBranch}>
            Branch
          </button>
          <button
            className="button"
            disabled={!snapshot.branches.some((branch) => branch.name === "what-if")}
            onClick={handleMerge}
          >
            Merge
          </button>
        </div>
      </header>

      <section className="summary-row">
        <MetricCard label="Graph Nodes" value={snapshot.graphNodes.toLocaleString()} />
        <MetricCard label="Branches" value={String(snapshot.branches.length)} />
        <MetricCard label="Suppressed" value={`${suppression}%`} />
        <MetricCard label="Evaluated" value={String(snapshot.latestSummary?.nodesEvaluated ?? 0)} />
        <MetricCard label="Touched" value={String(snapshot.latestSummary?.touchedNodes ?? 0)} />
        <MetricCard label="View" value={fpsView ? "FPS" : "Panel"} accent={fpsView} />
      </section>

      {snapshot.error ? <div className="status status--error">{snapshot.error}</div> : null}
      {!snapshot.ready ? (
        <div className="status">Booting runtime. {snapshot.debugStatus ?? "starting worker"}</div>
      ) : null}

      <section className="workspace">
        <div className="workspace__main">
          <section className="viewport-grid">
            {snapshot.branches.map((branch) => (
              <ViewportCard
                key={branch.id}
                branchId={branch.id}
                branchName={branch.name}
                state={branch.state}
                hud={branch.hud}
                active={branch.id === activeBranch?.id}
                bitmap={frameStoreRef.current.get(branch.id) ?? null}
                frameVersion={frameVersion}
                onActivate={() => handleActivateBranch(branch.id)}
                onRegisterCanvas={(canvas) => {
                  if (branch.id === activeBranch?.id) {
                    activeCanvasRef.current = canvas;
                  }
                }}
                fpsActive={fpsView && branch.id === activeBranch?.id}
              />
            ))}
          </section>

          <section className="timeline-panel">
            <div className="timeline-panel__head">
              <div>
                <p className="section-label">History + Replay</p>
                <h2>Scrub the journal</h2>
              </div>
              <p className="timeline-panel__badge">{replayParity}</p>
            </div>

            <input
              className="timeline-slider"
              type="range"
              min={0}
              max={Math.max(snapshot.timeline.length - 1, 0)}
              step={1}
              value={Math.min(snapshot.timelineIndex, Math.max(snapshot.timeline.length - 1, 0))}
              disabled={snapshot.timeline.length === 0}
              onChange={(event) => handleTimelineScrub(Number(event.target.value))}
            />

            <div className="timeline-labels">
              {snapshot.timeline.map((entry, index) => (
                <button
                  key={`${entry.frameIndex}-${index}`}
                  className={index === snapshot.timelineIndex ? "timeline-chip timeline-chip--active" : "timeline-chip"}
                  onClick={() => handleTimelineScrub(index)}
                  type="button"
                >
                  {entry.label}
                </button>
              ))}
            </div>
          </section>
        </div>

        <aside className="workspace__sidebar">
          <section className="panel">
            <div className="panel__head">
              <div>
                <p className="section-label">Parameters</p>
                <h2>{activeBranch ? `${activeBranch.name} gear` : "No branch selected"}</h2>
              </div>
            </div>
            {activeBranch ? (
              <ParameterPanel branch={activeBranch} onPatch={handleScenePatch} />
            ) : (
              <p className="panel__empty">Booting the runtime.</p>
            )}
          </section>

          <section className="panel">
            <div className="panel__head">
              <div>
                <p className="section-label">FPS Controls</p>
                <h2>Video game navigation</h2>
              </div>
            </div>
            <div className="insight-list">
              <Insight label="Viewport" value={fpsView ? "Pointer lock active" : "Click to enter FPS mode"} />
              <Insight label="Move" value="W A S D" />
              <Insight label="Vertical" value="Space / Shift" />
              <Insight label="Look" value="Mouse" />
            </div>
          </section>

          <section className="panel">
            <div className="panel__head">
              <div>
                <p className="section-label">Metrics</p>
                <h2>Live signal HUD</h2>
              </div>
            </div>
            <dl className="stats-grid">
              <Stat label="Last Run" value={formatNanos(Number(snapshot.latestSummary?.totalNanos ?? 0))} />
              <Stat label="Nodes Evaluated" value={String(snapshot.latestSummary?.nodesEvaluated ?? 0)} />
              <Stat label="Nodes Suppressed" value={String(snapshot.latestSummary?.nodesSuppressed ?? 0)} />
              <Stat label="Nodes Touched" value={String(snapshot.latestSummary?.touchedNodes ?? 0)} />
              <Stat label="Render ms" value={activeBranch ? activeBranch.hud.renderMs.toFixed(2) : "0.00"} />
              <Stat label="Frame" value={String(activeBranch?.hud.frameIndex ?? 0)} />
            </dl>
          </section>
        </aside>
      </section>
    </main>
  );
}

function ParameterPanel({
  branch,
  onPatch,
}: {
  branch: WorkerSnapshot["branches"][number];
  onPatch: (patch: ScenePatch, label: string) => void;
}) {
  const gear = branch.state.gear;
  const light = branch.state.light;

  return (
    <div className="controls">
      <ControlRow
        label="Teeth"
        value={gear.teeth}
        min={8}
        max={32}
        step={1}
        format={(value) => `${value}`}
        onChange={(value) => onPatch({ gear: { teeth: Math.round(value) } }, "teeth")}
      />
      <ControlRow
        label="Outer radius"
        value={gear.outerRadius}
        min={0.8}
        max={1.9}
        step={0.01}
        onChange={(value) => onPatch({ gear: { outerRadius: value } }, "outer")}
      />
      <ControlRow
        label="Inner radius"
        value={gear.innerRadius}
        min={0.18}
        max={Math.max(gear.outerRadius - 0.12, 0.19)}
        step={0.01}
        onChange={(value) => onPatch({ gear: { innerRadius: value } }, "inner")}
      />
      <ControlRow
        label="Thickness"
        value={gear.thickness}
        min={0.1}
        max={0.5}
        step={0.01}
        onChange={(value) => onPatch({ gear: { thickness: value } }, "thickness")}
      />
      <ControlRow
        label="Rotation"
        value={gear.rotation}
        min={-Math.PI}
        max={Math.PI}
        step={0.01}
        onChange={(value) => onPatch({ gear: { rotation: value } }, "rotation")}
      />
      <ControlRow
        label="Light intensity"
        value={light.intensity}
        min={0.4}
        max={2.2}
        step={0.01}
        onChange={(value) => onPatch({ light: { intensity: value } }, "light")}
      />
    </div>
  );
}

function ControlRow({
  label,
  value,
  min,
  max,
  step,
  onChange,
  format,
}: {
  label: string;
  value: number;
  min: number;
  max: number;
  step: number;
  onChange: (value: number) => void;
  format?: (value: number) => string;
}) {
  return (
    <label className="control">
      <span className="control__head">
        <span>{label}</span>
        <span>{format ? format(value) : value.toFixed(2)}</span>
      </span>
      <input type="range" min={min} max={max} step={step} value={value} onChange={(e) => onChange(Number(e.target.value))} />
    </label>
  );
}

function ViewportCard({
  branchId,
  branchName,
  state,
  hud,
  active,
  bitmap,
  frameVersion,
  onActivate,
  onRegisterCanvas,
  fpsActive,
}: {
  branchId: BranchId;
  branchName: string;
  state: WorkerSnapshot["branches"][number]["state"];
  hud: WorkerSnapshot["branches"][number]["hud"];
  active: boolean;
  bitmap: ImageBitmap | null;
  frameVersion: number;
  onActivate: () => void;
  onRegisterCanvas: (canvas: HTMLCanvasElement | null) => void;
  fpsActive: boolean;
}) {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  useEffect(() => {
    onRegisterCanvas(canvasRef.current);
  }, [onRegisterCanvas]);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || !bitmap) {
      return;
    }
    const context = canvas.getContext("2d");
    if (!context) {
      return;
    }
    context.clearRect(0, 0, RENDER_WIDTH, RENDER_HEIGHT);
    context.drawImage(bitmap, 0, 0, RENDER_WIDTH, RENDER_HEIGHT);
  }, [bitmap, frameVersion]);

  return (
    <article className={active ? "viewport-card viewport-card--active" : "viewport-card"}>
      <div className="viewport-card__head">
        <div>
          <p className="viewport-card__eyebrow">{branchName}</p>
          <h3>{branchName === "what-if" ? "What-if branch" : `Branch ${String(branchId)}`}</h3>
        </div>
        <div className="viewport-card__meta">
          <span>frame {hud.frameIndex}</span>
          <span>{state.gear.teeth} teeth</span>
          <span>{fpsActive ? "fps view" : "panel view"}</span>
        </div>
      </div>
      <canvas
        ref={canvasRef}
        className="viewport-canvas"
        width={RENDER_WIDTH}
        height={RENDER_HEIGHT}
        onClick={(event) => {
          onActivate();
          if (document.pointerLockElement === event.currentTarget) {
            void document.exitPointerLock?.();
          } else {
            void event.currentTarget.requestPointerLock?.();
          }
          event.preventDefault();
        }}
        onWheel={(event) => {
          event.preventDefault();
        }}
      />
      <div className="viewport-card__foot">
        <span>{fpsActive ? "Click to exit FPS view" : "Click viewport for FPS view"}</span>
        <span>WASD / Space / Shift</span>
        <span>rotation {state.gear.rotation.toFixed(2)}</span>
      </div>
    </article>
  );
}

function MetricCard({ label, value, accent = false }: { label: string; value: string; accent?: boolean }) {
  return (
    <div className={accent ? "metric-card metric-card--accent" : "metric-card"}>
      <span className="metric-card__label">{label}</span>
      <strong className="metric-card__value">{value}</strong>
    </div>
  );
}

function Stat({ label, value }: { label: string; value: string }) {
  return (
    <div className="stat">
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function Insight({ label, value }: { label: string; value: string }) {
  return (
    <div className="insight">
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function post(worker: Worker, command: WorkerCommand) {
  console.info("[forge-signal-demo] post", command.type);
  worker.postMessage(command);
}

function flushInputs(
  worker: Worker | null,
  pressed: Set<string>,
  lastSignature: string,
  commit: (nextSignature: string) => void,
) {
  if (!worker) {
    return;
  }
  const next = Array.from(pressed).sort();
  const signature = next.join("|");
  if (signature === lastSignature) {
    return;
  }
  commit(signature);
  post(worker, { type: "setInputs", pressed: next });
}

function mapFpsKey(event: KeyboardEvent): string {
  if (event.code === "Space") {
    return "space";
  }
  if (event.code === "ShiftLeft" || event.code === "ShiftRight") {
    return "shift";
  }
  return event.key.toLowerCase();
}

function suppressionPercent(summary: WorkerSnapshot["latestSummary"], graphNodes: number): string {
  if (!summary || graphNodes === 0) {
    return "0.0";
  }
  const untouched = Math.max(graphNodes - summary.nodesEvaluated, 0);
  return ((untouched / graphNodes) * 100).toFixed(1);
}

function formatNanos(value: number) {
  if (!Number.isFinite(value) || value <= 0) return "0 ms";
  if (value >= 1_000_000) return `${(value / 1_000_000).toFixed(2)} ms`;
  if (value >= 1_000) return `${(value / 1_000).toFixed(1)} us`;
  return `${value.toFixed(0)} ns`;
}

export default App;
