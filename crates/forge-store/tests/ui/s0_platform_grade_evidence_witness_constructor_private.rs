use forge_store::{
    BackendCapabilityDeclaration, PlatformGradeEvidenceWitness, Roadmap2SequenceId,
    StoreBackendCapabilityTier,
};

fn main() {
    let declaration = BackendCapabilityDeclaration::new(
        "backend:forge-store",
        StoreBackendCapabilityTier::PlatformGrade,
    )
    .unwrap();
    let _witness = PlatformGradeEvidenceWitness::from_foundation_witnesses(
        declaration,
        Vec::new(),
        [Roadmap2SequenceId::new("S1").unwrap()],
    )
    .unwrap();
}
