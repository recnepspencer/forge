mod identity_inputs;
mod lanes;
mod rejections;
mod rows;

use super::lane::IdentityEvolutionCertificationMatrix;
use super::row_catalog::{
    IDENTITY_EVOLUTION_CANONICAL_ROW_SPECS, IDENTITY_EVOLUTION_REJECTION_ROW_SPECS,
};
use lanes::{
    ambiguous_comparison_lane, branch_local_lane, branch_to_branch_authoritative_lane,
    current_to_historical_advisory_lane, identity_break_lane, preview_to_authoritative_lane,
    replacement_lane, split_lane,
};
use rows::{canonical_row, rejection_row};

pub struct MilestoneSevenIdentityEvolutionCertificationAdapter;

impl MilestoneSevenIdentityEvolutionCertificationAdapter {
    pub fn lineage_and_correspondence_query_parity_test() -> IdentityEvolutionCertificationMatrix {
        let replacement = replacement_lane();
        let split = split_lane();
        let branch_local = branch_local_lane();
        let ambiguous = ambiguous_comparison_lane();
        let identity_break = identity_break_lane();
        let branch_to_branch = branch_to_branch_authoritative_lane();
        let current_to_historical = current_to_historical_advisory_lane();
        let preview_to_authoritative = preview_to_authoritative_lane();

        IdentityEvolutionCertificationMatrix {
            suite_name: "Lineage And Correspondence Query Parity Test",
            rows: IDENTITY_EVOLUTION_CANONICAL_ROW_SPECS
                .iter()
                .map(|spec| {
                    canonical_row(
                        spec,
                        &replacement,
                        &split,
                        &branch_local,
                        &ambiguous,
                        &identity_break,
                        &branch_to_branch,
                        &current_to_historical,
                        &preview_to_authoritative,
                    )
                })
                .collect(),
            rejection_rows: IDENTITY_EVOLUTION_REJECTION_ROW_SPECS
                .iter()
                .map(rejection_row)
                .collect(),
        }
    }
}
