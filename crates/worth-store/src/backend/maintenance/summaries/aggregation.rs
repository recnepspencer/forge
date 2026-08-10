use crate::backend::records::StoreState;

use super::budget_accumulation::accumulate_reserved_resources;
use super::lane_accumulation::accumulate_lane_facts;
use super::lane_materialization::materialize_lane_and_debt_summaries;
use super::locality_materialization::materialize_locality_summaries;
use super::posture::clear_scheduler_summary_records;
use super::reservation_materialization::materialize_reservation_summaries;
use super::resource_summary_publication::publish_resource_budget_summary;

pub(crate) fn refresh_scheduler_summaries(state: &mut StoreState) {
    let posture = clear_scheduler_summary_records(state);
    if !posture.has_maintenance_state {
        return;
    }

    let lane_facts = accumulate_lane_facts(state);
    let materialized_lanes = materialize_lane_and_debt_summaries(state, lane_facts);
    materialize_locality_summaries(state, &materialized_lanes);
    materialize_reservation_summaries(state, &materialized_lanes);

    let reserved_resources = accumulate_reserved_resources(state);
    publish_resource_budget_summary(state, reserved_resources);
}
