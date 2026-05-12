import { parseFailureArtifact } from "../validation/artifacts.js";
import { FormDeclarationError } from "../form_errors.js";
import {
  cloneFormValue,
  deletePath,
  readPath,
  stableValueDigest,
  writePath,
} from "../values/value_paths.js";
import { compareSemanticValues } from "../values/semantic_equality.js";

export function createFieldHandle(field, form, state) {
  let handle;
  const baseHandle = {
    id: field.id,
    path: field.path,
    locus() {
      return Object.freeze({
        field: field.id,
        path: field.path,
        segments: Object.freeze([...field.segments]),
      });
    },
    sourceValue() {
      return cloneFormValue(readPath(form.source(), field.segments));
    },
    draftValue() {
      return cloneFormValue(readPath(form.draft(), field.segments));
    },
    effectiveValue() {
      return cloneFormValue(readPath(form.effective(), field.segments));
    },
    value() {
      return handle.effectiveValue();
    },
    set(value) {
      denyFieldWriteIfBlocked(field, form, "edit");
      state.rawInputs.delete(field.id);
      state.parseFailures.delete(field.id);
      state.writeDraft(writePath(form.draft(), field.segments, value));
      return handle;
    },
    clearDraft() {
      denyFieldWriteIfBlocked(field, form, "patch");
      state.rawInputs.delete(field.id);
      state.parseFailures.delete(field.id);
      state.writeDraft(deletePath(form.draft(), field.segments));
      return handle;
    },
    input(rawValue, options = {}) {
      denyFieldWriteIfBlocked(field, form, "edit");
      state.parseFailures.delete(field.id);
      state.rawInputs.set(field.id, {
        field: field.id,
        rawValue: cloneFormValue(rawValue),
        committed: false,
      });
      if (options.commit === true) {
        return handle.commitInput();
      }
      return handle;
    },
    commitInput(parser) {
      denyFieldWriteIfBlocked(field, form, "patch");
      const pending = state.rawInputs.get(field.id);
      if (!pending) {
        return handle;
      }
      const parse = parser ?? field.parse;
      let parsedValue;
      try {
        parsedValue = parse ? parse(pending.rawValue) : pending.rawValue;
      } catch (error) {
        state.parseFailures.set(field.id, parseFailureArtifact(field, error, pending.rawValue));
        state.rawInputs.delete(field.id);
        return handle;
      }
      state.parseFailures.delete(field.id);
      state.rawInputs.delete(field.id);
      state.writeDraft(writePath(form.draft(), field.segments, parsedValue));
      return handle;
    },
    dirty() {
      const sourceValue = readPath(form.source(), field.segments);
      const effectiveValue = readPath(form.effective(), field.segments);
      const comparison = compareSemanticValues(sourceValue, effectiveValue);
      return Object.freeze({
        field: field.id,
        path: field.path,
        isDirty: !comparison.equal,
        semanticDirty: !comparison.equal,
        equality: comparison.counters,
      });
    },
    diagnostics() {
      return fieldDiagnostics(field, form, state, handle);
    },
  };
  if (field.family === "repeated") {
    handle = Object.freeze({
      ...baseHandle,
    addItem(item) {
      denyFieldWriteIfBlocked(field, form, "edit");
      const current = requireArrayValue(handle.effectiveValue(), field, "addItem");
      const nextItemId = collectionItemId(field, item);
      if (current.some((entry) => collectionItemId(field, entry) === nextItemId)) {
        throw new FormDeclarationError("repeated field item identity must be unique", {
          field: field.id,
          itemId: nextItemId,
        });
      }
      state.writeDraft(writePath(form.draft(), field.segments, [...current, cloneFormValue(item)]));
      return handle;
    },
    removeItem(itemId) {
      denyFieldWriteIfBlocked(field, form, "edit");
      const current = requireArrayValue(handle.effectiveValue(), field, "removeItem");
      const removedItemId = normalizeIdentityValue(itemId, "repeated field remove identity", {
        field: field.id,
      });
      if (!current.some((entry) => collectionItemId(field, entry) === removedItemId)) {
        throw new FormDeclarationError("repeated field remove target was not found", {
          field: field.id,
          itemId: removedItemId,
        });
      }
      state.writeDraft(writePath(
        form.draft(),
        field.segments,
        current.filter((entry) => collectionItemId(field, entry) !== removedItemId),
      ));
      return handle;
    },
    replaceItem(itemId, nextItem) {
      denyFieldWriteIfBlocked(field, form, "edit");
      const current = requireArrayValue(handle.effectiveValue(), field, "replaceItem");
      const expectedItemId = normalizeIdentityValue(itemId, "repeated field replacement identity", {
        field: field.id,
      });
      const nextItemId = collectionItemId(field, nextItem);
      if (nextItemId !== expectedItemId) {
        throw new FormDeclarationError("repeated field replacement must preserve item identity", {
          field: field.id,
          expectedItemId,
          nextItemId,
        });
      }
      if (!current.some((entry) => collectionItemId(field, entry) === expectedItemId)) {
        throw new FormDeclarationError("repeated field replacement target was not found", {
          field: field.id,
          itemId: expectedItemId,
        });
      }
      state.writeDraft(writePath(
        form.draft(),
        field.segments,
        current.map((entry) => (
          collectionItemId(field, entry) === expectedItemId ? cloneFormValue(nextItem) : entry
        )),
      ));
      return handle;
    },
    moveItem(itemId, beforeItemId = null) {
      denyFieldWriteIfBlocked(field, form, "edit");
      const current = requireArrayValue(handle.effectiveValue(), field, "moveItem");
      const movingItemId = normalizeIdentityValue(itemId, "repeated field move identity", {
        field: field.id,
      });
      const movingIndex = current.findIndex((entry) => collectionItemId(field, entry) === movingItemId);
      if (movingIndex < 0) {
        throw new FormDeclarationError("repeated field move target was not found", {
          field: field.id,
          itemId: movingItemId,
        });
      }
      const moving = current[movingIndex];
      const withoutMoving = current.filter((_, index) => index !== movingIndex);
      const insertionIndex = beforeItemId === null
        ? withoutMoving.length
        : withoutMoving.findIndex((entry) => collectionItemId(field, entry) === normalizeIdentityValue(
            beforeItemId,
            "repeated field move insertion identity",
            { field: field.id },
          ));
      if (insertionIndex < 0) {
        throw new FormDeclarationError("repeated field move insertion target was not found", {
          field: field.id,
          beforeItemId,
        });
      }
      const next = withoutMoving.slice();
      next.splice(insertionIndex, 0, moving);
      state.writeDraft(writePath(form.draft(), field.segments, next));
      return handle;
    },
    collectionIdentity() {
      const value = requireArrayValue(handle.effectiveValue(), field, "collectionIdentity");
      return Object.freeze({
        field: field.id,
        posture: field.collectionIdentity.posture,
        items: Object.freeze(value.map((entry) => Object.freeze({
          itemId: collectionItemId(field, entry),
          digest: stableValueDigest(entry),
        }))),
      });
    },
    });
    return handle;
  }
  if (field.family === "attachment") {
    handle = Object.freeze({
      ...baseHandle,
    attachmentIdentity(value = handle.effectiveValue()) {
      const attachmentDigest = attachmentId(field, value);
      return Object.freeze({
        field: field.id,
        attachmentDigest,
        metadata: field.attachment.metadata,
        posture: field.attachment.posture,
        valueDigest: stableValueDigest(value),
      });
    },
    });
    return handle;
  }
  handle = Object.freeze(baseHandle);
  return handle;
}

