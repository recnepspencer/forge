import React from "react";
import "./resourceOptimisticArtifact.css";

type ResourceArtifactStage = "optimistic" | "saving" | "saved";

const resourceStageSequence: Array<{ duration: number; stage: ResourceArtifactStage }> = [
  { duration: 2000, stage: "optimistic" },
  { duration: 760, stage: "saving" },
  { duration: 2600, stage: "saved" },
  { duration: 560, stage: "optimistic" },
];

const nextStageIndex = (index: number) => (index + 1) % resourceStageSequence.length;

export function ResourceOptimisticArtifact() {
  const [stageIndex, setStageIndex] = React.useState(0);
  const stage = resourceStageSequence[stageIndex].stage;
  const label = stage === "saved" ? "saved" : "optimistic";

  React.useEffect(() => {
    const timeout = window.setTimeout(() => setStageIndex(nextStageIndex), resourceStageSequence[stageIndex].duration);
    return () => window.clearTimeout(timeout);
  }, [stageIndex]);

  return (
    <div className="xai-resource-cinema" aria-hidden="true">
      <div className="xai-resource-orb xai-resource-orb-left" />
      <div className="xai-resource-orb xai-resource-orb-right" />

      <div className="xai-resource-list">
        <div className="xai-resource-row xai-resource-row-saved">
          <span className="xai-resource-dot" />
          <span>Northstar Carry-On</span>
          <em>saved</em>
        </div>
        <div className="xai-resource-row xai-resource-row-saved">
          <span className="xai-resource-dot" />
          <span>Garment sleeve</span>
          <em>saved</em>
        </div>

        <div className="xai-resource-row xai-resource-row-live" data-stage={stage}>
          <span className="xai-resource-dot xai-resource-dot-live" />
          <span>Packing cubes</span>
          <span className="xai-resource-status">{label}</span>
          <span className="xai-resource-save-sweep" data-stage={stage} />
          <span className="xai-resource-sheen" />
        </div>
      </div>
    </div>
  );
}
