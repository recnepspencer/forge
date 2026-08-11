use super::super::{classification_error, SupportActionBreadthBudget, SupportActionId};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportMissingSupportMaintenanceAdmission {
    retained_rebuild_basis_digest: Option<String>,
    action_id: SupportActionId,
    breadth_budget: SupportActionBreadthBudget,
    payload_header_bytes: u64,
}

impl SubscriptionSupportMissingSupportMaintenanceAdmission {
    pub fn new(
        action_id: SupportActionId,
        breadth_budget: SupportActionBreadthBudget,
        payload_header_bytes: u64,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            retained_rebuild_basis_digest: None,
            action_id,
            breadth_budget,
            payload_header_bytes,
        })
    }

    pub(super) fn bind_retained_rebuild_basis_digest(
        mut self,
        retained_rebuild_basis_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let retained_rebuild_basis_digest = retained_rebuild_basis_digest.into();
        if retained_rebuild_basis_digest.trim().is_empty() {
            return Err(classification_error(
                "subscription-support missing support recovery requires non-empty retained rebuild basis evidence",
            ));
        }
        self.retained_rebuild_basis_digest = Some(retained_rebuild_basis_digest);
        Ok(self)
    }

    pub(crate) fn retained_rebuild_basis_digest(&self) -> Option<&str> {
        self.retained_rebuild_basis_digest.as_deref()
    }

    pub(crate) fn action_id(&self) -> &SupportActionId {
        &self.action_id
    }

    pub(crate) fn breadth_budget(&self) -> &SupportActionBreadthBudget {
        &self.breadth_budget
    }

    pub(crate) fn payload_header_bytes(&self) -> u64 {
        self.payload_header_bytes
    }
}
