import { useEffect, useMemo, useRef, type MutableRefObject } from "react";
import type { RunSummary } from "@forge/signal";

import { RENDER_HEIGHT, RENDER_WIDTH, type BranchId, type BranchInspect, type MergePlan, type MergeResult } from "../core/types";
import type { BranchSummary, TimelineEntry } from "../worker/protocol";

type Props = {
  canvasRef?: MutableRefObject<HTMLCanvasElement | null>;
  branches: BranchSummary[];
  activeBranchId: BranchId | null;
  graphNodes: number;
  latestSummary: RunSummary | null;
  mergePlan: MergePlan | null;
  mergeResult: MergeResult | null;
  timeline: TimelineEntry[];
  timelineIndex: number;
  inspect: BranchInspect | null;
  frameStoreRef: MutableRefObject<Map<BranchId, Uint8ClampedArray>>;
  frameVersion: number;
  onActivateBranch: (branchId: BranchId) => void;
  onScrubTimeline: (timelineIndex: number) => void;
};

type Region = { x: number; y: number; width: number; height: number };
type StageLayout = { branchRegions: Array<{ id: BranchId; region: Region }>; timeline: Region };

const STAGE_WIDTH = 1600;
const STAGE_HEIGHT = 980;

export function DemoStage({
  canvasRef: externalCanvasRef,
  branches,
  activeBranchId,
  graphNodes,
  latestSummary,
  mergePlan,
  mergeResult,
  timeline,
  timelineIndex,
  inspect,
  frameStoreRef,
  frameVersion,
  onActivateBranch,
  onScrubTimeline,
}: Props) {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const layoutRef = useRef<StageLayout | null>(null);
  const draggingTimelineRef = useRef(false);
  const rasterRef = useRef<OffscreenCanvas | null>(null);
  const rasterContextRef = useRef<OffscreenCanvasRenderingContext2D | null>(null);
  const rasterImageRef = useRef<ImageData | null>(null);

  const activeBranch = useMemo(
    () => branches.find((branch) => branch.id === activeBranchId) ?? branches[0] ?? null,
    [activeBranchId, branches],
  );

  useEffect(() => {
    if (externalCanvasRef) {
      externalCanvasRef.current = canvasRef.current;
    }
  }, [externalCanvasRef]);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const context = canvas.getContext("2d");
    if (!context) return;

    layoutRef.current = drawStage(context, {
      branches,
      activeBranchId,
      activeBranch,
      graphNodes,
      latestSummary,
      mergePlan,
      mergeResult,
      timeline,
      timelineIndex,
      inspect,
      frameStoreRef,
      rasterRef,
      rasterContextRef,
      rasterImageRef,
    });
  }, [
    activeBranch,
    activeBranchId,
    branches,
    frameStoreRef,
    frameVersion,
    graphNodes,
    inspect,
    latestSummary,
    mergePlan,
    mergeResult,
    timeline,
    timelineIndex,
  ]);

  function handlePointerDown(event: React.PointerEvent<HTMLCanvasElement>) {
    const point = toCanvasPoint(event.currentTarget, event.clientX, event.clientY);
    const layout = layoutRef.current;
    if (!layout) return;

    for (const branch of layout.branchRegions) {
      if (contains(branch.region, point.x, point.y)) {
        onActivateBranch(branch.id);
        if (branch.id === activeBranchId) {
          if (document.pointerLockElement === event.currentTarget) {
            void document.exitPointerLock?.();
          } else {
            void event.currentTarget.requestPointerLock?.();
          }
        }
        return;
      }
    }

    if (contains(layout.timeline, point.x, point.y)) {
      draggingTimelineRef.current = true;
      scrubFromPoint(layout.timeline, point.x, timeline.length, onScrubTimeline);
      event.currentTarget.setPointerCapture(event.pointerId);
    }
  }

  function handlePointerMove(event: React.PointerEvent<HTMLCanvasElement>) {
    if (!draggingTimelineRef.current) return;
    const layout = layoutRef.current;
    if (!layout) return;
    const point = toCanvasPoint(event.currentTarget, event.clientX, event.clientY);
    scrubFromPoint(layout.timeline, point.x, timeline.length, onScrubTimeline);
  }

  function handlePointerUp(event: React.PointerEvent<HTMLCanvasElement>) {
    draggingTimelineRef.current = false;
    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId);
    }
  }

  return (
    <canvas
      ref={canvasRef}
      className="demo-stage"
      width={STAGE_WIDTH}
      height={STAGE_HEIGHT}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onPointerCancel={handlePointerUp}
      aria-label="Forge Signal 3D gear demo"
    />
  );
}

