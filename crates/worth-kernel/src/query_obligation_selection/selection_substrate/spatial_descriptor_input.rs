use worth_spatial::facade::workload_vocabulary::SpatialEvidenceQueryTouchDescriptor;

use super::selection_error::QueryObligationSelectionError;
use super::selection_request::{
    QueryObligationSelectionAuthorityKind, QueryObligationSelectionInput,
};

impl QueryObligationSelectionInput {
    pub fn from_spatial_query_descriptor(
        descriptor: &SpatialEvidenceQueryTouchDescriptor,
    ) -> Result<Self, QueryObligationSelectionError> {
        Ok(Self::from_authority_parts(
            descriptor.touch_descriptor().clone(),
            descriptor.operating_world().clone(),
            descriptor.product_digest().as_str(),
            QueryObligationSelectionAuthorityKind::SpatialQueryDescriptor,
        )?
        .with_spatial_descriptor(descriptor.clone()))
    }
}
