use forge_store::{
    admit_platform_grade_claim, audit_forbidden_claims, bind_roadmap2_evidence,
    classify_backend_claim, BackendCapabilityDeclaration, PhysicalDebtWitness,
    Roadmap2EvidenceBound, Roadmap2SequenceId, StoreBackendCapabilityTier,
    UnclassifiedBackendClaim,
};

fn main() {
    let declaration = BackendCapabilityDeclaration::new(
        "backend:forge-store",
        StoreBackendCapabilityTier::PlatformGrade,
    )
    .unwrap();
    let classified = classify_backend_claim(UnclassifiedBackendClaim::new(
        declaration,
        StoreBackendCapabilityTier::PlatformGrade,
    ))
    .unwrap();
    let audited = audit_forbidden_claims(classified).unwrap();
    let debt = PhysicalDebtWitness::new(Roadmap2SequenceId::new("S1").unwrap(), "deferred");
    let bound = bind_roadmap2_evidence(audited, Roadmap2EvidenceBound::PhysicalDebt(debt)).unwrap();

    let _ = admit_platform_grade_claim(bound).unwrap();
}
