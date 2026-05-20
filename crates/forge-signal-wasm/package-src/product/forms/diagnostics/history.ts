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
    actionLifecycleDigest: stableValueDigest(state.actionHistory),
    actionExecutionLifecycleDigest: stableValueDigest(state.actionExecutionHistory),
    asyncValidationDigest: stableValueDigest(state.asyncValidationHistory),
    canonicalizationDigest: stableValueDigest(state.canonicalizationHistory),
    sourceCompatibilityDigest: stableValueDigest(state.sourceCompatibility),
    sourceCompatibilityHistoryDigest: stableValueDigest(state.sourceCompatibilityHistory),
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
