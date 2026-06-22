use forge_query::facade::runtime::{
    ForgeQueryLiveGraphReadAccessPlan, ForgeQueryLiveGraphReadAccessPosture,
    ForgeQueryLiveGraphReadMaintenanceBudget,
};

fn main() {
    let _forged = ForgeQueryLiveGraphReadAccessPlan {
        digest: String::new(),
        one_shot_access_plan_digest: String::new(),
        one_shot_access_shape_digest: String::new(),
        required_index_digest: String::new(),
        posture: ForgeQueryLiveGraphReadAccessPosture::AdmittedLiveIncrementalMaintenance,
        maintenance_budget: ForgeQueryLiveGraphReadMaintenanceBudget::bounded(),
        maintenance_equivalence_digest: String::new(),
    };
}
