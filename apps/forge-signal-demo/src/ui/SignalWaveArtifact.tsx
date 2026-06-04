import { useEffect, useMemo, useState } from "react";
import "./signalWaveArtifact.css";

type SignalNode = { id: string; level: number; radius: number; x: number; y: number };
type SignalEdge = { child: string; id: string; parent: string; path: string };
type PropagationRun = { activeEdges: readonly string[]; offset: number };
type TimelineEdge = SignalEdge & { end: number; start: number };
type TimelineHit = SignalNode & { start: number };

const cycleSeconds = 8.6;
const travelSeconds = 0.54;
const settleSeconds = 1.08;
const nodes: readonly SignalNode[] = [
  { id: "root", level: 0, radius: 9, x: 64, y: 132 },
  { id: "a", level: 1, radius: 6, x: 174, y: 74 },
  { id: "b", level: 1, radius: 7, x: 178, y: 132 },
  { id: "c", level: 1, radius: 6, x: 174, y: 190 },
  { id: "a1", level: 2, radius: 5, x: 302, y: 48 },
  { id: "a2", level: 2, radius: 5, x: 314, y: 88 },
  { id: "b1", level: 2, radius: 6, x: 322, y: 132 },
  { id: "c1", level: 2, radius: 5, x: 314, y: 176 },
  { id: "c2", level: 2, radius: 5, x: 302, y: 216 },
  { id: "a2x", level: 3, radius: 5, x: 484, y: 72 },
  { id: "b1x", level: 3, radius: 4, x: 500, y: 118 },
  { id: "b1y", level: 3, radius: 5, x: 494, y: 154 },
  { id: "c1x", level: 3, radius: 4, x: 476, y: 198 },
  { id: "a2z", level: 4, radius: 4, x: 624, y: 58 },
  { id: "b1z", level: 4, radius: 4, x: 638, y: 112 },
  { id: "c1z", level: 4, radius: 4, x: 644, y: 210 },
];

const nodeById = new Map(nodes.map((node) => [node.id, node]));

function dependencyPath(parent: SignalNode, child: SignalNode) {
  const deltaX = child.x - parent.x;
  const bend = Math.max(42, deltaX * 0.42);
  return `M${parent.x} ${parent.y} C${parent.x + bend} ${parent.y}, ${child.x - bend} ${child.y}, ${child.x} ${child.y}`;
}

const edges: readonly SignalEdge[] = nodes.flatMap((parent) =>
  nodes
    .filter((child) => child.level > parent.level)
    .map((child) => ({
      child: child.id,
      id: `${parent.id}-${child.id}`,
      parent: parent.id,
      path: dependencyPath(parent, child),
    })),
);

const edgeById = new Map(edges.map((edge) => [edge.id, edge]));
const outgoingEdges = new Map<string, SignalEdge[]>();
for (const edge of edges) {
  outgoingEdges.set(edge.parent, [...(outgoingEdges.get(edge.parent) ?? []), edge]);
}

function seededRandom(seed: number) {
  let state = seed;
  return () => {
    state = (state * 1664525 + 1013904223) >>> 0;
    return state / 4294967296;
  };
}

function chooseEdges(parent: string, random: () => number, reached: ReadonlySet<string>) {
  const candidates = (outgoingEdges.get(parent) ?? []).filter((edge) => !reached.has(edge.child));
  if (candidates.length === 0) return [];
  const fanout = Math.min(candidates.length, 1 + Math.floor(random() * 4));
  const chosen: SignalEdge[] = [];
  const pool = [...candidates];
  while (pool.length > 0 && chosen.length < fanout) {
    const index = Math.floor(random() * pool.length);
    const [edge] = pool.splice(index, 1);
    if (chosen.length === 0 || random() < 0.74) chosen.push(edge);
  }
  return chosen;
}

function createRun(seed: number, offset: number): PropagationRun {
  const random = seededRandom(seed);
  const reached = new Set<string>(["root"]);
  let frontier = ["root"];
  const activeEdges: string[] = [];

  for (let depth = 0; depth < 4; depth += 1) {
    const nextFrontier: string[] = [];
    for (const parent of frontier) {
      const selected = chooseEdges(parent, random, reached);
      for (const edge of selected) {
        activeEdges.push(edge.id);
        reached.add(edge.child);
        nextFrontier.push(edge.child);
      }
    }
    frontier = nextFrontier;
    if (frontier.length === 0) break;
  }

  return { activeEdges, offset };
}

const runs: readonly PropagationRun[] = [
  createRun(1147, 0.05),
  createRun(9821, 1.72),
  createRun(4553, 3.38),
  createRun(7289, 5.02),
  createRun(3319, 6.64),
];

function smooth(value: number) {
  const clamped = Math.max(0, Math.min(1, value));
  return clamped * clamped * (3 - 2 * clamped);
}

