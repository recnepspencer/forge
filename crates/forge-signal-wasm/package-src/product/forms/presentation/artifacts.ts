import { stableValueDigest } from "../values/value_paths.js";
import { actionLanes } from "./action_lanes.js";
import { acknowledgedOrSettlingLane, baseLane, collaborationLane, exitLane, externalLane, navigationLanes } from "./auxiliary_lanes.js";
import { entryLane } from "./entry_lane.js";
import { busyVisibilityStatus } from "./policy_timing.js";

export function readPresentationReport(policy, form, actionDeclarations, stepDeclarations, settlements) {
  const nowMs = Date.now();
  const sourceCompatibility = form.sourceCompatibility();
  const sourceAdmission = form.sourceAdmission();
  const draftRestore = form.draftRestore();
  const host = form.host();
  const inputCapabilities = form.inputCapabilities();
  const accessibility = form.accessibility();
  const collaboration = form.collaboration();
  const exit = form.exit();
  const interaction = form.interaction();
  const navigation = form.navigation();
  const availability = form.availability();
  const validation = form.validation();
  const layoutMeasurement = form.layoutMeasurement();
  const messageVisibility = form.messages();
  const actions = form.actions();
  const actionExecutions = form.actionExecutionHistory();
  const asyncValidationHistory = form.asyncValidationHistory();
  const canonicalizationHistory = form.canonicalizationHistory();
  const messages = messagesLane(policy.messages, messageVisibility, validation, nowMs);
  const layoutLaneArtifact = layoutLane(policy.layout, layoutMeasurement, form);
  const canonicalization = canonicalizationLane(policy.canonicalization, canonicalizationHistory, settlements, nowMs);
  const handoff = externalLane("handoff", policy.handoff, settlements, nowMs);
  const navigationLanesList = navigationLanes(policy.navigation, navigation, actionDeclarations, stepDeclarations, settlements, nowMs);
  const controllerLocalNavigation = navigationLanesList.find((lane) => lane.id === "navigation:local") ?? baseLane(
    "navigation:local",
    "navigation",
    "step",
    policy.navigation,
    "ready",
    {
      target: null,
      reason: "controller-local navigation has no active step",
      token: navigation.digest,
    },
  );
  const lanes = [
    entryLane(
      policy.entry,
      sourceAdmission,
      draftRestore,
      sourceCompatibility,
      validation,
      asyncValidationHistory,
      form.readiness(),
      host,
      inputCapabilities,
      accessibility,
      layoutMeasurement,
      nowMs,
    ),
    interactionLane(policy.interaction, interaction),
    availabilityLane(policy.availability, availability),
    messages,
    layoutLaneArtifact,
    ...actionLanes(policy.action, actions.catalog, actionExecutions, settlements, nowMs, {
      canonicalization,
      messages,
      layout: layoutLaneArtifact,
      navigation: controllerLocalNavigation,
      handoff,
      focusTarget: accessibility.focusTarget,
    }),
    canonicalization,
    resourceDriftLane(policy.resourceDrift, form, canonicalizationHistory),
    collaborationLane(policy.collaboration, collaboration, settlements, nowMs),
    exitLane(policy.exit, exit, settlements, nowMs),
    externalLane("attachments", policy.attachments, settlements, nowMs),
    externalLane("media", policy.media, settlements, nowMs),
    handoff,
    ...navigationLanesList,
  ];
  const summary = Object.freeze({
    total: lanes.length,
    pending: lanes.filter((lane) => lane.status === "pending").length,
    busy: lanes.filter((lane) => lane.status === "busy").length,
    settling: lanes.filter((lane) => lane.status === "settling").length,
    ready: lanes.filter((lane) => lane.status === "ready").length,
    failed: lanes.filter((lane) => lane.status === "failed").length,
    unavailable: lanes.filter((lane) => lane.status === "unavailable").length,
    acknowledgementRequired: lanes.filter((lane) => lane.acknowledgement.required).length,
  });
  const history = settlements.history();
  const acknowledgements = acknowledgementSummary(lanes, history);
  const counters = Object.freeze({
    costBasis: "derivedPresentationLifecycleScan",
    incrementalStatus: "notIncremental",
    lanes: lanes.length,
    interactionLanes: lanes.filter((lane) => lane.lane === "interaction").length,
    actionLanes: lanes.filter((lane) => lane.lane === "action").length,
    canonicalizationLanes: lanes.filter((lane) => lane.lane === "canonicalization").length,
    resourceDriftLanes: lanes.filter((lane) => lane.lane === "resourceDrift").length,
    externalLanes: lanes.filter((lane) => (
      lane.lane === "collaboration" ||
      lane.lane === "exit" ||
      lane.lane === "attachments" ||
      lane.lane === "media" ||
      lane.lane === "handoff"
    )).length,
    navigationLanes: lanes.filter((lane) => lane.lane === "navigation").length,
    settlingLanes: summary.settling,
    unavailableLanes: summary.unavailable,
    requiredAcknowledgements: summary.acknowledgementRequired,
    settlementArtifacts: history.length,
  });
  return Object.freeze({
    lanes: Object.freeze(lanes),
    summary,
    acknowledgements,
    counters,
    history,
    digest: stableValueDigest({ lanes, summary, acknowledgements, counters, history }),
  });
}

