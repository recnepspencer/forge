use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceReceipt, SpatialGeometryEvidenceTouchAuthority,
    SpatialGeometryEvidenceTouchRequest,
};

use super::{WorkloadCompositionError, WorthWorkload};

impl WorthWorkload {
    pub fn admit_spatial_geometry_evidence_touch<T: BooleanEvidenceReceipt + 'static>(
        &self,
        receipt: &T,
    ) -> Result<SpatialGeometryEvidenceTouchAuthority, WorkloadCompositionError> {
        SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(receipt)
            .with_complete_ledger(self.evidence_ledger())
            .admit()
            .map_err(WorkloadCompositionError::SpatialTouchAuthority)
    }
}
