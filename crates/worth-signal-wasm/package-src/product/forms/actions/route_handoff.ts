export function reportRouteHandoffForExecution(form, plan, execution) {
  if (
    execution.resultKind !== "pending"
    || plan?.kind !== "step"
    || plan.step?.routeCoupled !== true
  ) {
    return;
  }
  const authorityReport = form.routeAuthority();
  if (!authorityReport.summary.authorityAvailable || authorityReport.current === null) {
    return;
  }
  form.reportHandoff({
    status: "busy",
    target: plan.step.stepId,
    reason: `route-coupled step action "${plan.id}" handed off through admitted route authority`,
    token: execution.executionDigest,
    scopeKind: "route",
    surfaceId: authorityReport.current.surfaceId,
    operation: "handoff",
  });
}
