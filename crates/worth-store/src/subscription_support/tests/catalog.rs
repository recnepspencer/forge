use super::super::{
    new, pipeline, RawSubscriptionSupportDeclaration, SubscriptionSupportAccessStructure,
    SubscriptionSupportAdmissionViolation, SubscriptionSupportAuthority,
    SubscriptionSupportCatalog, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportPayloadDigest, SubscriptionSupportPublicationPipeline,
    SubscriptionSupportRole, SubscriptionSupportScope,
};
use super::raw_exact;
use super::StoreErrorKind;

#[test]
fn catalog_rejects_family_role_mismatch() {
    let raw = RawSubscriptionSupportDeclaration::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::DegradedContinuation,
        SubscriptionSupportAuthority::WorthQuery,
        "worth-query-live-v1",
        SubscriptionSupportScope::from_canonical(vec!["feed:2".into()]).unwrap(),
        SubscriptionSupportPayloadDigest::new("payload:abc").unwrap(),
    );

    let error = SubscriptionSupportCatalog::first_ship()
        .admit(raw)
        .expect_err("role mismatch should reject before publication");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportAdmissionViolation
    );
}

#[test]
fn scope_rejects_noncanonical_order() {
    let error = SubscriptionSupportScope::from_canonical(vec!["z".into(), "a".into()])
        .expect_err("noncanonical declaration scopes must reject");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportAdmissionViolation
    );
}

#[test]
fn catalog_rejects_unadmitted_upstream_authority() {
    let raw = RawSubscriptionSupportDeclaration::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportAuthority::Unadmitted("external-test".into()),
        "worth-query-live-v1",
        SubscriptionSupportScope::from_canonical(vec!["feed:2".into()]).unwrap(),
        SubscriptionSupportPayloadDigest::new("payload:abc").unwrap(),
    );

    let error = SubscriptionSupportCatalog::first_ship()
        .admit(raw)
        .expect_err("unknown upstream authority must reject before publication");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportAdmissionViolation
    );
}

#[test]
fn catalog_declares_required_access_structures() {
    let report = SubscriptionSupportCatalog::first_ship().access_structures();

    assert_eq!(
        report.required(),
        &[
            SubscriptionSupportAccessStructure::FamilyLookup,
            SubscriptionSupportAccessStructure::ArtifactLookupByFamilyAndArtifact,
            SubscriptionSupportAccessStructure::DeclarationLookup,
            SubscriptionSupportAccessStructure::BasisLookup,
            SubscriptionSupportAccessStructure::CursorLookup,
            SubscriptionSupportAccessStructure::CheckpointLookup,
            SubscriptionSupportAccessStructure::CompatibilityLookup,
            SubscriptionSupportAccessStructure::ClassificationLookup,
            SubscriptionSupportAccessStructure::RestartManifestSequence,
        ]
    );
}

#[test]
fn artifact_identity_is_deterministic_and_family_bound() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let admitted = pipeline.admit(raw_exact()).unwrap();
    let left = pipeline
        .prepare_exact(
            admitted.clone(),
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    let right = pipeline
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();

    assert_eq!(left.artifact_id(), right.artifact_id());
    assert!(left
        .artifact_id()
        .as_str()
        .starts_with("subscription-support:basis-bound-continuation-support:"));
}
