use super::alignment_summary::{
    current_spatial_public_closeout_alignment_summary, SpatialPublicCloseoutAlignmentSummary,
    SpatialPublicCloseoutFreshnessRequirementPosture,
    SpatialPublicCloseoutRenderedOutputComparisonPosture, SpatialPublicCloseoutSeedSupportError,
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
    let summary = current_spatial_public_closeout_alignment_summary()?;
    Ok(SpatialMilestoneFifteenPlannerSeedSupport::from_alignment_summary(&summary))
}

impl SpatialMilestoneFifteenPlannerSeedSupport {
    fn from_alignment_summary(summary: &SpatialPublicCloseoutAlignmentSummary) -> Self {
        Self {
            selected_equivalence_family_identity: summary
                .selected_equivalence_family_identity()
                .to_string(),
            compiled_product_identity_digest: summary
                .compiled_product_identity_digest()
                .to_string(),
            equivalence_policy_identity_digest: summary
                .equivalence_policy_identity_digest()
                .to_string(),
            freshness_requirement_posture: summary.freshness_requirement_posture(),
            rendered_output_comparison_posture: summary.rendered_output_comparison_posture(),
            receipt_proof_row_count: summary.receipt_proof_row_count(),
            non_ordinary_residue_row_count: summary.non_ordinary_residue_row_count(),
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
