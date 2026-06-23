use super::super::inventory_lane::{
    WorthGraphReadAccessClassification, WorthGraphReadAccessCostPosture,
    WorthGraphReadAccessDeletionAction, WorthGraphReadAccessInventoryRow,
    WorthGraphReadAccessMilestoneSevenDisposition, WorthGraphReadAccessScopeBinding,
};
use super::row_identity::WorthGraphReadAccessInventoryRowIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessInventoryRowContext {
    identity: WorthGraphReadAccessInventoryRowIdentity,
    classification: WorthGraphReadAccessClassification,
    cost_posture: WorthGraphReadAccessCostPosture,
    deletion_action: WorthGraphReadAccessDeletionAction,
    milestone_seven_disposition: WorthGraphReadAccessMilestoneSevenDisposition,
    scope_binding: WorthGraphReadAccessScopeBinding,
}

impl WorthGraphReadAccessInventoryRowContext {
    pub(crate) fn from_row(row: &WorthGraphReadAccessInventoryRow) -> Self {
        Self {
            identity: WorthGraphReadAccessInventoryRowIdentity::from_row(row),
            classification: row.classification(),
            cost_posture: row.cost_posture(),
            deletion_action: row.deletion_action(),
            milestone_seven_disposition: row.milestone_seven_disposition(),
            scope_binding: row.scope_binding().clone(),
        }
    }

    pub fn identity(&self) -> &WorthGraphReadAccessInventoryRowIdentity {
        &self.identity
    }

    pub const fn classification(&self) -> WorthGraphReadAccessClassification {
        self.classification
    }

    pub const fn cost_posture(&self) -> WorthGraphReadAccessCostPosture {
        self.cost_posture
    }

    pub const fn deletion_action(&self) -> WorthGraphReadAccessDeletionAction {
        self.deletion_action
    }

    pub const fn milestone_seven_disposition(
        &self,
    ) -> WorthGraphReadAccessMilestoneSevenDisposition {
        self.milestone_seven_disposition
    }

    pub fn scope_binding(&self) -> &WorthGraphReadAccessScopeBinding {
        &self.scope_binding
    }
}
