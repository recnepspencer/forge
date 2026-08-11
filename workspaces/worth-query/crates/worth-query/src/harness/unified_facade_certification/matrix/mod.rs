mod capability_lanes;
mod configuration_lanes;
mod query_context;
mod rejections;
mod rows;

use super::lane::UnifiedFacadeCertificationMatrix;
use super::row_catalog::{UNIFIED_FACADE_CANONICAL_ROW_SPECS, UNIFIED_FACADE_REJECTION_ROW_SPECS};
use capability_lanes::{
    historical_lane, identity_evolution_lane, live_lane, preview_lane, workflow_lane,
};
use configuration_lanes::{
    identity_evolution_support_sync_lane, support_sync_lane, workflow_section_lane,
};
use query_context::query_context_lane;
use rejections::rejection_row;
use rows::canonical_row;

pub struct MilestoneFivePointSixUnifiedFacadeCertificationAdapter;

impl MilestoneFivePointSixUnifiedFacadeCertificationAdapter {
    pub fn unified_facade_and_configuration_boundary_test() -> UnifiedFacadeCertificationMatrix {
        let runtime_query = capability_lanes::query_read_lane();
        let query_context = query_context_lane();
        let identity_evolution = identity_evolution_lane();
        let runtime_live = live_lane();
        let preview = preview_lane();
        let workflow = workflow_lane();
        let historical = historical_lane();
        let config_section = workflow_section_lane();
        let support_sync = support_sync_lane();
        let identity_support_sync = identity_evolution_support_sync_lane();

        UnifiedFacadeCertificationMatrix {
            suite_name: "Unified Facade And Configuration Boundary Test",
            rows: UNIFIED_FACADE_CANONICAL_ROW_SPECS
                .iter()
                .map(|spec| {
                    canonical_row(
                        spec,
                        &runtime_query,
                        &query_context,
                        &identity_evolution,
                        &runtime_live,
                        &preview,
                        &workflow,
                        &historical,
                        &config_section,
                        &support_sync,
                        &identity_support_sync,
                    )
                })
                .collect(),
            rejection_rows: UNIFIED_FACADE_REJECTION_ROW_SPECS
                .iter()
                .map(rejection_row)
                .collect(),
        }
    }
}
