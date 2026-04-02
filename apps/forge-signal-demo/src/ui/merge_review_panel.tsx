import { useMemo } from "react";

import type { WorkerSnapshot } from "../gear-scene/worker/protocol";
import {
  buildConflictWalkthroughItems,
  describeMergeOutcome,
  friendlyPolicy,
  shortDigest,
  type ConflictWalkthroughItem,
} from "../state/merge_view";
import { ProofStat } from "./review_shared";

function ConflictWalkthroughModal({
  items,
  index,
  onClose,
  onNext,
  onPrev,
}: {
  items: ConflictWalkthroughItem[];
  index: number;
  onClose: () => void;
  onNext: () => void;
  onPrev: () => void;
}) {
  const item = items[index];
  if (!item) {
    return null;
  }

  return (
    <div className="walkthrough-overlay" role="dialog" aria-modal="true" aria-label="Merge conflict walkthrough">
      <div className="walkthrough-modal">
        <div className="walkthrough-modal__head">
          <div>
            <div className="panel__eyebrow">Conflict Walkthrough</div>
            <h3 className="walkthrough-modal__title">Conflict {index + 1} of {items.length}</h3>
          </div>
          <button className="walkthrough-modal__close" type="button" onClick={onClose}>x</button>
        </div>
        <div className="walkthrough-modal__body">
          <div className="walkthrough-modal__pair">
            <span>{item.sourceNode}</span>
            <span className="walkthrough-modal__arrow">-&gt;</span>
            <span>{item.targetNode ?? "new target artifact"}</span>
          </div>
          <div className={`walkthrough-modal__badge ${item.manual ? "walkthrough-modal__badge--manual" : "walkthrough-modal__badge--auto"}`}>
            {item.manual ? "Manual decision required" : "Resolved automatically"}
          </div>
          <p className="walkthrough-modal__summary">{item.summary}</p>
          <div className="walkthrough-modal__section">
            <div className="walkthrough-modal__label">Why this happened</div>
            <p>{item.reason}</p>
          </div>
          {item.aspects.length > 0 && (
            <div className="walkthrough-modal__section">
              <div className="walkthrough-modal__label">Affected aspects</div>
              <div className="walkthrough-modal__chips">
                {item.aspects.map((aspect) => (
                  <span key={aspect} className="node-chip node-chip--sm">{aspect}</span>
                ))}
              </div>
            </div>
          )}
          {item.outcomes.length > 0 && (
            <div className="walkthrough-modal__section">
              <div className="walkthrough-modal__label">Decision record</div>
              {item.outcomes.map((outcome) => (
                <div key={outcome} className="merge-proof__line">{outcome}</div>
              ))}
            </div>
          )}
          <div className="walkthrough-modal__section">
            <div className="walkthrough-modal__label">Operator note</div>
            <p className="walkthrough-modal__note">
              This behaves like a git conflict review step. In this demo run, the selected conflict policy resolved it automatically. A manual chooser would only appear when the active policy rejects or leaves ambiguity unresolved.
            </p>
          </div>
        </div>
        <div className="walkthrough-modal__actions">
          <button className="btn" type="button" onClick={onPrev} disabled={index === 0}>Previous</button>
          <button className="btn btn--primary" type="button" onClick={index === items.length - 1 ? onClose : onNext}>
            {index === items.length - 1 ? "Done" : "Next Conflict"}
          </button>
        </div>
      </div>
    </div>
  );
}

