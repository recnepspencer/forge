import { FormDeclarationError } from "../form_errors.js";
import { fieldAvailabilityArtifact } from "../availability/artifacts.js";
import { stableValueDigest } from "../values/value_paths.js";

const INVALID_KINDS = new Set(["invalid", "parseFailure"]);

export function readAccessibilityReport(fieldDeclarations, form) {
  const validation = form.validation();
  const availability = form.availability();
  const steps = form.steps();
  const messages = visibleAccessibilityMessages(form.visibleMessages(), fieldDeclarations);
  const fields = fieldDeclarations.map((declaration, index) =>
    fieldAccessibilityArtifact(declaration, index, validation, availability, messages),
  );
  const sections = sectionAccessibilityArtifacts(steps);
  const focusTarget = firstActionableFocusTarget(fields, messages);
  const orderHints = Object.freeze({
    readingOrder: Object.freeze([...fields].sort(compareReadingOrder).map((field) => field.field)),
    focusOrder: Object.freeze(
      fields
        .filter((field) => !field.hidden)
        .slice()
        .sort(compareFocusOrder)
        .map((field) => field.field),
    ),
    sectionOrder: Object.freeze(sections.map((section) => section.id)),
    summaryOrder: Object.freeze(messages.slice().sort(compareSummaryOrder).map((message) => message.id)),
  });
  const declaredOrderHints = explicitOrderHints(fieldDeclarations, sections);
  const summary = accessibilitySummary(fields, messages);
  const counters = accessibilityCounters(fields, messages, sections);
  return Object.freeze({
    fields: Object.freeze(fields),
    messages: Object.freeze(messages),
    sections: Object.freeze(sections),
    focusTarget,
    orderHints,
    orderDigest: declaredOrderHints === null ? null : stableValueDigest(declaredOrderHints),
    summary,
    counters,
    digest: stableValueDigest({
      fields,
      messages,
      sections,
      focusTarget,
      orderHints,
      summary,
      counters,
    }),
  });
}

function explicitOrderHints(fieldDeclarations, sections) {
  const readingOrder = fieldDeclarations
    .filter((declaration) => declaration.accessibility.readingOrder !== null)
    .sort((left, right) => (
      left.accessibility.readingOrder - right.accessibility.readingOrder ||
      left.id.localeCompare(right.id)
    ))
    .map((declaration) => declaration.id);
  const focusOrder = fieldDeclarations
    .filter((declaration) => declaration.accessibility.focusOrder !== null)
    .sort((left, right) => (
      left.accessibility.focusOrder - right.accessibility.focusOrder ||
      left.id.localeCompare(right.id)
    ))
    .map((declaration) => declaration.id);
  const summaryOrder = fieldDeclarations
    .filter((declaration) => declaration.accessibility.summaryOrder !== null)
    .sort((left, right) => (
      left.accessibility.summaryOrder - right.accessibility.summaryOrder ||
      left.id.localeCompare(right.id)
    ))
    .map((declaration) => declaration.id);
  const sectionOrder = sections
    .filter((section) => section.orderDeclared === true)
    .sort((left, right) => left.order - right.order || left.id.localeCompare(right.id))
    .map((section) => section.id);
  if (
    readingOrder.length === 0 &&
    focusOrder.length === 0 &&
    summaryOrder.length === 0 &&
    sectionOrder.length === 0
  ) {
    return null;
  }
  return Object.freeze({
    readingOrder: Object.freeze(readingOrder),
    focusOrder: Object.freeze(focusOrder),
    sectionOrder: Object.freeze(sectionOrder),
    summaryOrder: Object.freeze(summaryOrder),
  });
}

function fieldAccessibilityArtifact(declaration, index, validation, availability, messages) {
  const availabilityArtifact = fieldAvailabilityArtifact(availability, declaration.id);
  const fieldValidationArtifacts = validation.artifacts.filter((artifact) => artifact.field === declaration.id);
  const fieldMessages = messages.filter((message) => message.target === declaration.id);
  const invalid = fieldValidationArtifacts.some((artifact) => INVALID_KINDS.has(artifact.kind));
  const state = availabilityArtifact?.state ?? "enabled";
  const messageIds = Object.freeze(fieldMessages.map((message) => message.id));
  const messageDescriptions = fieldMessages.flatMap((message) => message.describedBy);
  const summaryMessageIds = Object.freeze(
    fieldMessages
      .filter((message) => message.visibility === "summary" || message.visibility === "blocked")
      .map((message) => message.id),
  );
  return Object.freeze({
    kind: "fieldAccessibility",
    field: declaration.id,
    path: declaration.path,
    label: declaration.accessibility.label,
    description: declaration.accessibility.description,
    summaryLabel: declaration.accessibility.summaryLabel,
    describedBy: Object.freeze([
      ...declaration.accessibility.describedBy,
      ...messageDescriptions,
      ...messageIds,
    ]),
    messageIds,
    summaryMessageIds,
    required: state === "required",
    invalid,
    disabled: state === "disabled" || state === "blocked" || state === "unavailable",
    readonly: state === "readonly",
    hidden: state === "hidden" || state === "omitted",
    readingOrder: declaration.accessibility.readingOrder ?? index,
    focusOrder: declaration.accessibility.focusOrder ?? index,
    summaryOrder: declaration.accessibility.summaryOrder ?? index,
    announcementPriority: highestAnnouncementPriority(fieldMessages),
    focusCapability: declaration.inputAdapter.capabilities.reportsFocus
      ? Object.freeze({ posture: "supported", reason: null })
      : Object.freeze({
          posture: "unavailable",
          reason: `${declaration.id} adapter does not report focus`,
        }),
  });
}

