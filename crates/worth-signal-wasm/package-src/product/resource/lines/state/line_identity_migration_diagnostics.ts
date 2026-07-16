function createIdentityMigratedDiagnostics(previous, identityMigration) {
  const diagnostics = {
    ...previous,
    identityMigrationCount:
      ("identityMigrationCount" in previous
        ? previous.identityMigrationCount
        : 0) + 1,
    lastIdentityMigration: identityMigration,
  };
  Object.defineProperty(diagnostics, "visibleSelection", {
    value: previous.visibleSelection,
    enumerable: true,
  });
  return Object.freeze(diagnostics);
}

export { createIdentityMigratedDiagnostics };
