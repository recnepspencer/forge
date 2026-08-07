import { FormDeclarationError } from "../form_errors.js";
import { stableValueDigest } from "../values/value_paths.js";
import {
  deriveCollaborationResourceProof,
  noResourceProof,
  pinCollaborationBranchId,
  readCollaborationBranchId,
} from "./collaboration_resource_proof.js";
import { collaborationDeclarationDefaults } from "./declarations.js";
import { deriveCollaborationEvents } from "./events.js";

export function readCollaborationReport(declaration, store, resourceSource = null) {
  if (!declaration) {
    const events = Object.freeze([]);
    return Object.freeze({
      declared: false,
      mode: "notDeclared",
      actorId: null,
      posture: "notDeclared",
      reason: "collaboration is not declared for this form",
      lockOwnerId: null,
      leasedFields: Object.freeze([]),
      branchId: null,
      readOnly: false,
      remoteUpdateDigest: null,
      presence: Object.freeze([]),
      comments: Object.freeze([]),
      resourceProof: noResourceProof(),
      history: store.history(),
      events,
      eventsDigest: stableValueDigest(events),
      counters: counters("notDeclared", noResourceProof(), [], [], [], 0, events),
      digest: stableValueDigest({ declared: false, posture: "notDeclared" }),
    });
  }
  const resourceProof = deriveCollaborationResourceProof(declaration, resourceSource);
  const current = store.current();
  const posture = effectivePosture(current?.posture, declaration.mode, resourceProof);
  const history = store.history();
  const events = deriveCollaborationEvents(history);
  const report = {
    declared: true,
    mode: declaration.mode,
    actorId: declaration.actorId,
    posture,
    reason: effectiveReason(current?.reason, declaration.mode, resourceProof),
    lockOwnerId: current?.lockOwnerId ?? null,
    leasedFields: Object.freeze(current?.leasedFields ?? []),
    branchId: readCollaborationBranchId(resourceProof, current?.branchId ?? null),
    readOnly: current?.readOnly ?? declaration.mode === "reviewerCommentOnly",
    remoteUpdateDigest: current?.remoteUpdateDigest ?? null,
    presence: Object.freeze(current?.presence ?? []),
    comments: Object.freeze(current?.comments ?? []),
    resourceProof,
    history,
    events,
    eventsDigest: stableValueDigest(events),
  };
  return Object.freeze({
    ...report,
    counters: counters(
      posture,
      resourceProof,
      report.leasedFields,
      report.presence,
      report.comments,
      report.history.length,
      report.events,
    ),
    digest: stableValueDigest(report),
  });
}

export function collaborationFieldWriteBlocker(report, fieldId, capability) {
  if (!report.declared || report.posture === "notDeclared" || report.mode === "unavailable") {
    return null;
  }
  if (report.resourceProof.required && !report.resourceProof.admitted) {
    return blocker("collaboration:resourceProofUnavailable", {
      field: fieldId,
      capability,
      reason: report.reason,
    });
  }
  if (report.readOnly) {
    return blocker("collaboration:readOnly", {
      field: fieldId,
      capability,
      reason: report.reason,
    });
  }
  if (report.mode === "singleWriterLock" && report.lockOwnerId && report.lockOwnerId !== report.actorId) {
    return blocker("collaboration:locked", {
      field: fieldId,
      capability,
      collaborator: report.lockOwnerId,
      reason: report.reason,
    });
  }
  if (report.mode === "fieldLease") {
    const lease = report.leasedFields.find((entry) => entry.field === fieldId && entry.ownerId !== report.actorId) ?? null;
    if (lease) {
      return blocker("collaboration:leased", {
        field: fieldId,
        capability,
        collaborator: lease.ownerId,
        reason: report.reason,
      });
    }
  }
  return null;
}

