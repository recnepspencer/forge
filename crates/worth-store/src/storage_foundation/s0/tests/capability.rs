use super::support::*;
use crate::storage_foundation::s0::*;

#[test]
fn phase1_semantic_evidence_stays_report_only() {
    let declaration = BackendCapabilityDeclaration::new(
        "backend:sqlite",
        StoreBackendCapabilityTier::PlatformGrade,
    )
    .unwrap()
    .with_forbidden_claim(
        BackendForbiddenClaim::new(BackendForbiddenClaimKind::PlatformGradeDurability, "S1")
            .unwrap(),
    );
    let raw = UnclassifiedBackendClaim::new(declaration, StoreBackendCapabilityTier::PlatformGrade);
    let classified = classify_backend_claim(raw).unwrap();
    let audited = audit_forbidden_claims(classified).unwrap();
    let semantic = SemanticOnlyClaimWitness::new("semantic certification only");
    let bound =
        bind_roadmap2_evidence(audited, Roadmap2EvidenceBound::SemanticOnly(semantic)).unwrap();

    assert_eq!(
        bound.evidence_kind(),
        StoreBackendCapabilityTier::SemanticCertification
    );
}

#[test]
fn phase1_forbidden_claim_sequence_is_typed_rejection_not_panic() {
    let error = BackendForbiddenClaim::new(BackendForbiddenClaimKind::PhysicalPersistence, "")
        .expect_err("empty deferred sequence must reject typed");

    assert_eq!(error, S0ClaimPromotionRejection::MissingSequenceMapping);
}

#[test]
fn phase1_foundation_witnesses_promote_only_when_all_required_sequences_exist() {
    let declaration = BackendCapabilityDeclaration::new(
        "backend:future-platform",
        StoreBackendCapabilityTier::PlatformGrade,
    )
    .unwrap();
    let raw = UnclassifiedBackendClaim::new(
        declaration.clone(),
        StoreBackendCapabilityTier::PlatformGrade,
    );
    let classified = classify_backend_claim(raw).unwrap();
    let audited = audit_forbidden_claims(classified).unwrap();
    let foundation = FoundationEvidenceWitness::new(
        Roadmap2SequenceId::new("S1").unwrap(),
        digest("evidence:s1"),
    );
    let platform = PlatformGradeEvidenceWitness::from_foundation_witnesses(
        declaration,
        vec![foundation],
        [Roadmap2SequenceId::new("S1").unwrap()],
    )
    .unwrap();
    let admitted =
        admit_platform_grade_claim(bind_platform_grade_evidence(audited, platform).unwrap())
            .unwrap();

    assert_eq!(admitted.subject(), "backend:future-platform");
    assert_eq!(
        admitted.admitted_tier(),
        StoreBackendCapabilityTier::PlatformGrade
    );
}
