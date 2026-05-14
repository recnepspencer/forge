import { FormDeclarationError } from "../form_errors.js";

const PRESENTATION_SCOPES = new Set([
  "field",
  "section",
  "action",
  "control",
  "wholeForm",
  "step",
  "modal",
  "route",
  "externalHandoff",
]);
const ACKNOWLEDGEMENT_POLICIES = new Set(["none", "required"]);
const SUPERSESSION_POLICIES = new Set(["replace", "handoff"]);
const ACTION_SETTLEMENT_DEPENDENCIES = new Set([
  "canonicalization",
  "messages",
  "focusTarget",
  "layout",
  "navigation",
  "handoff",
]);
const DEFAULT_ENTRY_BOOTSTRAP = Object.freeze({
  sourceAdmission: false,
  draftRestore: false,
  sourceCompatibility: false,
  validation: false,
  readiness: false,
  hostFacts: false,
  inputCapabilities: false,
  focusTarget: false,
  layoutMeasurement: false,
});

export function materializePresentationDeclaration(declaration) {
  const declared = declaration.presentation ?? {};
  if (declared == null || typeof declared !== "object" || Array.isArray(declared)) {
    throw new FormDeclarationError("form presentation metadata must be an object", {
      presentation: declared,
    });
  }
  return Object.freeze({
    entry: normalizeLanePolicy(declared.entry, "wholeForm", 0, 0, "none", 0, "replace", "none", "entry", true),
    interaction: normalizeLanePolicy(declared.interaction, "field", 0, 0, "none", 0, "replace", "none", "interaction"),
    availability: normalizeLanePolicy(declared.availability, "wholeForm", 0, 0, "none", 0, "replace", "none", "availability"),
    messages: normalizeLanePolicy(declared.messages, "field", 0, 0, "none", 0, "replace", "none", "messages"),
    layout: normalizeLanePolicy(declared.layout, "section", 0, 0, "none", 0, "replace", "required", "layout"),
    action: normalizeLanePolicy(declared.action, "control", 150, 300, "required", 5000, "handoff", "none", "action", false, true),
    canonicalization: normalizeLanePolicy(declared.canonicalization, "wholeForm", 0, 0, "required", 5000, "handoff", "none", "canonicalization"),
    resourceDrift: normalizeLanePolicy(declared.resourceDrift, "wholeForm", 0, 0, "none", 0, "replace", "none", "resourceDrift"),
    collaboration: normalizeLanePolicy(declared.collaboration, "wholeForm", 0, 0, "none", 0, "replace", "none", "collaboration"),
    attachments: normalizeLanePolicy(declared.attachments, "section", 0, 0, "none", 0, "replace", "none", "attachments"),
    media: normalizeLanePolicy(declared.media, "modal", 0, 0, "none", 0, "replace", "none", "media"),
    handoff: normalizeLanePolicy(declared.handoff, "externalHandoff", 0, 0, "required", 5000, "handoff", "none", "handoff"),
    navigation: normalizeLanePolicy(declared.navigation, "route", 0, 0, "required", 5000, "handoff", "required", "navigation"),
    exit: normalizeLanePolicy(declared.exit, "wholeForm", 0, 0, "none", 0, "replace", "none", "exit"),
  });
}

