use super::alignment_summary::{
    SpatialPublicCloseoutFreshnessRequirementPosture,
    SpatialPublicCloseoutRenderedOutputComparisonPosture, SpatialPublicCloseoutSeedSupportError,
};
use crate::facade::evidence_lookup_route::{
    current_evidence_lookup_route_source, CurrentEvidenceLookupRouteSource,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialMilestoneFifteenPlannerSeedSupport {
    selected_equivalence_family_identity: String,
    compiled_product_identity_digest: String,
    equivalence_policy_identity_digest: String,
    freshness_requirement_posture: SpatialPublicCloseoutFreshnessRequirementPosture,
    rendered_output_comparison_posture: SpatialPublicCloseoutRenderedOutputComparisonPosture,
    receipt_proof_row_count: usize,
    non_ordinary_residue_row_count: usize,
}

pub fn current_spatial_milestone_fifteen_planner_seed_support(
) -> Result<SpatialMilestoneFifteenPlannerSeedSupport, SpatialPublicCloseoutSeedSupportError> {
    let route_source = current_evidence_lookup_route_source().map_err(|error| {
        SpatialPublicCloseoutSeedSupportError::new(format!(
            "spatial planner seed support requires planner-owned evidence lookup route: {}",
            error.detail()
        ))
    })?;
    Ok(SpatialMilestoneFifteenPlannerSeedSupport::from_current_route_source(&route_source))
}

impl SpatialMilestoneFifteenPlannerSeedSupport {
    fn from_current_route_source(route_source: &CurrentEvidenceLookupRouteSource) -> Self {
        let boundary = route_source.left_boundary();
        let handoff = boundary.workload_handoff();
        let index_product = boundary.index_product();
        Self {
            selected_equivalence_family_identity: index_product
                .selected_equivalence_family_identity()
                .as_str()
                .to_string(),
            compiled_product_identity_digest: index_product
                .compiled_product_identity_digest()
                .to_string(),
            equivalence_policy_identity_digest: index_product
                .equivalence_policy_identity_digest()
                .to_string(),
            freshness_requirement_posture: index_product
                .selected_freshness_requirement_posture()
                .into(),
            rendered_output_comparison_posture: index_product
                .selected_rendered_output_comparison_posture()
                .into(),
            receipt_proof_row_count: handoff.milestone_twelve_seed().receipt_proof_row_count(),
            non_ordinary_residue_row_count: handoff
                .milestone_twelve_seed()
                .non_ordinary_residue_row_count(),
        }
    }

    pub fn selected_equivalence_family_identity(&self) -> &str {
        &self.selected_equivalence_family_identity
    }

    pub fn compiled_product_identity_digest(&self) -> &str {
        &self.compiled_product_identity_digest
    }

    pub fn equivalence_policy_identity_digest(&self) -> &str {
        &self.equivalence_policy_identity_digest
    }

    pub const fn freshness_requirement_posture(
        &self,
    ) -> SpatialPublicCloseoutFreshnessRequirementPosture {
        self.freshness_requirement_posture
    }

    pub const fn rendered_output_comparison_posture(
        &self,
    ) -> SpatialPublicCloseoutRenderedOutputComparisonPosture {
        self.rendered_output_comparison_posture
    }

    pub const fn receipt_proof_row_count(&self) -> usize {
        self.receipt_proof_row_count
    }

    pub const fn non_ordinary_residue_row_count(&self) -> usize {
        self.non_ordinary_residue_row_count
    }
}
