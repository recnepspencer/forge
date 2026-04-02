import "./App.css";

import type { ScenePatch } from "./gear-scene/core/types";
import { demoState, useDemoSignal } from "./state/ui_state";
import {
  mergeConflictNodeIds,
  mergeResolvedNodeIds,
} from "./state/merge_view";
import { MergeProofPanel } from "./ui/merge_review_panel";
import { ScenarioPanel } from "./ui/scenario_review_panel";
import {
  ControlsPanel,
  LiveHudPanel,
  SignalGraphPanel,
  TimelinePanel,
} from "./ui/workspace_layout";
import { HudOverlay } from "./ui/hud_panels";
import { NodeTrace } from "./ui/node_trace_panel";
import { Viewport } from "./ui/viewport_panel";

/* ??? main app ????????????????????????????????????????????????????? */

function App() {
  const branches = useDemoSignal(demoState.branches.branches);
  const activeBranch = useDemoSignal(demoState.branches.activeBranch);
  const hasFeatureBranch = useDemoSignal(demoState.branches.hasFeatureBranch);
  const latestSummary = useDemoSignal(demoState.branches.latestSummary);
  const suppression = useDemoSignal(demoState.branches.suppressionPercent);
  const graphNodes = useDemoSignal(demoState.branches.graphNodes);
  const frameVersion = useDemoSignal(demoState.branches.frameVersion);
  const mergePlan = useDemoSignal(demoState.merge.mergePlan);
  const mergeResult = useDemoSignal(demoState.merge.mergeResult);
  const timeline = useDemoSignal(demoState.timeline.timeline);
  const timelineIndex = useDemoSignal(demoState.timeline.timelineIndex);
  const inspect = useDemoSignal(demoState.inspection.inspect);
  const tracedNode = useDemoSignal(demoState.inspection.tracedNode);
  const scenario = useDemoSignal(demoState.scenario.scenario);
  const diagnosticsTier = useDemoSignal(demoState.scenario.diagnosticsTier);
  const error = useDemoSignal(demoState.status.error);
  const debugStatus = useDemoSignal(demoState.status.debugStatus);
  const controlsOpen = useDemoSignal(demoState.controls.controlsOpen);
  const walkthroughOpen = useDemoSignal(demoState.controls.walkthroughOpen);
  const walkthroughIndex = useDemoSignal(demoState.controls.walkthroughIndex);

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
            onClick={() => demoState.merge.mergeNow()}
          >
            Merge
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
            onPatch={(patch: ScenePatch, label: string) => {
              if (!activeBranch) return;
              demoState.transport.applyScenePatch(activeBranch.id, patch, label);
            }}
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
              onExecute={() => demoState.scenario.execute()}
              onReplay={() => demoState.scenario.replay()}
              onSetDiagnosticsTier={(tier) => demoState.scenario.setDiagnosticsTier(tier)}
            />
          </section>

          <section className="panel">
            <span className="panel__eyebrow">Merge Proof</span>
            <MergeProofPanel
              mergePlan={mergePlan}
              mergeResult={mergeResult}
              walkthroughOpen={walkthroughOpen}
              walkthroughIndex={walkthroughIndex}
              onOpenWalkthrough={() => demoState.controls.openWalkthrough()}
              onCloseWalkthrough={() => demoState.controls.closeWalkthrough()}
              onNextWalkthrough={(maxIndex) => demoState.controls.nextWalkthrough(maxIndex)}
              onPrevWalkthrough={() => demoState.controls.prevWalkthrough()}
            />
          </section>

          <SignalGraphPanel
            teethCount={activeBranch?.state.gear.teeth ?? 16}
            tracedNode={tracedNode}
            conflictedNodes={mergeConflictNodeIds(mergePlan, mergeResult)}
            resolvedNodes={mergeResolvedNodeIds(mergePlan, mergeResult)}
            onInspect={(nodeId) => {
              if (!activeBranch) return;
              demoState.inspection.inspectNode(activeBranch.id, nodeId);
            }}
          />
        </aside>
      </section>
    </main>
  );
}

export default App;

