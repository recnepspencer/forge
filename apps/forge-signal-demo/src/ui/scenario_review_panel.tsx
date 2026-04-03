import type { WorkerSnapshot } from "../gear-scene/worker/protocol";
import {
  describeScenarioStatus,
  friendlyDiagnosticsTier,
  friendlyReplayStatus,
  friendlyScenarioMode,
  friendlyScenarioStatus,
} from "../state/scenario_view";
import { shortDigest } from "../state/merge_view";
import { ProofStat } from "./review_shared";

export function ScenarioPanel({
  scenario,
  diagnosticsTier,
  hasFeatureBranch,
  onSetMode,
  onRun,
  onPlan,
  onExecute,
  onReplay,
  onSetDiagnosticsTier,
}: {
  scenario: WorkerSnapshot["scenario"];
  diagnosticsTier: "webDevelopment" | "development" | "forensic" | "kernel";
  hasFeatureBranch: boolean;
  onSetMode: (mode: "manual-gear" | "adversarial-gear-merge") => void;
  onRun: () => void;
  onPlan: () => void;
  onExecute: () => void;
  onReplay: () => void;
  onSetDiagnosticsTier: (tier: "webDevelopment" | "development" | "forensic" | "kernel") => void;
}) {
  const mode = scenario?.mode ?? "manual-gear";
  const arenaMode = mode === "adversarial-gear-merge";
  return (
    <div className="scenario-panel">
      <p className="panel__hint">
        Run the scripted gear merge workload to certify bounded merge behavior under topology churn, render divergence, and replay.
      </p>
      <label className="scenario-panel__field">
        <span className="scenario-panel__label">Scenario mode</span>
        <select
          className="scenario-panel__select"
          value={mode}
          onChange={(event) => onSetMode(event.target.value as "manual-gear" | "adversarial-gear-merge")}
        >
          <option value="manual-gear">Manual Gear</option>
          <option value="adversarial-gear-merge">Adversarial Merge Arena</option>
        </select>
      </label>
      <label className="scenario-panel__field">
        <span className="scenario-panel__label">Diagnostics tier</span>
        <select
          className="scenario-panel__select"
          value={diagnosticsTier}
          onChange={(event) =>
            onSetDiagnosticsTier(
              event.target.value as "webDevelopment" | "development" | "forensic" | "kernel",
            )
          }
        >
          <option value="webDevelopment">Web Development</option>
          <option value="development">Development</option>
          <option value="forensic">Forensic</option>
          <option value="kernel">Kernel</option>
        </select>
      </label>
      <div className="scenario-panel__actions">
        <button className="btn btn--primary" type="button" onClick={onRun}>
          Run Script
        </button>
        <button className="btn" type="button" onClick={onPlan} disabled={!arenaMode || !hasFeatureBranch}>
          Plan Merge
        </button>
        <button className="btn" type="button" onClick={onExecute} disabled={!arenaMode || !hasFeatureBranch}>
          Execute + Review
        </button>
        <button className="btn" type="button" onClick={onReplay} disabled={!arenaMode || !scenario || scenario.status === "idle"}>
          Replay
        </button>
      </div>
      {scenario ? (
        <div className="merge-proof">
          <div className="merge-proof__block merge-proof__block--hero">
            <div className="merge-proof__title">Scenario Summary</div>
            <div className="merge-proof__summary">{describeScenarioStatus(scenario, diagnosticsTier)}</div>
            <div className="merge-proof__policy-grid">
              <ProofStat label="Mode" value={friendlyScenarioMode(scenario.mode)} />
              <ProofStat label="Status" value={friendlyScenarioStatus(scenario.status)} />
              <ProofStat label="Diagnostics" value={friendlyDiagnosticsTier(diagnosticsTier)} />
              <ProofStat label="Replay" value={friendlyReplayStatus(scenario.proof)} />
            </div>
            {!arenaMode && (
              <div className="merge-proof__microcopy">Manual Gear mode is for freeform edits. The arena mode is the scripted certification run.</div>
            )}
            {arenaMode && (
              <div className="merge-proof__microcopy">Run the script, plan the merge, then execute it to enter the guided merge review. Changing diagnostics tier should only change retained detail. Merge choices, proof digests, and replay parity stay invariant.</div>
            )}
          </div>
          {scenario.steps.length > 0 && (
            <div className="merge-proof__block">
              <div className="merge-proof__title">What The Script Does</div>
              {scenario.steps.map((step) => (
                <div key={step} className="merge-proof__line">
                  {step}
                </div>
              ))}
            </div>
          )}
          <details className="merge-proof__details">
            <summary>Certification details</summary>
            <div className="merge-proof__details-body">
              {scenario.proof && (
                <div className="merge-proof__block">
                  <div className="merge-proof__title">Proof Chain</div>
                  <div className="merge-proof__line">Proof schema: {scenario.proof.proofSchemaVersion ?? "pending"}</div>
                  <div className="merge-proof__line">Schema digest: {shortDigest(scenario.proof.schemaDigest)}</div>
                  <div className="merge-proof__line">Registry bundle digest: {shortDigest(scenario.proof.registryBundleDigest)}</div>
                  <div className="merge-proof__line">Lowered bundle digest: {shortDigest(scenario.proof.loweredStrategyBundleDigest)}</div>
                  <div className="merge-proof__line">Semantics digest: {shortDigest(scenario.proof.semanticsDigest)}</div>
                  <div className="merge-proof__line">Plan digest: {shortDigest(scenario.proof.mergePlanDigest)}</div>
                  <div className="merge-proof__line">Result digest: {shortDigest(scenario.proof.mergeResultDigest)}</div>
                  <div className="merge-proof__line">Lineage digest: {shortDigest(scenario.proof.lineageDigest)}</div>
                  <div className="merge-proof__line">Replay bundle digest: {shortDigest(scenario.proof.replayedLoweredStrategyBundleDigest)}</div>
                  <div className="merge-proof__line">Replay plan digest: {shortDigest(scenario.proof.replayedMergePlanDigest)}</div>
                  <div className="merge-proof__line">Replay result digest: {shortDigest(scenario.proof.replayedMergeResultDigest)}</div>
                  <div className="merge-proof__line">Replay lineage digest: {shortDigest(scenario.proof.replayedLineageDigest)}</div>
                  <div className="merge-proof__line">Merged state digest: {shortDigest(scenario.proof.mergedBranchStateDigest)}</div>
                  <div className="merge-proof__line">Replay state digest: {shortDigest(scenario.proof.replayBranchStateDigest)}</div>
                  <div className="merge-proof__line">Replay contract: branch-state digest is core-owned and compared canonically across merged and rebuilt runtimes.</div>
                  {scenario.proof.replayMismatchClasses.length > 0 && (
                    <div className="merge-proof__line">
                      Replay mismatches: {scenario.proof.replayMismatchClasses.join(", ")}
                    </div>
                  )}
                </div>
              )}
              <div className="merge-proof__block">
                <div className="merge-proof__title">Inspection Targets</div>
                {scenario.inspectedNodes.map((nodeId) => (
                  <div key={nodeId} className="merge-proof__line">
                    {nodeId}
                  </div>
                ))}
              </div>
            </div>
          </details>
        </div>
      ) : null}
    </div>
  );
}
