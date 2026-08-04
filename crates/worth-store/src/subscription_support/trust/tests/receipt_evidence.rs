use super::super::{
    SupportBasisReceipt, SupportCompatibilityReceipt, SupportCursorCheckpointReceipt,
    SupportFamilyRoleReceipt, SupportMaintenanceReceipt, SupportOperationalVerdictReceipt,
    SupportPortabilityReceipt, SupportResumeClassificationReceipt, SupportRetentionReceipt,
    SupportTrustReceiptBundle, SupportTrustReceiptStatus,
};
use super::operational_basis::basis;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportArtifactId, SubscriptionSupportFamilyId,
    SubscriptionSupportOperationalBasis, SubscriptionSupportOperationalVerdict,
    SubscriptionSupportRole,
};

pub(super) fn family_role_receipt() -> SupportFamilyRoleReceipt {
    family_role_receipt_for(
        "basis-bound-continuation-support",
        SubscriptionSupportRole::ExactContinuation,
        "artifact:trust:phase-1",
    )
}

pub(super) fn family_role_receipt_for(
    family_id: &str,
    support_role: SubscriptionSupportRole,
    artifact_id: &str,
) -> SupportFamilyRoleReceipt {
    SupportFamilyRoleReceipt::new(
        SubscriptionSupportFamilyId::new(family_id).unwrap(),
        support_role,
        SubscriptionSupportArtifactId(artifact_id.into()),
        "family-role:proof",
        SupportTrustReceiptStatus::Proven,
    )
    .unwrap()
}

pub(super) fn phase2_receipts(
    classification: SubscriptionResumeClassification,
    verdict: SubscriptionSupportOperationalVerdict,
) -> SupportTrustReceiptBundle {
    phase2_receipts_for_basis(basis(), classification, verdict)
}

pub(super) fn phase2_receipts_for_basis(
    basis: SubscriptionSupportOperationalBasis,
    classification: SubscriptionResumeClassification,
    verdict: SubscriptionSupportOperationalVerdict,
) -> SupportTrustReceiptBundle {
    let artifact_id = basis.artifact_id().clone();
    let cursor_checkpoint_digest =
        format!("{}:{}", basis.cursor_digest(), basis.checkpoint_digest());
    let bundle = SupportTrustReceiptBundle::new(
        SupportResumeClassificationReceipt::new(
            artifact_id.clone(),
            classification,
            "resume:proof",
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
        SupportOperationalVerdictReceipt::new(
            basis.clone(),
            verdict,
            "operational:proof",
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
        family_role_receipt_for(
            basis.family_id().as_str(),
            basis.support_role(),
            basis.artifact_id().as_str(),
        ),
        SupportBasisReceipt::new(
            artifact_id.clone(),
            basis.basis_digest(),
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
        SupportCursorCheckpointReceipt::new(
            artifact_id.clone(),
            cursor_checkpoint_digest,
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
        SupportCompatibilityReceipt::new(
            artifact_id.clone(),
            basis.compatibility_digest(),
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
        SupportPortabilityReceipt::new(
            artifact_id.clone(),
            basis.portability_digest(),
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
    );
    match verdict {
        SubscriptionSupportOperationalVerdict::ExactResumePreserved
        | SubscriptionSupportOperationalVerdict::DegradedResumePreserved => bundle.with_retention(
            SupportRetentionReceipt::new(
                artifact_id,
                "retention:trust",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
        ),
        SubscriptionSupportOperationalVerdict::RebuildRequired => bundle.with_maintenance(
            SupportMaintenanceReceipt::new(
                artifact_id,
                "maintenance:admission",
                "maintenance:proof",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
        ),
        _ => bundle,
    }
}