function visibleAccessibilityMessages(messages, fieldDeclarations) {
  const declaredFieldIds = new Set(fieldDeclarations.map((declaration) => declaration.id));
  return messages
    .filter((message) => message.visibility !== "hidden")
    .map((message, index) => accessibilityMessageArtifact(message, index, declaredFieldIds));
}

function accessibilityMessageArtifact(message, index, declaredFieldIds) {
  const focusTarget = normalizeMessageFocusTarget(message, declaredFieldIds);
  const target = typeof message.target === "string" && message.target.length > 0 ? message.target : null;
  return Object.freeze({
    id: `message:${index}:${message.code}:${target ?? "form"}`,
    code: message.code,
    target,
    visibility: message.visibility,
    severity: message.severity,
    announce: normalizeAnnouncementPriority(message),
    describedBy: Object.freeze(message.accessibility?.describedBy ?? []),
    summaryOrder: index,
    focusTarget,
  });
}

function normalizeMessageFocusTarget(message, declaredFieldIds) {
  const target = message.accessibility?.focusTarget;
  if (target === undefined || target === null || target === "") {
    return typeof message.target === "string" && declaredFieldIds.has(message.target)
      ? message.target
      : null;
  }
  if (!declaredFieldIds.has(target)) {
    throw new FormDeclarationError("validation message focusTarget must reference a declared field", {
      focusTarget: target,
    });
  }
  return target;
}

function sectionAccessibilityArtifacts(steps) {
  return steps.artifacts
    .slice()
    .sort((left, right) => left.order - right.order)
    .map((artifact) => Object.freeze({
      id: artifact.id,
      group: artifact.group,
      order: artifact.order,
      orderDeclared: artifact.orderDeclared === true,
      posture: artifact.posture,
      fields: artifact.fields,
    }));
}

function firstActionableFocusTarget(fields, messages) {
  const explicitMessage = messages.find((message) => message.focusTarget !== null);
  if (explicitMessage) {
    return focusTargetForField(fields, explicitMessage.focusTarget, explicitMessage.code);
  }
  const invalidField = fields
    .filter((field) => field.invalid && !field.hidden)
    .slice()
    .sort(compareFocusOrder)[0];
  if (invalidField) {
    return focusTargetForField(fields, invalidField.field, "validation");
  }
  return Object.freeze({
    posture: "none",
    field: null,
    target: null,
    reason: "no actionable accessibility focus target is pending",
  });
}

function focusTargetForField(fields, fieldId, reasonPrefix) {
  const field = fields.find((entry) => entry.field === fieldId);
  if (!field) {
    return Object.freeze({
      posture: "none",
      field: null,
      target: null,
      reason: "no actionable accessibility focus target is pending",
    });
  }
  if (field.focusCapability.posture === "unavailable") {
    return Object.freeze({
      posture: "unavailable",
      field: field.field,
      target: null,
      reason: `${reasonPrefix} focus target is unavailable because ${field.focusCapability.reason}`,
    });
  }
  return Object.freeze({
    posture: "ready",
    field: field.field,
    target: field.field,
    reason: `${reasonPrefix} should focus ${field.field}`,
  });
}

function accessibilitySummary(fields, messages) {
  return Object.freeze({
    fields: fields.length,
    invalidFields: fields.filter((field) => field.invalid).length,
    requiredFields: fields.filter((field) => field.required).length,
    readonlyFields: fields.filter((field) => field.readonly).length,
    disabledFields: fields.filter((field) => field.disabled).length,
    hiddenFields: fields.filter((field) => field.hidden).length,
    messages: messages.length,
    summaryMessages: messages.filter((message) => message.visibility !== "visible").length,
  });
}

function accessibilityCounters(fields, messages, sections) {
  return Object.freeze({
    costBasis: "derivedAccessibilityArtifactScan",
    incrementalStatus: "notIncremental",
    declaredFields: fields.length,
    describedRelationships: fields.reduce((total, field) => total + field.describedBy.length, 0),
    invalidFields: fields.filter((field) => field.invalid).length,
    requiredFields: fields.filter((field) => field.required).length,
    readonlyFields: fields.filter((field) => field.readonly).length,
    disabledFields: fields.filter((field) => field.disabled).length,
    hiddenFields: fields.filter((field) => field.hidden).length,
    summaryMessages: messages.filter((message) => message.visibility !== "visible").length,
    sections: sections.length,
    focusUnavailableFields: fields.filter((field) => field.focusCapability.posture === "unavailable").length,
  });
}

function highestAnnouncementPriority(messages) {
  if (messages.some((message) => normalizeAnnouncementPriority(message) === "assertive")) {
    return "assertive";
  }
  if (messages.some((message) => normalizeAnnouncementPriority(message) === "polite")) {
    return "polite";
  }
  return "off";
}

function normalizeAnnouncementPriority(message) {
  if (message.accessibility?.announce === "assertive" || message.severity === "error") {
    return "assertive";
  }
  if (message.accessibility?.announce === "polite" || message.severity === "warning") {
    return "polite";
  }
  return "off";
}

function compareReadingOrder(left, right) {
  return left.readingOrder - right.readingOrder || left.focusOrder - right.focusOrder;
}

function compareFocusOrder(left, right) {
  return left.focusOrder - right.focusOrder || left.readingOrder - right.readingOrder;
}

function compareSummaryOrder(left, right) {
  return left.summaryOrder - right.summaryOrder || left.id.localeCompare(right.id);
}
