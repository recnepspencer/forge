export function routeAuthorityWriteBlocker(routeAuthority, fieldId) {
  const current = routeAuthority.current;
  if (current === null || current.continuity !== "freeze") {
    return null;
  }
  return Object.freeze({
    kind: "routeAuthority:frozen",
    field: fieldId,
    reason: current.reason,
    routeAuthority: current,
  });
}

export function routeAuthorityReadinessBlockers(routeAuthority) {
  const current = routeAuthority.current;
  if (current === null || current.continuity !== "freeze") {
    return Object.freeze([]);
  }
  return Object.freeze([Object.freeze({
    kind: "routeAuthority:frozen",
    reason: current.reason ?? "router admitted route authority froze draft continuity",
    routeAuthority: current,
  })]);
}
