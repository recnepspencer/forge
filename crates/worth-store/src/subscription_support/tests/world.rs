use super::super::{
    new, RawSubscriptionSupportDeclaration, SubscriptionSupportActionOrigin,
    SubscriptionSupportArtifactId, SubscriptionSupportAuthority, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportOperationalBasis,
    SubscriptionSupportPayloadDigest, SubscriptionSupportRole, SubscriptionSupportScope,
    SupportCompatibilityReceiptWitness, SupportFamilyVersionWindow,
};
use super::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion,
    CompatibilityAdmissionCounters, CompatibilityAdmissionPath, CompatibilityAdmissionReceipt,
    CompatibilityManifestDigest, CompatibilityReadAdmissionOutcome, CompatibilityRejection,
    CompatibilityRejectionKind, CompatibilityRelation, QuarantinedDecodedArtifact,
    ReadCompatibilityReceipt,
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

pub(super) fn operational_basis(
    action_origin: SubscriptionSupportActionOrigin,
) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId("artifact:phase-1".into()),
        "basis:phase-1",
        "cursor:phase-1",
        "checkpoint:phase-1",
        "compatibility:phase-1",
        "portability:phase-1",
        action_origin,
    )
    .unwrap()
}

pub(super) fn retention_basis(artifact_suffix: &str) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:retention:{artifact_suffix}")),
        "basis:retention",
        "cursor:retention",
        "checkpoint:retention",
        "compatibility:retention",
        "portability:retention",
        SubscriptionSupportActionOrigin::Retention,
    )
    .unwrap()
}

pub(super) fn retention_basis_for_family(
    family_id: &str,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    artifact_suffix: &str,
    action_origin: SubscriptionSupportActionOrigin,
) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new(family_id).unwrap(),
        family_kind,
        support_role,
        SubscriptionSupportArtifactId(format!("artifact:retention:{artifact_suffix}")),
        "basis:retention",
        "cursor:retention",
        "checkpoint:retention",
        "compatibility:retention",
        "portability:retention",
        action_origin,
    )
    .unwrap()
}

pub(super) fn compatibility_basis(artifact_suffix: &str) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:compatibility:{artifact_suffix}")),
        "basis:compatibility",
        "cursor:compatibility",
        "checkpoint:compatibility",
        "compatibility:manifest-v2",
        "portability:compatibility",
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
        SubscriptionSupportArtifactId(format!("artifact:portability:{artifact_suffix}")),
        "basis:portability",
        "cursor:portability",
        "checkpoint:portability",
        "compatibility:portability",
        format!("portability:{artifact_suffix}"),
        action_origin,
    )
    .unwrap()
}

pub(super) fn maintenance_basis(artifact_suffix: &str) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:maintenance:{artifact_suffix}")),
        format!("basis:maintenance:{artifact_suffix}"),
        "cursor:maintenance",
        "checkpoint:maintenance",
        "compatibility:maintenance",
        "portability:maintenance",
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

pub(super) fn read_receipt_witness(
    relation: CompatibilityRelation,
) -> SupportCompatibilityReceiptWitness {
    let family_id = ArtifactFamilyId::new("basis-bound-continuation-support");
    let manifest_digest = compatibility_manifest_digest(&family_id);
    let receipt = ReadCompatibilityReceipt::new(CompatibilityAdmissionReceipt::new(
        family_id,
        manifest_digest,
        "support-registry:snapshot",
        "support-manifest:frontier",
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(1),
        CompatibilityAdmissionPath::BatchRead,
        relation,
    ));
    SupportCompatibilityReceiptWitness::from_read_receipt(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        &receipt,
    )
    .unwrap()
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
        "structural:support-compatibility",
        "support compatibility rejection fixture",
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
