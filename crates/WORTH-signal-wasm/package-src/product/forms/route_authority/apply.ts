import { stableValueDigest } from "../values/value_paths.js";
import { continuityAppliedKind, routeAuthorityHandoffReason } from "./continuity.js";
import { routeAuthorityDraftContinuityReason } from "./draft_continuity.js";
import { classifyRouteAuthorityTransition } from "./transition.js";

export function applyReportedRouteAuthority(
  authority,
  form,
  routeAuthority,
  writeDraft,
  recordDraftWrite,
) {
  const previousAuthority = routeAuthority.current();
  const transitionKind = classifyRouteAuthorityTransition(previousAuthority, authority);
  const previousDraft = form.draft();
  const previousDraftDigest = stableValueDigest(previousDraft);
  let continuityApplied = transitionKind === "authorityRefreshed"
    ? "maintainedAuthority"
    : continuityAppliedKind(authority.continuity);
  if (
    transitionKind !== "authorityRefreshed"
    && authority.continuity === "discard"
    && previousDraftDigest !== "{}"
  ) {
    writeDraft({});
    recordDraftWrite(
      `routeAuthority:${authority.surfaceId}`,
      "routeAuthorityDiscard",
      previousDraft,
      null,
      authority.reason ?? "route authority discarded prior draft continuity",
    );
  }
  const nextDraftDigest = stableValueDigest(form.draft());
  return routeAuthority.report(authority, {
    continuityApplied,
    handoffReason: routeAuthorityHandoffReason(authority, continuityApplied),
    draftContinuityReason: routeAuthorityDraftContinuityReason(authority, continuityApplied),
    previousDraftDigest,
    nextDraftDigest,
    previousAuthorityDigest: previousAuthority?.routeAuthorityDigest ?? null,
    transitionKind,
  });
}
