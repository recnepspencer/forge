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
    if basis.spatial_touch_digest() != selected_plan.spatial_touch_digest() {
        return Err(SpatialCompiledProductAdmissionError::new(
            SpatialCompiledProductAdmissionErrorKind::WrongAuthorityBasis,
            "evidence lookup basis spatial-touch digest does not match the selected plan",
        ));
    }
    Ok(selected_plan.spatial_touch_digest().to_string())
}

pub(crate) fn evidence_lookup_from_product(
    selected_plan: &EvidenceLookupSelectedPlan,
    product: &EvidenceLookupIndexProduct,
) -> Result<String, SpatialCompiledProductAdmissionError> {
    if product.spatial_touch_digest() != selected_plan.spatial_touch_digest() {
        return Err(SpatialCompiledProductAdmissionError::new(
            SpatialCompiledProductAdmissionErrorKind::WrongAuthorityBasis,
            "evidence lookup product spatial-touch digest does not match the selected plan",
        ));
    }
    Ok(selected_plan.spatial_touch_digest().to_string())
}

pub(crate) fn retained_replay(projection_consumption_digest: &str) -> String {
    projection_consumption_digest.to_string()
}

pub(crate) fn retained_cancellation(receipt: &RetainedCancellationChainReceipt) -> String {
    receipt.projection_consumed_identity().to_string()
}
