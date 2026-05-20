import { readFormStateSnapshot } from "../recovery/form_state_snapshot.js";
import { stableValueDigest } from "../values/value_paths.js";

export function createStateHistoryControllerBindings(options) {
  return Object.freeze({
    recordRawInput(fieldId, operation, rawValue, source = null, reason = null) {
      options.stateHistory.recordRawInput(readFormStateSnapshot(options.formRef()), {
        field: fieldId,
        operation,
        source,
        reason,
        rawValueDigest: rawValue === null ? null : stableValueDigest(rawValue),
        previousDraftDigest: stableValueDigest(options.formRef().draft()),
      });
    },
    recordDraftWrite(fieldId, operation, previousDraft, parsedValue = null, reason = null) {
      options.stateHistory.recordDraftWrite(readFormStateSnapshot(options.formRef()), {
        field: fieldId,
        operation,
        reason,
        previousDraftDigest: stableValueDigest(previousDraft),
        parsedValueDigest: parsedValue === null ? null : stableValueDigest(parsedValue),
      });
    },
  });
}