function drawStage(
  context: CanvasRenderingContext2D,
  state: {
    branches: BranchSummary[];
    activeBranchId: BranchId | null;
    activeBranch: BranchSummary | null;
    graphNodes: number;
    latestSummary: RunSummary | null;
    mergePlan: MergePlan | null;
    mergeResult: MergeResult | null;
    timeline: TimelineEntry[];
    timelineIndex: number;
    inspect: BranchInspect | null;
    frameStoreRef: MutableRefObject<Map<BranchId, Uint8ClampedArray>>;
    rasterRef: MutableRefObject<OffscreenCanvas | null>;
    rasterContextRef: MutableRefObject<OffscreenCanvasRenderingContext2D | null>;
    rasterImageRef: MutableRefObject<ImageData | null>;
  },
): StageLayout {
  context.clearRect(0, 0, STAGE_WIDTH, STAGE_HEIGHT);

  const background = context.createLinearGradient(0, 0, 0, STAGE_HEIGHT);
  background.addColorStop(0, "#081419");
  background.addColorStop(0.56, "#091116");
  background.addColorStop(1, "#05090d");
  context.fillStyle = background;
  context.fillRect(0, 0, STAGE_WIDTH, STAGE_HEIGHT);

  drawNoiseGlow(context);
  drawHeader(context, state);

  const branchRegions = layoutBranches(state.branches.length);
  for (let index = 0; index < state.branches.length; index += 1) {
    const branch = state.branches[index];
    const region = branchRegions[index];
    drawViewport(
      context,
      branch,
      state.frameStoreRef.current.get(branch.id) ?? null,
      region,
      branch.id === state.activeBranchId,
      state.rasterRef,
      state.rasterContextRef,
      state.rasterImageRef,
    );
  }

  if (state.activeBranch) {
    drawReplayColumn(context, state.inspect);
    drawHud(context, state.activeBranch, state.graphNodes, state.latestSummary);
  }

  if (state.mergePlan) {
    drawMergeBox(context, state.mergePlan, state.mergeResult);
  }

  const timelineRegion = drawTimeline(context, state.timeline, state.timelineIndex);
  return {
    branchRegions: state.branches.map((branch, index) => ({ id: branch.id, region: branchRegions[index] })),
    timeline: timelineRegion,
  };
}

function drawNoiseGlow(context: CanvasRenderingContext2D) {
  context.save();
  context.globalAlpha = 0.18;
  const glow = context.createRadialGradient(1180, 200, 40, 1180, 200, 360);
  glow.addColorStop(0, "rgba(168, 255, 225, 0.34)");
  glow.addColorStop(1, "rgba(168, 255, 225, 0)");
  context.fillStyle = glow;
  context.fillRect(820, 0, 780, 560);

  const lime = context.createRadialGradient(240, 840, 20, 240, 840, 280);
  lime.addColorStop(0, "rgba(225, 255, 115, 0.18)");
  lime.addColorStop(1, "rgba(225, 255, 115, 0)");
  context.fillStyle = lime;
  context.fillRect(0, 620, 520, 360);
  context.restore();
}

function drawHeader(
  context: CanvasRenderingContext2D,
  state: {
    branches: BranchSummary[];
    activeBranch: BranchSummary | null;
    graphNodes: number;
    latestSummary: RunSummary | null;
    timeline: TimelineEntry[];
    timelineIndex: number;
  },
) {
  context.fillStyle = "#d7ff72";
  context.font = "600 13px 'Segoe UI', sans-serif";
  context.fillText("Forge Signal Demo", 68, 56);

  context.fillStyle = "#f5fffb";
  context.font = "700 48px 'Segoe UI', sans-serif";
  context.fillText("3D gear. Branch it. Merge it. Rewind it.", 64, 106);

  context.fillStyle = "rgba(220, 236, 232, 0.82)";
  context.font = "500 18px 'Segoe UI', sans-serif";
  context.fillText("Single runtime. Real branch state. Replay and HUD scrub with the frame.", 68, 140);

  const activeLabel = state.activeBranch ? `${state.activeBranch.name} live` : "booting";
  const suppressed = suppressionPercent(state.latestSummary, state.graphNodes);
  const tiles = [
    `${state.graphNodes.toLocaleString()} nodes`,
    `${state.branches.length} branches`,
    `${suppressed}% suppressed`,
    `scrub ${Math.min(state.timelineIndex + 1, state.timeline.length)}/${Math.max(state.timeline.length, 1)}`,
    activeLabel,
  ];

  let x = 68;
  for (const tile of tiles) {
    const width = context.measureText(tile).width + 34;
    roundRect(context, x, 164, width, 34, 17, "rgba(255,255,255,0.045)", "rgba(255,255,255,0.08)");
    context.fillStyle = "#d7efe8";
    context.font = "600 14px 'Segoe UI', sans-serif";
    context.fillText(tile, x + 17, 186);
    x += width + 10;
  }
}