export function collaborationReadinessBlockers(report, patchPlan) {
  if (!report.declared || report.posture === "notDeclared" || report.mode === "unavailable") {
    return Object.freeze([]);
  }
  if (report.resourceProof.required && !report.resourceProof.admitted) {
    return Object.freeze([
      blocker("collaboration:resourceProofUnavailable", {
        reason: report.reason,
      }),
    ]);
  }
  if (report.readOnly) {
    return Object.freeze([
      blocker("collaboration:readOnly", {
        reason: report.reason,
      }),
    ]);
  }
  if (report.mode === "singleWriterLock" && report.lockOwnerId && report.lockOwnerId !== report.actorId) {
    return Object.freeze([
      blocker("collaboration:locked", {
        collaborator: report.lockOwnerId,
        reason: report.reason,
      }),
    ]);
  }
  if (report.mode !== "fieldLease") {
    return Object.freeze([]);
  }
  const leasedFields = patchPlan.operations
    .map((operation) => operation.field)
    .filter((field, index, values) => values.indexOf(field) === index)
    .filter((field) => report.leasedFields.some((lease) => lease.field === field && lease.ownerId !== report.actorId));
  if (leasedFields.length === 0) {
    return Object.freeze([]);
  }
  return Object.freeze([
    blocker("collaboration:leased", {
      fields: Object.freeze(leasedFields),
      reason: report.reason,
    }),
  ]);
}

export function normalizeCollaborationUpdate(declaration, update, resourceSource = null) {
  if (!declaration) {
    throw new FormDeclarationError("collaboration is not declared for this form");
  }
  if (!update || typeof update !== "object" || Array.isArray(update)) {
    throw new FormDeclarationError("collaboration update must be an object", { update });
  }
  const leasedFields = normalizeLeasedFields(update.leasedFields ?? [], declaration.declaredFieldIds);
  const presence = normalizePresence(update.presence ?? [], declaration.supportsPresence === true);
  const comments = normalizeComments(update.comments ?? [], declaration.supportsComments === true);
  const declaredDefaults = collaborationDeclarationDefaults(declaration.mode);
  const posture = normalizePosture(update.posture ?? declaredDefaults.posture);
  const resourceProof = deriveCollaborationResourceProof(declaration, resourceSource);
  return Object.freeze({
    posture,
    reason: String(update.reason ?? declaredDefaults.reason),
    mode: declaration.mode,
    actorId: declaration.actorId,
    lockOwnerId: optionalString(update.lockOwnerId),
    leasedFields,
    branchId: pinCollaborationBranchId(
      resourceProof,
      optionalBranchId(update.branchId),
    ),
    readOnly: update.readOnly === true,
    remoteUpdateDigest: optionalString(update.remoteUpdateDigest),
    presence,
    comments,
  });
}

function counters(posture, resourceProof, leasedFields, presence, comments, historyArtifacts, events) {
  return Object.freeze({
    costBasis: "derivedCollaborationPostureScan",
    incrementalStatus: "notIncremental",
    blockingFields: leasedFields.length,
    presenceActors: presence.length,
    commentArtifacts: comments.length,
    historyArtifacts,
    eventArtifacts: events.length,
    postureChanges: events.filter((event) => event.kind === "postureChange").length,
    lockChanges: events.filter((event) => event.kind === "lockChange").length,
    leaseChanges: events.filter((event) => event.kind === "leaseChange").length,
    branchChanges: events.filter((event) => event.kind === "branchChange").length,
    presenceChanges: events.filter((event) => event.kind === "presenceChange").length,
    commentChanges: events.filter((event) => event.kind === "commentChange").length,
    blocked: posture === "blocked" ? 1 : 0,
    settling: posture === "settling" ? 1 : 0,
    unavailable: posture === "unavailable" ? 1 : 0,
    resourceProofRequired: resourceProof.required ? 1 : 0,
    resourceProofUnavailable: resourceProof.required && !resourceProof.admitted ? 1 : 0,
  });
}

function normalizeLeasedFields(leasedFields, declaredFieldIds) {
  if (!Array.isArray(leasedFields)) {
    throw new FormDeclarationError("collaboration leasedFields must be an array", { leasedFields });
  }
  return Object.freeze(leasedFields.map((entry) => {
    if (!entry || typeof entry !== "object" || Array.isArray(entry)) {
      throw new FormDeclarationError("collaboration lease entry must be an object", { entry });
    }
    const field = String(entry.field ?? "");
    if (!declaredFieldIds.includes(field)) {
      throw new FormDeclarationError("collaboration lease references an undeclared field", { field });
    }
    return Object.freeze({
      field,
      ownerId: requireNonEmptyString(entry.ownerId, "collaboration lease ownerId"),
    });
  }));
}

