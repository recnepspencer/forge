use worth_store_physical_integrity::{
    IntegrityValidatedPhysicalWorkObligation, IntegrityValidatedRootManifest,
    PhysicalArtifactScope,
};

fn retarget(validated: IntegrityValidatedRootManifest<'_>, other_scope: PhysicalArtifactScope) {
    let _ = validated.with_scope(other_scope);
}

fn retarget_physical_work(
    validated: IntegrityValidatedPhysicalWorkObligation<'_>,
    other_scope: PhysicalArtifactScope,
) {
    let _ = validated.with_scope(other_scope);
}

fn invent_prevalidation_lsn(scope: PhysicalArtifactScope) {
    let _ = scope.with_wal_lsn_range(3, 4);
}

fn main() {}
