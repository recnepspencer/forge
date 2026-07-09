export function classifyRouteAuthorityTransition(previousAuthority, nextAuthority) {
  if (nextAuthority === null) {
    return previousAuthority === null ? "alreadyCleared" : "authorityCleared";
  }
  if (previousAuthority === null) {
    return "initialAuthority";
  }
  return previousAuthority.verificationDigest === nextAuthority.verification().formsAuthorityDigest
    ? "authorityRefreshed"
    : "authorityChanged";
}
