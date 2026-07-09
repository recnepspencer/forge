export function createRouteAuthorityHandoffArtifact({
  routeId,
  href,
  scopeKind,
  surfaceId,
  posture,
  draftDisposition,
  routeCoupledBehavior,
  transitionKind,
  reason,
}) {
  return Object.freeze({
    kind: "routeAuthorityHandoff",
    routeId,
    href,
    scopeKind,
    surfaceId,
    posture,
    draftDisposition,
    routeCoupledBehavior,
    transitionKind,
    reason,
  });
}

export function routeAuthorityAllowsRouteCoupledBehavior(routeAuthorityReport) {
  return routeAuthorityReport.summary.handoff?.routeCoupledBehavior === "admitted";
}

export function routeAuthorityUnavailableReason(routeAuthorityReport, subject) {
  const handoff = routeAuthorityReport.summary.handoff;
  if (
    handoff?.routeCoupledBehavior === "deferred"
    || handoff?.routeCoupledBehavior === "cleared"
  ) {
    return handoff.reason;
  }
  return `${subject} requires route authority outside controller-local navigation`;
}
