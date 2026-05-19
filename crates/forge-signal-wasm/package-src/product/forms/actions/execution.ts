import { cloneFormValue, isPlainObject, stableValueDigest } from "../values/value_paths.js";

export function createActionExecutionStore(actionAttempts) {
  let nextOperationId = 1;
  const history = [];
  const pending = new Map();
  return Object.freeze({
    execute(plan, options = {}) {
      const attempt = actionAttempts.attempt(plan);
      const superseded = supersedePendingOperation(plan);
      const execution = executionForAttempt(nextOperationId, plan, attempt, superseded, options);
      nextOperationId += 1;
      if (execution.resultKind === "pending") {
        pending.set(execution.operationId, execution);
      }
      history.push(execution);
      return execution;
    },
    executeResolved(plan, payload) {
      const attempt = actionAttempts.attempt(plan);
      const superseded = supersedePendingOperation(plan);
      const execution = resolvedExecutionForAttempt(nextOperationId, plan, attempt, superseded, payload);
      nextOperationId += 1;
      history.push(execution);
      return execution;
    },
    fulfill(operationId, payload = {}, currentPlanForAction = null) {
      return settlePendingOperation(operationId, "fulfilled", payload, currentPlanForAction);
    },
    reject(operationId, payload = {}, currentPlanForAction = null) {
      return settlePendingOperation(operationId, "rejected", payload, currentPlanForAction);
    },
    cancel(operationId, payload = {}, currentPlanForAction = null) {
      return settlePendingOperation(operationId, "cancelled", payload, currentPlanForAction);
    },
    timeout(operationId, payload = {}, currentPlanForAction = null) {
      return settlePendingOperation(operationId, "timedOut", payload, currentPlanForAction);
    },
    retry(operationId, currentPlanForAction = null) {
      const previous = latestExecutionForOperation(operationId);
      if (!isRetryableExecution(previous)) {
        return recordStaleCompletion(nextOperationId++, operationId, "retry target is not retryable", previous);
      }
      const currentPlan = currentPlanSnapshot(previous, currentPlanForAction);
      const staleReason = staleOperationReason(previous, currentPlan);
      if (staleReason !== null) {
        return recordStaleCompletion(nextOperationId++, operationId, staleReason, previous);
      }
      const retryPlan = previous.planSnapshot;
      const retry = executionArtifact(nextOperationId++, retryPlan, previous.attempt, "pending", {
        reason: "action execution retry started",
        retryOfOperationId: operationId,
      });
      pending.set(retry.operationId, retry);
      history.push(retry);
      return retry;
    },
    history() {
      return Object.freeze([...history]);
    },
  });

  function settlePendingOperation(operationId, resultKind, payload, currentPlanForAction) {
    const operation = pending.get(operationId);
    if (!operation) {
      return recordStaleCompletion(
        nextOperationId++,
        operationId,
        `action execution ${resultKind} arrived after the operation was no longer pending`,
        latestExecutionForOperation(operationId),
      );
    }
    const currentPlan = currentPlanSnapshot(operation, currentPlanForAction);
    const staleReason = staleOperationReason(operation, currentPlan);
    if (staleReason !== null) {
      pending.delete(operationId);
      return recordStaleCompletion(nextOperationId++, operationId, staleReason, operation);
    }
    pending.delete(operationId);
    const settled = executionArtifact(nextOperationId++, operation.planSnapshot, operation.attempt, resultKind, {
      reason: payload.reason ?? `action execution ${resultKind}`,
      serverMessages: normalizeServerMessages(payload.messages),
      canonicalValue: cloneCanonicalValue(payload.canonicalValue),
      recoveryActions: recoveryActionsForExecution(resultKind, operation, currentPlan),
      operationId,
    });
    history.push(settled);
    return settled;
  }

  function recordStaleCompletion(operationId, targetOperationId, reason, targetOperation) {
    const stale = staleCompletionArtifact(operationId, targetOperationId, reason, targetOperation);
    history.push(stale);
    return stale;
  }

  function latestExecutionForOperation(operationId) {
    return [...history].reverse().find((entry) => entry.operationId === operationId);
  }

  function supersedePendingOperation(plan) {
    if (plan.idempotency !== "supersede") {
      return null;
    }
    const previous = [...pending.values()]
      .reverse()
      .find((operation) => operation.action === plan.id);
    if (!previous) {
      return null;
    }
    pending.delete(previous.operationId);
    const superseded = executionArtifact(
      nextOperationId++,
      previous.planSnapshot,
      previous.attempt,
      "superseded",
      {
        reason: "pending action execution was superseded by a newer execution",
        supersededByOperationId: nextOperationId,
        operationId: previous.operationId,
      },
    );
    history.push(superseded);
    return superseded;
  }
}

function executionForAttempt(operationId, plan, attempt, superseded, options = {}) {
  if (attempt.resultKind === "denied" || attempt.resultKind === "noOp") {
    return executionArtifact(operationId, plan, attempt, attempt.resultKind, {
      reason: "action execution did not start because action attempt was terminal",
      effectStarted: false,
      resourceSettlement: options.resourceSettlement,
      recoveryActions: plan.recoveryActions,
    });
  }
  if (!requiresPendingExecution(plan)) {
    return executionArtifact(operationId, plan, attempt, "fulfilled", {
      reason: "action execution fulfilled synchronously",
      effectStarted: false,
      supersededOperationId: superseded?.operationId,
    });
  }
  return executionArtifact(operationId, plan, attempt, "pending", {
    reason: "action execution is pending external or asynchronous settlement",
    effectStarted: true,
    supersededOperationId: superseded?.operationId,
  });
}