function layoutBranches(branchCount: number): Region[] {
  if (branchCount <= 1) {
    return [{ x: 64, y: 220, width: 1120, height: 630 }];
  }

  return [
    { x: 64, y: 220, width: 760, height: 428 },
    { x: 850, y: 220, width: 760, height: 428 },
  ];
}

function drawViewport(
  context: CanvasRenderingContext2D,
  branch: BranchSummary,
  pixels: Uint8ClampedArray | null,
  region: Region,
  active: boolean,
  rasterRef: MutableRefObject<OffscreenCanvas | null>,
  rasterContextRef: MutableRefObject<OffscreenCanvasRenderingContext2D | null>,
  rasterImageRef: MutableRefObject<ImageData | null>,
) {
  roundRect(
    context,
    region.x,
    region.y,
    region.width,
    region.height,
    26,
    active ? "rgba(14, 24, 31, 0.98)" : "rgba(11, 18, 25, 0.96)",
    active ? "rgba(220, 255, 115, 0.42)" : "rgba(255,255,255,0.08)",
  );

  const headerHeight = 54;
  context.fillStyle = active ? "rgba(220,255,115,0.08)" : "rgba(255,255,255,0.03)";
  context.fillRect(region.x, region.y, region.width, headerHeight);

  context.fillStyle = "#d7ff72";
  context.font = "700 14px 'Segoe UI', sans-serif";
  context.fillText(branch.name.toUpperCase(), region.x + 20, region.y + 22);

  context.fillStyle = "#eef8f4";
  context.font = "600 24px 'Segoe UI', sans-serif";
  context.fillText(`branch ${String(branch.id)}`, region.x + 20, region.y + 45);

  context.fillStyle = "rgba(214, 233, 227, 0.74)";
  context.font = "500 14px 'Segoe UI', sans-serif";
  context.fillText(
    `camera ${branch.state.camera.x.toFixed(2)}, ${branch.state.camera.y.toFixed(2)}, ${branch.state.camera.z.toFixed(2)}`,
    region.x + region.width - 270,
    region.y + 33,
  );

  const imageRegion = {
    x: region.x + 14,
    y: region.y + headerHeight + 14,
    width: region.width - 28,
    height: region.height - headerHeight - 28,
  };

  drawPixelBuffer(context, pixels, imageRegion, rasterRef, rasterContextRef, rasterImageRef);

  if (active) {
    context.strokeStyle = "rgba(220,255,115,0.5)";
    context.lineWidth = 2;
    context.strokeRect(imageRegion.x - 1, imageRegion.y - 1, imageRegion.width + 2, imageRegion.height + 2);
  }
}

function drawPixelBuffer(
  context: CanvasRenderingContext2D,
  pixels: Uint8ClampedArray | null,
  region: Region,
  rasterRef: MutableRefObject<OffscreenCanvas | null>,
  rasterContextRef: MutableRefObject<OffscreenCanvasRenderingContext2D | null>,
  rasterImageRef: MutableRefObject<ImageData | null>,
) {
  if (!pixels) {
    context.fillStyle = "rgba(255,255,255,0.03)";
    context.fillRect(region.x, region.y, region.width, region.height);
    return;
  }

  if (!rasterRef.current) {
    if (typeof OffscreenCanvas === "undefined") {
      throw new Error("OffscreenCanvas is required for the canvas renderer.");
    }
    rasterRef.current = new OffscreenCanvas(RENDER_WIDTH, RENDER_HEIGHT);
    rasterContextRef.current = rasterRef.current.getContext("2d");
  }

  const raster = rasterRef.current;
  const rasterContext = rasterContextRef.current;
  if (!raster || !rasterContext) return;

  if (!rasterImageRef.current) {
    rasterImageRef.current = new ImageData(RENDER_WIDTH, RENDER_HEIGHT);
  }

  rasterImageRef.current.data.set(pixels);
  rasterContext.putImageData(rasterImageRef.current, 0, 0);
  context.save();
  context.imageSmoothingEnabled = true;
  context.filter = "contrast(1.06) saturate(1.08)";
  context.drawImage(raster, region.x, region.y, region.width, region.height);
  context.restore();
}

