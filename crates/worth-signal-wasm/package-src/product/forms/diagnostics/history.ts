import { readRouteAuthorityContinuityAudit } from "../route_authority/continuity_audit.js";
import { stableValueDigest } from "../values/value_paths.js";

export function createFormDiagnosticsHistoryStore() {
  let nextArtifactId = 1;
  const history = [];

  return Object.freeze({
    reconcile(state, summary, diagnosticsStateDigest) {
      const artifact = createArtifact(nextArtifactId, state, summary, diagnosticsStateDigest);
      const latest = history.at(-1) ?? null;
      if (
        latest !== null
        && latest.summaryDigest === artifact.summaryDigest
        && latest.diagnosticsStateDigest === artifact.diagnosticsStateDigest
      ) {
        return latest;
      }
      nextArtifactId += 1;
      history.push(artifact);
      return artifact;
    },
    history() {
      return Object.freeze([...history]);
    },
  });
}

export function digestFormDiagnosticsHistory(history) {
  return stableValueDigest(history.map((artifact) => artifact.diagnosticsDigest));
}

function createArtifact(artifactId, state, summary, diagnosticsStateDigest) {
  const digestInput = {
    kind: "formDiagnosticsHistory",
    artifactId,
    summaryDigest: summary.digest,
    diagnosticsStateDigest,
    sourceAuthorityDigest: state.sourceAuthority.sourceAuthorityDigest,
    patchPlanDigest: state.patchPlan.equivalenceDigest,
    readinessDigest: stableValueDigest(state.readiness),
    validationDigest: stableValueDigest(state.validation),
    availabilityDigest: stableValueDigest(state.availability),
    admissionDigest: stableValueDigest(state.admission),
    actionPlanDigestSetDigest: state.actions.digests.planDigestSetDigest,
    actionLifecycleDigest: stableValueDigest(historyDigestChain(state.actionHistory, ["resultDigest"])),
    actionExecutionLifecycleDigest: stableValueDigest(
      historyDigestChain(state.actionExecutionHistory, ["executionDigest"]),
    ),
    asyncValidationDigest: stableValueDigest(
      historyDigestChain(state.asyncValidationHistory, ["lifecycleDigest"]),
    ),
    canonicalizationDigest: stableValueDigest(
      historyDigestChain(state.canonicalizationHistory, ["canonicalizationDigest"]),
    ),
    sourceCompatibilityDigest: stableValueDigest(state.sourceCompatibility),
    sourceCompatibilityHistoryDigest: stableValueDigest(
      historyDigestChain(state.sourceCompatibilityHistory, ["compatibilityDigest"]),
    ),
    routeAuthorityDigest: state.routeAuthority.digest,
    routeAuthorityTransitionKind: state.routeAuthority.summary.transitionKind,
    routeAuthorityHandoffPosture: state.routeAuthority.summary.handoff?.posture ?? null,
    routeAuthorityRouteCoupledBehavior:
      state.routeAuthority.summary.handoff?.routeCoupledBehavior ?? null,
    routeAuthorityDraftResolution:
      state.routeAuthority.summary.draftContinuity?.draftResolution ?? null,
    routeAuthorityContinuityAuditDigest: readRouteAuthorityContinuityAudit(
      state.routeAuthority,
      state.steps,
      state.actions,
    ).digest,
    resourceSourceDigest: state.resourceSource?.digest ?? null,
    collaborationDigest: state.collaboration.digest,
    interactionDigest: state.interaction.digest,
    navigationDigest: state.navigation.digest,
    presentationDigest: state.presentation.digest,
    historyCounts: summary.histories,
  };
  const artifact = {
    ...digestInput,
    observedAtMs: Date.now(),
  };
  return Object.freeze({
    ...artifact,
    diagnosticsDigest: stableValueDigest(digestInput),
  });
}

function historyDigestChain(history, digestKeys) {
  return history.map((artifact) => artifactDigestToken(artifact, digestKeys));
}

function artifactDigestToken(artifact, digestKeys) {
  if (artifact === null || artifact === undefined) {
    return null;
  }
  for (const key of digestKeys) {
    const digest = artifact[key];
    if (typeof digest === "string") {
      return digest;
    }
  }
  return stableValueDigest(artifact);
}
