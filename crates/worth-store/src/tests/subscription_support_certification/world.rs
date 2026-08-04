use super::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion,
    CompatibilityAdmissionCounters, CompatibilityAdmissionPath, CompatibilityAdmissionReceipt,
    CompatibilityManifestDigest, CompatibilityReadAdmissionOutcome, CompatibilityRejection,
    CompatibilityRejectionKind, CompatibilityRelation, QuarantinedDecodedArtifact,
    RawSubscriptionSupportDeclaration, ReadCompatibilityReceipt, SubscriptionSupportArtifactId,
    SubscriptionSupportAuthority, SubscriptionSupportClassificationPlan,
    SubscriptionSupportClassificationReport, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportFetchRequest,
    SubscriptionSupportPayloadDigest, SubscriptionSupportResumeEvidence,
    SubscriptionSupportResumeRequest, SubscriptionSupportRole, SubscriptionSupportScope,
    SupportCompatibilityReceiptWitness, SupportFamilyVersionWindow, WORTHStore,
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

pub(super) fn retention_basis(artifact_suffix: &str) -> crate::SubscriptionSupportOperationalBasis {
    crate::SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:cert-retention:{artifact_suffix}")),
        "basis:cert-retention",
        "cursor:cert-retention",
        "checkpoint:cert-retention",
        "compatibility:cert-retention",
        "portability:cert-retention",
        crate::SubscriptionSupportActionOrigin::Retention,
    )
    .unwrap()
}

pub(super) fn compatibility_basis(
    artifact_suffix: &str,
) -> crate::SubscriptionSupportOperationalBasis {
    crate::SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:cert-compatibility:{artifact_suffix}")),
        "basis:cert-compatibility",
        "cursor:cert-compatibility",
        "checkpoint:cert-compatibility",
        "compatibility:manifest-v2",
        "portability:cert-compatibility",
        crate::SubscriptionSupportActionOrigin::Compatibility,
    )
    .unwrap()
}

pub(super) fn portability_basis(
    action_origin: crate::SubscriptionSupportActionOrigin,
    artifact_suffix: &str,
) -> crate::SubscriptionSupportOperationalBasis {
    crate::SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:cert-portability:{artifact_suffix}")),
        "basis:cert-portability",
        "cursor:cert-portability",
        "checkpoint:cert-portability",
        "compatibility:cert-portability",
        format!("portability:cert-portability:{artifact_suffix}"),
        action_origin,
    )
    .unwrap()
}

pub(super) fn maintenance_basis(
    artifact_suffix: &str,
) -> crate::SubscriptionSupportOperationalBasis {
    crate::SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:cert-maintenance:{artifact_suffix}")),
        format!("basis:cert-maintenance:{artifact_suffix}"),
        "cursor:cert-maintenance",
        "checkpoint:cert-maintenance",
        "compatibility:cert-maintenance",
        "portability:cert-maintenance",
        crate::SubscriptionSupportActionOrigin::Maintenance,
    )
    .unwrap()
}

pub(super) fn support_version_window() -> SupportFamilyVersionWindow {
    SupportFamilyVersionWindow::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        1,
        2,
    )
    .unwrap()
}

pub(super) fn compatibility_manifest_digest(
    family_id: &ArtifactFamilyId,
) -> CompatibilityManifestDigest {
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
        "structural:cert-support-compatibility",
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

pub(super) fn publish_exact(
    store: &mut WORTHStore,
    basis: &str,
    cursor: &str,
    checkpoint: &str,
) -> SubscriptionSupportArtifactId {
    let admitted = store
        .admit_subscription_support_declaration(raw_exact())
        .unwrap();
    let publishable = store
        .subscription_support_pipeline()
        .prepare_exact(
            admitted,
            basis,
            cursor,
            checkpoint,
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    store
        .publish_subscription_support(publishable)
        .unwrap()
        .artifact_id()
        .clone()
}

pub(super) fn fetched_exact_report(
    store: &mut WORTHStore,
) -> SubscriptionSupportClassificationReport {
    let artifact_id = publish_exact(store, "basis:1", "cursor:1", "checkpoint:1");
    let fetched = store
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            artifact_id,
        ))
        .unwrap();
    let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true).unwrap();
    store
        .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
            fetched,
            evidence,
            SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
        ))
        .unwrap()
}
