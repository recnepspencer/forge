mod capability_gap;
mod shape_projection;

pub use capability_gap::{
    WorthGraphReadRequirementDerivationCapabilityGap,
    WorthGraphReadRequirementDerivationCapabilityGapKind,
};
pub(crate) use shape_projection::{
    derivation_attempt_for_catalog_record, derive_query_requirement_outcome_for_catalog_record,
};

#[cfg(test)]
pub(crate) use shape_projection::derive_query_requirement_outcome_for_catalog_record_with_requirement_labels;