function fieldDiagnostics(field, form, state, handle) {
  return Object.freeze({
    locus: handle.locus(),
    dirty: handle.dirty(),
    pendingRawInput: state.rawInputs.has(field.id),
    parseFailure: state.parseFailures.get(field.id) ?? null,
    writePosture: form.fieldWritePosture(field.id),
    inputAdapter: field.inputAdapter,
    ...(field.family === "repeated" ? { collectionIdentity: handle.collectionIdentity() } : {}),
    ...(field.family === "attachment" ? { attachment: handle.attachmentIdentity() } : {}),
  });
}

function requireArrayValue(value, field, operation) {
  if (!Array.isArray(value)) {
    throw new FormDeclarationError(`repeated field ${operation} requires an array value`, {
      field: field.id,
    });
  }
  return value;
}

function collectionItemId(field, item) {
  if (field.collectionIdentity.kind === "resolver") {
    return normalizeIdentityValue(
      field.collectionIdentity.resolver(item),
      "repeated field resolver returned an invalid item identity",
      { field: field.id },
    );
  }
  const identity = readPath(item, [field.collectionIdentity.field]);
  if (!isUsableIdentity(identity)) {
    throw new FormDeclarationError("repeated field item is missing declared item identity", {
      field: field.id,
      itemIdentity: field.collectionIdentity.field,
    });
  }
  return String(identity);
}

function attachmentId(field, attachment) {
  if (field.attachment.identityKind === "resolver") {
    return normalizeIdentityValue(
      field.attachment.identityResolver(attachment),
      "attachment field resolver returned an invalid attachment identity",
      { field: field.id },
    );
  }
  const identity = readPath(attachment, [field.attachment.identityField]);
  if (!isUsableIdentity(identity)) {
    throw new FormDeclarationError("attachment field value is missing declared attachment identity", {
      field: field.id,
      attachmentIdentity: field.attachment.identityField,
    });
  }
  return String(identity);
}

function normalizeIdentityValue(identity, message, details) {
  if (!isUsableIdentity(identity)) {
    throw new FormDeclarationError(message, details);
  }
  return String(identity);
}

function isUsableIdentity(identity) {
  return identity !== undefined && identity !== null && String(identity).length > 0;
}

function denyFieldWriteIfBlocked(field, form, capability) {
  const posture = form.fieldWritePosture(field.id, capability);
  if (posture.canWrite) {
    return;
  }
  const error = new TypeError(posture.reason);
  error.name = "FormFieldWriteDenied";
  error.details = posture;
  throw error;
}
