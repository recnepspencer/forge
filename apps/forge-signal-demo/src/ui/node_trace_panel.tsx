import type { WorkerSnapshot } from "../gear-scene/worker/protocol";
import { getAncestors, getDescendants, nodeLabel } from "../state/node_view";
import { buildTraceStory, buildWhyProse, humanLabel, humanWhyState } from "../state/timeline_view";

export function NodeTrace({
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
  const upstream = inspect.why.upstream.filter((u) => !u.startsWith("Clean {") && !u.startsWith("NodeId("));
  const touchedIndices = timeline.map((entry, i) => (entry.touchedNodes.includes(nodeId) ? i : -1)).filter((i) => i >= 0);
  const whyProse = buildWhyProse(label, whyState, suppressed, upstream);
  const storySteps = buildTraceStory(nodeId, timeline, touchedIndices, inspect.lineage.events.length);
  const ancestors = getAncestors(nodeId);
  const descendants = getDescendants(nodeId);

  return (
    <div className="trace-overlay">
      <div className="trace__head">
        <strong className="trace__title">{label}</strong>
        <button className="trace__close" type="button" onClick={onClose}>x</button>
      </div>
      <p className="trace__why">{whyProse}</p>
      <div className="trace__badges">
        <span className={`trace-chip ${whyState === "dirty" ? "trace-chip--warn" : ""}`}>{whyState}</span>
        <span className="trace-chip">{suppressed ? "suppressed" : "propagated"}</span>
      </div>
      {(ancestors.length > 0 || descendants.length > 0) && (
        <div className="trace__graph">
          {ancestors.length > 0 && (
            <div className="trace__graph-row">
              <span className="trace__graph-label">from</span>
              {ancestors.map((a) => (
                <button key={a} type="button" className="node-chip node-chip--sm" onClick={() => { onClose(); setTimeout(() => document.querySelector<HTMLButtonElement>(`[data-node-id="${a}"]`)?.click(), 50); }}>{nodeLabel(a)}</button>
              ))}
            </div>
          )}
          {descendants.length > 0 && (
            <div className="trace__graph-row">
              <span className="trace__graph-label">into</span>
              {descendants.map((d) => (
                <button key={d} type="button" className="node-chip node-chip--sm" onClick={() => { onClose(); setTimeout(() => document.querySelector<HTMLButtonElement>(`[data-node-id="${d}"]`)?.click(), 50); }}>{nodeLabel(d)}</button>
              ))}
            </div>
          )}
        </div>
      )}
      {touchedIndices.length > 0 && (
        <div className="trace__rail">
          {touchedIndices.map((idx) => (
            <button
              key={idx}
              type="button"
              className={`trace-dot ${idx === timelineIndex ? "trace-dot--active" : ""}`}
              onClick={() => onJump(idx)}
              title={`${humanLabel(timeline[idx].label)} - frame ${timeline[idx].frameIndex}`}
            >
              <span className="trace-dot__pip" />
              <span className="trace-dot__text">{humanLabel(timeline[idx].label)}</span>
            </button>
          ))}
        </div>
      )}
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
