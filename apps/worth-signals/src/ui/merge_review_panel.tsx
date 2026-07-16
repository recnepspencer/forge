import { useCallback, useMemo, type CSSProperties } from "react";

import type { SceneState } from "../gear-scene/core/types";
import type { ReviewManualSelections, WorkerSnapshot } from "../gear-scene/worker/protocol";
import {
  buildMergeDecisionSteps,
  describeMergeOutcome,
  friendlyPolicy,
  metricsForAspect,
  shortDigest,
} from "../state/merge_view";
import { ProofStat } from "./review_shared";

function ReviewFrameStage({
  stageRole = "result",
  eyebrow,
  title,
  accent,
  state,
  frame,
  frameVersion,
  aspectKind,
  metrics,
  highlights = [],
  emphasis,
  visualMode = "rendered",
}: {
  stageRole?: "source" | "target" | "result";
  eyebrow?: string;
  title: string;
  accent: string;
  state: SceneState | null;
  frame: ImageBitmap | null;
  frameVersion: number;
  aspectKind: "topology" | "lighting" | "motion" | "mixed";
  metrics: string[];
  highlights?: string[];
  emphasis?: string;
  visualMode?: "rendered" | "manual-review";
}) {
  const drawCanvas = useCallback(
    (canvas: HTMLCanvasElement | null) => {
      if (!canvas) return;
      const ctx = canvas.getContext("2d");
      if (!ctx) return;
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      if (!frame) return;
      try {
        if (aspectKind === "topology") {
          const sourceWidth = frame.width;
          const sourceHeight = frame.height;
          const cropWidth = sourceWidth * 0.42;
          const cropHeight = sourceHeight * 0.62;
          const sx = (sourceWidth - cropWidth) / 2;
          const sy = sourceHeight * 0.2;
          ctx.drawImage(frame, sx, sy, cropWidth, cropHeight, 0, 0, canvas.width, canvas.height);
        } else {
          ctx.drawImage(frame, 0, 0, canvas.width, canvas.height);
        }
      } catch (error) {
        if (!(error instanceof DOMException) || error.name !== "InvalidStateError") {
          throw error;
        }
      }
    },
    [aspectKind, frame, frameVersion],
  );

  return (
    <section
      className={`review-stage review-stage--${aspectKind} review-stage--${stageRole} ${visualMode === "manual-review" ? "review-stage--manual" : ""}`}
      style={{ "--review-accent": accent } as CSSProperties}
    >
      <div className="review-stage__chrome">
        <div className="review-stage__titles">
          {eyebrow ? <span className="review-stage__eyebrow">{eyebrow}</span> : null}
          <span className="review-stage__title">{title}</span>
        </div>
        {emphasis ? <span className="review-stage__emphasis">{emphasis}</span> : null}
      </div>
      <div className={`review-stage__viewport review-stage__viewport--${aspectKind}`}>
        <canvas ref={drawCanvas} className="review-stage__canvas" width={640} height={360} />
        <div className="review-stage__wash" />
        <div className="review-stage__edge" />
        {aspectKind === "topology" ? null : <ReviewAspectOverlay state={state} aspectKind={aspectKind} />}
        <div className="review-stage__focus review-stage__focus--ring" />
        <div className="review-stage__focus review-stage__focus--beam" />
        <div className="review-stage__focus review-stage__focus--orbit" />
        {highlights.length > 0 ? (
          <div className="review-stage__overlay">
            {highlights.map((highlight) => (
              <span key={`${title}-overlay-${highlight}`} className="review-stage__overlay-highlight">
                {highlight}
              </span>
            ))}
          </div>
        ) : null}
        {visualMode === "manual-review" ? (
          <div className="review-stage__stop">
            <span>Review</span>
          </div>
        ) : null}
      </div>
      <div className="review-stage__metrics">
        {metrics.map((metric) => (
          <span key={`${title}-${metric}`} className="review-stage__metric">
            {metric}
          </span>
        ))}
      </div>
    </section>
  );
}

