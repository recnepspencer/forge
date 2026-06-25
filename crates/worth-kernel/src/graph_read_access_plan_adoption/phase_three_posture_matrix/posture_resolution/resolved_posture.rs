#[cfg(test)]
use crate::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionPostureKind;

use super::super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessResolvedPosture {
    requirement_identity: String,
    posture_family: String,
    source_attempt_digest: Option<String>,
    source_carried_gap_digest: Option<String>,
    source_pairing_digest: Option<String>,
    source_requirement_record_digest: String,
    read_family_identity_digest: Option<String>,
    requirement_row_digest: Option<String>,
    query_family_name: Option<String>,
    query_family_digest_seed: String,
    read_family_target: Option<String>,
    query_posture: String,
    denial_kind: Option<String>,
    owner: Option<&'static str>,
    expected_denial: Option<String>,
    suggested_posture: Option<String>,
    blocker: Option<String>,
    removal_trigger: Option<String>,
    row_digest: String,
}

pub(crate) struct WorthGraphReadAccessResolvedPostureInput {
    pub requirement_identity: String,
    pub posture_family: String,
    pub source_attempt_digest: Option<String>,
    pub source_carried_gap_digest: Option<String>,
    pub source_pairing_digest: Option<String>,
    pub source_requirement_record_digest: String,
    pub read_family_identity_digest: Option<String>,
    pub requirement_row_digest: Option<String>,
    pub query_family_name: Option<String>,
    pub query_family_digest_seed: String,
    pub read_family_target: Option<String>,
    pub query_posture: String,
    pub denial_kind: Option<String>,
    pub owner: Option<&'static str>,
    pub expected_denial: Option<String>,
    pub suggested_posture: Option<String>,
    pub blocker: Option<String>,
    pub removal_trigger: Option<String>,
}

impl WorthGraphReadAccessResolvedPosture {
    pub(crate) fn from_input(input: WorthGraphReadAccessResolvedPostureInput) -> Self {
        let row_digest = stable_digest(&[
            "worth_graph_read_access_resolved_posture_v1".to_string(),
            format!("requirement_identity:{}", input.requirement_identity),
            format!("posture_family:{}", input.posture_family),
            format!(
                "attempt:{}",
                input.source_attempt_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "carried_gap:{}",
                input.source_carried_gap_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "pairing:{}",
                input.source_pairing_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "requirement_record:{}",
                input.source_requirement_record_digest
            ),
            format!(
                "read_family:{}",
                input
                    .read_family_identity_digest
                    .as_deref()
                    .unwrap_or("none")
            ),
            format!(
                "requirement_row:{}",
                input.requirement_row_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "query_family_name:{}",
                input.query_family_name.as_deref().unwrap_or("none")
            ),
            format!("query_family_seed:{}", input.query_family_digest_seed),
            format!(
                "read_family_target:{}",
                input.read_family_target.as_deref().unwrap_or("none")
            ),
            format!("query_posture:{}", input.query_posture),
            format!("denial:{}", input.denial_kind.as_deref().unwrap_or("none")),
            format!("owner:{}", input.owner.unwrap_or("none")),
            format!(
                "expected_denial:{}",
                input.expected_denial.as_deref().unwrap_or("none")
            ),
            format!(
                "suggested_posture:{}",
                input.suggested_posture.as_deref().unwrap_or("none")
            ),
            format!("blocker:{}", input.blocker.as_deref().unwrap_or("none")),
            format!(
                "removal_trigger:{}",
                input.removal_trigger.as_deref().unwrap_or("none")
            ),
        ]);
        Self {
            requirement_identity: input.requirement_identity,
            posture_family: input.posture_family,
            source_attempt_digest: input.source_attempt_digest,
            source_carried_gap_digest: input.source_carried_gap_digest,
            source_pairing_digest: input.source_pairing_digest,
            source_requirement_record_digest: input.source_requirement_record_digest,
            read_family_identity_digest: input.read_family_identity_digest,
            requirement_row_digest: input.requirement_row_digest,
            query_family_name: input.query_family_name,
            query_family_digest_seed: input.query_family_digest_seed,
            read_family_target: input.read_family_target,
            query_posture: input.query_posture,
            denial_kind: input.denial_kind,
            owner: input.owner,
            expected_denial: input.expected_denial,
            suggested_posture: input.suggested_posture,
            blocker: input.blocker,
            removal_trigger: input.removal_trigger,
            row_digest,
        }
    }

    #[cfg(test)]
    pub(crate) fn for_tests(
        requirement_identity: &str,
        posture_kind: WorthGraphReadAccessPlanAdoptionPostureKind,
    ) -> Self {
        Self::from_input(WorthGraphReadAccessResolvedPostureInput {
            requirement_identity: requirement_identity.to_string(),
            posture_family: posture_kind.as_str().to_string(),
            source_attempt_digest: Some(format!("attempt:{requirement_identity}")),
            source_carried_gap_digest: None,
            source_pairing_digest: Some(format!("pairing:{requirement_identity}")),
            source_requirement_record_digest: format!("record:{requirement_identity}"),
            read_family_identity_digest: Some(format!("read_family:{requirement_identity}")),
            requirement_row_digest: Some(requirement_identity.to_string()),
            query_family_name: Some("test_family".to_string()),
            query_family_digest_seed: format!("query:{requirement_identity}"),
            read_family_target: Some(requirement_identity.to_string()),
            query_posture: posture_kind.as_str().to_string(),
            denial_kind: None,
            owner: None,
            expected_denial: None,
            suggested_posture: None,
            blocker: None,
            removal_trigger: None,
        })
    }

    pub fn requirement_identity(&self) -> &str {
        &self.requirement_identity
    }

    pub fn posture_family(&self) -> &str {
        &self.posture_family
    }

    pub fn source_attempt_digest(&self) -> Option<&str> {
        self.source_attempt_digest.as_deref()
    }

    pub fn source_carried_gap_digest(&self) -> Option<&str> {
        self.source_carried_gap_digest.as_deref()
    }

    pub fn source_pairing_digest(&self) -> Option<&str> {
        self.source_pairing_digest.as_deref()
    }

    pub fn source_requirement_record_digest(&self) -> &str {
        &self.source_requirement_record_digest
    }

    pub fn read_family_identity_digest(&self) -> Option<&str> {
        self.read_family_identity_digest.as_deref()
    }

    pub fn requirement_row_digest(&self) -> Option<&str> {
        self.requirement_row_digest.as_deref()
    }

    pub fn query_family_name(&self) -> Option<&str> {
        self.query_family_name.as_deref()
    }

    pub fn query_family_digest_seed(&self) -> &str {
        &self.query_family_digest_seed
    }

    pub fn read_family_target(&self) -> Option<&str> {
        self.read_family_target.as_deref()
    }

    pub fn query_posture(&self) -> &str {
        &self.query_posture
    }

    pub fn denial_kind(&self) -> Option<&str> {
        self.denial_kind.as_deref()
    }

    pub const fn owner(&self) -> Option<&'static str> {
        self.owner
    }

    pub fn expected_denial(&self) -> Option<&str> {
        self.expected_denial.as_deref()
    }

    pub fn suggested_posture(&self) -> Option<&str> {
        self.suggested_posture.as_deref()
    }

    pub fn blocker(&self) -> Option<&str> {
        self.blocker.as_deref()
    }

    pub fn removal_trigger(&self) -> Option<&str> {
        self.removal_trigger.as_deref()
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        false
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        false
    }

    pub const fn claims_graph_read_receipt(&self) -> bool {
        false
    }
}
