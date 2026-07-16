import React from "react";

export interface CacheEventDatum {
  readonly id: string;
  readonly kind: "optimistic" | "confirmed" | "restore" | "refetch";
  readonly atMs: number;
  readonly label: string;
  readonly restoreToMs?: number;
}

export interface WrongWindow {
  readonly fromMs: number;
  readonly toMs: number | null;
}

export interface DagLaneDatum {
  readonly effectId: string;
  readonly label: string;
  readonly admittedAtMs: number;
  readonly settledAtMs: number | null;
  readonly parentEffectId: string | null;
  readonly lifecycle: string;
  readonly terminal: string | null;
  readonly branchId: number;
}

const WIDTH = 640;
const PAD = 20;

const GREEN = "#79dfb1";
const RED = "#f09595";
const AMBER = "#f5b76d";
const BLUE = "#7dc7ff";
const TRUNK = "#5a6478";
const MUTED = "#8a93a8";

function useNowWhile(live: boolean): number {
  const [now, setNow] = React.useState(0);
  React.useEffect(() => {
    if (!live) return;
    const interval = window.setInterval(() => setNow(performance.now()), 200);
    return () => window.clearInterval(interval);
  }, [live]);
  return now;
}

function makeScale(baseMs: number, endMs: number): (t: number) => number {
  const span = Math.max(endMs - baseMs, 500);
  return (t: number) => PAD + ((t - baseMs) / span) * (WIDTH - 2 * PAD);
}

export function CacheTimelineStrip({
  baseMs,
  events,
  live,
  wrongWindows,
}: {
  baseMs: number | null;
  events: readonly CacheEventDatum[];
  live: boolean;
  wrongWindows: readonly WrongWindow[];
}): React.ReactElement {
  const now = useNowWhile(live);

  const body = (() => {
    if (baseMs === null || events.length === 0) {
      return <p className="po-strip-empty">one shared cache value — its history will draw here</p>;
    }
    const endMs = Math.max(
      live ? now : 0,
      ...events.map((event) => event.atMs),
      ...wrongWindows.map((window) => window.toMs ?? window.fromMs),
      baseMs + 1_000,
    );
    const x = makeScale(baseMs, endMs);
    const y = 40;
    return (
      <svg className="po-strip-svg" viewBox={`0 0 ${WIDTH} 74`} role="img" aria-label="Cache timeline: one lane of commits with restore arrows jumping backward">
        {wrongWindows.map((window, index) => {
          const from = x(window.fromMs);
          const to = x(window.toMs ?? endMs);
          return (
            <g key={`w${index}`}>
              <rect x={from} y={8} width={Math.max(to - from, 2)} height={48} fill={RED} opacity={0.12} />
              {window.toMs !== null ? (
                <text x={(from + to) / 2} y={18} fontSize={9} fill={RED} textAnchor="middle">
                  wrong for {((window.toMs - window.fromMs) / 1000).toFixed(1)}s — no record remains
                </text>
              ) : (
                <text x={(from + to) / 2} y={18} fontSize={9} fill={RED} textAnchor="middle">
                  contradicting the server
                </text>
              )}
            </g>
          );
        })}
        <line x1={PAD - 6} y1={y} x2={WIDTH - PAD + 6} y2={y} stroke={TRUNK} strokeWidth={2} />
        {events.map((event, index) => {
          const cx = x(event.atMs);
          const labelY = y + (index % 2 === 0 ? 16 : 28);
          if (event.kind === "restore" && event.restoreToMs !== undefined) {
            const backX = x(event.restoreToMs);
            const midX = (cx + backX) / 2;
            return (
              <g key={event.id}>
                <path
                  d={`M ${cx} ${y - 6} Q ${midX} ${y - 34} ${backX + 6} ${y - 8}`}
                  fill="none"
                  stroke={RED}
                  strokeWidth={1.5}
                  strokeDasharray="4 3"
                />
                <path d={`M ${backX + 10} ${y - 12} L ${backX + 3} ${y - 5} L ${backX + 13} ${y - 6} Z`} fill={RED} />
                <line x1={cx - 4} y1={y - 4} x2={cx + 4} y2={y + 4} stroke={RED} strokeWidth={2} />
                <line x1={cx - 4} y1={y + 4} x2={cx + 4} y2={y - 4} stroke={RED} strokeWidth={2} />
                <text x={cx} y={labelY} fontSize={9} fill={RED} textAnchor="middle">restore</text>
              </g>
            );
          }
          const color = event.kind === "confirmed" ? GREEN : event.kind === "refetch" ? BLUE : MUTED;
          return (
            <g key={event.id}>
              <circle
                cx={cx}
                cy={y}
                r={4.5}
                fill={event.kind === "optimistic" ? "none" : color}
                stroke={color}
                strokeWidth={1.5}
              />
              <text x={cx} y={labelY} fontSize={9} fill={MUTED} textAnchor="middle">{event.label}</text>
            </g>
          );
        })}
      </svg>
    );
  })();

  return (
    <section className="po-strip" aria-label="Cache timeline">
      <header className="po-strip-head">
        <span>cache timeline — one shared value</span>
        <code>setQueryData · snapshot restore · invalidateQueries</code>
      </header>
      {body}
    </section>
  );
}

function laneColor(lane: DagLaneDatum): string {
  if (lane.terminal === "merged") return GREEN;
  if (lane.terminal === "rejectedAndRetired") return RED;
  if (lane.terminal === "dependencyCancelled" || lane.terminal === "supersededAndRetired") return MUTED;
  return AMBER;
}