function buildRunTimeline(run: PropagationRun) {
  const arrivals = new Map<string, number>([["root", run.offset]]);
  const pending = new Set<string>(run.activeEdges);
  const timelineEdges: TimelineEdge[] = [];

  while (pending.size > 0) {
    let progressed = false;
    for (const edgeId of Array.from(pending)) {
      const edge = edgeById.get(edgeId);
      const parentArrival = edge ? arrivals.get(edge.parent) : undefined;
      if (!edge || parentArrival === undefined) continue;
      const start = parentArrival + 0.22;
      const end = start + travelSeconds;
      arrivals.set(edge.child, end);
      timelineEdges.push({ ...edge, end, start });
      pending.delete(edgeId);
      progressed = true;
    }
    if (!progressed) break;
  }

  const timelineHits = Array.from(arrivals.entries())
    .map(([id, start]) => {
      const node = nodeById.get(id);
      return node ? { ...node, start } : null;
    })
    .filter((hit): hit is TimelineHit => Boolean(hit));
  return { timelineEdges, timelineHits };
}

function buildTimeline() {
  const timelines = runs.map(buildRunTimeline);
  return {
    timelineEdges: timelines.flatMap((timeline) => timeline.timelineEdges),
    timelineHits: timelines.flatMap((timeline) => timeline.timelineHits),
  };
}

function useAnimationTime() {
  const [time, setTime] = useState(0);

  useEffect(() => {
    if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) {
      setTime(2.7);
      return;
    }
    let frame = 0;
    const startedAt = performance.now();
    const tick = (now: number) => {
      setTime(((now - startedAt) / 1000) % cycleSeconds);
      frame = requestAnimationFrame(tick);
    };
    frame = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(frame);
  }, []);

  return time;
}

function edgeProgress(edge: TimelineEdge, time: number) {
  return smooth((time - edge.start) / (edge.end - edge.start));
}

function trailOpacity(edge: TimelineEdge, time: number) {
  if (time < edge.start || time > edge.end + settleSeconds) return 0;
  if (time <= edge.end) return 0.2 + edgeProgress(edge, time) * 0.3;
  return 0.5 * (1 - smooth((time - edge.end) / settleSeconds));
}

function nodeGlow(hit: TimelineHit, time: number) {
  const age = time - hit.start;
  if (age < 0 || age > 0.74) return 0;
  return age < 0.2 ? smooth(age / 0.2) : 1 - smooth((age - 0.2) / 0.54);
}

export function SignalWaveArtifact() {
  const time = useAnimationTime();
  const { timelineEdges, timelineHits } = useMemo(buildTimeline, []);

  return (
    <div className="xai-signal-wave" aria-hidden="true">
      <svg className="xai-signal-tree" viewBox="0 0 720 260" role="img">
        <defs>
          <radialGradient id="signalNodeGlow">
            <stop offset="0%" stopColor="#effbff" />
            <stop offset="42%" stopColor="#64c6ff" />
            <stop offset="100%" stopColor="#1766ff" stopOpacity="0" />
          </radialGradient>
          <linearGradient id="signalPulse" x1="0%" x2="100%">
            <stop offset="0%" stopColor="#35a7ff" stopOpacity="0" />
            <stop offset="36%" stopColor="#54c6ff" stopOpacity="0.72" />
            <stop offset="72%" stopColor="#a9edff" />
            <stop offset="100%" stopColor="#ffffff" />
          </linearGradient>
          <filter id="signalGlow" x="-80%" y="-80%" width="260%" height="260%">
            <feGaussianBlur stdDeviation="7" result="blur" />
            <feMerge>
              <feMergeNode in="blur" />
              <feMergeNode in="SourceGraphic" />
            </feMerge>
          </filter>
        </defs>
        <rect className="xai-signal-tree-backdrop" x="8" y="10" width="704" height="240" rx="28" />
        <g className="xai-signal-branch-base">
          {edges.map((edge) => (
            <path d={edge.path} key={edge.id} />
          ))}
        </g>
        <g className="xai-signal-branch-trails">
          {timelineEdges.map((edge) => (
            <path
              className="xai-signal-trail"
              d={edge.path}
              key={`${edge.id}-${edge.start}-trail`}
              pathLength="1"
              style={{ opacity: trailOpacity(edge, time) }}
            />
          ))}
        </g>
        <g className="xai-signal-branch-pulses">
          {timelineEdges.map((edge) => {
            const progress = edgeProgress(edge, time);
            const visible = progress > 0 && progress < 1;
            const pulseStart = Math.max(0, progress - 0.34);
            const pulseLength = Math.max(0.001, progress - pulseStart);
            return (
              <path
                className="xai-signal-pulse"
                d={edge.path}
                key={`${edge.id}-${edge.start}`}
                pathLength="1"
                style={{
                  opacity: visible ? 1 : 0,
                  strokeDasharray: `${pulseLength} 1`,
                  strokeDashoffset: -pulseStart,
                }}
              />
            );
          })}
        </g>
        <g className="xai-signal-node-halo">
          {timelineHits.map((node) => {
            const glow = nodeGlow(node, time);
            return (
              <circle
                cx={node.x}
                cy={node.y}
                key={`${node.id}-${node.start}-halo`}
                r={node.radius * (3.1 + glow)}
                style={{ opacity: glow * 0.5 }}
              />
            );
          })}
        </g>
        <g className="xai-signal-nodes">
          {nodes.map((node) => (
            <circle
              cx={node.x}
              cy={node.y}
              key={node.id}
              r={node.radius}
              className={node.id === "root" ? "is-root" : ""}
            />
          ))}
        </g>
      </svg>
    </div>
  );
}
