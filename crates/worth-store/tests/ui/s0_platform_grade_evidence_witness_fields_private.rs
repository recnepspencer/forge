use worth_store::{
    BackendCapabilityDeclaration, PlatformGradeEvidenceWitness, StoreBackendCapabilityTier,
};

fn main() {
    let declaration = BackendCapabilityDeclaration::new(
        "backend:worth-store",
        StoreBackendCapabilityTier::PlatformGrade,
    )
    .unwrap();

    let _witness = PlatformGradeEvidenceWitness {
        declaration,
        foundation_witnesses: Vec::new(),
        accepted_sequence_count: 0,
    };
}