function acknowledgementSummary(lanes, history) {
  const requiredLanes = lanes.filter((lane) => lane.acknowledgement.required);
  const summary = {
    required: requiredLanes.length,
    pending: requiredLanes.filter((lane) => lane.acknowledgement.status === "pending").length,
    acknowledged: requiredLanes.filter((lane) => lane.acknowledgement.status === "acknowledged").length,
    timedOut: requiredLanes.filter((lane) => lane.acknowledgement.status === "timedOut").length,
    ignored: history.filter((entry) => entry.kind === "presentationSettlement" && entry.resultKind === "ignored").length,
    noOp: history.filter((entry) => entry.kind === "presentationSettlement" && entry.resultKind === "noOp").length,
  };
  return Object.freeze({
    ...summary,
    digest: summary.required === 0 ? null : stableValueDigest(summary),
  });
}

function interactionLane(policy, interaction) {
  const focusedField = interaction.summary.focusedField;
  const interactionTarget = focusedField ??
    interaction.summary.focusIntentField ??
    interaction.fields.find((field) => field.composing)?.field ??
    interaction.fields.find((field) => field.touched || field.visited)?.field ??
    null;
  return baseLane("interaction", "interaction", policy.scope, policy, "ready", {
    reason: interaction.summary.focusPosture === "unavailable"
      ? "interaction presentation is settled without focus support"
      : interaction.summary.submitIntent.active
        ? `interaction presentation is settled with ${interaction.summary.submitIntent.source} submit intent`
      : interaction.summary.composingFields > 0
        ? "interaction presentation is settled with active composition state"
      : interactionTarget === null
        ? "interaction presentation is settled with no active interaction target"
        : `interaction presentation is settled on ${interactionTarget}`,
    target: interactionTarget,
    token: interaction.digest,
  });
}

function availabilityLane(policy, availability) {
  if (availability.summary.unavailable > 0) {
    return baseLane("availability", "availability", policy.scope, policy, "unavailable", {
      reason: "availability presentation includes unavailable scopes",
      token: stableValueDigest(availability.summary),
      acknowledgementRequired: policy.unavailableAcknowledgement === "required",
    });
  }
  return baseLane("availability", "availability", policy.scope, policy, "ready", {
    reason: availability.summary.blocked > 0
      ? "availability presentation is settled with blocked scopes"
      : "availability presentation is settled",
  });
}

function messagesLane(policy, messageVisibility, validation, nowMs) {
  if (validation.summary.pending > 0) {
    const pendingArtifact = validation.artifacts.find((artifact) => artifact.posture === "pending") ?? null;
    const status = busyVisibilityStatus(policy, pendingArtifact?.observedAtMs ?? nowMs, nowMs);
    return baseLane("messages", "messages", policy.scope, policy, status, {
      reason: status === "pending"
        ? "message presentation is delaying busy reveal for pending validation artifacts"
        : "message presentation is waiting for pending validation artifacts",
      token: stableValueDigest(validation.summary),
    });
  }
  if (messageVisibility.current && messageVisibility.current.status !== "ready") {
    return baseLane("messages", "messages", policy.scope, policy, messageVisibility.current.status, {
      reason: messageVisibility.current.reason,
      target: messageVisibility.current.target,
      token: messageVisibility.current.messageDigest,
      acknowledgementRequired: messageVisibility.current.status === "unavailable"
        && policy.unavailableAcknowledgement === "required",
    });
  }
  return baseLane("messages", "messages", policy.scope, policy, "ready", {
    reason: messageVisibility.summary.semanticVisibleCount === 0 && messageVisibility.summary.externalVisibleCount === 0
      ? "message presentation is settled with no visible messages"
      : "message presentation is settled with visible messages",
    target: messageVisibility.current?.target ?? null,
  });
}