function resolvedExecutionForAttempt(operationId, plan, attempt, superseded, payload) {
  if (attempt.resultKind === "denied" || attempt.resultKind === "noOp") {
    return executionArtifact(operationId, plan, attempt, attempt.resultKind, {
      reason: "action execution did not start because action attempt was terminal",
      effectStarted: false,
      recoveryActions: plan.recoveryActions,
    });
  }
  return executionArtifact(operationId, plan, attempt, payload.resultKind, {
    reason: payload.reason,
    effectStarted: payload.effectStarted,
    canonicalValue: payload.canonicalValue,
    resourceSubmission: payload.resourceSubmission,
    resourceSettlement: payload.resourceSettlement,
    resourceLifecycle: payload.resourceLifecycle,
    resourceRecovery: payload.resourceRecovery,
    recoveryActions: payload.resultKind === "denied" ? plan.recoveryActions : Object.freeze([]),
    supersededOperationId: superseded?.operationId,
  });
}

function executionArtifact(operationId, plan, attempt, resultKind, options = {}) {
  const artifact = {
    kind: "actionExecution",
    operationId: options.operationId ?? operationId,
    observedAtMs: Date.now(),
    action: plan.id,
    actionKind: plan.kind,
    attemptId: attempt.attemptId,
    attemptResultKind: attempt.resultKind,
    resultKind,
    planDigest: plan.planDigest,
    attemptDigest: attempt.resultDigest,
    effectStarted: options.effectStarted ?? resultKind === "pending",
    stale: false,
    reason: options.reason,
    proof: plan.proof,
    planSnapshot: plan,
    attempt,
    serverMessages: options.serverMessages ?? Object.freeze([]),
    canonicalValue: options.canonicalValue,
    resourceSubmission: options.resourceSubmission,
    resourceSettlement: options.resourceSettlement,
    resourceLifecycle: options.resourceLifecycle,
    resourceRecovery: options.resourceRecovery,
    recoveryActions: options.recoveryActions ?? Object.freeze([]),
    retryOfOperationId: options.retryOfOperationId,
    supersededOperationId: options.supersededOperationId,
    supersededByOperationId: options.supersededByOperationId,
  };
  return Object.freeze({
    ...artifact,
    executionDigest: stableValueDigest(artifact),
  });
}

function staleCompletionArtifact(operationId, targetOperationId, reason, targetOperation) {
  const artifact = {
    kind: "actionExecution",
    operationId,
    observedAtMs: Date.now(),
    targetOperationId,
    targetAction: targetOperation?.action ?? null,
    targetPlanDigest: targetOperation?.planDigest ?? null,
    targetExecutionDigest: targetOperation?.executionDigest ?? null,
    action: null,
    actionKind: null,
    attemptId: null,
    attemptResultKind: null,
    resultKind: "staleCompletion",
    planDigest: null,
    attemptDigest: null,
    effectStarted: false,
    stale: true,
    reason,
    serverMessages: Object.freeze([]),
    resourceSubmission: null,
    resourceSettlement: null,
    resourceRecovery: null,
    recoveryActions: Object.freeze([]),
  };
  return Object.freeze({
    ...artifact,
    executionDigest: stableValueDigest(artifact),
  });
}

function currentPlanSnapshot(operation, currentPlanForAction) {
  if (typeof currentPlanForAction !== "function") {
    return null;
  }
  return currentPlanForAction(operation.action);
}

function staleOperationReason(operation, currentPlan) {
  if (currentPlan === null) {
    return null;
  }
  if (currentPlan.planDigest === operation.planDigest) {
    return null;
  }
  return "action execution completion targeted a superseded form truth snapshot";
}

function isRetryableExecution(operation) {
  return (
    operation !== undefined &&
    operation.planSnapshot !== undefined &&
    (operation.resultKind === "rejected" ||
      operation.resultKind === "timedOut" ||
      operation.resultKind === "cancelled")
  );
}

function requiresPendingExecution(plan) {
  return plan.effectPolicy === "deferred" || plan.hostEffect !== null;
}

function recoveryActionsForExecution(resultKind, operation, currentPlan) {
  if (resultKind === "fulfilled") {
    return Object.freeze([]);
  }
  if (currentPlan !== null && currentPlan.planDigest === operation.planDigest) {
    return currentPlan.recoveryActions;
  }
  return operation.planSnapshot?.recoveryActions ?? Object.freeze([]);
}

function cloneCanonicalValue(canonicalValue) {
  return canonicalValue === undefined ? undefined : freezeCanonicalValue(cloneFormValue(canonicalValue));
}

function freezeCanonicalValue(value) {
  if (Array.isArray(value)) {
    for (const entry of value) {
      freezeCanonicalValue(entry);
    }
    return Object.freeze(value);
  }
  if (isPlainObject(value)) {
    for (const entry of Object.values(value)) {
      freezeCanonicalValue(entry);
    }
    return Object.freeze(value);
  }
  return value;
}

function normalizeServerMessages(messages) {
  if (messages === undefined) {
    return Object.freeze([]);
  }
  return Object.freeze(
    messages.map((message) => Object.freeze({
      code: String(message.code),
      target: message.target === undefined ? null : String(message.target),
      scope: message.scope === undefined ? "form" : String(message.scope),
      severity: message.severity === undefined ? "error" : String(message.severity),
      source: "server",
    })),
  );
}
