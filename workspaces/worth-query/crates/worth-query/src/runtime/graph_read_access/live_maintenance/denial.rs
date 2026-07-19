use super::{WorthQueryLiveGraphReadAccessPosture, WorthQueryLiveGraphReadMaintenanceBudget};
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryLiveGraphReadAccessDenial {
    digest: String,
    posture: WorthQueryLiveGraphReadAccessPosture,
    one_shot_access_plan_digest: String,
    maintenance_budget_digest: String,
    message: String,
}

impl WorthQueryLiveGraphReadAccessDenial {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn posture(&self) -> &WorthQueryLiveGraphReadAccessPosture {
        &self.posture
    }

    pub fn one_shot_access_plan_digest(&self) -> &str {
        &self.one_shot_access_plan_digest
    }

    pub fn maintenance_budget_digest(&self) -> &str {
        &self.maintenance_budget_digest
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub(crate) fn new(
        posture: WorthQueryLiveGraphReadAccessPosture,
        one_shot_access_plan_digest: impl Into<String>,
        budget: &WorthQueryLiveGraphReadMaintenanceBudget,
        message: impl Into<String>,
    ) -> Self {
        let one_shot_access_plan_digest = one_shot_access_plan_digest.into();
        let message = message.into();
        let maintenance_budget_digest = budget.digest().to_string();
        let digest = hash_parts(&[
            "worth_query_live_graph_read_access_denial_v1".to_string(),
            format!("posture:{}", posture.as_str()),
            format!("one_shot_plan:{one_shot_access_plan_digest}"),
            format!("budget:{maintenance_budget_digest}"),
            format!("message:{message}"),
        ]);
        Self {
            digest,
            posture,
            one_shot_access_plan_digest,
            maintenance_budget_digest,
            message,
        }
    }
}