export function BranchDagStrip({
  baseMs,
  lanes,
  live,
  onSelect,
  selectedId,
}: {
  baseMs: number | null;
  lanes: readonly DagLaneDatum[];
  live: boolean;
  onSelect: (effectId: string) => void;
  selectedId: string | null;
}): React.ReactElement {
  const now = useNowWhile(live);

  const body = (() => {
    if (baseMs === null || lanes.length === 0) {
      return <p className="po-strip-empty">every admitted effect forks a branch — the graph will draw here</p>;
    }
    const ordered = [...lanes].sort((a, b) => a.admittedAtMs - b.admittedAtMs);
    const laneIndexById = new Map(ordered.map((lane, index) => [lane.effectId, index]));
    const endMs = Math.max(
      live ? now : 0,
      ...ordered.map((lane) => lane.settledAtMs ?? lane.admittedAtMs),
      baseMs + 1_000,
    );
    const x = makeScale(baseMs, endMs);
    const laneGap = 19;
    const trunkY = 20 + ordered.length * laneGap;
    const height = trunkY + 22;
    const laneY = (index: number) => trunkY - laneGap * (index + 1);

    return (
      <svg className="po-strip-svg" viewBox={`0 0 ${WIDTH} ${height}`} role="img" aria-label="Effect branch graph: one lane per optimistic write, merging into or retiring off the canonical trunk">
        <line x1={PAD - 6} y1={trunkY} x2={WIDTH - PAD + 6} y2={trunkY} stroke={TRUNK} strokeWidth={2} />
        <text x={PAD - 6} y={trunkY + 14} fontSize={9} fill={MUTED}>canonical</text>
        {ordered.map((lane, index) => {
          const yTop = laneY(index);
          const forkX = x(lane.admittedAtMs);
          const endX = x(lane.settledAtMs ?? endMs);
          const parentIndex = lane.parentEffectId !== null
            ? laneIndexById.get(lane.parentEffectId)
            : undefined;
          const originY = parentIndex !== undefined ? laneY(parentIndex) : trunkY;
          const color = laneColor(lane);
          const selected = lane.effectId === selectedId;
          const dashedFork = lane.parentEffectId !== null;
          return (
            <g
              aria-label={`Inspect receipt for ${lane.label}`}
              className="po-dag-lane"
              key={lane.effectId}
              onClick={() => onSelect(lane.effectId)}
              onKeyDown={(event) => {
                if (event.key === "Enter" || event.key === " ") {
                  event.preventDefault();
                  onSelect(lane.effectId);
                }
              }}
              role="button"
              tabIndex={0}
            >
              <rect
                x={Math.min(forkX - 12, endX) - 2}
                y={yTop - 12}
                width={Math.abs(endX - (forkX - 12)) + 18}
                height={22}
                fill="transparent"
              />
              <path
                d={`M ${forkX - 10} ${originY} L ${forkX} ${yTop}`}
                fill="none"
                stroke={color}
                strokeWidth={selected ? 2.5 : 1.5}
                strokeDasharray={dashedFork ? "4 3" : undefined}
              />
              <line
                x1={forkX}
                y1={yTop}
                x2={endX}
                y2={yTop}
                stroke={color}
                strokeWidth={selected ? 2.5 : 1.5}
              />
              {lane.terminal === "merged" ? (
                <g>
                  <path
                    d={`M ${endX} ${yTop} L ${endX + 10} ${trunkY}`}
                    fill="none"
                    stroke={color}
                    strokeWidth={selected ? 2.5 : 1.5}
                  />
                  <circle cx={endX + 10} cy={trunkY} r={3.5} fill={color} />
                </g>
              ) : lane.terminal === "rejectedAndRetired" ? (
                <g>
                  <line x1={endX - 4} y1={yTop - 4} x2={endX + 4} y2={yTop + 4} stroke={color} strokeWidth={2} />
                  <line x1={endX - 4} y1={yTop + 4} x2={endX + 4} y2={yTop - 4} stroke={color} strokeWidth={2} />
                </g>
              ) : lane.terminal !== null ? (
                <g>
                  <circle cx={endX} cy={yTop} r={4.5} fill="none" stroke={color} strokeWidth={1.5} />
                  <line x1={endX - 3} y1={yTop + 3} x2={endX + 3} y2={yTop - 3} stroke={color} strokeWidth={1.5} />
                </g>
              ) : (
                <circle className="po-dag-pulse" cx={endX} cy={yTop} r={3.5} fill={color} />
              )}
              <text x={forkX + 4} y={yTop - 5} fontSize={9} fill={selected ? "#f5f7fb" : MUTED}>
                {lane.label}
                {lane.lifecycle === "ResponseRecorded" && lane.terminal === null ? " · response recorded" : ""}
              </text>
            </g>
          );
        })}
      </svg>
    );
  })();

  return (
    <section className="po-strip" aria-label="Effect branch graph">
      <header className="po-strip-head">
        <span>effect branches — one per write</span>
        <code>line.effects().open() · get(effectId) · terminal receipts</code>
      </header>
      {body}
      <p className="po-strip-legend">
        <span style={{ color: GREEN }}>— merged into canonical</span>
        <span style={{ color: RED }}>✗ rejected · branch retired</span>
        <span style={{ color: MUTED }}>⊘ dependency cancelled</span>
        <span style={{ color: AMBER }}>● pending</span>
        <em>click a lane for its runtime receipt</em>
      </p>
    </section>
  );
}
