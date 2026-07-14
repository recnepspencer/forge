import { FormDeclarationError } from "../form_errors.js";

const SUBMIT_INTENT_SOURCES = new Set(["keyboard", "pointer", "programmatic"]);
const INPUT_SOURCES = new Set(["typing", "paste", "drop", "autofill"]);
const FIELD_INTERACTION_KINDS = new Set([
  "touch",
  "visit",
  "focus",
  "blur",
  "input",
  "compositionStart",
  "compositionCommit",
  "compositionCancel",
]);

export function createInteractionBindings(interactions, fieldDeclarations) {
  const declarationsById = new Map(fieldDeclarations.map((field) => [field.id, field]));
  return Object.freeze({
    reportFieldInteraction(fieldId, event) {
      const declaration = requireFieldDeclaration(declarationsById, fieldId);
      return applyFieldInteraction(interactions, declaration, event);
    },
    reportSubmitIntent(options = {}) {
      return interactions.reportSubmitIntent(normalizeSubmitIntentSource(options.source));
    },
    clearSubmitIntent(options = {}) {
      return interactions.clearSubmitIntent(
        options.reason === undefined ? null : String(options.reason),
      );
    },
  });
}

export function applyFieldInteraction(interactions, declaration, event) {
  const kind = normalizeFieldInteractionKind(event.kind);
  switch (kind) {
    case "touch":
      return interactions.touch(declaration.id, String(event.source ?? "imperative"));
    case "visit":
      return interactions.visit(declaration.id, String(event.source ?? "imperative"));
    case "focus":
      requireFocusCapability(declaration);
      return interactions.focus(declaration.id, String(event.source ?? "imperative"));
    case "blur":
      requireFocusCapability(declaration);
      return interactions.blur(declaration.id, String(event.source ?? "imperative"));
    case "input":
      requireRawInputCapability(declaration);
      return interactions.input(
        declaration.id,
        normalizeInteractionInputSource(event.source),
        event.rawValue ?? null,
      );
    case "compositionStart":
      requireCompositionCapability(declaration);
      return interactions.compose(declaration.id, event.rawValue ?? null);
    case "compositionCommit":
      requireCompositionCapability(declaration);
      return interactions.finishComposition(declaration.id, "compositionCommitted");
    case "compositionCancel":
      requireCompositionCapability(declaration);
      return interactions.finishComposition(declaration.id, "compositionCancelled");
    default:
      throw new FormDeclarationError("form interaction kind is not supported", {
        kind,
      });
  }
}

export function normalizeInteractionInputSource(source) {
  const normalized = source ?? "typing";
  if (!INPUT_SOURCES.has(normalized)) {
    throw new FormDeclarationError("form interaction input source is not supported", {
      source: normalized,
    });
  }
  return normalized;
}

function normalizeFieldInteractionKind(kind) {
  if (!FIELD_INTERACTION_KINDS.has(kind)) {
    throw new FormDeclarationError("form interaction kind is not supported", {
      kind,
    });
  }
  return kind;
}

function requireCompositionCapability(declaration) {
  if (declaration.inputAdapter.capabilities.reportsComposition !== false) {
    return;
  }
  throw new FormDeclarationError("form interaction composition is unavailable for this field", {
    field: declaration.id,
  });
}

function requireRawInputCapability(declaration) {
  if (declaration.inputAdapter.capabilities.reportsRawInput !== false) {
    return;
  }
  throw new FormDeclarationError("form interaction raw input is unavailable for this field", {
    field: declaration.id,
  });
}

function requireFocusCapability(declaration) {
  if (declaration.inputAdapter.capabilities.reportsFocus !== false) {
    return;
  }
  throw new FormDeclarationError("form interaction focus is unavailable for this field", {
    field: declaration.id,
  });
}

function normalizeSubmitIntentSource(source) {
  const normalized = source ?? "programmatic";
  if (!SUBMIT_INTENT_SOURCES.has(normalized)) {
    throw new FormDeclarationError("form submit intent source is not supported", {
      source: normalized,
    });
  }
  return normalized;
}

function requireFieldDeclaration(declarationsById, fieldId) {
  const declaration = declarationsById.get(fieldId);
  if (!declaration) {
    throw new FormDeclarationError("form field is not declared", { fieldId });
  }
  return declaration;
}
