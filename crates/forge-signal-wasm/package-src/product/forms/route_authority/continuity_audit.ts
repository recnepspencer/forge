import { stableValueDigest } from "../values/value_paths.js";

export function readRouteAuthorityContinuityAudit(routeAuthority, steps, actions) {
  const routeCoupledSteps = steps.artifacts.filter((step) => step.routeCoupled);
  const routeCoupledActions = actions.plans.filter((plan) => plan.diagnostics.routeSemantics === "routeAuthorityRequired");
  const blockingStep = routeCoupledSteps.find((step) => step.posture === "unavailable");
  const blockingAction = routeCoupledActions.find((plan) =>
    routeAuthorityBlockingActionReason(plan) !== null,
  );
  const artifact = {
    kind: "routeAuthorityContinuityAudit",
    handoffPosture: routeAuthority.summary.handoff?.posture ?? null,
    routeCoupledBehavior: routeAuthority.summary.handoff?.routeCoupledBehavior ?? null,
    draftDisposition: routeAuthority.summary.handoff?.draftDisposition ?? null,
    draftResolution: routeAuthority.summary.draftContinuity?.draftResolution ?? null,
    transitionKind: routeAuthority.summary.transitionKind,
    authorityAvailable: routeAuthority.summary.authorityAvailable,
    routeCoupledSteps: Object.freeze({
      total: routeCoupledSteps.length,
      active: routeCoupledSteps.filter((step) => step.posture === "active").length,
      unavailable: routeCoupledSteps.filter((step) => step.posture === "unavailable").length,
    }),
    routeCoupledActions: Object.freeze({
      total: routeCoupledActions.length,
      accepted: routeCoupledActions.filter((plan) => plan.status === "accepted").length,
      denied: routeCoupledActions.filter((plan) => plan.status === "denied").length,
    }),
    blockingReason:
      blockingStep?.reason
      ?? routeAuthorityBlockingActionReason(blockingAction)
      ?? null,
  };
  return Object.freeze({
    ...artifact,
    digest: stableValueDigest(artifact),
  });
}

function routeAuthorityBlockingActionReason(plan) {
  if (!plan) {
    return null;
  }
  const routeAuthorityBlocker = plan.readiness.blockers.find((blocker) =>
    blocker.kind === "action:deferred" || blocker.kind === "routeAuthority:frozen"
  );
  return routeAuthorityBlocker?.reason ?? null;
}
