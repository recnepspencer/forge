export function recoveryActionsForBlockers(blockers, options = {}) {
  const actions = [];
  for (const blocker of blockers) {
    if (blocker.field !== undefined) {
      actions.push(recoveryAction("editField", blocker));
      actions.push(recoveryAction("resetField", blocker));
    }
    if (blocker.section !== undefined) {
      actions.push(recoveryAction("revealSection", blocker));
    }
  }
  if (blockers.length > 0) {
    actions.push(recoveryAction("focusFirstActionableBlocker", blockers[0]));
  }
  if (options.canAcceptCanonicalValue === true && blockers.length > 0) {
    actions.push(recoveryAction("acceptCanonicalValue", blockers[0]));
  }
  actions.push(...resourceRecoveryActions(blockers, options));
  return Object.freeze(dedupeRecoveryActions(actions));
}

function resourceRecoveryActions(blockers, options) {
  const actions = [];
  for (const blocker of blockers) {
    if (blocker.kind === "resource:stale") {
      if (options.resourceSource !== null && options.resourceSource !== undefined) {
        const revalidateAction = findAcceptedResourceAction(options.availableActions, "revalidate");
        if (revalidateAction !== null) {
          actions.push(recoveryAction("revalidateResourceSource", blocker, { action: revalidateAction }));
        }
        if (options.resourceSource.history.availability.replayExact.kind === "available") {
          actions.push(recoveryAction("replayExactResourceSource", blocker));
        }
      }
      continue;
    }
    if (blocker.kind === "resource:deliveryBasisDrift") {
      if (options.resourceSource !== null && options.resourceSource !== undefined) {
        const revalidateAction = findAcceptedResourceAction(options.availableActions, "revalidate");
        if (revalidateAction !== null) {
          actions.push(recoveryAction("revalidateResourceSource", blocker, { action: revalidateAction }));
        }
        const refreshAction = findAcceptedResourceAction(options.availableActions, "refresh");
        if (refreshAction !== null) {
          actions.push(recoveryAction("refreshResourceSource", blocker, { action: refreshAction }));
        }
      }
      continue;
    }
    if (blocker.kind === "resource:rejected" || blocker.kind === "resource:timedOut") {
      if (options.resourceSource !== null && options.resourceSource !== undefined) {
        const refreshAction = findAcceptedResourceAction(options.availableActions, "refresh");
        if (refreshAction !== null) {
          actions.push(recoveryAction("refreshResourceSource", blocker, { action: refreshAction }));
        }
      }
      continue;
    }
    if (
      blocker.kind === "resource:mergeConflict"
      || blocker.kind === "resource:mergeMappingUnavailable"
    ) {
      if (
        options.resourceSource?.rollback?.kind === "compactInverseAvailable"
        || options.resourceSource?.rollback?.kind === "exactBranchRestoreAvailable"
      ) {
        actions.push(recoveryAction("rollbackLastResourceEffect", blocker));
      }
      if (options.resourceSource?.history.availability.restoreExact.kind === "available") {
        actions.push(recoveryAction("restoreExactResourceSource", blocker));
      }
    }
  }
  return actions;
}

function findAcceptedResourceAction(plans, resourceActionKind) {
  if (!Array.isArray(plans)) {
    return null;
  }
  return plans.find((plan) => (
    plan.status === "accepted"
    && plan.resourceAction?.action?.kind === resourceActionKind
  ))?.id ?? null;
}

function recoveryAction(kind, blocker, overrides = {}) {
  return Object.freeze({
    kind,
    field: blocker.field,
    action: overrides.action ?? blocker.action,
    control: blocker.control,
    group: blocker.group,
    section: blocker.section,
    blockerKind: blocker.kind,
    reason: blocker.reason,
  });
}

function dedupeRecoveryActions(actions) {
  const seen = new Set();
  const deduped = [];
  for (const action of actions) {
    const key = JSON.stringify(action);
    if (!seen.has(key)) {
      seen.add(key);
      deduped.push(action);
    }
  }
  return deduped;
}
