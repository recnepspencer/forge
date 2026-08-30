use worth_store_physical_integrity::{IntegrityValidatedRootManifest, PhysicalArtifactScope};

fn retarget(validated: IntegrityValidatedRootManifest<'_>, other_scope: PhysicalArtifactScope) {
    let _ = validated.with_scope(other_scope);
}

fn main() {}
