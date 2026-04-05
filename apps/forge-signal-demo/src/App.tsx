import "./App.css";
import { useState } from "react";

import { useReviewView, useScenarioView, useWorkspaceView } from "./state/app_views";
import { demoState } from "./state/ui_state";
import {
  mergeConflictNodeIds,
  mergeResolvedNodeIds,
} from "./state/merge_view";
import { MergeProofPanel } from "./ui/merge_review_panel";
import { ScenarioPanel } from "./ui/scenario_review_panel";
import {
  ControlsPanel,
} from "./ui/controls_panel";
import { HudOverlay } from "./ui/hud_panels";
import { LiveHudPanel } from "./ui/live_hud_panel";
import { NodeTrace } from "./ui/node_trace_panel";
import { SignalGraphPanel } from "./ui/signal_graph_panel";
import { TimelinePanel } from "./ui/timeline_panel";
import { Viewport } from "./ui/viewport_panel";

function App() {
  const [labOpen, setLabOpen] = useState(false);
  const {
    branches,
    activeBranch,
    latestSummary,
    suppression,
    graphNodes,
    frameVersion,
    timeline,
    timelineIndex,
    inspect,
    tracedNode,
    controlsOpen,
    applyActiveBranchPatch,
    inspectActiveBranchNode,
  } = useWorkspaceView();
  const {
    mergePlan,
    mergeResult,
    mergeReview,
    walkthroughOpen,
    walkthroughIndex,
    reviewPolicyLane,
    reviewManualSelections,
  } = useReviewView();
  const {
    hasFeatureBranch,
    scenario,
    diagnosticsTier,
    error,
    debugStatus,
  } = useScenarioView();

  function beginMergeReview() {
    demoState.controls.openWalkthrough();
  }

  function executeScenarioMergeAndReview() {
    demoState.scenario.execute();
    beginMergeReview();
  }

  function mergeNowAndReview() {
    demoState.merge.mergeNow();
    beginMergeReview();
  }

  const stageMode = walkthroughOpen
    ? "review"
    : scenario?.mode === "adversarial-gear-merge"
      ? "arena"
      : "manual";
  const stageEyebrow =
    stageMode === "review"
      ? "Merge Review Live"
      : stageMode === "arena"
        ? "Arena Stage"
        : "Manual Studio";
  const stageTitle =
    stageMode === "review"
      ? "Resolution playing out on the hero stage"
      : stageMode === "arena"
        ? "Adversarial merge surface"
        : "Direct parametric gear shaping";
  const stageSummary =
    stageMode === "review"
      ? `The guided comparison is live. Step through the merge on the stage instead of reading it in the dock.`
      : stageMode === "arena"
        ? "Run the scripted branch duel, then jump straight into the visual merge review."
        : "Shape a premium render target first, then fork, collide, and review the merge story.";

  return (
    <main className="shell">
      <header className="topbar">
        <div className="topbar__brand">
          <span className="topbar__eyebrow">Forge Signal</span>
          <h1>Parametric Gear</h1>
        </div>
        <div className="topbar__status">
          <span className="topbar__chip">{activeBranch?.name ?? "booting"}</span>
          <span className="topbar__chip">{activeBranch?.state.gear.teeth ?? 0} teeth</span>
          <span className="topbar__chip">{activeBranch?.hud.tileColumns ?? 0} x {activeBranch?.hud.tileRows ?? 0} tiles</span>
        </div>
        <div className="topbar__actions">
          <button className="btn btn--primary" onClick={() => demoState.transport.branch()}>
            Branch
          </button>
          <button
            className="btn"
            disabled={!hasFeatureBranch}
            onClick={mergeNowAndReview}
          >
            Merge And Review
          </button>
          <button className={`btn ${labOpen ? "btn--active" : ""}`} onClick={() => setLabOpen((value) => !value)}>
            {labOpen ? "Hide Lab" : "Open Lab"}
          </button>
        </div>
      </header>

      {error && <div className="alert alert--error">{error}</div>}
      {!branches.length && !error && (
        <div className="alert">Booting runtime... {debugStatus ?? ""}</div>
      )}

      <section className="workspace">
        <div className="workspace__main workspace__main--hero">
          <div className="stage-shell">
            <div className="stage-shell__header">
              <div>
                <span className="stage-shell__eyebrow">{stageEyebrow}</span>
                <h2 className="stage-shell__title">{stageTitle}</h2>
                <p className="stage-shell__summary">{stageSummary}</p>
              </div>
              <div className="stage-shell__chips">
                <span className="stage-shell__chip stage-shell__chip--accent">
                  {stageMode === "review" ? "Review mode" : stageMode === "arena" ? "Arena mode" : "Manual mode"}
                </span>
                <span className="stage-shell__chip">Nodes {graphNodes}</span>
                <span className="stage-shell__chip">Touched {latestSummary?.touchedNodes ?? 0}</span>
                <span className="stage-shell__chip">Suppressed {suppression}%</span>
              </div>
            </div>

          <div className={`viewport-row ${branches.length > 1 ? "viewport-row--split" : ""}`}>
            {branches.map((branch) => (
              <Viewport
                key={branch.id}
                branch={branch}
                active={branch.id === activeBranch?.id}
                bitmap={demoState.transport.getFrame(branch.id)}
                frameVersion={frameVersion}
                onActivate={() => demoState.transport.activateBranch(branch.id)}
              />
            ))}
          </div>
          </div>

          <HudOverlay
            graphNodes={graphNodes}
            summary={latestSummary}
            suppression={suppression}
            tileCount={activeBranch?.hud.tileCount ?? 0}
            dirtyTiles={activeBranch?.hud.dirtyTiles ?? 0}
            uploadSpans={activeBranch?.hud.uploadSpans ?? 0}
          />

          {tracedNode && inspect && (
            <NodeTrace
              nodeId={tracedNode}
              inspect={inspect}
              timeline={timeline}
              timelineIndex={timelineIndex}
              onJump={demoState.inspection.jumpToTrace}
              onClose={() => demoState.inspection.setTracedNode(null)}
            />
          )}

          <TimelinePanel
            timeline={timeline}
            timelineIndex={timelineIndex}
            onScrub={demoState.timeline.scrub}
          />
        </div>

        <aside className="sidebar sidebar--dock">
          <section className="dock-card dock-card--compact">
            <span className="panel__eyebrow">Action Dock</span>
            <p className="dock-card__summary">
              Tune the active branch, run the adversarial merge, then review the resolution on the main stage.
            </p>
          </section>

          <ControlsPanel
            controlsOpen={controlsOpen}
            activeBranch={activeBranch}
            onToggle={() => demoState.controls.toggleControls()}
            onPatch={applyActiveBranchPatch}
          />

          <section className="panel">
            <span className="panel__eyebrow">Arena</span>
            <ScenarioPanel
              scenario={scenario}
              diagnosticsTier={diagnosticsTier}
              hasFeatureBranch={hasFeatureBranch}
              onSetMode={(mode) => demoState.scenario.setMode(mode)}
              onRun={() => demoState.scenario.run()}
              onPlan={() => demoState.scenario.plan()}
              onExecute={executeScenarioMergeAndReview}
              onReplay={() => demoState.scenario.replay()}
              onSetDiagnosticsTier={(tier) => demoState.scenario.setDiagnosticsTier(tier)}
            />
          </section>

          <section className="panel panel--review">
            <span className="panel__eyebrow">Review</span>
            <MergeProofPanel
              mergePlan={mergePlan}
              mergeResult={mergeResult}
              mergeReview={mergeReview}
              walkthroughOpen={walkthroughOpen}
              walkthroughIndex={walkthroughIndex}
              reviewPolicyLane={reviewPolicyLane}
              reviewManualSelections={reviewManualSelections}
              frameVersion={frameVersion}
              getReviewFrame={demoState.transport.getReviewFrame}
              onOpenWalkthrough={() => demoState.controls.openWalkthrough()}
              onCloseWalkthrough={() => demoState.controls.closeWalkthrough()}
              onNextWalkthrough={(maxIndex) => demoState.controls.nextWalkthrough(maxIndex)}
              onPrevWalkthrough={() => demoState.controls.prevWalkthrough()}
              onSetReviewPolicyLane={(lane) => demoState.controls.setReviewPolicyLane(lane)}
              onSetReviewManualSelections={(selections) => demoState.controls.setReviewManualSelections(selections)}
            />
          </section>
        </aside>
      </section>

      <aside className={`lab-drawer ${labOpen ? "lab-drawer--open" : ""}`}>
        <div className="lab-drawer__head">
          <div>
            <span className="panel__eyebrow">Technical Lab</span>
            <h2 className="lab-drawer__title">Diagnostics and graph detail</h2>
          </div>
          <button className="lab-drawer__close" onClick={() => setLabOpen(false)}>x</button>
        </div>
        <div className="lab-drawer__body">
          <LiveHudPanel
            latestSummary={latestSummary}
            suppression={suppression}
            branchesCount={branches.length}
            frameIndex={activeBranch?.hud.frameIndex ?? 0}
            tileCount={activeBranch?.hud.tileCount ?? 0}
            tileGrid={`${activeBranch?.hud.tileColumns ?? 0} x ${activeBranch?.hud.tileRows ?? 0}`}
            dirtyTiles={activeBranch?.hud.dirtyTiles ?? 0}
            uploadedTiles={activeBranch?.hud.uploadedTiles ?? 0}
            uploadSpans={activeBranch?.hud.uploadSpans ?? 0}
            uploadBytes={activeBranch?.hud.uploadBytes ?? 0}
            changedDetails={activeBranch?.hud.changedDetails ?? 0}
          />
          <SignalGraphPanel
            teethCount={activeBranch?.state.gear.teeth ?? 16}
            tracedNode={tracedNode}
            conflictedNodes={mergeConflictNodeIds(mergePlan, mergeResult)}
            resolvedNodes={mergeResolvedNodeIds(mergePlan, mergeResult)}
            onInspect={inspectActiveBranchNode}
          />
        </div>
      </aside>
    </main>
  );
}

export default App;