function layoutLane(policy, layoutMeasurement, form) {
  const layoutReport = form.layout();
  if (layoutReport.summary.unavailableFields > 0) {
    return baseLane("layout", "layout", policy.scope, policy, "unavailable", {
      reason: "layout presentation is unavailable for one or more declared fields",
      token: stableValueDigest(layoutReport.summary),
      acknowledgementRequired: policy.unavailableAcknowledgement === "required",
    });
  }
  if (layoutMeasurement.latestSnapshot === null) {
    return baseLane("layout", "layout", policy.scope, policy, "pending", {
      reason: "layout presentation is waiting for the first measurement snapshot",
      token: layoutReport.digest,
    });
  }
  const currentSemanticDigest = stableValueDigest({
    validationDigest: stableValueDigest(form.validation()),
    readinessDigest: stableValueDigest(form.readiness().blockers),
    actionPlanDigestSetDigest: form.actions().digests.planDigestSetDigest,
  });
  const snapshotDigest = stableValueDigest(layoutMeasurement.latestSnapshot.semanticDigests);
  if (snapshotDigest !== currentSemanticDigest) {
    return baseLane("layout", "layout", policy.scope, policy, "settling", {
      reason: "layout presentation is waiting for measurement to catch up with current semantic truth",
      token: layoutMeasurement.latestSnapshot.snapshotDigest,
    });
  }
  return baseLane("layout", "layout", policy.scope, policy, "ready", {
    reason: "layout presentation is settled",
    token: layoutMeasurement.latestSnapshot.snapshotDigest,
  });
}

function canonicalizationLane(policy, canonicalizationHistory, settlements, nowMs) {
  const latest = canonicalizationHistory.at(-1) ?? null;
  if (!latest) {
    return baseLane("canonicalization", "canonicalization", policy.scope, policy, "ready", {
      reason: "canonicalization presentation has no pending visible work",
    });
  }
  return acknowledgedOrSettlingLane(
    "canonicalization",
    "canonicalization",
    policy,
    latest.action ?? "canonicalization",
    latest.canonicalizationDigest,
    settlements,
    nowMs,
    latest.observedAtMs,
  );
}

function resourceDriftLane(policy, form, canonicalizationHistory) {
  const latest = canonicalizationHistory.at(-1) ?? null;
  if (!latest || latest.sourceProjection !== "serverCanonicalUntilAuthoritativeSourceDrift") {
    return baseLane("resourceDrift", "resourceDrift", policy.scope, policy, "ready", {
      reason: "resource drift presentation has no authoritative drift to report",
    });
  }
  const sourceDigest = stableValueDigest(form.source());
  if (sourceDigest === latest.canonicalSourceDigest) {
    return baseLane("resourceDrift", "resourceDrift", policy.scope, policy, "ready", {
      target: latest.action ?? "canonicalization",
      reason: "resource drift presentation is settled on the current canonical source projection",
      token: latest.canonicalizationDigest,
    });
  }
  const dirty = form.dirty();
  const status = dirty.isDirty ? "failed" : "busy";
  return baseLane("resourceDrift", "resourceDrift", policy.scope, policy, status, {
    target: latest.action ?? "canonicalization",
    reason: dirty.isDirty
      ? "authoritative source drift diverged from the last canonicalized source while local draft edits remain"
      : "authoritative source drift replaced the last canonicalized source projection",
    token: stableValueDigest({
      canonicalizationDigest: latest.canonicalizationDigest,
      sourceDigest,
      dirtyDigest: stableValueDigest(dirty),
    }),
  });
}
