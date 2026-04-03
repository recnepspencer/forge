import "./App.css";

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
    reviewManualChoice,
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

  return (
    <main className="shell">
      <header className="topbar">
        <div className="topbar__brand">
          <span className="topbar__eyebrow">Forge Signal</span>
          <h1>Parametric Gear</h1>
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
        </div>
      </header>

      {error && <div className="alert alert--error">{error}</div>}
      {!branches.length && !error && (
        <div className="alert">Booting runtime... {debugStatus ?? ""}</div>
      )}

      <section className="workspace">
        <div className="workspace__main">
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

          <HudOverlay
            graphNodes={graphNodes}
            summary={latestSummary}
            suppression={suppression}
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

        <aside className="sidebar">
          <ControlsPanel
            controlsOpen={controlsOpen}
            activeBranch={activeBranch}
            onToggle={() => demoState.controls.toggleControls()}
            onPatch={applyActiveBranchPatch}
          />

          <LiveHudPanel
            latestSummary={latestSummary}
            suppression={suppression}
            branchesCount={branches.length}
            frameIndex={activeBranch?.hud.frameIndex ?? 0}
          />

          <section className="panel">
            <span className="panel__eyebrow">Adversarial Arena</span>
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

          <section className="panel">
            <span className="panel__eyebrow">Merge Review</span>
            <MergeProofPanel
              mergePlan={mergePlan}
              mergeResult={mergeResult}
              mergeReview={mergeReview}
              walkthroughOpen={walkthroughOpen}
              walkthroughIndex={walkthroughIndex}
              reviewPolicyLane={reviewPolicyLane}
              reviewManualChoice={reviewManualChoice}
              frameVersion={frameVersion}
              getReviewFrame={demoState.transport.getReviewFrame}
              onOpenWalkthrough={() => demoState.controls.openWalkthrough()}
              onCloseWalkthrough={() => demoState.controls.closeWalkthrough()}
              onNextWalkthrough={(maxIndex) => demoState.controls.nextWalkthrough(maxIndex)}
              onPrevWalkthrough={() => demoState.controls.prevWalkthrough()}
              onSetReviewPolicyLane={(lane) => demoState.controls.setReviewPolicyLane(lane)}
              onSetReviewManualChoice={(choice) => demoState.controls.setReviewManualChoice(choice)}
            />
          </section>

          <SignalGraphPanel
            teethCount={activeBranch?.state.gear.teeth ?? 16}
            tracedNode={tracedNode}
            conflictedNodes={mergeConflictNodeIds(mergePlan, mergeResult)}
            resolvedNodes={mergeResolvedNodeIds(mergePlan, mergeResult)}
            onInspect={inspectActiveBranchNode}
          />
        </aside>
      </section>
    </main>
  );
}

export default App;