function drawReplayColumn(context: CanvasRenderingContext2D, inspect: BranchInspect | null) {
  const region = { x: 1210, y: 674, width: 330, height: 172 };
  roundRect(context, region.x, region.y, region.width, region.height, 22, "rgba(10, 15, 20, 0.92)", "rgba(255,255,255,0.08)");

  context.fillStyle = "#d7ff72";
  context.font = "700 12px 'Segoe UI', sans-serif";
  context.fillText("REPLAY", region.x + 18, region.y + 24);

  context.fillStyle = "#eef8f4";
  context.font = "700 20px 'Segoe UI', sans-serif";
  context.fillText(inspect?.why.state ?? "sampling", region.x + 18, region.y + 52);

  context.fillStyle = "rgba(220,236,232,0.72)";
  context.font = "500 13px 'Segoe UI', sans-serif";
  let y = region.y + 82;
  for (const frame of inspect?.replay.slice(-5) ?? []) {
    const detail = frame.detail ? `${frame.kind}: ${frame.detail}` : `${frame.kind} @ ${frame.cursor}`;
    context.fillText(detail, region.x + 18, y);
    y += 22;
  }
}

function drawHud(context: CanvasRenderingContext2D, branch: BranchSummary, graphNodes: number, latestSummary: RunSummary | null) {
  const region = { x: 1210, y: 220, width: 330, height: 430 };
  roundRect(context, region.x, region.y, region.width, region.height, 24, "rgba(8, 14, 18, 0.92)", "rgba(220,255,115,0.16)");

  context.fillStyle = "#d7ff72";
  context.font = "700 12px 'Segoe UI', sans-serif";
  context.fillText("LIVE HUD", region.x + 20, region.y + 24);

  context.fillStyle = "#eef8f4";
  context.font = "700 28px 'Segoe UI', sans-serif";
  context.fillText(branch.name, region.x + 20, region.y + 58);

  const fps = branch.hud.renderMs > 0 ? (1000 / branch.hud.renderMs).toFixed(1) : "0.0";
  const items = [
    ["fps-ish", fps],
    ["profile pts", String(branch.hud.raysMarched)],
    ["outer radius", branch.hud.averageSteps.toFixed(2)],
    ["suppressed", `${suppressionPercent(latestSummary, graphNodes)}%`],
    ["teeth", String(branch.hud.hits)],
    ["bore x100", String(branch.hud.misses)],
    ["nodes touched", String(branch.hud.touchedNodes)],
    ["evals", String(branch.hud.nodesEvaluated)],
    ["last run", formatNanos(branch.hud.totalNanos)],
  ];

  let y = region.y + 92;
  for (const [label, value] of items) {
    roundRect(context, region.x + 16, y, region.width - 32, 34, 17, "rgba(255,255,255,0.035)", "rgba(255,255,255,0.06)");
    context.fillStyle = "rgba(220,236,232,0.68)";
    context.font = "600 11px 'Segoe UI', sans-serif";
    context.fillText(label.toUpperCase(), region.x + 30, y + 14);
    context.fillStyle = "#f6fffc";
    context.font = "700 16px 'Segoe UI', sans-serif";
    context.fillText(value, region.x + 30, y + 28);
    y += 42;
  }
}

