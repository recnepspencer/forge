use super::classification::{
    PlanarBooleanOverlapOperatorClassification, PlanarBooleanOverlapOperatorTruthAuthority,
    PlanarBooleanOverlapRequiredQuerySurface,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapOperatorRow {
    operator_name: &'static str,
    classification: PlanarBooleanOverlapOperatorClassification,
    truth_authority: PlanarBooleanOverlapOperatorTruthAuthority,
    required_query_surface: PlanarBooleanOverlapRequiredQuerySurface,
}

impl PlanarBooleanOverlapOperatorRow {
    pub(crate) const fn new(
        operator_name: &'static str,
        classification: PlanarBooleanOverlapOperatorClassification,
        truth_authority: PlanarBooleanOverlapOperatorTruthAuthority,
        required_query_surface: PlanarBooleanOverlapRequiredQuerySurface,
    ) -> Self {
        Self {
            operator_name,
            classification,
            truth_authority,
            required_query_surface,
        }
    }

    pub fn operator_name(&self) -> &'static str {
        self.operator_name
    }

    pub fn classification(&self) -> PlanarBooleanOverlapOperatorClassification {
        self.classification
    }

    pub fn truth_authority(&self) -> PlanarBooleanOverlapOperatorTruthAuthority {
        self.truth_authority
    }

    pub fn required_query_surface(&self) -> PlanarBooleanOverlapRequiredQuerySurface {
        self.required_query_surface
    }
}
