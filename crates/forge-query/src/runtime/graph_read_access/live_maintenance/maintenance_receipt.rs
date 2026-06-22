use super::{ForgeQueryLiveGraphReadAccessPlan, ForgeQueryLiveGraphReadMaintenanceCounters};
use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::identity::hash_parts;
use crate::subscription::QuerySubscriptionMaintenanceDelta;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLiveGraphReadMaintenanceReceipt {
    digest: String,
    live_access_plan_digest: String,
    mutation_delta_scope_digest: String,
    maintenance_delta_identity: ForgeQueryEvidenceIdentity,
    maintenance_counters: ForgeQueryLiveGraphReadMaintenanceCounters,
}

impl ForgeQueryLiveGraphReadMaintenanceReceipt {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn live_access_plan_digest(&self) -> &str {
        &self.live_access_plan_digest
    }

    pub fn mutation_delta_scope_digest(&self) -> &str {
        &self.mutation_delta_scope_digest
    }

    pub fn maintenance_delta_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.maintenance_delta_identity
    }

    pub fn maintenance_delta_for_reporting(&self) -> &str {
        self.maintenance_delta_identity.as_str()
    }

    pub fn maintenance_counters(&self) -> &ForgeQueryLiveGraphReadMaintenanceCounters {
        &self.maintenance_counters
    }

    pub(crate) fn from_maintenance_delta(
        plan: &ForgeQueryLiveGraphReadAccessPlan,
        delta: &QuerySubscriptionMaintenanceDelta,
        patch_group_width: usize,
    ) -> Self {
        let maintenance_counters =
            ForgeQueryLiveGraphReadMaintenanceCounters::observed_mutation_delivery(
                plan.mutation_delta_scope().affected_requirement_row_count(),
                delta.width() as usize,
                patch_group_width,
                1,
            );
        let live_access_plan_digest = plan.digest().to_string();
        let mutation_delta_scope_digest =
            plan.mutation_delta_scope().delta_scope_digest().to_string();
        let maintenance_delta_identity = delta.evidence_identity().clone();
        let digest = hash_parts(&[
            "forge_query_live_graph_read_maintenance_receipt_v1".to_string(),
            format!("live_plan:{live_access_plan_digest}"),
            format!("delta_scope:{mutation_delta_scope_digest}"),
            format!("maintenance_delta:{}", maintenance_delta_identity.as_str()),
            format!("counters:{}", maintenance_counters.digest()),
        ]);
        Self {
            digest,
            live_access_plan_digest,
            mutation_delta_scope_digest,
            maintenance_delta_identity,
            maintenance_counters,
        }
    }
}
