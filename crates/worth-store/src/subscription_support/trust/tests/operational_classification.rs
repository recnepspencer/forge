use super::super::{
    admit_support_trust_request, check_support_trust_drift, check_support_trust_equivalence,
    classify_operational_support_trust, translate_support_trust_inputs,
    OperationalSupportTrustClassified, SupportExactTrustTranslation, SupportTrustDriftScanPlan,
    SupportTrustEquivalenceEvidence, SupportTrustFailure, SupportTrustProvenance,
    SupportTrustStrength,
};
use super::operational_basis::{basis, raw_phase2_request_for};
use super::receipt_evidence::phase2_receipts_for_basis;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdict,
};

pub(super) fn exact_translation() -> SupportExactTrustTranslation {
    SupportExactTrustTranslation::new(
        basis(),
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
    )
    .unwrap()
}

pub(super) fn classify_phase2(
    requested_strength: SupportTrustStrength,
    provenance: SupportTrustProvenance,
    classification: SubscriptionResumeClassification,
    verdict: SubscriptionSupportOperationalVerdict,
) -> Result<OperationalSupportTrustClassified, SupportTrustFailure> {
    classify_phase2_for_basis(
        basis(),
        requested_strength,
        provenance,
        classification,
        verdict,
    )
}

pub(super) fn classify_phase2_for_basis(
    basis: SubscriptionSupportOperationalBasis,
    requested_strength: SupportTrustStrength,
    provenance: SupportTrustProvenance,
    classification: SubscriptionResumeClassification,
    verdict: SubscriptionSupportOperationalVerdict,
) -> Result<OperationalSupportTrustClassified, SupportTrustFailure> {
    let admitted = admit_support_trust_request(
        raw_phase2_request_for(
            basis.family_id().as_str(),
            basis.support_role(),
            basis.artifact_id().as_str(),
            requested_strength,
            provenance,
        ),
        phase2_receipts_for_basis(basis, classification, verdict),
    )?;
    let translated = translate_support_trust_inputs(admitted)?;
    let drift_checked = check_support_trust_drift(
        translated,
        SupportTrustDriftScanPlan::foreground_support_identity(),
    )?;
    let equivalence_checked =
        check_support_trust_equivalence(drift_checked, SupportTrustEquivalenceEvidence::none())?;
    classify_operational_support_trust(equivalence_checked)
}
