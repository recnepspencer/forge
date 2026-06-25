use forge_query::facade::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportStatus,
};

use crate::validator_invariant_catalog::WorthTopologyLegalityFamilyRecord;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyQueryGraphObligationRegistrationProjectionRow {
    worth_family_identity_digest: String,
    query_rule_identity_digest: String,
    query_obligation_kind: ForgeQueryGraphObligationKind,
    support_lane: ForgeQueryGraphObligationSupportLane,
    support_status: ForgeQueryGraphObligationSupportStatus,
    support_posture_digest: String,
    operating_world_selector: String,
    operating_world_selector_digest: String,
    touch_selector_digest: String,
    registration_digest: String,
    row_digest: String,
}

impl WorthTopologyQueryGraphObligationRegistrationProjectionRow {
    pub(in crate::validator_invariant_catalog) fn from_registration(
        family: &WorthTopologyLegalityFamilyRecord,
        registration: &ForgeQueryGraphObligationRegistration,
    ) -> Self {
        let support_posture = registration.support_posture();
        let operating_world_selector = registration.operating_world_selector();
        let operating_world_selector_digest = operating_world_selector
            .selector_digest()
            .terminal_projection_for_reporting()
            .to_string();
        let row_digest = [
            "worth-topo-query-registration-projection-row-v1",
            family.identity().identity_digest(),
            registration.rule_identity().identity_digest(),
            registration.kind().as_str(),
            support_posture.lane().as_str(),
            support_posture.status().as_str(),
            support_posture.posture_digest(),
            operating_world_selector.as_str(),
            operating_world_selector_digest.as_str(),
            registration.touch_selector().selector_digest(),
            registration.registration_digest(),
        ]
        .join("|");
        Self {
            worth_family_identity_digest: family.identity().identity_digest().to_string(),
            query_rule_identity_digest: registration.rule_identity().identity_digest().to_string(),
            query_obligation_kind: registration.kind(),
            support_lane: support_posture.lane(),
            support_status: support_posture.status(),
            support_posture_digest: support_posture.posture_digest().to_string(),
            operating_world_selector: operating_world_selector.as_str().to_string(),
            operating_world_selector_digest,
            touch_selector_digest: registration.touch_selector().selector_digest().to_string(),
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

    pub fn operating_world_selector(&self) -> &str {
        &self.operating_world_selector
    }

    pub fn operating_world_selector_digest(&self) -> &str {
        &self.operating_world_selector_digest
    }

    pub fn touch_selector_digest(&self) -> &str {
        &self.touch_selector_digest
    }

    pub fn registration_digest(&self) -> &str {
        &self.registration_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
