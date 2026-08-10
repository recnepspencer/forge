use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::super::performance::SupportTrustDensityClass;
use super::super::receipt_bundle::SupportTrustReceiptBundle;
use super::super::receipts::SupportTrustReceiptStatus;
use super::request::{
    RawSupportTrustRequest, SupportTrustBatchCardinality, SupportTrustRequestedUse,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustRequestAdmitted {
    request: RawSupportTrustRequest,
    receipt_bundle: SupportTrustReceiptBundle,
}

impl SupportTrustRequestAdmitted {
    pub(super) fn request(&self) -> &RawSupportTrustRequest {
        &self.request
    }

    pub(super) fn receipt_bundle(&self) -> &SupportTrustReceiptBundle {
        &self.receipt_bundle
    }
}

pub fn admit_support_trust_request(
    request: RawSupportTrustRequest,
    receipt_bundle: SupportTrustReceiptBundle,
) -> Result<SupportTrustRequestAdmitted, SupportTrustFailure> {
    let family_role_receipt = receipt_bundle.family_role();
    require_proven(family_role_receipt.status(), "family-role")?;
    reject_certified_platform_claim(&request)?;
    require_nonempty_batch_cardinality(request.batch_cardinality())?;
    require_cardinality_density_match(
        request.batch_cardinality(),
        request.performance_plan().density_class(),
    )?;
    require_request_evidence_budget(&request, &receipt_bundle)?;
    require_request_identity_matches(&request, family_role_receipt)?;
    Ok(SupportTrustRequestAdmitted {
        request,
        receipt_bundle,
    })
}

pub(super) fn require_proven(
    status: SupportTrustReceiptStatus,
    label: &'static str,
) -> Result<(), SupportTrustFailure> {
    if !status.is_proven() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            format!("support trust {label} receipt must be proven"),
        ));
    }
    Ok(())
}

fn reject_certified_platform_claim(
    request: &RawSupportTrustRequest,
) -> Result<(), SupportTrustFailure> {
    if request.requested_use() == SupportTrustRequestedUse::CertifiedPlatformClaim {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "certified platform trust claims require certification coverage before admission",
        ));
    }
    Ok(())
}

fn require_nonempty_batch_cardinality(
    cardinality: SupportTrustBatchCardinality,
) -> Result<(), SupportTrustFailure> {
    if cardinality.artifact_count() == 0 {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust batch cardinality must include at least one artifact",
        ));
    }
    Ok(())
}

fn require_request_evidence_budget(
    request: &RawSupportTrustRequest,
    receipt_bundle: &SupportTrustReceiptBundle,
) -> Result<(), SupportTrustFailure> {
    if !request.evidence_budget().admits(
        receipt_bundle.receipt_bytes(),
        receipt_bundle.receipt_count(),
        request.batch_cardinality().artifact_count(),
    ) {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust receipt bundle exceeds the admitted evidence budget",
        ));
    }
    Ok(())
}

fn require_request_identity_matches(
    request: &RawSupportTrustRequest,
    family_role_receipt: &super::super::receipts::SupportFamilyRoleReceipt,
) -> Result<(), SupportTrustFailure> {
    if request.family_id() != family_role_receipt.family_id() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustFamilyMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust request family must match family-role receipt",
        ));
    }
    if request.support_role() != family_role_receipt.support_role() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustRoleMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust request role must match family-role receipt",
        ));
    }
    if request.artifact_id() != family_role_receipt.artifact_id() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustBasisMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust request artifact must match family-role receipt",
        ));
    }
    Ok(())
}

fn require_cardinality_density_match(
    cardinality: SupportTrustBatchCardinality,
    density_class: SupportTrustDensityClass,
) -> Result<(), SupportTrustFailure> {
    match (cardinality, density_class) {
        (
            SupportTrustBatchCardinality::SingleSupportArtifact,
            SupportTrustDensityClass::SingleSupportArtifact,
        ) => Ok(()),
        (
            SupportTrustBatchCardinality::FamilyRoleBatch { artifact_count },
            SupportTrustDensityClass::FamilyLocal | SupportTrustDensityClass::RoleLocal,
        ) if artifact_count > 1 => Ok(()),
        (SupportTrustBatchCardinality::FamilyRoleBatch { .. }, _) => Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustAccessStructureDebt,
            SupportTrustRecoveryPosture::RebuildTrustCache,
            "family-role support trust batches require family-local or role-local density",
        )),
        _ => Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustAccessStructureDebt,
            SupportTrustRecoveryPosture::RebuildTrustCache,
            "single support trust requests require single-artifact density",
        )),
    }
}
