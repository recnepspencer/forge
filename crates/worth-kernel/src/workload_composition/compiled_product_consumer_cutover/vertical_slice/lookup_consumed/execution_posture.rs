use super::reuse_resolution::ResolvedLookupConsumedVerticalSlice;
use crate::workload_composition::{
    admit_spatial_conflict_input, AdmittedSpatialConflictInput, SpatialConflictInputRequest,
    WorkloadCompositionError,
};

impl<'a> ResolvedLookupConsumedVerticalSlice<'a> {
    pub(crate) fn admit_spatial_conflict_input(
        &'a self,
    ) -> Result<AdmittedSpatialConflictInput<'a>, WorkloadCompositionError> {
        let product = match self.reuse_product() {
            super::reuse_resolution::LookupConsumedVerticalSliceReuseProduct::Reused(product)
            | super::reuse_resolution::LookupConsumedVerticalSliceReuseProduct::Rebuilt(product) => {
                product
            }
        };
        admit_spatial_conflict_input(
            SpatialConflictInputRequest::new(self.slice().boundary().authority())
                .with_lookup_compiled_product(self.slice().boundary().workload_handoff(), product),
        )
    }
}
