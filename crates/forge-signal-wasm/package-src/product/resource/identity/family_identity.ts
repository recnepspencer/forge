function createFamilyIdentity(kind, familyId) {
  return Object.freeze({
    kind,
    familyId,
  });
}

export { createFamilyIdentity };
