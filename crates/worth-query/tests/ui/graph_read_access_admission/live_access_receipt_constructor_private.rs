use worth_query::facade::runtime::{
    WorthQueryLiveGraphReadAccessReceipt, WorthQueryLiveGraphReadMaintenanceCounters,
};

fn main() {
    let _worthd = WorthQueryLiveGraphReadAccessReceipt {
        digest: String::new(),
        live_access_plan_digest: String::new(),
        one_shot_access_plan_digest: String::new(),
        one_shot_access_shape_digest: String::new(),
        required_index_digest: String::new(),
        maintenance_counters: WorthQueryLiveGraphReadMaintenanceCounters {
            mutation_delta_count: 0,
            affected_requirement_row_count: 0,
            touched_edge_count: 0,
            touched_frontier_count: 0,
            index_update_count: 0,
            live_view_update_count: 0,
            skipped_unaffected_requirement_count: 0,
            strategy_recompute_count: 0,
            background_index_build_count: 0,
        },
    };
}