function ReviewAspectOverlay({
  state,
  aspectKind,
}: {
  state: SceneState | null;
  aspectKind: "topology" | "lighting" | "motion" | "mixed";
}) {
  if (!state) {
    return null;
  }

  const isTopologyPrimary = aspectKind === "topology";
  const gearCx = 190;
  const gearCy = isTopologyPrimary ? 124 : 164;
  const outer = isTopologyPrimary
    ? 74 + state.gear.outerRadius * 78
    : 44 + state.gear.outerRadius * 120;
  const inner = isTopologyPrimary
    ? 20 + state.gear.innerRadius * 54
    : 12 + state.gear.innerRadius * 68;
  const toothRoot = inner + (outer - inner) * 0.62;
  const toothCount = Math.max(6, Math.min(24, state.gear.teeth));
  const gearPath = buildGearPath({
    cx: gearCx,
    cy: gearCy,
    outerRadius: outer,
    rootRadius: toothRoot,
    innerRadius: inner,
    teeth: toothCount,
    rotation: state.gear.rotation,
  });

  const lightX = 310 + state.light.x * 18;
  const lightY = 84 - state.light.y * 10;
  const rotationDegrees = state.gear.rotation * (180 / Math.PI);
  const arcEndX = gearCx + Math.cos((rotationDegrees / 180) * Math.PI) * (outer + 42);
  const arcEndY = gearCy + Math.sin((rotationDegrees / 180) * Math.PI) * (outer + 42);

  return (
    <svg className="review-stage__diagram" viewBox="0 0 380 220" aria-hidden="true">
      {(aspectKind === "topology" || aspectKind === "mixed") ? (
        <g className="review-stage__diagram-layer review-stage__diagram-layer--topology">
          <path d={gearPath} className="review-stage__diagram-gear" />
          <text x={gearCx} y={gearCy + outer + (isTopologyPrimary ? 18 : 34)} textAnchor="middle" className="review-stage__diagram-label">
            {state.gear.teeth} teeth
          </text>
        </g>
      ) : null}
      {(aspectKind === "lighting" || aspectKind === "mixed") ? (
        <g className="review-stage__diagram-layer review-stage__diagram-layer--lighting">
          <path
            d={`M ${lightX} ${lightY} L ${gearCx - 18} ${gearCy - 10} L ${gearCx + 24} ${gearCy + 14} Z`}
            className="review-stage__diagram-beam"
          />
          <circle cx={lightX} cy={lightY} r={14 + state.light.intensity * 5} className="review-stage__diagram-light" />
          <text x={lightX} y={lightY - 22} textAnchor="middle" className="review-stage__diagram-label">
            {state.light.intensity.toFixed(2)}x
          </text>
        </g>
      ) : null}
      {(aspectKind === "motion" || aspectKind === "mixed") ? (
        <g className="review-stage__diagram-layer review-stage__diagram-layer--motion">
          <path
            d={`M ${gearCx - outer - 24} ${gearCy + 42} A ${outer + 42} ${outer + 42} 0 0 1 ${arcEndX.toFixed(1)} ${arcEndY.toFixed(1)}`}
            className="review-stage__diagram-arc"
          />
          <circle cx={arcEndX} cy={arcEndY} r="7" className="review-stage__diagram-marker" />
          <text x={gearCx} y={gearCy - outer - 26} textAnchor="middle" className="review-stage__diagram-label">
            {rotationDegrees >= 0 ? "+" : ""}{rotationDegrees.toFixed(0)}°
          </text>
        </g>
      ) : null}
    </svg>
  );
}

function buildGearPath({
  cx,
  cy,
  outerRadius,
  rootRadius,
  innerRadius,
  teeth,
  rotation,
}: {
  cx: number;
  cy: number;
  outerRadius: number;
  rootRadius: number;
  innerRadius: number;
  teeth: number;
  rotation: number;
}) {
  const points: string[] = [];
  const totalSteps = teeth * 2;
  for (let index = 0; index < totalSteps; index += 1) {
    const angle = rotation + (Math.PI * 2 * index) / totalSteps;
    const radius = index % 2 === 0 ? outerRadius : rootRadius;
    const x = cx + Math.cos(angle) * radius;
    const y = cy + Math.sin(angle) * radius;
    points.push(`${index === 0 ? "M" : "L"} ${x.toFixed(1)} ${y.toFixed(1)}`);
  }

  const holeStartX = cx + innerRadius;
  const holeStartY = cy;
  return [
    ...points,
    "Z",
    `M ${holeStartX.toFixed(1)} ${holeStartY.toFixed(1)}`,
    `A ${innerRadius.toFixed(1)} ${innerRadius.toFixed(1)} 0 1 0 ${(cx - innerRadius).toFixed(1)} ${cy.toFixed(1)}`,
    `A ${innerRadius.toFixed(1)} ${innerRadius.toFixed(1)} 0 1 0 ${holeStartX.toFixed(1)} ${holeStartY.toFixed(1)}`,
    "Z",
  ].join(" ");
}

