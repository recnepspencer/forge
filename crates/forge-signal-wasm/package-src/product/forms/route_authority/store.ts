import { createRouteAuthorityDraftContinuityArtifact } from "./draft_continuity.js";
import { classifyRouteAuthorityTransition } from "./transition.js";
import { createRouteAuthorityHandoffArtifact } from "./handoff.js";
import { stableValueDigest } from "../values/value_paths.js";

export function createRouteAuthorityStore() {
  let nextArtifactId = 1;
  let current = null;
  const history = [];
  return Object.freeze({
    report(authority, diagnostics) {
      const artifact = freezeRouteAuthorityArtifact(nextArtifactId++, {
        source: "report",
        routeId: authority.routeId,
        href: authority.href,
        scopeKind: authority.scopeKind,
        surfaceId: authority.surfaceId,
        continuity: authority.continuity,
        reason: authority.reason ?? "router admitted route authority",
        handoff: createRouteAuthorityHandoffArtifact({
          routeId: authority.routeId,
          href: authority.href,
          scopeKind: authority.scopeKind,
          surfaceId: authority.surfaceId,
          posture: authority.continuity,
          draftDisposition: diagnostics.continuityApplied,
          routeCoupledBehavior: authority.continuity === "defer" ? "deferred" : "admitted",
          transitionKind: diagnostics.transitionKind,
          reason: diagnostics.handoffReason,
        }),
        draftContinuity: createRouteAuthorityDraftContinuityArtifact({
          routeId: authority.routeId,
          href: authority.href,
          surfaceId: authority.surfaceId,
          continuityApplied: diagnostics.continuityApplied,
          transitionKind: diagnostics.transitionKind,
          previousAuthorityDigest: diagnostics.previousAuthorityDigest,
          previousDraftDigest: diagnostics.previousDraftDigest,
          nextDraftDigest: diagnostics.nextDraftDigest,
          reason: diagnostics.draftContinuityReason,
        }),
        continuityApplied: diagnostics.continuityApplied,
        transitionKind: diagnostics.transitionKind,
        previousAuthorityDigest: diagnostics.previousAuthorityDigest,
        previousDraftDigest: diagnostics.previousDraftDigest,
        nextDraftDigest: diagnostics.nextDraftDigest,
        verificationDigest: authority.verification().formsAuthorityDigest,
      });
      current = artifact;
      history.push(artifact);
      return artifact;
    },
    clear(reason = null) {
      const previousAuthority = current;
      current = null;
      const artifact = freezeRouteAuthorityArtifact(nextArtifactId++, {
        source: "clear",
        routeId: null,
        href: null,
        scopeKind: null,
        surfaceId: null,
        continuity: null,
        reason: reason ?? "route authority was cleared",
        handoff: createRouteAuthorityHandoffArtifact({
          routeId: null,
          href: null,
          scopeKind: null,
          surfaceId: null,
          posture: "cleared",
          draftDisposition: "clearedAuthority",
          routeCoupledBehavior: "cleared",
          transitionKind: classifyRouteAuthorityTransition(previousAuthority, null),
          reason: reason ?? "route authority was cleared",
        }),
        draftContinuity: createRouteAuthorityDraftContinuityArtifact({
          routeId: null,
          href: null,
          surfaceId: null,
          continuityApplied: "clearedAuthority",
          transitionKind: classifyRouteAuthorityTransition(previousAuthority, null),
          previousAuthorityDigest: previousAuthority?.routeAuthorityDigest ?? null,
          previousDraftDigest: null,
          nextDraftDigest: null,
          reason: reason ?? "route authority was cleared",
        }),
        continuityApplied: "clearedAuthority",
        transitionKind: classifyRouteAuthorityTransition(previousAuthority, null),
        previousAuthorityDigest: previousAuthority?.routeAuthorityDigest ?? null,
        previousDraftDigest: null,
        nextDraftDigest: null,
        verificationDigest: null,
      });
      history.push(artifact);
      return artifact;
    },
    current() {
      return current;
    },
    history() {
      return Object.freeze([...history]);
    },
  });
}

function freezeRouteAuthorityArtifact(artifactId, artifact) {
  return Object.freeze({
    kind: "routeAuthorityUpdate",
    artifactId,
    observedAtMs: Date.now(),
    ...artifact,
    routeAuthorityDigest: stableValueDigest({
      artifactId,
      ...artifact,
    }),
  });
}
