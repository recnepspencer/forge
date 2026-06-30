use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::facade::planar_projection_consumption::ProjectionConsumedPlanarFactsReceipt;
use crate::facade::planar_retained_facts::RetainedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarHistoricalInspection;
use crate::workload_platform::compiled_product_admission::denial::{
    SpatialCompiledProductAdmissionError, SpatialCompiledProductAdmissionErrorKind,
};
use crate::workload_platform::evidence_lookup_index_product::{
    EvidenceLookupIndexProduct, EvidenceLookupLedgerBasis,
};
use crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;
use crate::workload_platform::retained_cancellation_chain::RetainedCancellationChainReceipt;

pub(crate) fn evidence_lookup_from_basis(
    selected_plan: &EvidenceLookupSelectedPlan,
    basis: &EvidenceLookupLedgerBasis,
) -> Result<String, SpatialCompiledProductAdmissionError> {
    if basis.selected_plan_digest() != selected_plan.selected_plan_digest() {
        return Err(SpatialCompiledProductAdmissionError::new(
            SpatialCompiledProductAdmissionErrorKind::WrongAuthorityBasis,
            "evidence lookup basis selected-plan digest does not match the selected plan",
        ));
    }
    if basis.stage_receipt_digest() != selected_plan.stage_receipt_digest() {
        return Err(SpatialCompiledProductAdmissionError::new(
            SpatialCompiledProductAdmissionErrorKind::WrongReceiptFamily,
            "evidence lookup basis stage-receipt digest does not match the selected plan",
        ));
    }
    Ok(basis.basis_digest().to_string())
}

pub(crate) fn evidence_lookup_from_product(
    selected_plan: &EvidenceLookupSelectedPlan,
    product: &EvidenceLookupIndexProduct,
) -> Result<String, SpatialCompiledProductAdmissionError> {
    if product.selected_plan_digest() != selected_plan.selected_plan_digest() {
        return Err(SpatialCompiledProductAdmissionError::new(
            SpatialCompiledProductAdmissionErrorKind::WrongAuthorityBasis,
            "evidence lookup product selected-plan digest does not match the selected plan",
        ));
    }
    if product.stage_receipt_digest() != selected_plan.stage_receipt_digest() {
        return Err(SpatialCompiledProductAdmissionError::new(
            SpatialCompiledProductAdmissionErrorKind::WrongReceiptFamily,
            "evidence lookup product stage-receipt digest does not match the selected plan",
        ));
    }
    Ok(product.evidence_ledger_basis_digest().to_string())
}

pub(crate) fn retained_replay(
    historical: &RetainedPlanarHistoricalInspection,
    retained: &RetainedPlanarFactsReceipt,
    projection: &ProjectionConsumedPlanarFactsReceipt,
) -> Result<String, SpatialCompiledProductAdmissionError> {
    if historical.retained_fact_digest() != retained.retained_fact_digest() {
        return Err(SpatialCompiledProductAdmissionError::new(
            SpatialCompiledProductAdmissionErrorKind::WrongAuthorityBasis,
            "retained replay historical inspection does not match the retained fact receipt",
        ));
    }
    if projection.retained_planar_fact_digest() != retained.retained_fact_digest() {
        return Err(SpatialCompiledProductAdmissionError::new(
            SpatialCompiledProductAdmissionErrorKind::WrongAuthorityBasis,
            "projection-consumed receipt does not match the retained fact receipt",
        ));
    }
    Ok(historical.historical_digest().to_string())
}

pub(crate) fn retained_cancellation(receipt: &RetainedCancellationChainReceipt) -> String {
    retained_cancellation_source_authority_digest(
        receipt.workload_identity(),
        receipt.retained_basis_identity(),
    )
}

pub(crate) fn retained_cancellation_source_authority_digest(
    workload_identity: &str,
    retained_basis_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:retained-cancellation-source-authority:v1".to_string(),
            format!("workload:{workload_identity}"),
            format!("retained-basis:{retained_basis_identity}"),
        ],
    )
}
