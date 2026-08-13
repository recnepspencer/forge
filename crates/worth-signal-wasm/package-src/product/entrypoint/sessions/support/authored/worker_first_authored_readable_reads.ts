/**
 * Authored readable `spec.reads` admission for worker-first.
 *
 * Host dependency tracking uses signal ids only. Aspect / scope filters travel
 * intact in the published recipe so the worker DAG owns semantic fan-out.
 */

export function assertAuthoredReadableReads(family, reads, hasKnownSignalId) {
  if (reads === undefined) {
    return;
  }
  if (!Array.isArray(reads)) {
    throw new TypeError(
      `worker-first ${family}(...) requires spec.reads as an array when provided`,
    );
  }
  for (const entry of reads) {
    const readId = resolveAuthoredReadableReadId(entry, family);
    if (!hasKnownSignalId(readId)) {
      throw new TypeError(
        `worker-first ${family}(...) can read only currently available worker-first signals; \`${readId}\` is not currently available`,
      );
    }
  }
}

export function authoredReadableDependencyIds(reads) {
  if (!Array.isArray(reads)) {
    return [];
  }
  return reads.map((entry) => resolveAuthoredReadableReadId(entry, "recipe"));
}

export function resolveAuthoredReadableReadId(entry, family) {
  if (typeof entry === "string" && entry.length > 0) {
    return entry;
  }
  if (
    entry &&
    typeof entry === "object" &&
    !Array.isArray(entry) &&
    typeof entry.id === "string" &&
    entry.id.length > 0
  ) {
    return entry.id;
  }
  throw new TypeError(
    `worker-first ${family}(...) requires every spec.reads entry to be a non-empty signal id or read descriptor`,
  );
}
