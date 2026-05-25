import { readRouteAuthorityContinuityAudit } from "../route_authority/continuity_audit.js";
import { stableValueDigest } from "../values/value_paths.js";

export function readFormDiagnosticsSummary(state) {
  const dirty = Object.freeze({
    isDirty: state.dirty.isDirty,
    semanticDirty: state.dirty.semanticDirty,
    changedFields: state.dirty.breadth.changedFields,
    omittedFields: state.dirty.breadth.omittedFields,
    clearedFields: state.dirty.breadth.clearedFields,
    digest: stableValueDigest(state.dirty),
  });
  const patch = Object.freeze({
    empty: state.patchPlan.empty,
    semanticDirty: state.patchPlan.semanticDirty,
    operationCount: state.patchPlan.operations.length,
    blockerCount: state.patchPlan.blocked.length,
    broadReplacement: state.patchPlan.broadReplacement,
    digest: state.patchPlan.equivalenceDigest,
  });
  const readiness = Object.freeze({
    canSubmit: state.readiness.canSubmit,
    blockerCount: state.readiness.blockers.length,
    blockerKinds: Object.freeze(uniqueBlockerKinds(state.readiness.blockers)),
    digest: stableValueDigest(state.readiness),
  });
  const validation = summarizedLane(state.validation.summary);
  const availability = summarizedLane(state.availability.summary);
  const admission = summarizedLane(state.admission.summary);
  const host = Object.freeze({
    summary: state.host.summary,
    digest: state.host.digest,
  });
  const interaction = Object.freeze({
    summary: state.interaction.summary,
    digest: state.interaction.digest,
  });
  const navigation = Object.freeze({
    summary: state.navigation.summary,
    digest: state.navigation.digest,
  });
  const presentation = Object.freeze({
    summary: state.presentation.summary,
    digest: state.presentation.digest,
  });
  const routeAuthority = Object.freeze({
    authorityAvailable: state.routeAuthority.summary.authorityAvailable,
    continuity: state.routeAuthority.summary.continuity,
    transitionKind: state.routeAuthority.summary.transitionKind,
    handoff: state.routeAuthority.summary.handoff === null
      ? null
      : Object.freeze({
        posture: state.routeAuthority.summary.handoff.posture,
        routeCoupledBehavior: state.routeAuthority.summary.handoff.routeCoupledBehavior,
        draftDisposition: state.routeAuthority.summary.handoff.draftDisposition,
      }),
    draftContinuity: state.routeAuthority.summary.draftContinuity === null
      ? null
      : Object.freeze({
        posture: state.routeAuthority.summary.draftContinuity.posture,
        authorityChange: state.routeAuthority.summary.draftContinuity.authorityChange,
        draftResolution: state.routeAuthority.summary.draftContinuity.draftResolution,
        draftChanged: state.routeAuthority.summary.draftContinuity.draftChanged,
      }),
    continuityAudit: readRouteAuthorityContinuityAudit(
      state.routeAuthority,
      state.steps,
      state.actions,
    ),
    digest: state.routeAuthority.digest,
  });
  const sourceCompatibility = Object.freeze({
    posture: state.sourceCompatibility.posture,
    digest: stableValueDigest(state.sourceCompatibility),
  });
  const resourceSource = state.resourceSource === null
    ? Object.freeze({
      present: false,
      digest: null,
      settlementKind: null,
      lifecycleActivity: null,
    })
    : Object.freeze({
      present: true,
      digest: state.resourceSource.digest,
      settlementKind: state.resourceSource.settlement.kind,
      lifecycleActivity: state.resourceSource.lifecycle.activity,
    });
  const steps = Object.freeze({
    summary: state.steps.summary,
    digest: stableValueDigest(state.steps.summary),
  });
  const actions = Object.freeze({
    summary: state.actions.summary,
    digest: stableValueDigest({
      summary: state.actions.summary,
      digests: state.actions.digests,
    }),
  });
  const histories = Object.freeze({
    actionAttempts: state.actionHistory.length,
    actionExecutions: state.actionExecutionHistory.length,
    asyncValidations: state.asyncValidationHistory.length,
    canonicalizations: state.canonicalizationHistory.length,
    resets: state.resetHistory.length,
    stateTransitions: state.stateHistory.length,
    replayRestores: state.replayRestoreHistory.length,
    sourceCompatibility: state.sourceCompatibilityHistory.length,
    presentations: state.presentationHistory.length,
  });
  const summary = {
    kind: "formDiagnosticsSummary",
    fieldCount: state.fieldCount,
    dirty,
    patch,
    readiness,
    validation,
    availability,
    admission,
    resourceSource,
    host,
    interaction,
    navigation,
    presentation,
    routeAuthority,
    sourceCompatibility,
    steps,
    actions,
    histories,
  };
  return Object.freeze({
    ...summary,
    digest: stableValueDigest(summary),
  });
}

function summarizedLane(summary) {
  return Object.freeze({
    summary,
    digest: stableValueDigest(summary),
  });
}

function uniqueBlockerKinds(blockers) {
  return [...new Set(blockers.map((blocker) => blocker.kind))];
}
