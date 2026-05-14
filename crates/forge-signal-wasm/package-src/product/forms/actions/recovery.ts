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
  return Object.freeze(dedupeRecoveryActions(actions));
}

function recoveryAction(kind, blocker) {
  return Object.freeze({
    kind,
    field: blocker.field,
    action: blocker.action,
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