function normalizePresence(presence, supported) {
  if (supported !== true && Array.isArray(presence) && presence.length > 0) {
    throw new FormDeclarationError("collaboration presence artifacts require supportsPresence: true");
  }
  if (!Array.isArray(presence)) {
    throw new FormDeclarationError("collaboration presence must be an array", { presence });
  }
  return Object.freeze(presence.map((entry) => {
    if (!entry || typeof entry !== "object" || Array.isArray(entry)) {
      throw new FormDeclarationError("collaboration presence entry must be an object", { entry });
    }
    const status = requireNonEmptyString(entry.status, "collaboration presence status");
    if (status !== "active" && status !== "idle" && status !== "viewing") {
      throw new FormDeclarationError("collaboration presence status is not supported", { status });
    }
    return Object.freeze({
      actorId: requireNonEmptyString(entry.actorId, "collaboration presence actorId"),
      status,
    });
  }));
}

function normalizeComments(comments, supported) {
  if (supported !== true && Array.isArray(comments) && comments.length > 0) {
    throw new FormDeclarationError("collaboration comments require supportsComments: true");
  }
  if (!Array.isArray(comments)) {
    throw new FormDeclarationError("collaboration comments must be an array", { comments });
  }
  return Object.freeze(comments.map((entry) => {
    if (!entry || typeof entry !== "object" || Array.isArray(entry)) {
      throw new FormDeclarationError("collaboration comment entry must be an object", { entry });
    }
    return Object.freeze({
      id: requireNonEmptyString(entry.id, "collaboration comment id"),
      authorId: requireNonEmptyString(entry.authorId, "collaboration comment authorId"),
      target: optionalString(entry.target),
    });
  }));
}

function normalizePosture(posture) {
  if (
    posture !== "active" &&
    posture !== "blocked" &&
    posture !== "settling" &&
    posture !== "unavailable"
  ) {
    throw new FormDeclarationError("collaboration posture is not supported", { posture });
  }
  return posture;
}

function blocker(kind, entry) {
  return Object.freeze({
    kind,
    ...(entry.field ? { field: entry.field } : {}),
    ...(entry.fields ? { fields: entry.fields } : {}),
    ...(entry.capability ? { capability: entry.capability } : {}),
    ...(entry.collaborator ? { collaborator: entry.collaborator } : {}),
    reason: entry.reason,
  });
}

function defaultPosture(mode, resourceProof) {
  if (mode === "unavailable") {
    return "unavailable";
  }
  if (resourceProof.required && !resourceProof.admitted) {
    return "unavailable";
  }
  return "active";
}

function defaultReason(mode, resourceProof) {
  return mode === "unavailable"
    ? "collaboration is explicitly unavailable for this form"
    : resourceProof.required && !resourceProof.admitted
      ? resourceProof.reason
      : "collaboration posture is settled";
}

function effectivePosture(currentPosture, mode, resourceProof) {
  if (resourceProof.required && !resourceProof.admitted) {
    return "unavailable";
  }
  return currentPosture ?? defaultPosture(mode, resourceProof);
}

function effectiveReason(currentReason, mode, resourceProof) {
  if (resourceProof.required && !resourceProof.admitted) {
    return resourceProof.reason;
  }
  return currentReason ?? defaultReason(mode, resourceProof);
}

function optionalString(value) {
  return value === undefined || value === null ? null : String(value);
}

function optionalBranchId(value) {
  if (value === undefined || value === null) {
    return null;
  }
  if (typeof value !== "string" && typeof value !== "number") {
    throw new FormDeclarationError("collaboration branchId must be a string or number", { value });
  }
  return value;
}

function requireNonEmptyString(value, label) {
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new FormDeclarationError(`${label} must be a non-empty string`, { value });
  }
  return value;
}

