use super::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion,
    CompatibilityAdmissionCounters, CompatibilityManifestDigest, CompatibilityReadAdmissionOutcome,
    CompatibilityRejection, CompatibilityRejectionKind, QuarantinedDecodedArtifact,
    RawSubscriptionSupportDeclaration, SubscriptionSupportActionOrigin,
    SubscriptionSupportArtifactId, SubscriptionSupportAuthority, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportOperationalBasis,
    SubscriptionSupportPayloadDigest, SubscriptionSupportRole, SubscriptionSupportScope,
    SupportCompatibilityReceiptWitness, SupportFamilyVersionWindow,
};

pub(super) fn raw_exact() -> RawSubscriptionSupportDeclaration {
    RawSubscriptionSupportDeclaration::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportAuthority::WorthQuery,
        "worth-query-live-v1",
        SubscriptionSupportScope::from_canonical(vec!["account:1".into(), "feed:2".into()])
            .unwrap(),
        SubscriptionSupportPayloadDigest::new("payload:abc").unwrap(),
    )
}

pub(super) fn raw_degraded() -> RawSubscriptionSupportDeclaration {
    RawSubscriptionSupportDeclaration::new(
        SubscriptionSupportFamilyId::new("degraded-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::DegradedContinuationSupport,
        SubscriptionSupportRole::DegradedContinuation,
        SubscriptionSupportAuthority::WorthQuery,
        "worth-query-live-v1",
        SubscriptionSupportScope::from_canonical(vec!["feed:2".into()]).unwrap(),
        SubscriptionSupportPayloadDigest::new("payload:def").unwrap(),
    )
}

pub(super) fn raw_materialized() -> RawSubscriptionSupportDeclaration {
    RawSubscriptionSupportDeclaration::new(
        SubscriptionSupportFamilyId::new("materialized-narrowing-support").unwrap(),
        SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
        SubscriptionSupportRole::NarrowingMaterialization,
        SubscriptionSupportAuthority::WORTHRuntimeBridge,
        "worth-runtime-bridge-v1",
        SubscriptionSupportScope::from_canonical(vec!["account:1".into(), "narrow:active".into()])
            .unwrap(),
        SubscriptionSupportPayloadDigest::new("payload:materialized").unwrap(),
    )
}

pub(super) fn retention_basis(artifact_suffix: &str) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:store-retention:{artifact_suffix}")),
        "basis:store-retention",
        "cursor:store-retention",
        "checkpoint:store-retention",
        "compatibility:store-retention",
        "portability:store-retention",
        SubscriptionSupportActionOrigin::Retention,
    )
    .unwrap()
}

pub(super) fn compatibility_basis(artifact_suffix: &str) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:store-compatibility:{artifact_suffix}")),
        "basis:store-compatibility",
        "cursor:store-compatibility",
        "checkpoint:store-compatibility",
        "compatibility:store-compatibility",
        "portability:store-compatibility",
        SubscriptionSupportActionOrigin::Compatibility,
    )
    .unwrap()
}

pub(super) fn portability_basis(
    action_origin: SubscriptionSupportActionOrigin,
    artifact_suffix: &str,
) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:store-portability:{artifact_suffix}")),
        "basis:store-portability",
        "cursor:store-portability",
        "checkpoint:store-portability",
        "compatibility:store-portability",
        format!("portability:store-portability:{artifact_suffix}"),
        action_origin,
    )
    .unwrap()
}

pub(super) fn maintenance_basis(artifact_suffix: &str) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:store-maintenance:{artifact_suffix}")),
        format!("basis:store-maintenance:{artifact_suffix}"),
        "cursor:store-maintenance",
        "checkpoint:store-maintenance",
        "compatibility:store-maintenance",
        "portability:store-maintenance",
        SubscriptionSupportActionOrigin::Maintenance,
    )
    .unwrap()
}

fn support_version_window() -> SupportFamilyVersionWindow {
    SupportFamilyVersionWindow::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        1,
        2,
    )
    .unwrap()
}

fn compatibility_manifest_digest(family_id: &ArtifactFamilyId) -> CompatibilityManifestDigest {
    CompatibilityManifestDigest::compute(
        family_id,
        &ArtifactCompatibilityWindow::native(1),
        "authoritative",
    )
}

pub(super) fn rejected_read_outcome_witness(
    rejection_kind: CompatibilityRejectionKind,
) -> SupportCompatibilityReceiptWitness {
    let family_id = ArtifactFamilyId::new("basis-bound-continuation-support");
    let manifest_digest = compatibility_manifest_digest(&family_id);
    let artifact = QuarantinedDecodedArtifact::new(
        family_id.clone(),
        ArtifactFormatVersion::new(2),
        ArtifactSemanticVersion::new(2),
        manifest_digest,
        "structural:store-support-compatibility",
        "store support compatibility rejection fixture",
    );
    let rejection = CompatibilityRejection::new(
        rejection_kind,
        family_id,
        "support compatibility rejected by Milestone 12",
    );
    let counters = CompatibilityAdmissionCounters::default();
    let outcome = CompatibilityReadAdmissionOutcome::rejected(&artifact, &rejection, &counters);
    SupportCompatibilityReceiptWitness::from_read_admission_outcome(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        support_version_window(),
        &outcome,
    )
    .unwrap()
}
