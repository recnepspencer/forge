import { useEffect, useMemo, useRef, useState, useCallback } from "react";

import "./App.css";

import type { BranchId, ScenePatch } from "./gear-scene/core/types";
import { RENDER_HEIGHT, RENDER_WIDTH } from "./gear-scene/core/types";
import type { WorkerCommand, WorkerEvent, WorkerSnapshot } from "./gear-scene/worker/protocol";

const DEBUG_CONSOLE = false;

/* ─── empty snapshot ──────────────────────────────────────────────── */

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

/* ─── main app ────────────────────────────────────────────────────── */

function App() {
  const workerRef = useRef<Worker | null>(null);
  const frameStoreRef = useRef<Map<BranchId, ImageBitmap>>(new Map());
  const initPostedRef = useRef(false);
  const timelineScrollRef = useRef<HTMLDivElement | null>(null);
  const pendingInspectNodeRef = useRef<string | null>(null);

  const [snapshot, setSnapshot] = useState<WorkerSnapshot>(() => createEmptySnapshot());
  const [frameVersion, setFrameVersion] = useState(0);
  const [tracedNode, setTracedNode] = useState<string | null>(null);
  const [controlsOpen, setControlsOpen] = useState(true);

  /* ── worker lifecycle ─────────────────────────────────────────── */

  useEffect(() => {
    const worker = new Worker(new URL("./gear-scene/worker/demo-worker.ts", import.meta.url), {
      type: "module",
    });
    workerRef.current = worker;

    worker.onerror = (event) => {
      console.error("[forge-signal-demo] worker error", event);
      setSnapshot((s) => ({ ...s, error: event.message || "Worker failed to load", debugStatus: "worker:error" }));
    };

    worker.onmessageerror = () => {
      setSnapshot((s) => ({ ...s, error: "Worker message deserialization failed", debugStatus: "worker:message-error" }));
    };

    worker.onmessage = (event: MessageEvent<WorkerEvent>) => {
      const msg = event.data;
      if (msg.type === "debug") {
        if (DEBUG_CONSOLE) {
          console.log(
            "[forge-signal-demo]",
            msg.phase,
            msg.detail ? `- ${msg.detail}` : "",
            msg.elapsedMs != null ? `(${msg.elapsedMs.toFixed(1)} ms)` : "",
          );
        }
        if (msg.phase === "worker:handler-attached" && !initPostedRef.current) {
          initPostedRef.current = true;
          post(worker, { type: "init" });
        }
        setSnapshot((s) => ({ ...s, debugStatus: `${msg.phase}${msg.detail ? ` — ${msg.detail}` : ""}` }));
        return;
      }
      if (msg.type === "error") {
        setSnapshot((s) => ({ ...s, error: msg.error }));
        return;
      }
      const staleBitmaps: ImageBitmap[] = [];
      for (const frame of msg.frames) {
        const previous = frameStoreRef.current.get(frame.branchId);
        if (previous) {
          staleBitmaps.push(previous);
        }
        frameStoreRef.current.set(frame.branchId, frame.bitmap);
      }
      const liveBranchIds = new Set(msg.snapshot.branches.map((branch) => branch.id));
      for (const [branchId, bitmap] of frameStoreRef.current.entries()) {
        if (!liveBranchIds.has(branchId)) {
          frameStoreRef.current.delete(branchId);
          staleBitmaps.push(bitmap);
        }
      }
      if (staleBitmaps.length > 0) {
        requestAnimationFrame(() => {
          for (const bitmap of staleBitmaps) {
            try {
              bitmap.close();
            } catch {
              // Ignore already-detached/closed bitmaps during turnover.
            }
          }
        });
      }
      if (msg.frames.length > 0) setFrameVersion((v) => v + 1);
      if (DEBUG_CONSOLE) {
        console.log(
          "[forge-signal-demo] snapshot",
          msg.snapshot.branches.map((branch) => ({
            id: branch.id,
            name: branch.name,
            teeth: branch.state.gear.teeth,
            innerRadius: branch.state.gear.innerRadius,
            thickness: branch.state.gear.thickness,
          })),
        );
      }
      setSnapshot(msg.snapshot);
    };

    return () => {
      for (const bmp of frameStoreRef.current.values()) bmp.close();
      worker.terminate();
      workerRef.current = null;
      initPostedRef.current = false;
    };
  }, []);

  /* ── auto-scroll timeline ─────────────────────────────────────── */

  useEffect(() => {
    const container = timelineScrollRef.current;
    if (!container) return;
    const active = container.querySelector<HTMLElement>(".timeline-commit--active");
    if (!active) return;
    const targetLeft = active.offsetLeft - container.clientWidth * 0.5 + active.offsetWidth * 0.5;
    container.scrollTo({ left: Math.max(targetLeft, 0), behavior: "smooth" });
  }, [snapshot.timelineIndex, snapshot.timeline.length]);

  useEffect(() => {
    const pending = pendingInspectNodeRef.current;
    if (!pending || !snapshot.ready || snapshot.activeBranchId == null) return;
    pendingInspectNodeRef.current = null;
    const worker = workerRef.current;
    if (worker) {
      post(worker, { type: "inspectNode", branchId: snapshot.activeBranchId, nodeId: pending });
    }
  }, [snapshot.ready, snapshot.activeBranchId, snapshot.timelineIndex]);

  /* ── derived ──────────────────────────────────────────────────── */

  const activeBranch = useMemo(
    () => snapshot.branches.find((b) => b.id === snapshot.activeBranchId) ?? snapshot.branches[0] ?? null,
    [snapshot.activeBranchId, snapshot.branches],
  );

  /* ── commands ─────────────────────────────────────────────────── */

  const cmd = useCallback(
    (command: WorkerCommand) => {
      const w = workerRef.current;
      if (w) post(w, command);
    },
    [],
  );

  function handlePatch(patch: ScenePatch, label: string) {
    if (!activeBranch) return;
    cmd({ type: "setScenePatch", branchId: activeBranch.id, patch, label });
  }

  function handleInspect(nodeId: string) {
    if (!activeBranch) return;
    setTracedNode(nodeId);
    cmd({ type: "inspectNode", branchId: activeBranch.id, nodeId });
  }

  function handleScrub(index: number) {
    cmd({ type: "scrub", index });
  }

  function handleTraceJump(index: number) {
    if (!tracedNode) return;
    pendingInspectNodeRef.current = tracedNode;
    cmd({ type: "scrub", index });
  }

  /* ── render ───────────────────────────────────────────────────── */

  const suppression = suppressionPercent(snapshot.latestSummary, snapshot.graphNodes);

  return (
    <main className="shell">
      {/* ── header ─────────────────────────────────────────────── */}
      <header className="topbar">
        <div className="topbar__brand">
          <span className="topbar__eyebrow">Forge Signal</span>
          <h1>Parametric Gear</h1>
        </div>
        <div className="topbar__actions">
          <button className="btn btn--primary" onClick={() => cmd({ type: "branch" })}>
            Branch
          </button>
          <button
            className="btn"
            disabled={!snapshot.branches.some((b) => b.name === "what-if")}
            onClick={() => cmd({ type: "merge" })}
          >
            Merge
          </button>
        </div>
      </header>

      {/* ── booting / error ────────────────────────────────────── */}
      {snapshot.error && <div className="alert alert--error">{snapshot.error}</div>}
      {!snapshot.ready && !snapshot.error && (
        <div className="alert">Booting runtime… {snapshot.debugStatus ?? ""}</div>
      )}

      {/* ── workspace ──────────────────────────────────────────── */}
      <section className="workspace">
        <div className="workspace__main">
          {/* viewports */}
          <div className={`viewport-row ${snapshot.branches.length > 1 ? "viewport-row--split" : ""}`}>
            {snapshot.branches.map((branch) => (
              <Viewport
                key={branch.id}
                branch={branch}
                active={branch.id === activeBranch?.id}
                bitmap={frameStoreRef.current.get(branch.id) ?? null}
                frameVersion={frameVersion}
                onActivate={() => cmd({ type: "activateBranch", branchId: branch.id })}
              />
            ))}
          </div>

          {/* HUD overlay */}
          <HudOverlay
            graphNodes={snapshot.graphNodes}
            summary={snapshot.latestSummary}
            suppression={suppression}
          />

          {/* node trace overlay */}
          {tracedNode && snapshot.inspect && (
            <NodeTrace
              nodeId={tracedNode}
              inspect={snapshot.inspect}
              timeline={snapshot.timeline}
              timelineIndex={snapshot.timelineIndex}
              onJump={handleTraceJump}
              onClose={() => setTracedNode(null)}
            />
          )}

          {/* timeline */}
          <section className="timeline">
            <div className="timeline__head">
              <span className="timeline__label">Timeline</span>
              <span className="timeline__badge">
                {snapshot.timeline.length > 0
                  ? `${snapshot.timelineIndex + 1}/${snapshot.timeline.length}`
                  : "—"}
              </span>
            </div>
            <input
              className="timeline__slider"
              type="range"
              min={0}
              max={Math.max(snapshot.timeline.length - 1, 0)}
              step={1}
              value={Math.min(snapshot.timelineIndex, Math.max(snapshot.timeline.length - 1, 0))}
              disabled={snapshot.timeline.length === 0}
              onChange={(e) => handleScrub(Number(e.target.value))}
            />
            <div ref={timelineScrollRef} className="timeline__scroll">
              <CommitGraph
                timeline={snapshot.timeline}
                activeIndex={snapshot.timelineIndex}
                onScrub={handleScrub}
              />
              <div className="timeline__rail" role="list">
                {snapshot.timeline.map((entry, i) => {
                  const prev = snapshot.timeline[i - 1];
                  const hasFork = entry.branchCount > 1 || (prev?.branchCount ?? 1) > 1;
                  return (
                    <button
                      key={`${entry.frameIndex}-${i}`}
                      className={`tl-dot ${i === snapshot.timelineIndex ? "tl-dot--active" : ""} ${hasFork ? "tl-dot--fork" : ""}`}
                      onClick={() => handleScrub(i)}
                      type="button"
                      role="listitem"
                      title={`${humanLabel(entry.label)} · frame ${entry.frameIndex}`}
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
        </div>

        {/* ── sidebar ──────────────────────────────────────────── */}
        <aside className="sidebar">
          {/* controls */}
          <section className="panel">
            <button className="panel__toggle" type="button" onClick={() => setControlsOpen((v) => !v)}>
              <span className="panel__eyebrow">Controls</span>
              <span className="panel__chevron">{controlsOpen ? "▾" : "▸"}</span>
            </button>
            {controlsOpen && activeBranch && (
              <div key={activeBranch.id} className="sliders">
                <Slider label="Teeth" value={activeBranch.state.gear.teeth} min={8} max={32} step={1} fmt={(v) => `${v}`} onChange={(v) => handlePatch({ gear: { teeth: Math.round(v) } }, "teeth")} />
                <Slider label="Outer radius" value={activeBranch.state.gear.outerRadius} min={0.8} max={1.9} step={0.01} onChange={(v) => handlePatch({ gear: { outerRadius: v } }, "outer")} />
                <Slider label="Inner radius" value={activeBranch.state.gear.innerRadius} min={0.18} max={Math.max(activeBranch.state.gear.outerRadius - 0.12, 0.19)} step={0.01} onChange={(v) => handlePatch({ gear: { innerRadius: v } }, "inner")} />
                <Slider label="Thickness" value={activeBranch.state.gear.thickness} min={0.1} max={0.5} step={0.01} onChange={(v) => handlePatch({ gear: { thickness: v } }, "thickness")} />
                <Slider label="Rotation" value={activeBranch.state.gear.rotation} min={-Math.PI} max={Math.PI} step={0.01} onChange={(v) => handlePatch({ gear: { rotation: v } }, "rotation")} />
                <Slider label="Light" value={activeBranch.state.light.intensity} min={0.4} max={2.2} step={0.01} onChange={(v) => handlePatch({ light: { intensity: v } }, "light")} />
              </div>
            )}
          </section>

          {/* live HUD */}
          <section className="panel">
            <span className="panel__eyebrow">Live HUD</span>
            <dl className="hud-grid">
              <HudStat label="Evaluated" value={String(snapshot.latestSummary?.nodesEvaluated ?? 0)} />
              <HudStat label="Suppressed" value={`${suppression}%`} />
              <HudStat label="Touched" value={String(snapshot.latestSummary?.touchedNodes ?? 0)} />
              <HudStat label="Last run" value={formatNanos(Number(snapshot.latestSummary?.totalNanos ?? 0))} />
              <HudStat label="Branches" value={String(snapshot.branches.length)} />
              <HudStat label="Frame" value={String(activeBranch?.hud.frameIndex ?? 0)} />
            </dl>
          </section>

          {/* node tree explorer */}
          <section className="panel">
            <span className="panel__eyebrow">Signal Graph</span>
            <p className="panel__hint">Tap a node to explore its lineage.</p>
            <NodeTree
              teethCount={activeBranch?.state.gear.teeth ?? 16}
              tracedNode={tracedNode}
              onInspect={handleInspect}
            />
          </section>
        </aside>
      </section>
    </main>
  );
}

/* ─── dependency tree ────────────────────────────────────────────── */

type TreeNode = { id: string; label: string; children: string[] };

const NODES: Record<string, TreeNode> = {
  gearTeeth:               { id: "gearTeeth",               label: "Teeth",       children: ["gearDimensionsModel"] },
  gearOuterRadius:         { id: "gearOuterRadius",         label: "Outer radius", children: ["gearDimensionsModel"] },
  gearInnerRadius:         { id: "gearInnerRadius",         label: "Inner radius", children: ["gearDimensionsModel"] },
  gearThickness:           { id: "gearThickness",           label: "Thickness",   children: ["gearDimensionsModel"] },
  gearRotation:            { id: "gearRotation",            label: "Rotation",    children: ["gearDimensionsModel"] },
  lightIntensity:          { id: "lightIntensity",          label: "Light",       children: ["lightingModel"] },
  gearDimensionsModel:     { id: "gearDimensionsModel",     label: "Dimensions",  children: ["gearProfileModel","gearMeshModel"] },
  gearProfileModel:        { id: "gearProfileModel",        label: "Profile",     children: ["gearTopologyModel"] },
  gearTopologyModel:       { id: "gearTopologyModel",       label: "Topology",    children: ["gearMeshModel"] },
  gearMeshModel:           { id: "gearMeshModel",           label: "Mesh",        children: ["viewportProjectionModel"] },
  lightingModel:           { id: "lightingModel",           label: "Lighting",    children: ["viewportShadingModel"] },
  viewportProjectionModel: { id: "viewportProjectionModel", label: "Projection",  children: ["viewportShadingModel"] },
  viewportShadingModel:    { id: "viewportShadingModel",    label: "Shading",     children: ["hudModel"] },
  hudModel:                { id: "hudModel",                label: "HUD",         children: [] },
};

const STATIC_LAYERS = [
  { label: "Sources",  ids: ["gearTeeth", "gearOuterRadius", "gearInnerRadius", "gearThickness", "gearRotation", "lightIntensity"] },
  { label: "Derived",  ids: ["gearDimensionsModel", "gearProfileModel", "gearTopologyModel"] },
  { label: "Render",   ids: ["gearMeshModel", "lightingModel", "viewportProjectionModel", "viewportShadingModel"] },
  { label: "Output",   ids: ["hudModel"] },
];

function buildToothNodes(count: number): Array<{ id: string; label: string }> {
  return Array.from({ length: count }, (_, i) => ({
    id: `gearToothModel::tooth-${i}`,
    label: `Tooth ${i}`,
  }));
}

function nodeLabel(id: string): string {
  if (NODES[id]) return NODES[id].label;
  const match = id.match(/^gearToothModel::tooth-(\d+)$/);
  if (match) return `Tooth ${match[1]}`;
  return id;
}

function getAncestors(id: string): string[] {
  const ancestors: string[] = [];
  for (const [nodeId, node] of Object.entries(NODES)) {
    if (node.children.includes(id)) ancestors.push(nodeId);
  }
  // tooth family nodes are children of gearDimensionsModel + gearProfileModel
  if (id.startsWith("gearToothModel::")) {
    return ["gearDimensionsModel", "gearProfileModel"];
  }
  return ancestors;
}

function getDescendants(id: string): string[] {
  return NODES[id]?.children ?? [];
}

function NodeTree({
  teethCount,
  tracedNode,
  onInspect,
}: {
  teethCount: number;
  tracedNode: string | null;
  onInspect: (id: string) => void;
}) {
  const toothNodes = useMemo(() => buildToothNodes(teethCount), [teethCount]);

  // For gearTeeth, children = tooth family members
  const getChildren = useCallback((id: string): Array<{ id: string; label: string }> => {
    if (id === "gearTeeth") {
      return toothNodes;
    }
    const node = NODES[id];
    if (!node) return [];
    return node.children.map((cid) => ({ id: cid, label: NODES[cid]?.label ?? cid }));
  }, [toothNodes]);

  return (
    <div className="node-tree">
      {STATIC_LAYERS.map((layer) => (
        <div key={layer.label} className="node-tree__layer">
          <span className="node-tree__layer-label">{layer.label}</span>
          <div className="node-tree__nodes">
            {layer.ids.map((id) => {
              const isSelected = tracedNode === id;
              const children = isSelected ? getChildren(id) : [];
              return (
                <div key={id} className="node-tree__node-group">
                  <button
                    type="button"
                    data-node-id={id}
                    className={`node-chip ${isSelected ? "node-chip--active" : ""}`}
                    onClick={() => onInspect(id)}
                  >
                    {NODES[id]?.label ?? id}
                    {id === "gearTeeth" && <span className="node-tree__count">{teethCount}</span>}
                  </button>
                  {children.length > 0 && (
                    <div className="node-tree__children">
                      {children.map((c) => (
                        <button
                          key={c.id}
                          type="button"
                          data-node-id={c.id}
                          className={`node-chip node-chip--sm ${tracedNode === c.id ? "node-chip--active" : ""}`}
                          onClick={() => onInspect(c.id)}
                        >
                          {c.label}
                        </button>
                      ))}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      ))}
    </div>
  );
}

/* ─── viewport ───────────────────────────────────────────────────── */

function CommitGraph({
  timeline,
  activeIndex,
  onScrub,
}: {
  timeline: WorkerSnapshot["timeline"];
  activeIndex: number;
  onScrub: (index: number) => void;
}) {
  const laneY = (branchName: string | null) => (branchName === "what-if" ? 78 : 28);
  const stepX = 92;
  const startX = 28;
  const width = Math.max(timeline.length * stepX + 40, 240);
  const height = 110;
  const byId = new Map(timeline.map((entry, index) => [entry.id, { entry, index }]));

  return (
    <div className="timeline-graph" style={{ width }} role="list">
      <svg className="timeline-graph__svg" width={width} height={height} viewBox={`0 0 ${width} ${height}`} aria-hidden="true">
        <line x1={0} y1={28} x2={width} y2={28} className="timeline-graph__lane" />
        <line x1={0} y1={78} x2={width} y2={78} className="timeline-graph__lane timeline-graph__lane--ghost" />
        {timeline.flatMap((entry, index) => {
          const x = startX + index * stepX;
          const y = laneY(entry.branchName);
          return entry.parentIds.map((parentId) => {
            const parent = byId.get(parentId);
            if (!parent) return null;
            const px = startX + parent.index * stepX;
            const py = laneY(parent.entry.branchName);
            const midX = (px + x) * 0.5;
            const path = `M ${px} ${py} C ${midX} ${py}, ${midX} ${y}, ${x} ${y}`;
            return (
              <path
                key={`${entry.id}-${parentId}`}
                d={path}
                className={`timeline-graph__edge ${entry.kind === "merge" ? "timeline-graph__edge--merge" : ""}`}
              />
            );
          });
        })}
      </svg>
      <div className="timeline-graph__nodes">
        {timeline.map((entry, index) => {
          const x = startX + index * stepX;
          const y = laneY(entry.branchName);
          return (
            <button
              key={entry.id}
              type="button"
              role="listitem"
              className={`timeline-commit ${index === activeIndex ? "timeline-commit--active" : ""} timeline-commit--${entry.kind}`}
              style={{ left: x, top: y }}
              title={`${humanLabel(entry.label)} · ${entry.branchName ?? "main"} · frame ${entry.frameIndex}`}
              onClick={() => onScrub(index)}
            >
              <span className="timeline-commit__dot" />
              <span className="timeline-commit__label">{humanLabel(entry.label)}</span>
            </button>
          );
        })}
      </div>
    </div>
  );
}

function Viewport({
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
  const ref = useRef<HTMLCanvasElement | null>(null);

  useEffect(() => {
    const canvas = ref.current;
    if (!canvas || !bitmap) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    ctx.clearRect(0, 0, RENDER_WIDTH, RENDER_HEIGHT);
    try {
      ctx.drawImage(bitmap, 0, 0, RENDER_WIDTH, RENDER_HEIGHT);
    } catch (error) {
      if (!(error instanceof DOMException) || error.name !== "InvalidStateError") {
        throw error;
      }
    }
  }, [bitmap, frameVersion]);

  return (
    <div className={`vp ${active ? "vp--active" : ""}`} onClick={onActivate}>
      <div className="vp__head">
        <span className="vp__name">{branch.name}</span>
        <span className="vp__meta">{branch.state.gear.teeth} teeth · frame {branch.hud.frameIndex}</span>
      </div>
      <canvas ref={ref} className="vp__canvas" width={RENDER_WIDTH} height={RENDER_HEIGHT} />
    </div>
  );
}

/* ─── HUD overlay (glass badges on viewport) ─────────────────────── */

function HudOverlay({
  graphNodes,
  summary,
  suppression,
}: {
  graphNodes: number;
  summary: WorkerSnapshot["latestSummary"];
  suppression: string;
}) {
  return (
    <div className="hud-overlay">
      <span className="hud-badge">{graphNodes} nodes</span>
      <span className="hud-badge">{suppression}% suppressed</span>
      <span className="hud-badge">{summary?.nodesEvaluated ?? 0} evaluated</span>
      <span className="hud-badge">{formatNanos(Number(summary?.totalNanos ?? 0))}</span>
    </div>
  );
}

/* ─── node trace ─────────────────────────────────────────────────── */

function NodeTrace({
  nodeId,
  inspect,
  timeline,
  timelineIndex,
  onJump,
  onClose,
}: {
  nodeId: string;
  inspect: NonNullable<WorkerSnapshot["inspect"]>;
  timeline: WorkerSnapshot["timeline"];
  timelineIndex: number;
  onJump: (index: number) => void;
  onClose: () => void;
}) {
  const label = nodeLabel(nodeId);
  const whyState = humanWhyState(inspect.why.state);
  const suppressed = inspect.why.propagationSuppressed;
  const upstream = inspect.why.upstream.filter(
    (u) => !u.startsWith("Clean {") && !u.startsWith("NodeId("),
  );

  // find timeline indices where this node was touched
  const touchedIndices = timeline
    .map((entry, i) => (entry.touchedNodes.includes(nodeId) ? i : -1))
    .filter((i) => i >= 0);

  // build one-line WHY prose
  const whyProse = buildWhyProse(label, whyState, suppressed, upstream);
  const storySteps = buildTraceStory(nodeId, timeline, touchedIndices, inspect.lineage.events.length);

  const ancestors = getAncestors(nodeId);
  const descendants = getDescendants(nodeId);

  return (
    <div className="trace-overlay">
      <div className="trace__head">
        <strong className="trace__title">{label}</strong>
        <button className="trace__close" type="button" onClick={onClose}>✕</button>
      </div>

      {/* why prose */}
      <p className="trace__why">{whyProse}</p>

      {/* state badges */}
      <div className="trace__badges">
        <span className={`trace-chip ${whyState === "dirty" ? "trace-chip--warn" : ""}`}>{whyState}</span>
        <span className="trace-chip">{suppressed ? "suppressed" : "propagated"}</span>
      </div>

      {/* ancestors + descendants */}
      {(ancestors.length > 0 || descendants.length > 0) && (
        <div className="trace__graph">
          {ancestors.length > 0 && (
            <div className="trace__graph-row">
              <span className="trace__graph-label">← from</span>
              {ancestors.map((a) => (
                <button key={a} type="button" className="node-chip node-chip--sm" onClick={() => { onClose(); setTimeout(() => document.querySelector<HTMLButtonElement>(`[data-node-id="${a}"]`)?.click(), 50); }}>{nodeLabel(a)}</button>
              ))}
            </div>
          )}
          {descendants.length > 0 && (
            <div className="trace__graph-row">
              <span className="trace__graph-label">→ into</span>
              {descendants.map((d) => (
                <button key={d} type="button" className="node-chip node-chip--sm" onClick={() => { onClose(); setTimeout(() => document.querySelector<HTMLButtonElement>(`[data-node-id="${d}"]`)?.click(), 50); }}>{nodeLabel(d)}</button>
              ))}
            </div>
          )}
        </div>
      )}

      {/* trace dots — only commits that touched this node */}
      {touchedIndices.length > 0 && (
        <div className="trace__rail">
          {touchedIndices.map((idx) => (
            <button
              key={idx}
              type="button"
              className={`trace-dot ${idx === timelineIndex ? "trace-dot--active" : ""}`}
              onClick={() => onJump(idx)}
              title={`${humanLabel(timeline[idx].label)} · frame ${timeline[idx].frameIndex}`}
            >
              <span className="trace-dot__pip" />
              <span className="trace-dot__text">{humanLabel(timeline[idx].label)}</span>
            </button>
          ))}
        </div>
      )}

      {/* causal story */}
      {storySteps.length > 0 && (
        <div className="trace__lineage">
          {storySteps.map((step, i) => (
            <div key={`${step.commitIndex}-${i}`} className="trace-event">
              <span className="trace-event__kind">{step.title}</span>
              <span className="trace-event__detail">{step.detail}</span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

/* ─── slider ─────────────────────────────────────────────────────── */

function Slider({
  label,
  value,
  min,
  max,
  step,
  onChange,
  fmt,
}: {
  label: string;
  value: number;
  min: number;
  max: number;
  step: number;
  onChange: (v: number) => void;
  fmt?: (v: number) => string;
}) {
  return (
    <label className="slider">
      <span className="slider__head">
        <span>{label}</span>
        <span>{fmt ? fmt(value) : value.toFixed(2)}</span>
      </span>
      <input type="range" min={min} max={max} step={step} value={value} onChange={(e) => onChange(Number(e.target.value))} />
    </label>
  );
}

/* ─── HUD stat cell ──────────────────────────────────────────────── */

function HudStat({ label, value }: { label: string; value: string }) {
  return (
    <div className="hud-cell">
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

/* ─── helpers ────────────────────────────────────────────────────── */

function post(worker: Worker, command: WorkerCommand) {
  if (DEBUG_CONSOLE) {
    console.log("[forge-signal-demo] post", command);
  }
  worker.postMessage(command);
}

function buildWhyProse(
  nodeName: string,
  state: string,
  suppressed: boolean,
  upstream: string[],
): string {
  if (suppressed) {
    return `${nodeName} was checked, but propagation was suppressed because its output didn't change.`;
  }
  if (upstream.length === 0) {
    return `${nodeName} is a source node with no upstream dependencies.`;
  }
  const depNames = upstream
    .slice(0, 3)
    .map((id) => nodeLabel(id))
    .join(", ");
  const extra = upstream.length > 3 ? ` and ${upstream.length - 3} more` : "";
  return `${nodeName} is ${state} because ${depNames}${extra} changed upstream. ${suppressed ? "Output unchanged, so propagation stopped here." : "Its value was recalculated and propagated forward."}`;
}

function buildTraceStory(
  nodeId: string,
  timeline: WorkerSnapshot["timeline"],
  touchedIndices: number[],
  lineageEventCount: number,
) {
  const recent = touchedIndices.slice(-4).reverse();
  const steps = recent.map((idx) => {
    const entry = timeline[idx];
    const title = explainCommitForNode(nodeId, entry.label);
    const detail = `${entry.branchName ?? "main"} · frame ${entry.frameIndex} · commit ${idx + 1}/${timeline.length}`;
    return { commitIndex: idx, title, detail };
  });

  if (steps.length === 0 && lineageEventCount > 0) {
    return [
      {
        commitIndex: -1,
        title: `${nodeLabel(nodeId)} has recorded lineage`,
        detail: `${lineageEventCount} runtime lineage events were captured for this node.`,
      },
    ];
  }

  return steps;
}

function explainCommitForNode(nodeId: string, label: string): string {
  const node = nodeLabel(nodeId);
  const change = humanLongLabel(label);

  if (isSourceNode(nodeId)) {
    return `${node} changed directly via ${change}.`;
  }

  if (nodeId.startsWith("gearToothModel::")) {
    if (label === "teeth") {
      return `${node} was re-derived because Teeth changed.`;
    }
    return `${node} was re-derived after ${change}.`;
  }

  if (nodeId === "gearDimensionsModel") {
    return `Gear dimensions recomputed after ${change}.`;
  }
  if (nodeId === "gearProfileModel") {
    return `Gear profile recomputed from updated dimensions after ${change}.`;
  }
  if (nodeId === "gearTopologyModel") {
    return `Gear topology updated after ${change}.`;
  }
  if (nodeId === "gearMeshModel") {
    return `Gear mesh regenerated after ${change}.`;
  }
  if (nodeId === "lightingModel") {
    return `Lighting updated after ${change}.`;
  }
  if (nodeId === "viewportProjectionModel") {
    return `Projection refreshed after ${change}.`;
  }
  if (nodeId === "viewportShadingModel") {
    return `Shading refreshed after ${change}.`;
  }
  if (nodeId === "hudModel") {
    return `HUD summary updated after ${change}.`;
  }

  return `${node} was touched after ${change}.`;
}

function isSourceNode(nodeId: string): boolean {
  return [
    "gearTeeth",
    "gearOuterRadius",
    "gearInnerRadius",
    "gearThickness",
    "gearRotation",
    "lightIntensity",
  ].includes(nodeId);
}

function humanLabel(label: string): string {
  switch (label) {
    case "boot": return "Boot";
    case "branch": return "Branch";
    case "merge": return "Merge";
    case "teeth": return "Teeth";
    case "outer": return "Outer";
    case "inner": return "Inner";
    case "thickness": return "Thick";
    case "rotation": return "Rot";
    case "light": return "Light";
    default: return label;
  }
}

function humanLongLabel(label: string): string {
  switch (label) {
    case "boot": return "initial boot";
    case "branch": return "branch creation";
    case "merge": return "merge";
    case "teeth": return "Teeth";
    case "outer": return "Outer radius";
    case "inner": return "Inner radius";
    case "thickness": return "Thickness";
    case "rotation": return "Rotation";
    case "light": return "Light intensity";
    default: return label;
  }
}

function humanWhyState(state: string): string {
  if (/clean/i.test(state)) return "clean";
  if (/dirty/i.test(state)) return "dirty";
  if (/stale/i.test(state)) return "stale";
  return state;
}

function suppressionPercent(summary: WorkerSnapshot["latestSummary"], graphNodes: number): string {
  if (!summary || graphNodes === 0) return "0.0";
  const untouched = Math.max(graphNodes - summary.nodesEvaluated, 0);
  return ((untouched / graphNodes) * 100).toFixed(1);
}

function formatNanos(value: number) {
  if (!Number.isFinite(value) || value <= 0) return "0 ms";
  if (value >= 1_000_000) return `${(value / 1_000_000).toFixed(2)} ms`;
  if (value >= 1_000) return `${(value / 1_000).toFixed(1)} µs`;
  return `${value.toFixed(0)} ns`;
}

export default App;
