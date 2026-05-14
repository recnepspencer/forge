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

export function materializePresentationDeclaration(declaration) {
  const declared = declaration.presentation ?? {};
  if (declared == null || typeof declared !== "object" || Array.isArray(declared)) {
    throw new FormDeclarationError("form presentation metadata must be an object", {
      presentation: declared,
    });
  }
  return Object.freeze({
    entry: normalizeLanePolicy(declared.entry, "wholeForm", 0, 0, "none", 0, "replace", "none", "entry"),
    interaction: normalizeLanePolicy(declared.interaction, "field", 0, 0, "none", 0, "replace", "none", "interaction"),
    availability: normalizeLanePolicy(declared.availability, "wholeForm", 0, 0, "none", 0, "replace", "none", "availability"),
    messages: normalizeLanePolicy(declared.messages, "field", 0, 0, "none", 0, "replace", "none", "messages"),
    layout: normalizeLanePolicy(declared.layout, "section", 0, 0, "none", 0, "replace", "required", "layout"),
    action: normalizeLanePolicy(declared.action, "control", 150, 300, "required", 5000, "handoff", "none", "action"),
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
    });
  }
  if (declared == null || typeof declared !== "object" || Array.isArray(declared)) {
    throw new FormDeclarationError("form presentation lane metadata must be an object", {
      lane,
      policy: declared,
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
