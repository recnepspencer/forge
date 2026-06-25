use crate::graph_read_access_declarations::WorthGraphReadAdmissionCapabilityGap;

use super::super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionCarriedGapRow {
    source_gap_digest: String,
    source_requirement_record_digest: String,
    query_family_anchor_digest: String,
    read_family_target: String,
    gap_kind: &'static str,
    owner: &'static str,
    expected_denial: String,
    suggested_posture: String,
    blocker: String,
    removal_trigger: String,
    row_digest: String,
}

impl WorthGraphReadAccessPlanAdoptionCarriedGapRow {
    pub(crate) fn from_admission_gap(gap: &WorthGraphReadAdmissionCapabilityGap) -> Self {
        let expected_denial = gap.expected_denial().digest_part();
        let suggested_posture = gap.suggested_posture().digest_part();
        let row_digest = stable_digest(&[
            "worth_graph_read_access_plan_adoption_carried_gap_row_v1".to_string(),
            format!("source_gap:{}", gap.gap_digest()),
            format!(
                "requirement_record:{}",
                gap.source_requirement_record_digest()
            ),
            format!("query_family:{}", gap.query_family_anchor_digest()),
            format!("read_family_target:{}", gap.read_family_target()),
            format!("kind:{}", gap.kind().as_str()),
            format!("owner:{}", gap.owner()),
            format!("expected_denial:{expected_denial}"),
            format!("suggested_posture:{suggested_posture}"),
            format!("blocker:{}", gap.blocker()),
            format!("removal_trigger:{}", gap.removal_trigger()),
        ]);
        Self {
            source_gap_digest: gap.gap_digest().to_string(),
            source_requirement_record_digest: gap.source_requirement_record_digest().to_string(),
            query_family_anchor_digest: gap.query_family_anchor_digest().to_string(),
            read_family_target: gap.read_family_target().to_string(),
            gap_kind: gap.kind().as_str(),
            owner: gap.owner(),
            expected_denial,
            suggested_posture,
            blocker: gap.blocker().to_string(),
            removal_trigger: gap.removal_trigger().to_string(),
            row_digest,
        }
    }

    #[cfg(test)]
    pub(crate) fn for_posture_matrix_test(requirement_identity: &str) -> Self {
        let source_gap_digest = format!("gap:{requirement_identity}");
        let source_requirement_record_digest = format!("requirement_record:{requirement_identity}");
        let query_family_anchor_digest = format!("query_family_seed:{requirement_identity}");
        let read_family_target = requirement_identity.to_string();
        let gap_kind = "missing_query_support";
        let owner = "forge-query";
        let expected_denial = "required_access_capability_registration".to_string();
        let suggested_posture = "access_capability_registration_required".to_string();
        let blocker = "Register the missing query graph-read access capability.".to_string();
        let removal_trigger =
            "Delete this carried gap after Query capability registration lands.".to_string();
        let row_digest = stable_digest(&[
            "worth_graph_read_access_plan_adoption_carried_gap_row_v1".to_string(),
            format!("source_gap:{source_gap_digest}"),
            format!("requirement_record:{source_requirement_record_digest}"),
            format!("query_family:{query_family_anchor_digest}"),
            format!("read_family_target:{read_family_target}"),
            format!("kind:{gap_kind}"),
            format!("owner:{owner}"),
            format!("expected_denial:{expected_denial}"),
            format!("suggested_posture:{suggested_posture}"),
            format!("blocker:{blocker}"),
            format!("removal_trigger:{removal_trigger}"),
        ]);
        Self {
            source_gap_digest,
            source_requirement_record_digest,
            query_family_anchor_digest,
            read_family_target,
            gap_kind,
            owner,
            expected_denial,
            suggested_posture,
            blocker,
            removal_trigger,
            row_digest,
        }
    }

    pub fn source_gap_digest(&self) -> &str {
        &self.source_gap_digest
    }

    pub fn source_requirement_record_digest(&self) -> &str {
        &self.source_requirement_record_digest
    }

    pub fn query_family_anchor_digest(&self) -> &str {
        &self.query_family_anchor_digest
    }

    pub fn read_family_target(&self) -> &str {
        &self.read_family_target
    }

    pub const fn gap_kind(&self) -> &'static str {
        self.gap_kind
    }

    pub const fn owner(&self) -> &'static str {
        self.owner
    }

    pub fn expected_denial(&self) -> &str {
        &self.expected_denial
    }

    pub fn suggested_posture(&self) -> &str {
        &self.suggested_posture
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
