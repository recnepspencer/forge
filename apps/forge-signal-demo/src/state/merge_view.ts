import type { WorkerSnapshot } from "../gear-scene/worker/protocol";

export type ConflictWalkthroughItem = {
  sourceNode: string;
  targetNode: string | null;
  manual: boolean;
  summary: string;
  reason: string;
  aspects: string[];
  outcomes: string[];
};

export function mergeConflictNodeIds(
  mergePlan: WorkerSnapshot["mergePlan"],
  mergeResult: WorkerSnapshot["mergeResult"],
): Set<string> {
  const nodes = new Set<string>();
  for (const record of mergePlan?.conflictIsolation.records ?? mergeResult?.conflictIsolation.records ?? []) {
    nodes.add(record.sourceNode);
    if (record.targetNode) nodes.add(record.targetNode);
  }
  return nodes;
}

export function mergeResolvedNodeIds(
  _mergePlan: WorkerSnapshot["mergePlan"],
  mergeResult: WorkerSnapshot["mergeResult"],
): Set<string> {
  const nodes = new Set<string>();
  for (const record of mergeResult?.records ?? []) {
    if ((record.resolvedConflictKinds?.length ?? 0) > 0) {
      nodes.add(record.sourceNode);
      if (record.targetNode) nodes.add(record.targetNode);
    }
  }
  return nodes;
}

export function friendlyPolicy(name: string | null, basis: string | null): string {
  if (!name) {
    return "Pending";
  }
  const shortName = name.replace(/^signal\./, "");
  return basis ? `${shortName} (${basis})` : shortName;
}

export function describeMergeOutcome(
  mergePlan: WorkerSnapshot["mergePlan"],
  mergeResult: WorkerSnapshot["mergeResult"],
): string {
  if (mergeResult) {
    return `${mergeResult.conflictCount} conflict regions were reviewed, ${mergeResult.adoptedCount} artifacts were adopted, and ${mergeResult.replacedCount} artifacts were replaced. The selected policies determined the outcome before execution.`;
  }
  if (mergePlan) {
    return `The merge is planned but not yet executed. ${mergePlan.candidateCount} candidate artifacts are in scope, and ${mergePlan.aspectDecisions.length} aspect-level decisions have already been lowered.`;
  }
  return "No merge data yet.";
}

export function buildConflictWalkthroughItems(
  mergePlan: WorkerSnapshot["mergePlan"],
  mergeResult: WorkerSnapshot["mergeResult"],
): ConflictWalkthroughItem[] {
  const semantics = mergeResult?.semantics ?? mergePlan?.semantics ?? null;
  const isolationRecords = mergeResult?.conflictIsolation.records ?? mergePlan?.conflictIsolation.records ?? [];
  const aspectDecisions = mergeResult?.aspectDecisions ?? mergePlan?.aspectDecisions ?? [];

  return isolationRecords.map((record) => {
    const matchingDecisions = aspectDecisions.filter(
      (decision) => decision.sourceNode === record.sourceNode && decision.targetNode === record.targetNode,
    );
    const outcomes = matchingDecisions.map(
      (decision) => `${decision.aspect}: ${decision.outcome} via ${decision.policyName} [${decision.policyBasis}]`,
    );
    const autoResolved = !/reject|manual|ambiguous/i.test(semantics?.conflictPolicyName ?? "");
    return {
      sourceNode: record.sourceNode,
      targetNode: record.targetNode,
      manual: !autoResolved,
      summary: autoResolved
        ? `${record.sourceNode} was auto-resolved against ${record.targetNode ?? "a new target artifact"} under the active conflict policy.`
        : `${record.sourceNode} needs a human decision before merge can continue.`,
      reason: autoResolved
        ? `The runtime isolated this conflict at ${record.granularity} granularity and then applied ${friendlyPolicy(semantics?.conflictPolicyName ?? null, semantics?.conflictPolicyBasis ?? null)}.`
        : `The active conflict policy does not permit an automatic resolution for this conflict shape, so a manual chooser should appear here.`,
      aspects: record.isolatedAspects,
      outcomes,
    };
  });
}

export function shortDigest(value: string | null): string {
  if (!value) {
    return "pending";
  }
  return `${value.slice(0, 12)}...`;
}
