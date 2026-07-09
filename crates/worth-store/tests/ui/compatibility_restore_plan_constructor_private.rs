use worth_store::{
    ArtifactFamilyId, ArtifactSemanticVersion, CompatibilityManifestDigest, CompatibilityRelation,
    RestoreCompatibilityPlan, RestorePublicationWitness,
};

fn main() {
    let _ = RestoreCompatibilityPlan::new(
        ArtifactFamilyId::new("commit_envelope"),
        digest(),
        ArtifactSemanticVersion::new(1),
        CompatibilityRelation::Native,
        0,
        witness(),
    );
}

fn digest() -> CompatibilityManifestDigest {
    panic!("compile-fail fixture")
}

fn witness() -> RestorePublicationWitness {
    panic!("compile-fail fixture")
}
