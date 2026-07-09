export function continuityAppliedKind(continuity) {
  if (continuity === "preserve") {
    return "preservedDraft";
  }
  if (continuity === "freeze") {
    return "frozeDraft";
  }
  if (continuity === "discard") {
    return "discardedDraft";
  }
  return "deferredDraft";
}

export function routeAuthorityHandoffReason(authority, continuityApplied) {
  if (authority.continuity === "defer") {
    return authority.reason
      ?? "route authority deferred route-coupled form behavior until later admitted truth is present";
  }
  if (continuityApplied === "maintainedAuthority") {
    return authority.reason
      ?? "router admitted route authority maintained existing route-scoped draft continuity";
  }
  if (authority.continuity === "freeze") {
    return authority.reason
      ?? "router admitted route authority froze route-scoped draft continuity";
  }
  if (authority.continuity === "discard") {
    return authority.reason
      ?? "router admitted route authority discarded route-scoped draft continuity";
  }
  return authority.reason
    ?? "router admitted route authority preserved route-scoped draft continuity";
}
