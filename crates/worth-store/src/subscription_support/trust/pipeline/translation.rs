use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::super::taxonomy::SupportTrustProvenance;
use super::super::translation::SupportTrustTranslationPlan;
use super::admission::{require_proven, SupportTrustRequestAdmitted};
use crate::subscription_support::{
    SubscriptionSupportArtifactId, SubscriptionSupportOperationalVerdict,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustTranslatedInputs {
    admitted: SupportTrustRequestAdmitted,
    translation_plan: SupportTrustTranslationPlan,
    receipt_count: u64,
}

impl SupportTrustTranslatedInputs {
    pub(super) fn admitted(&self) -> &SupportTrustRequestAdmitted {
        &self.admitted
    }

    pub(super) fn into_operational_inputs(
        self,
    ) -> (
        SupportTrustRequestAdmitted,
        SupportTrustTranslationPlan,
        u64,
    ) {
        (self.admitted, self.translation_plan, self.receipt_count)
    }
}

pub fn translate_support_trust_inputs(
    admitted: SupportTrustRequestAdmitted,
) -> Result<SupportTrustTranslatedInputs, SupportTrustFailure> {
    let resume_receipt = admitted.receipt_bundle().resume();
    let operational_receipt = admitted.receipt_bundle().operational();
    let basis_receipt = admitted.receipt_bundle().basis();
    let cursor_checkpoint_receipt = admitted.receipt_bundle().cursor_checkpoint();
    let compatibility_receipt = admitted.receipt_bundle().compatibility();
    let portability_receipt = admitted.receipt_bundle().portability();
    require_proven(resume_receipt.status(), "resume classification")?;
    require_proven(operational_receipt.status(), "operational verdict")?;
    require_proven(basis_receipt.status(), "basis")?;
    require_proven(cursor_checkpoint_receipt.status(), "cursor/checkpoint")?;
    require_proven(compatibility_receipt.status(), "compatibility")?;
    require_proven(portability_receipt.status(), "portability")?;
    require_contextual_receipts(&admitted)?;
    let artifact_id = admitted.request().artifact_id();
    for receipt_artifact_id in [
        resume_receipt.artifact_id(),
        operational_receipt.basis().artifact_id(),
        basis_receipt.artifact_id(),
        cursor_checkpoint_receipt.artifact_id(),
        compatibility_receipt.artifact_id(),
        portability_receipt.artifact_id(),
    ] {
        if receipt_artifact_id != artifact_id {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustBasisMismatch,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "support trust receipts must all be bound to the requested artifact",
            ));
        }
    }
    let translation_plan = SupportTrustTranslationPlan::from_resume_and_operational(
        operational_receipt.basis().clone(),
        resume_receipt.classification(),
        operational_receipt.verdict(),
    )?;
    let receipt_count = admitted.receipt_bundle().receipt_count();
    Ok(SupportTrustTranslatedInputs {
        admitted,
        translation_plan,
        receipt_count,
    })
}

fn require_contextual_receipts(
    admitted: &SupportTrustRequestAdmitted,
) -> Result<(), SupportTrustFailure> {
    let operational_verdict = admitted.receipt_bundle().operational().verdict();
    if matches!(
        operational_verdict,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved
            | SubscriptionSupportOperationalVerdict::DegradedResumePreserved
    ) {
        require_resume_context_receipt(admitted)?;
    }
    if operational_verdict == SubscriptionSupportOperationalVerdict::RebuildRequired {
        require_rebuild_context_receipt(admitted)?;
    }
    if admitted.request().provenance() == SupportTrustProvenance::Imported {
        require_import_context_receipt(admitted)?;
    }
    Ok(())
}

fn require_resume_context_receipt(
    admitted: &SupportTrustRequestAdmitted,
) -> Result<(), SupportTrustFailure> {
    let retention = admitted.receipt_bundle().retention().ok_or_else(|| {
        SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "exact and degraded support trust require retention proof",
        )
    })?;
    require_proven(retention.status(), "retention")?;
    require_receipt_artifact(
        retention.artifact_id(),
        admitted.request().artifact_id(),
        "retention",
    )
}

fn require_rebuild_context_receipt(
    admitted: &SupportTrustRequestAdmitted,
) -> Result<(), SupportTrustFailure> {
    let maintenance = admitted.receipt_bundle().maintenance().ok_or_else(|| {
        SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "rebuild-derived support trust requires maintenance admission proof",
        )
    })?;
    require_proven(maintenance.status(), "maintenance")?;
    require_receipt_artifact(
        maintenance.artifact_id(),
        admitted.request().artifact_id(),
        "maintenance",
    )
}

fn require_import_context_receipt(
    admitted: &SupportTrustRequestAdmitted,
) -> Result<(), SupportTrustFailure> {
    let import = admitted
        .receipt_bundle()
        .import_admission()
        .ok_or_else(|| {
            SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "imported support trust requires target-side import admission proof",
            )
        })?;
    require_proven(import.status(), "import admission")?;
    require_receipt_artifact(
        import.artifact_id(),
        admitted.request().artifact_id(),
        "import admission",
    )?;
    if import.target_family_id() != admitted.request().family_id() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustFamilyMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "import admission target family must match the trust request family",
        ));
    }
    Ok(())
}

fn require_receipt_artifact(
    receipt_artifact_id: &SubscriptionSupportArtifactId,
    request_artifact_id: &SubscriptionSupportArtifactId,
    label: &'static str,
) -> Result<(), SupportTrustFailure> {
    if receipt_artifact_id != request_artifact_id {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustBasisMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            format!("support trust {label} receipt must be bound to the requested artifact"),
        ));
    }
    Ok(())
}