function DecisionModal({
  mergeReview,
  reviewPolicyLane,
  reviewManualSelections,
  items,
  index,
  frameVersion,
  getReviewFrame,
  onClose,
  onNext,
  onPrev,
  onSetReviewPolicyLane,
  onSetReviewManualSelections,
}: {
  mergeReview: NonNullable<WorkerSnapshot["mergeReview"]>;
  reviewPolicyLane: string;
  reviewManualSelections: ReviewManualSelections;
  items: ReturnType<typeof buildMergeDecisionSteps>;
  index: number;
  frameVersion: number;
  getReviewFrame: (frameId: string) => ImageBitmap | null;
  onClose: () => void;
  onNext: () => void;
  onPrev: () => void;
  onSetReviewPolicyLane: (lane: string) => void;
  onSetReviewManualSelections: (selections: ReviewManualSelections) => void;
}) {
  const item = items[index];
  if (!item) {
    return null;
  }

  const activeLane = item.lanes.find((lane) => lane.id === reviewPolicyLane) ?? item.lanes[0];
  const sourceFrame = getReviewFrame(mergeReview.sourceFrameId);
  const targetFrame = getReviewFrame(mergeReview.targetFrameId);
  const resultFrame = activeLane.frameId ? getReviewFrame(activeLane.frameId) : null;
  const resultMetrics = metricsForAspect(activeLane.resultState, item.aspectKind);

  return (
    <div className="walkthrough-overlay" role="dialog" aria-modal="true" aria-label="Merge review">
      <div className="review-modal">
        <div className="review-modal__head">
          <div>
            <div className="panel__eyebrow">Merge Review</div>
            <h3 className="review-modal__title">{item.title}</h3>
          </div>
          <button className="walkthrough-modal__close" type="button" onClick={onClose}>
            x
          </button>
        </div>

        <div className="review-modal__status">
          <span className="review-modal__step">Decision {index + 1} / {items.length}</span>
          <span className="review-modal__focus">{item.focusLabel}</span>
          <span className={`review-modal__verdict ${activeLane.manual ? "review-modal__verdict--manual" : ""}`}>
            {activeLane.statusLabel}
          </span>
        </div>

        <div className="review-modal__lane-rail" role="tablist" aria-label="Policy comparison">
          {mergeReview.previews.map((preview) => {
            const active = preview.id === activeLane.id;
            return (
              <button
                key={preview.id}
                type="button"
                className={`review-modal__lane ${active ? "review-modal__lane--active" : ""}`}
                style={{ "--lane-accent": preview.accent } as CSSProperties}
                onClick={() => onSetReviewPolicyLane(preview.id)}
              >
                <span>{item.lanes.find((lane) => lane.id === preview.id)?.policyFamily ?? preview.label}</span>
                <small>{item.lanes.find((lane) => lane.id === preview.id)?.policyLabel ?? preview.label}</small>
              </button>
            );
          })}
        </div>

        <div className="review-modal__policy-card">
          <div className="review-modal__policy-family">{activeLane.policyFamily}</div>
          <div className="review-modal__policy-name">{activeLane.policyLabel}</div>
        </div>

        <div className="review-modal__grid">
          <ReviewFrameStage
            stageRole="source"
            eyebrow="Source branch edits"
            title={mergeReview.source.name}
            accent="#75d9ff"
            state={mergeReview.source.state}
            frame={sourceFrame}
            frameVersion={frameVersion}
            aspectKind={item.aspectKind}
            metrics={item.sourceMetrics}
            highlights={item.sourceHighlights}
          />
          <ReviewFrameStage
            stageRole="target"
            eyebrow="Target branch edits"
            title={mergeReview.target.name}
            accent="#ffb679"
            state={mergeReview.target.state}
            frame={targetFrame}
            frameVersion={frameVersion}
            aspectKind={item.aspectKind}
            metrics={item.targetMetrics}
            highlights={item.targetHighlights}
          />
          <ReviewFrameStage
            stageRole="result"
            eyebrow={activeLane.policyFamily}
            title={activeLane.policyLabel}
            accent={activeLane.accent}
            state={activeLane.resultState}
            frame={resultFrame}
            frameVersion={frameVersion}
            aspectKind={item.aspectKind}
            metrics={resultMetrics}
            emphasis={activeLane.manual ? "Aspect picks applied" : activeLane.actionLabel}
            visualMode={activeLane.visualMode}
          />
        </div>

        {activeLane.manual ? (
          <div className="review-modal__manual-choice">
            {([
              ["teeth", "Teeth"],
              ["outerRadius", "Outer radius"],
              ["innerRadius", "Inner radius"],
              ["thickness", "Thickness"],
              ["lightIntensity", "Light intensity"],
              ["lightPosition", "Light position"],
              ["rotation", "Rotation"],
              ["camera", "Camera"],
            ] as const).map(([aspectKey, label]) => {
              const winner = reviewManualSelections[aspectKey];
              return (
                <div key={aspectKey} className="review-modal__manual-row">
                  <span className="review-modal__manual-label">{label}</span>
                  <div className="review-modal__manual-picks">
                    <button
                      type="button"
                      className={`review-modal__manual-btn ${winner === "source" ? "review-modal__manual-btn--active" : ""}`}
                      onClick={() => onSetReviewManualSelections({ ...reviewManualSelections, [aspectKey]: "source" })}
                    >
                      <span>{mergeReview.source.name}</span>
                      <small>Use {mergeReview.source.name.toLowerCase()} {label.toLowerCase()}</small>
                    </button>
                    <button
                      type="button"
                      className={`review-modal__manual-btn ${winner === "target" ? "review-modal__manual-btn--active" : ""}`}
                      onClick={() => onSetReviewManualSelections({ ...reviewManualSelections, [aspectKey]: "target" })}
                    >
                      <span>{mergeReview.target.name}</span>
                      <small>Use {mergeReview.target.name.toLowerCase()} {label.toLowerCase()}</small>
                    </button>
                  </div>
                </div>
              );
            })}
          </div>
        ) : null}

        <div className="review-modal__trail">
          {items.map((step, stepIndex) => (
            <div
              key={step.id}
              className={`review-modal__trail-step ${stepIndex === index ? "review-modal__trail-step--active" : ""}`}
            >
              <span className="review-modal__trail-dot" />
              <span>{step.trailLabel}</span>
            </div>
          ))}
        </div>

        <div className="walkthrough-modal__actions">
          <button className="btn" type="button" onClick={onPrev} disabled={index === 0}>
            Previous
          </button>
          <button className="btn btn--primary" type="button" onClick={index === items.length - 1 ? onClose : onNext}>
            {index === items.length - 1 ? "Close Review" : "Next"}
          </button>
        </div>
      </div>
    </div>
  );
}

