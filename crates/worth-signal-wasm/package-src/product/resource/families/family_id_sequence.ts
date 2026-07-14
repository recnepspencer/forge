const FAMILY_COUNTERS = new WeakMap();

function nextResourceFamilyId(rawSignals, kind) {
  const next = (FAMILY_COUNTERS.get(rawSignals) ?? 0) + 1;
  FAMILY_COUNTERS.set(rawSignals, next);
  return `__resourceFamily.${kind}.${next}`;
}

export { nextResourceFamilyId };
