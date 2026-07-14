import { stableValueDigest } from "./values/value_paths.js";

export function dedupeReadinessBlockers(blockers) {
  const seen = new Set();
  const deduped = [];
  for (const blocker of blockers) {
    const digest = stableValueDigest(blocker);
    if (seen.has(digest)) {
      continue;
    }
    seen.add(digest);
    deduped.push(blocker);
  }
  return Object.freeze(deduped);
}
