import type { KeyboardEvent } from "react";

import type {
  GearHistoryNode,
  GearHistorySelection,
} from "../../local-truth-gear/gear_scenario_view.ts";
import "./gearHistoryGraph.css";

interface GearHistoryGraphProps {
  busy: boolean;
  nodes: readonly GearHistoryNode[];
  onSelect: (node: GearHistoryNode) => void;
  selection: GearHistorySelection | null;
}

const MIN_GRAPH_WIDTH = 960;
const GRAPH_HEIGHT = 330;
const FIRST_COMMIT_X = 150;
const COMMIT_GAP = 190;
const GRAPH_RIGHT_PADDING = 150;
const laneY = { main: 110, design: 220 } as const;

export function GearHistoryGraph({
  busy,
  nodes,
  onSelect,
  selection,
}: GearHistoryGraphProps) {
  const chronologicalNodes = [...nodes].reverse();
  const width = Math.max(
    MIN_GRAPH_WIDTH,
    FIRST_COMMIT_X + Math.max(0, chronologicalNodes.length - 1) * COMMIT_GAP + GRAPH_RIGHT_PADDING,
  );
  const positions = new Map(chronologicalNodes.map((node, index) => [
    node.id,
    { x: FIRST_COMMIT_X + index * COMMIT_GAP, y: laneY[node.lane] },
  ]));
  return (
    <section className="gear-history" aria-label="Local Truth history">
      <div className="gear-history-heading">
        <div>
          <span>Runtime history</span>
          <h3>Every commit is still here.</h3>
        </div>
        <p>{selection
          ? `Viewing ${shortIdentity(selection.commitId)} · ${selection.visitedCommits} commits visited · click a live head to resume`
          : "Click any commit to see the exact gear it sealed. The live heads won't move."}</p>
      </div>
      <div className="gear-history-track">
        <svg
          aria-label="Git-style Local Truth commit graph, oldest to newest from left to right"
          className="gear-history-graph"
          role="img"
          style={{ minWidth: `${width}px` }}
          viewBox={`0 0 ${width} ${GRAPH_HEIGHT}`}
        >
          <text className="gear-history-time-label" x={width - 28} y="20">OLDEST → NEWEST</text>
          <text className="gear-history-lane-label main" x="24" y={laneY.main + 3}>MAIN</text>
          <text className="gear-history-lane-label design" x="24" y={laneY.design + 3}>DESIGN</text>
          <line className="gear-history-lane main" x1="72" x2={width - 28} y1={laneY.main} y2={laneY.main} />
          <line className="gear-history-lane design" x1="72" x2={width - 28} y1={laneY.design} y2={laneY.design} />
          {chronologicalNodes.flatMap((node) => node.parentIds.map((parentId) => {
            const child = positions.get(node.id);
            const parent = positions.get(parentId);
            if (!child || !parent) return null;
            const midpoint = (child.x + parent.x) / 2;
            return (
              <path
                className={`gear-history-edge ${node.lane}`}
                d={`M ${child.x} ${child.y} C ${midpoint} ${child.y}, ${midpoint} ${parent.y}, ${parent.x} ${parent.y}`}
                key={`${node.id}:${parentId}`}
              />
            );
          }))}
          {chronologicalNodes.map((node) => {
            const position = positions.get(node.id)!;
            const selected = selection?.commitId === node.id;
            const titleLines = node.title.split(" · ");
            const titleY = node.lane === "main"
              ? position.y - 43 - (titleLines.length - 1) * 13
              : position.y + 43;
            const detailY = node.lane === "main"
              ? position.y - 16
              : titleY + titleLines.length * 13 + 3;
            const headY = node.lane === "main" ? position.y + 20 : position.y - 48;
            return (
              <g
                aria-label={`${node.title}. ${node.detail}${node.isLiveHead ? ". Live head" : ""}`}
                aria-pressed={selected}
                className={`gear-history-node ${node.lane}${selected ? " selected" : ""}${node.isLiveHead ? " head" : ""}`}
                key={node.id}
                onClick={() => !busy && onSelect(node)}
                onKeyDown={(event) => activateFromKeyboard(event, () => !busy && onSelect(node))}
                role="button"
                tabIndex={busy ? -1 : 0}
              >
                <circle className="gear-history-node-halo" cx={position.x} cy={position.y} r="16" />
                <circle className="gear-history-node-core" cx={position.x} cy={position.y} r="7" />
                <text className="gear-history-node-title" x={position.x} y={titleY}>
                  {titleLines.map((line, index) => (
                    <tspan dy={index === 0 ? 0 : 13} key={`${node.id}:title:${index}`} x={position.x}>{line}</tspan>
                  ))}
                </text>
                <text className="gear-history-node-detail" x={position.x} y={detailY}>{node.detail}</text>
                {node.headLabels.length > 0 ? (
                  <g className="gear-history-head-label">
                    <rect height="26" rx="13" width="152" x={position.x - 76} y={headY} />
                    <text x={position.x} y={headY + 17}>{node.headLabels.join(" + ")}</text>
                  </g>
                ) : null}
              </g>
            );
          })}
        </svg>
      </div>
    </section>
  );
}

function activateFromKeyboard(event: KeyboardEvent<SVGGElement>, activate: () => void) {
  if (event.key !== "Enter" && event.key !== " ") return;
  event.preventDefault();
  activate();
}

function shortIdentity(id: string) {
  return id.length > 22 ? `${id.slice(0, 19)}…` : id;
}
