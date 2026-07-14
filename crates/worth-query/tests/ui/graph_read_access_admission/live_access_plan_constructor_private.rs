use worth_query::facade::runtime::{WorthQueryLiveGraphReadAccessPlan, WorthQueryLiveGraphReadAccessPosture, WorthQueryLiveGraphReadMaintenanceBudget};

fn main() {
    let _worthd = WorthQueryLiveGraphReadAccessPlan {
        digest: String::new(),
        one_shot_access_plan_digest: String::new(),
        one_shot_access_shape_digest: String::new(),
        required_index_digest: String::new(),
        posture: WorthQueryLiveGraphReadAccessPosture::AdmittedLiveIncrementalMaintenance,
        maintenance_budget: WorthQueryLiveGraphReadMaintenanceBudget::bounded(),
        maintenance_equivalence_digest: String::new(),
    };
}
