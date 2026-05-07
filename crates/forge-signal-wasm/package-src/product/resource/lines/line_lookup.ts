function lookupOrCreateLine(linesByCanonicalKey, canonicalKey, createLine) {
  const existing = linesByCanonicalKey.get(canonicalKey);
  if (existing) {
    return existing.handle;
  }
  const created = createLine();
  linesByCanonicalKey.set(canonicalKey, created);
  return created.handle;
}

export { lookupOrCreateLine };
