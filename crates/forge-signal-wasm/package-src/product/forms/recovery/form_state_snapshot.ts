import { stableValueDigest } from "../values/value_paths.js";

export function readFormStateSnapshot(form) {
  return Object.freeze({
    sourceDigest: stableValueDigest(form.source()),
    draftDigest: stableValueDigest(form.draft()),
    effectiveDigest: stableValueDigest(form.effective()),
  });
}
