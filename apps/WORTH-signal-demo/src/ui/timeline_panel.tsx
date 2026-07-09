import type { WorkerSnapshot } from "../gear-scene/worker/protocol";
import { humanLabel } from "../state/timeline_view";
import { CommitGraph } from "./commit_graph_panel";

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