export function MergeProofPanel({
  mergePlan,
  mergeResult,
  mergeReview,
  walkthroughOpen,
  walkthroughIndex,
  reviewPolicyLane,
  reviewManualSelections,
  frameVersion,
  getReviewFrame,
  onOpenWalkthrough,
  onCloseWalkthrough,
  onNextWalkthrough,
  onPrevWalkthrough,
  onSetReviewPolicyLane,
  onSetReviewManualSelections,
}: {
  mergePlan: WorkerSnapshot["mergePlan"];
  mergeResult: WorkerSnapshot["mergeResult"];
  mergeReview: WorkerSnapshot["mergeReview"];
  walkthroughOpen: boolean;
  walkthroughIndex: number;
  reviewPolicyLane: string;
  reviewManualSelections: ReviewManualSelections;
  frameVersion: number;
  getReviewFrame: (frameId: string) => ImageBitmap | null;
  onOpenWalkthrough: () => void;
  onCloseWalkthrough: () => void;
  onNextWalkthrough: (maxIndex: number) => void;
  onPrevWalkthrough: () => void;
  onSetReviewPolicyLane: (lane: string) => void;
  onSetReviewManualSelections: (selections: ReviewManualSelections) => void;
}) {
  const semantics = mergeResult?.semantics ?? mergePlan?.semantics ?? null;
  const walkthroughItems = useMemo(
    () => buildMergeDecisionSteps(mergeReview, mergePlan, mergeResult),
    [mergeReview, mergePlan, mergeResult],
  );

  if (!mergePlan && !mergeResult && !mergeReview) {
    return <p className="panel__hint">Run the arena and execute the merge to open the visual review.</p>;
  }

  const currentLane = walkthroughItems[0]?.lanes.find((lane) => lane.id === reviewPolicyLane)
    ?? walkthroughItems[0]?.lanes[0]
    ?? null;
  const reviewCount = walkthroughItems.length;

  return (
    <>
      <div className="merge-proof merge-proof--visual">
        <div className="merge-proof__block merge-proof__block--hero merge-proof__block--visual-hero">
          <div className="merge-proof__title">Merge Review</div>
          <div className="merge-proof__summary">{describeMergeOutcome(mergePlan, mergeResult)}</div>

          <div className="merge-proof__launch-strip">
            <div className="merge-proof__launch-stat">
              <span className="merge-proof__launch-label">Review steps</span>
              <strong>{reviewCount || 0}</strong>
            </div>
            <div className="merge-proof__launch-stat">
              <span className="merge-proof__launch-label">Primary lane</span>
              <strong>{currentLane?.policyFamily ?? "Executed merge stack"}</strong>
            </div>
            <div className="merge-proof__launch-stat">
              <span className="merge-proof__launch-label">Current outcome</span>
              <strong>{currentLane?.actionLabel ?? "Merged"}</strong>
            </div>
          </div>

          <div className="merge-proof__actions">
            <button className="btn btn--primary" type="button" onClick={onOpenWalkthrough} disabled={walkthroughItems.length === 0}>
              Launch Visual Review
            </button>
          </div>

          {semantics ? (
            <div className="merge-proof__policy-grid">
              <ProofStat label="Strategy" value={friendlyPolicy(semantics.strategyName, semantics.strategyBasis)} />
              <ProofStat label="Conflict" value={friendlyPolicy(semantics.conflictPolicyName, semantics.conflictPolicyBasis)} />
              <ProofStat label="Isolation" value={friendlyPolicy(semantics.conflictIsolationName, semantics.conflictIsolationBasis)} />
              <ProofStat label="Identity" value={friendlyPolicy(semantics.identityMatcherName, semantics.identityMatcherBasis)} />
            </div>
          ) : null}
        </div>

        <details className="merge-proof__details">
          <summary>Certification details</summary>
          <div className="merge-proof__details-body">
            {(mergePlan?.proof || mergeResult?.proof) && (
              <div className="merge-proof__block">
                <div className="merge-proof__title">Proof Chain</div>
                <div className="merge-proof__line">Plan digest: {shortDigest(mergePlan?.proof?.planDigest ?? null)}</div>
                <div className="merge-proof__line">Result digest: {shortDigest(mergeResult?.proof?.resultDigest ?? null)}</div>
                <div className="merge-proof__line">
                  Lowered bundle digest: {shortDigest(mergeResult?.proof?.loweredStrategyBundleDigest ?? mergePlan?.proof?.loweredStrategyBundleDigest ?? null)}
                </div>
                <div className="merge-proof__line">
                  Semantics digest: {shortDigest(mergeResult?.proof?.semanticsDigest ?? mergePlan?.proof?.semanticsDigest ?? null)}
                </div>
              </div>
            )}

            {mergeResult && (
              <div className="merge-proof__block">
                <div className="merge-proof__title">Execution Counters</div>
                <div className="merge-proof__line">Candidates: {mergeResult.counters.finalCandidateBreadth}</div>
                <div className="merge-proof__line">Reconciled: {mergeResult.counters.reconciliationBreadth}</div>
                <div className="merge-proof__line">Conflict regions: {mergeResult.conflictCount}</div>
              </div>
            )}
          </div>
        </details>
      </div>

      {walkthroughOpen && mergeReview && walkthroughItems.length > 0 ? (
        <DecisionModal
          mergeReview={mergeReview}
          reviewPolicyLane={reviewPolicyLane}
          reviewManualSelections={reviewManualSelections}
          items={walkthroughItems}
          index={walkthroughIndex}
          frameVersion={frameVersion}
          getReviewFrame={getReviewFrame}
          onClose={onCloseWalkthrough}
          onNext={() => onNextWalkthrough(walkthroughItems.length - 1)}
          onPrev={onPrevWalkthrough}
          onSetReviewPolicyLane={onSetReviewPolicyLane}
          onSetReviewManualSelections={onSetReviewManualSelections}
        />
      ) : null}
    </>
  );
}