function drawMergeBox(context: CanvasRenderingContext2D, mergePlan: MergePlan, mergeResult: MergeResult | null) {
  const region = { x: 850, y: 674, width: 334, height: 172 };
  roundRect(context, region.x, region.y, region.width, region.height, 22, "rgba(10, 15, 20, 0.92)", "rgba(255,255,255,0.08)");

  context.fillStyle = "#d7ff72";
  context.font = "700 12px 'Segoe UI', sans-serif";
  context.fillText("MERGE PLAN", region.x + 18, region.y + 24);

  const lines = [
    `strategy: ${mergePlan.mergeStrategy ?? "none"}`,
    `kind: ${mergePlan.mergeKind ?? "none"}`,
    `candidate nodes: ${mergePlan.candidateCount}`,
    `shared nodes: ${mergePlan.sharedNodeCount}`,
  ];

  if (mergeResult) {
    lines.push(`merged: +${mergeResult.adoptedCount} / replace ${mergeResult.replacedCount}`);
  }

  context.fillStyle = "#eef8f4";
  context.font = "600 16px 'Segoe UI', sans-serif";
  let y = region.y + 54;
  for (const line of lines) {
    context.fillText(line, region.x + 18, y);
    y += 24;
  }
}

function drawTimeline(context: CanvasRenderingContext2D, timeline: TimelineEntry[], timelineIndex: number): Region {
  const region = { x: 66, y: 884, width: 1468, height: 34 };
  roundRect(context, region.x, region.y, region.width, region.height, 17, "rgba(255,255,255,0.04)", "rgba(255,255,255,0.1)");
  if (timeline.length === 0) return region;

  const innerX = region.x + 16;
  const innerWidth = region.width - 32;
  const bottom = region.y + region.height / 2;

  context.strokeStyle = "rgba(148, 175, 168, 0.38)";
  context.lineWidth = 4;
  context.beginPath();
  context.moveTo(innerX, bottom);
  context.lineTo(innerX + innerWidth, bottom);
  context.stroke();

  for (let index = 0; index < timeline.length; index += 1) {
    const x = innerX + (timeline.length === 1 ? 0 : (innerWidth * index) / (timeline.length - 1));
    context.fillStyle = index <= timelineIndex ? "#d7ff72" : "rgba(255,255,255,0.18)";
    context.beginPath();
    context.arc(x, bottom, index === timelineIndex ? 8 : 5, 0, Math.PI * 2);
    context.fill();
  }

  const activeLabel = timeline[timelineIndex]?.label ?? "boot";
  context.fillStyle = "rgba(220,236,232,0.78)";
  context.font = "600 14px 'Segoe UI', sans-serif";
  context.fillText(`timeline ${timelineIndex + 1}/${timeline.length} - ${activeLabel}`, region.x, region.y - 12);
  return region;
}

function scrubFromPoint(region: Region, x: number, length: number, onScrubTimeline: (index: number) => void) {
  if (length === 0) return;
  const ratio = clamp((x - (region.x + 16)) / (region.width - 32), 0, 1);
  const nextIndex = Math.round(ratio * Math.max(length - 1, 0));
  onScrubTimeline(nextIndex);
}

function toCanvasPoint(canvas: HTMLCanvasElement, clientX: number, clientY: number) {
  const rect = canvas.getBoundingClientRect();
  return {
    x: ((clientX - rect.left) / rect.width) * canvas.width,
    y: ((clientY - rect.top) / rect.height) * canvas.height,
  };
}

function contains(region: Region, x: number, y: number) {
  return x >= region.x && x <= region.x + region.width && y >= region.y && y <= region.y + region.height;
}

function roundRect(
  context: CanvasRenderingContext2D,
  x: number,
  y: number,
  width: number,
  height: number,
  radius: number,
  fill: string,
  stroke: string,
) {
  context.beginPath();
  context.moveTo(x + radius, y);
  context.arcTo(x + width, y, x + width, y + height, radius);
  context.arcTo(x + width, y + height, x, y + height, radius);
  context.arcTo(x, y + height, x, y, radius);
  context.arcTo(x, y, x + width, y, radius);
  context.closePath();
  context.fillStyle = fill;
  context.fill();
  context.strokeStyle = stroke;
  context.lineWidth = 1;
  context.stroke();
}

function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max);
}

function formatNanos(value: number) {
  if (!Number.isFinite(value) || value <= 0) return "0";
  if (value >= 1_000_000) return `${(value / 1_000_000).toFixed(2)} ms`;
  if (value >= 1_000) return `${(value / 1_000).toFixed(1)} us`;
  return `${value.toFixed(0)} ns`;
}

function suppressionPercent(summary: RunSummary | null, graphNodes: number): string {
  if (!summary || graphNodes === 0) return "0.0";
  const untouched = Math.max(graphNodes - summary.nodesEvaluated, 0);
  return ((untouched / graphNodes) * 100).toFixed(1);
}
