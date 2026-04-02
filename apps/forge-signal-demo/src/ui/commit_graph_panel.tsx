import type { WorkerSnapshot } from "../gear-scene/worker/protocol";
import { humanLabel } from "../state/timeline_view";

export function CommitGraph({
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
              title={`${humanLabel(entry.label)} - ${entry.branchName ?? "main"} - frame ${entry.frameIndex}`}
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
