import { parseFailureArtifact } from "../validation/artifacts.js";
import {
  cloneFormValue,
  deletePath,
  readPath,
  writePath,
} from "../values/value_paths.js";
import { compareSemanticValues } from "../values/semantic_equality.js";

export function createFieldHandle(field, form, state) {
  const handle = {
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
      return Object.freeze({
        locus: handle.locus(),
        dirty: handle.dirty(),
        pendingRawInput: state.rawInputs.has(field.id),
        parseFailure: state.parseFailures.get(field.id) ?? null,
        writePosture: form.fieldWritePosture(field.id),
        inputAdapter: field.inputAdapter,
      });
    },
  };
  return Object.freeze(handle);
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
