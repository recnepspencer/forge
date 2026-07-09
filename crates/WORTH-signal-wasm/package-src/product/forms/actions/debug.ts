import { stableValueDigest } from "../values/value_paths.js";

export function readActionDebug(form, actionId) {
  const plan = form.actionPlan(actionId);
  const attempts = Object.freeze(
    form.actionHistory().filter((attempt) => attempt.action === actionId),
  );
  const executions = Object.freeze(
    form.actionExecutionHistory().filter((execution) => (
      execution.action === actionId || execution.targetAction === actionId
    )),
  );
  const latestAttempt = attempts.at(-1) ?? null;
  const latestExecution = executions.at(-1) ?? null;
  const blockers = Object.freeze(latestAttempt?.blockers ?? plan.readiness.blockers);
  const verification = form.verification();
  const latestReason = latestExecution?.reason
    ?? latestAttempt?.reason
    ?? blockers[0]?.reason
    ?? (plan.status === "accepted" ? "action plan accepted" : "action plan is not ready");
  const debug = {
    kind: "actionDebug",
    action: actionId,
    canRun: plan.readiness.canRun,
    pending: latestExecution?.resultKind === "pending",
    latestReason,
    blockers,
    plan,
    latestAttempt,
    latestExecution,
    attempts,
    executions,
    verification: Object.freeze({
      packageDigest: verification.packageDigest,
      actionPlanDigest: plan.planDigest,
      actionLifecycleDigest: verification.digests.actionLifecycleDigest,
      actionExecutionLifecycleDigest: verification.digests.actionExecutionLifecycleDigest,
    }),
  };
  return Object.freeze({
    ...debug,
    digest: stableValueDigest({
      action: debug.action,
      canRun: debug.canRun,
      pending: debug.pending,
      latestReason: debug.latestReason,
      blockerKinds: debug.blockers.map((blocker) => blocker.kind),
      planDigest: debug.plan.planDigest,
      latestAttemptDigest: debug.latestAttempt?.resultDigest ?? null,
      latestExecutionDigest: debug.latestExecution?.executionDigest ?? null,
      attemptDigests: debug.attempts.map((attempt) => attempt.resultDigest),
      executionDigests: debug.executions.map((execution) => execution.executionDigest),
      verification: debug.verification,
    }),
  });
}
