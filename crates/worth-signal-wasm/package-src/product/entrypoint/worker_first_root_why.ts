/** Resolve worker-first root diagnostics().why(id) across import and authored ids. */
export function resolveWorkerFirstRootWhy({
  id,
  activeImportContext,
  authoredRuntime,
  bridge,
}) {
  if (typeof id !== "string" || id.length === 0) {
    throw new TypeError("worker-first root diagnostics().why(...) requires a non-empty signal id");
  }
  if (activeImportContext?.whyById.has(id)) {
    return activeImportContext.whyById.get(id);
  }
  if (
    authoredRuntime.hasAuthoredSignalId(id)
    || authoredRuntime.hasKnownSignalId(id)
  ) {
    return bridge.why(id);
  }
  if (activeImportContext !== null) {
    throw new TypeError(
      `worker-first root diagnostics().why(${JSON.stringify(id)}) requires an id from the active imported graph`,
    );
  }
  throw new TypeError(
    `worker-first root diagnostics().why(${JSON.stringify(id)}) requires a known authored signal id`,
  );
}
