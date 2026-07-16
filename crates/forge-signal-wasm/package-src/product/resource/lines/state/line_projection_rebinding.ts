function rebindLineProjectionDiagnostics(diagnostics, projection) {
  const previous = diagnostics.visibleSelection;
  const visibleSelection = projection.kind === "derivedEffectProjectionBranch"
    ? Object.freeze({
        kind: "derivedEffectProjectionBranch",
        source: previous.kind === "derivedEffectProjectionBranch"
          ? previous.source
          : "projectionRebuild",
        effectId: previous.effectId ?? null,
        branchId: Number(projection.branch.id),
        snapshotId: projection.basis.snapshotId,
        basisId: previous.basisId ?? null,
        affectedEffectIds: projection.affectedEffectIds,
        projectionDigest: projection.projectionDigest,
        detail: projection.detail,
      })
    : Object.freeze({
        kind: "committed",
        source: "projectionRebuild",
        effectId: previous.effectId ?? null,
        branchId: projection.basis.branchId,
        snapshotId: projection.basis.snapshotId,
        basisId: previous.basisId ?? null,
        detail: "resource line visible truth is canonical after projection rebuild",
      });
  return Object.freeze({ ...diagnostics, visibleSelection });
}

export { rebindLineProjectionDiagnostics };