export function MergeProofPanel({
  mergePlan,
  mergeResult,
  walkthroughOpen,
  walkthroughIndex,
  onOpenWalkthrough,
  onCloseWalkthrough,
  onNextWalkthrough,
  onPrevWalkthrough,
}: {
  mergePlan: WorkerSnapshot["mergePlan"];
  mergeResult: WorkerSnapshot["mergeResult"];
  walkthroughOpen: boolean;
  walkthroughIndex: number;
  onOpenWalkthrough: () => void;
  onCloseWalkthrough: () => void;
  onNextWalkthrough: (maxIndex: number) => void;
  onPrevWalkthrough: () => void;
}) {
  const semantics = mergeResult?.semantics ?? mergePlan?.semantics ?? null;
  const identityRecords = mergeResult?.identity.records ?? mergePlan?.identity.records ?? [];
  const deletion = mergeResult?.deletion ?? mergePlan?.deletion ?? null;
  const conflictIsolation = mergeResult?.conflictIsolation ?? mergePlan?.conflictIsolation ?? null;
  const aspectPolicies = mergeResult?.aspectPolicies ?? mergePlan?.aspectPolicies ?? [];
  const aspectDecisions = mergeResult?.aspectDecisions ?? mergePlan?.aspectDecisions ?? [];
  const walkthroughItems = useMemo(
    () => buildConflictWalkthroughItems(mergePlan, mergeResult),
    [mergePlan, mergeResult],
  );

  if (!mergePlan && !mergeResult) {
    return <p className="panel__hint">Create a branch and merge to inspect the merge walkthrough and certification details.</p>;
  }

  return (
    <>
      <div className="merge-proof">
        <div className="merge-proof__block merge-proof__block--hero">
          <div className="merge-proof__title">Merge Outcome</div>
          <div className="merge-proof__summary">{describeMergeOutcome(mergePlan, mergeResult)}</div>
          {semantics && (
            <div className="merge-proof__policy-grid">
              <ProofStat label="Strategy" value={friendlyPolicy(semantics.strategyName, semantics.strategyBasis)} />
              <ProofStat label="Conflict" value={friendlyPolicy(semantics.conflictPolicyName, semantics.conflictPolicyBasis)} />
              <ProofStat label="Identity" value={friendlyPolicy(semantics.identityMatcherName, semantics.identityMatcherBasis)} />
              <ProofStat label="Isolation" value={friendlyPolicy(semantics.conflictIsolationName, semantics.conflictIsolationBasis)} />
            </div>
          )}
          {walkthroughItems.length > 0 && (
            <div className="merge-proof__actions">
              <button className="btn btn--primary" type="button" onClick={onOpenWalkthrough}>
                Walk Through Conflicts
              </button>
              <div className="merge-proof__microcopy">
                Step through each conflict like a git resolver. This run auto-resolves under the selected policy, so no manual choice is required.
              </div>
            </div>
          )}
        </div>

        {mergeResult && (
          <div className="merge-proof__block">
            <div className="merge-proof__title">Work Performed</div>
            <div className="merge-proof__policy-grid">
              <ProofStat label="Candidates" value={String(mergeResult.counters.finalCandidateBreadth)} />
              <ProofStat label="Reconciled" value={String(mergeResult.counters.reconciliationBreadth)} />
              <ProofStat label="Identity" value={`${mergeResult.counters.identityTargetCandidatesIndexed} indexed / ${mergeResult.counters.identitySourceLookups} lookups`} />
              <ProofStat label="Isolation" value={`${mergeResult.counters.conflictIsolationRecordCount} records / ${mergeResult.counters.conflictIsolationExpansionBreadth} widened`} />
            </div>
          </div>
        )}

        <details className="merge-proof__details">
          <summary>Raw certification details</summary>
          <div className="merge-proof__details-body">
            {mergePlan && (
              <div className="merge-proof__block">
                <div className="merge-proof__title">Plan Bounds</div>
                <div className="merge-proof__line">Candidates: {mergePlan.candidateCount}</div>
                <div className="merge-proof__line">Proof-min overlap: {mergePlan.sharedNodeCount}</div>
                <div className="merge-proof__line">Conservative expansion: {mergePlan.expandedNodeCount}</div>
                <div className="merge-proof__line">Node plan count: {mergePlan.nodePlanCount}</div>
              </div>
            )}

            {(mergePlan?.proof || mergeResult?.proof) && (
              <div className="merge-proof__block">
                <div className="merge-proof__title">Proof Chain</div>
                <div className="merge-proof__line">
                  Schema: {mergeResult?.proof?.proofSchemaVersion ?? mergePlan?.proof?.proofSchemaVersion ?? "pending"}
                </div>
                <div className="merge-proof__line">
                  Canonical lowering happens before execution. The executor consumes the frozen lowered bundle, not live policy selection.
                </div>
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

            {deletion && (
              <div className="merge-proof__block">
                <div className="merge-proof__title">Deletion Plan</div>
                <div className="merge-proof__line">Target-only count: {deletion.targetOnlyCount}</div>
                <div className="merge-proof__line">Rejected target-only: {deletion.rejectedTargetOnlyCount}</div>
                {deletion.targetOnlyNodes.length > 0 && (
                  <div className="merge-proof__line">Nodes: {deletion.targetOnlyNodes.slice(0, 4).join(", ")}</div>
                )}
              </div>
            )}

            {conflictIsolation && (
              <div className="merge-proof__block">
                <div className="merge-proof__title">Conflict Isolation</div>
                <div className="merge-proof__line">
                  Policy: {conflictIsolation.policyName ?? "pending"}
                  {conflictIsolation.policyBasis ? ` [${conflictIsolation.policyBasis}]` : ""}
                </div>
                <div className="merge-proof__line">Policy digest: {shortDigest(conflictIsolation.policyDigest)}</div>
                <div className="merge-proof__line">Expansion breadth: {conflictIsolation.expansionBreadth}</div>
                <div className="merge-proof__line">
                  Witness: {conflictIsolation.witnessGranularity ?? "pending"} / {conflictIsolation.witnessConflictRecordCount} conflict records
                </div>
                <div className="merge-proof__line">
                  Regions: {conflictIsolation.isolatedRegionCount} isolated / {conflictIsolation.hostDeclaredRegionCount} host-declared
                </div>
                <div className="merge-proof__line">
                  Conservative expansion nodes: {conflictIsolation.conservativeExpandedNodeCount}
                </div>
              </div>
            )}

            {aspectPolicies.length > 0 && (
              <div className="merge-proof__block">
                <div className="merge-proof__title">Aspect Policies</div>
                {aspectPolicies.slice(0, 6).map((record) => (
                  <div key={`${record.aspect}-${record.policyName}`} className="merge-proof__line">
                    {record.aspect}: {record.policyName} [{record.policyBasis}]
                    {record.affectedSourceNodes.length > 0
                      ? ` on ${record.affectedSourceNodes.slice(0, 3).join(", ")}`
                      : ""}
                  </div>
                ))}
              </div>
            )}

            {aspectDecisions.length > 0 && (
              <div className="merge-proof__block">
                <div className="merge-proof__title">Aspect Decisions</div>
                {aspectDecisions.slice(0, 6).map((record) => (
                  <div
                    key={`${record.aspect}-${record.sourceNode}-${record.targetNode ?? "none"}-${record.outcome}`}
                    className="merge-proof__line"
                  >
                    {record.aspect} on {record.sourceNode}
                    {record.targetNode ? ` -> ${record.targetNode}` : ""}
                    : {record.outcome} via {record.policyName} [{record.policyBasis}]
                  </div>
                ))}
              </div>
            )}

            {identityRecords.length > 0 && (
              <div className="merge-proof__block">
                <div className="merge-proof__title">Identity Witnesses</div>
                {identityRecords.slice(0, 4).map((record) => (
                  <div key={`${record.sourceNode}-${record.targetNode ?? "none"}`} className="merge-proof__line">
                    {record.sourceNode} {"->"} {record.targetNode ?? "none"} [{record.status}
                    {record.basis ? ` via ${record.basis}` : ""}, c={record.candidateCount}]
                  </div>
                ))}
              </div>
            )}

            {mergeResult && mergeResult.records.length > 0 && (
              <div className="merge-proof__block">
                <div className="merge-proof__title">Artifact Decisions</div>
                {mergeResult.records.slice(0, 6).map((record) => (
                  <div key={`${record.sourceNode}-${record.targetNode ?? "none"}-${record.action}`} className="merge-proof__line">
                    {record.sourceNode}: {record.action}
                    {record.identityStatus ? ` [${record.identityStatus}]` : ""}
                  </div>
                ))}
              </div>
            )}
          </div>
        </details>
      </div>
      {walkthroughOpen && walkthroughItems.length > 0 && (
        <ConflictWalkthroughModal
          items={walkthroughItems}
          index={walkthroughIndex}
          onClose={onCloseWalkthrough}
          onNext={() => onNextWalkthrough(walkthroughItems.length - 1)}
          onPrev={onPrevWalkthrough}
        />
      )}
    </>
  );
}
