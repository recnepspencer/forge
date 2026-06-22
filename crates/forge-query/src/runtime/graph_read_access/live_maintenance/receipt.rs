use super::{ForgeQueryLiveGraphReadAccessPlan, ForgeQueryLiveGraphReadMaintenanceCounters};
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLiveGraphReadAccessReceipt {
    digest: String,
    live_access_plan_digest: String,
    one_shot_access_plan_digest: String,
    one_shot_access_shape_digest: String,
    required_index_digest: String,
    mutation_delta_scope_digest: String,
    maintenance_counters: ForgeQueryLiveGraphReadMaintenanceCounters,
}

impl ForgeQueryLiveGraphReadAccessReceipt {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn live_access_plan_digest(&self) -> &str {
        &self.live_access_plan_digest
    }

    pub fn one_shot_access_plan_digest(&self) -> &str {
        &self.one_shot_access_plan_digest
    }

    pub fn one_shot_access_shape_digest(&self) -> &str {
        &self.one_shot_access_shape_digest
    }

    pub fn required_index_digest(&self) -> &str {
        &self.required_index_digest
    }

    pub fn mutation_delta_scope_digest(&self) -> &str {
        &self.mutation_delta_scope_digest
    }

    pub fn maintenance_counters(&self) -> &ForgeQueryLiveGraphReadMaintenanceCounters {
        &self.maintenance_counters
    }

    pub fn proves_no_caller_owned_n_plus_one(&self) -> bool {
        self.maintenance_counters.per_result_neighbor_lookup_count() == 0
            && self.maintenance_counters.strategy_recompute_count() == 0
            && self.maintenance_counters.background_index_build_count() == 0
    }

    pub(crate) fn from_plan_and_counters(
        plan: &ForgeQueryLiveGraphReadAccessPlan,
        maintenance_counters: ForgeQueryLiveGraphReadMaintenanceCounters,
    ) -> Self {
        let live_access_plan_digest = plan.digest().to_string();
        let one_shot_access_plan_digest = plan.one_shot_access_plan_digest().to_string();
        let one_shot_access_shape_digest = plan.one_shot_access_shape_digest().to_string();
        let required_index_digest = plan.required_index_digest().to_string();
        let mutation_delta_scope_digest =
            plan.mutation_delta_scope().delta_scope_digest().to_string();
        let digest = hash_parts(&[
            "forge_query_live_graph_read_access_receipt_v1".to_string(),
            format!("live_plan:{live_access_plan_digest}"),
            format!("one_shot_plan:{one_shot_access_plan_digest}"),
            format!("shape:{one_shot_access_shape_digest}"),
            format!("indexes:{required_index_digest}"),
            format!("delta_scope:{mutation_delta_scope_digest}"),
            format!("counters:{}", maintenance_counters.digest()),
        ]);
        Self {
            digest,
            live_access_plan_digest,
            one_shot_access_plan_digest,
            one_shot_access_shape_digest,
            required_index_digest,
            mutation_delta_scope_digest,
            maintenance_counters,
        }
    }
}
