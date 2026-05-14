import { FormDeclarationError } from "../form_errors.js";

const COLLABORATION_MODES = new Set([
  "singleWriterLock",
  "fieldLease",
  "branchPerActor",
  "optimisticMerge",
  "reviewerCommentOnly",
  "unavailable",
]);

export function materializeCollaborationDeclaration(declaration, fieldDeclarations) {
  const declared = declaration.collaboration;
  if (declared === undefined) {
    return null;
  }
  if (declared == null || typeof declared !== "object" || Array.isArray(declared)) {
    throw new FormDeclarationError("form collaboration metadata must be an object", {
      collaboration: declared,
    });
  }
  const mode = normalizeMode(declared.mode);
  const actorId = mode === "unavailable" ? null : nonEmptyString(declared.actorId, "form collaboration actorId");
  return Object.freeze({
    mode,
    actorId,
    supportsPresence: declared.supportsPresence === true,
    supportsComments: declared.supportsComments === true,
    declaredFieldIds: Object.freeze(fieldDeclarations.map((field) => field.id)),
  });
}

function normalizeMode(mode) {
  if (typeof mode !== "string" || !COLLABORATION_MODES.has(mode)) {
    throw new FormDeclarationError("form collaboration mode is not supported", { mode });
  }
  return mode;
}

function nonEmptyString(value, label) {
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new FormDeclarationError(`${label} must be a non-empty string`, { value });
  }
  return value;
}