function normalizeLanePolicy(
  declared,
  scope,
  delayedBusyRevealMs,
  minimumBusyMs,
  settlementAcknowledgement,
  settlementTimeoutMs,
  supersessionHandoff,
  unavailableAcknowledgement,
  lane,
  allowBootstrap = false,
  allowActionDependencies = false,
) {
  if (declared === undefined) {
    return Object.freeze({
      scope,
      delayedBusyRevealMs,
      minimumBusyMs,
      settlementAcknowledgement,
      settlementTimeoutMs,
      supersessionHandoff,
      unavailableAcknowledgement,
      bootstrap: allowBootstrap ? DEFAULT_ENTRY_BOOTSTRAP : null,
    });
  }
  if (declared == null || typeof declared !== "object" || Array.isArray(declared)) {
    throw new FormDeclarationError("form presentation lane metadata must be an object", {
      lane,
      policy: declared,
    });
  }
  if (!allowBootstrap && declared.bootstrap !== undefined) {
    throw new FormDeclarationError("form presentation bootstrap policy is only supported for entry", {
      lane,
    });
  }
  if (!allowActionDependencies && declared.settleOn !== undefined) {
    throw new FormDeclarationError("form presentation settlement dependencies are only supported for action", {
      lane,
    });
  }
  return Object.freeze({
    scope: normalizeEnum(declared.scope ?? scope, PRESENTATION_SCOPES, `form presentation ${lane} scope`),
    delayedBusyRevealMs: nonNegativeInteger(
      declared.delayedBusyRevealMs ?? delayedBusyRevealMs,
      `form presentation ${lane} delayedBusyRevealMs`,
    ),
    minimumBusyMs: nonNegativeInteger(
      declared.minimumBusyMs ?? minimumBusyMs,
      `form presentation ${lane} minimumBusyMs`,
    ),
    settlementAcknowledgement: normalizeEnum(
      declared.settlementAcknowledgement ?? settlementAcknowledgement,
      ACKNOWLEDGEMENT_POLICIES,
      `form presentation ${lane} settlementAcknowledgement`,
    ),
    settlementTimeoutMs: nonNegativeInteger(
      declared.settlementTimeoutMs ?? settlementTimeoutMs,
      `form presentation ${lane} settlementTimeoutMs`,
    ),
    supersessionHandoff: normalizeEnum(
      declared.supersessionHandoff ?? supersessionHandoff,
      SUPERSESSION_POLICIES,
      `form presentation ${lane} supersessionHandoff`,
    ),
    unavailableAcknowledgement: normalizeEnum(
      declared.unavailableAcknowledgement ?? unavailableAcknowledgement,
      ACKNOWLEDGEMENT_POLICIES,
      `form presentation ${lane} unavailableAcknowledgement`,
    ),
    settleOn: allowActionDependencies ? normalizeActionSettlementDependencies(declared.settleOn) : null,
    bootstrap: allowBootstrap ? normalizeBootstrapPolicy(declared.bootstrap) : null,
  });
}

function normalizeActionSettlementDependencies(declared) {
  if (declared === undefined) {
    return Object.freeze([]);
  }
  if (!Array.isArray(declared)) {
    throw new FormDeclarationError("form presentation action settleOn must be an array", {
      settleOn: declared,
    });
  }
  return Object.freeze([...new Set(declared.map((dependency) => normalizeEnum(
    dependency,
    ACTION_SETTLEMENT_DEPENDENCIES,
    "form presentation action settleOn dependency",
  )))]);
}

function normalizeBootstrapPolicy(declared) {
  if (declared === undefined) {
    return DEFAULT_ENTRY_BOOTSTRAP;
  }
  if (declared == null || typeof declared !== "object" || Array.isArray(declared)) {
    throw new FormDeclarationError("form presentation entry bootstrap metadata must be an object", {
      bootstrap: declared,
    });
  }
  return Object.freeze({
    sourceAdmission: normalizeBoolean(declared.sourceAdmission, false, "sourceAdmission"),
    draftRestore: normalizeBoolean(declared.draftRestore, false, "draftRestore"),
    sourceCompatibility: normalizeBoolean(declared.sourceCompatibility, false, "sourceCompatibility"),
    validation: normalizeBoolean(declared.validation, false, "validation"),
    readiness: normalizeBoolean(declared.readiness, false, "readiness"),
    hostFacts: normalizeBoolean(declared.hostFacts, false, "hostFacts"),
    inputCapabilities: normalizeBoolean(declared.inputCapabilities, false, "inputCapabilities"),
    focusTarget: normalizeBoolean(declared.focusTarget, false, "focusTarget"),
    layoutMeasurement: normalizeBoolean(declared.layoutMeasurement, false, "layoutMeasurement"),
  });
}

function normalizeEnum(value, allowed, label) {
  if (typeof value !== "string" || !allowed.has(value)) {
    throw new FormDeclarationError(`${label} is not supported`, { value });
  }
  return value;
}

function nonNegativeInteger(value, label) {
  if (!Number.isInteger(value) || value < 0) {
    throw new FormDeclarationError(`${label} must be a non-negative integer`, { value });
  }
  return value;
}

function normalizeBoolean(value, fallback, label) {
  if (value === undefined) {
    return fallback;
  }
  if (typeof value !== "boolean") {
    throw new FormDeclarationError(`form presentation entry bootstrap ${label} must be a boolean`, {
      value,
    });
  }
  return value;
}
