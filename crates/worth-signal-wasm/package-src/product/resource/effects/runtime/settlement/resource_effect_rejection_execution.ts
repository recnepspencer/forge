import {
  retireAcquiredEffectBranchBatch,
} from "../../branches/resource_effect_branch_acquisition_execution.js";

async function rejectResourceEffectAndDependents(history, index, effect) {
  const ordered = dependencyRetirementOrder(index, effect);
  const retirements = await retireAcquiredEffectBranchBatch(
    history,
    ordered.map((entry) => Object.freeze({
      acquisition: entry.branch,
      reason: entry.effectId === effect.effectId
        ? "rejected"
        : "dependencyCancellation",
    })),
  );
  return Object.freeze(ordered.map((entry, position) => {
    const retirement = retirements[position];
    const terminalKind = entry.effectId === effect.effectId
      ? "rejectedAndRetired"
      : "dependencyCancelled";
    index.retire(entry.effectId, Object.freeze({
      kind: terminalKind,
      retirement,
    }));
    return Object.freeze({ effectId: entry.effectId, retirement });
  }));
}

function dependencyRetirementOrder(index, root) {
  const ordered = [];
  const visited = new Set();
  const visit = (entry) => {
    if (visited.has(entry.effectId)) return;
    visited.add(entry.effectId);
    for (const dependentId of index.reverseDependents(entry.effectId)) {
      const dependent = index.get(dependentId);
      if (dependent !== null && dependent.lifecycle !== "Retired") {
        requireDependencyCancellationPolicy(dependent);
        visit(dependent);
      }
    }
    ordered.push(entry);
  };
  visit(root);
  return ordered;
}

function requireDependencyCancellationPolicy(effect) {
  if (
    effect.dependencySet.closeoutPolicy
      !== "cancelOnDependencyRejection"
  ) {
    throw new TypeError(
      `resource effect ${effect.effectId} has no admitted dependency rejection closeout policy`,
    );
  }
}

export { rejectResourceEffectAndDependents };
