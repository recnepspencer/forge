use forge_query::facade::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportStatus,
};

use crate::validator_invariant_catalog::WorthTopologyQueryGraphObligationRegistrationProjectionRow;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologySelectedLegalityObligationRow {
    worth_family_identity_digest: String,
    query_rule_identity_digest: String,
    query_obligation_kind: ForgeQueryGraphObligationKind,
    support_lane: ForgeQueryGraphObligationSupportLane,
    support_status: ForgeQueryGraphObligationSupportStatus,
    support_posture_digest: String,
    execution_budget_digest: String,
    registration_digest: String,
    row_digest: String,
}

impl WorthTopologySelectedLegalityObligationRow {
    pub(super) fn from_registration(
        registration: &ForgeQueryGraphObligationRegistration,
        projection_row: &WorthTopologyQueryGraphObligationRegistrationProjectionRow,
    ) -> Self {
        let support_posture = registration.support_posture();
        let execution_budget = registration.execution_budget();
        let row_digest = [
            "worth-topo-selected-legality-obligation-row-v1",
            projection_row.worth_family_identity_digest(),
            registration.rule_identity().identity_digest(),
            registration.kind().as_str(),
            support_posture.lane().as_str(),
            support_posture.status().as_str(),
            support_posture.posture_digest(),
            execution_budget.budget_digest(),
            registration.registration_digest(),
        ]
        .join("|");
        Self {
            worth_family_identity_digest: projection_row.worth_family_identity_digest().to_string(),
            query_rule_identity_digest: registration.rule_identity().identity_digest().to_string(),
            query_obligation_kind: registration.kind(),
            support_lane: support_posture.lane(),
            support_status: support_posture.status(),
            support_posture_digest: support_posture.posture_digest().to_string(),
            execution_budget_digest: execution_budget.budget_digest().to_string(),
            registration_digest: registration.registration_digest().to_string(),
            row_digest,
        }
    }

    pub fn worth_family_identity_digest(&self) -> &str {
        &self.worth_family_identity_digest
    }

    pub fn query_rule_identity_digest(&self) -> &str {
        &self.query_rule_identity_digest
    }

    pub const fn query_obligation_kind(&self) -> ForgeQueryGraphObligationKind {
        self.query_obligation_kind
    }

    pub const fn support_lane(&self) -> ForgeQueryGraphObligationSupportLane {
        self.support_lane
    }

    pub const fn support_status(&self) -> ForgeQueryGraphObligationSupportStatus {
        self.support_status
    }

    pub fn support_posture_digest(&self) -> &str {
        &self.support_posture_digest
    }

    pub fn execution_budget_digest(&self) -> &str {
        &self.execution_budget_digest
    }

    pub fn registration_digest(&self) -> &str {
        &self.registration_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
