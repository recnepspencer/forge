use super::posture_outcome::WorthGraphReadAccessAdmissionPostureOutcome;
use super::query_admission_projection::admission_outcome_for_requirement_record;
use super::stable_identity_digest::stable_digest;
use crate::graph_read_access_declarations::WorthGraphReadRequirementDerivationRecord;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAdmissionPostureRecord {
    source_requirement_record_digest: String,
    source_catalog_record_digest: String,
    query_family_name: String,
    query_family_digest_seed: String,
    touched_authority_input: String,
    read_family_target: String,
    posture_outcome: WorthGraphReadAccessAdmissionPostureOutcome,
    record_digest: String,
}

impl WorthGraphReadAdmissionPostureRecord {
    pub(crate) fn from_requirement_record(
        record: &WorthGraphReadRequirementDerivationRecord,
    ) -> Self {
        let posture_outcome = admission_outcome_for_requirement_record(record);
        let record_digest = stable_digest(&[
            "worth_graph_read_admission_posture_record_v1".to_string(),
            format!("requirement_record:{}", record.record_digest()),
            format!("catalog_record:{}", record.source_catalog_record_digest()),
            format!("query_family:{}", record.query_family_digest_seed()),
            format!("outcome:{}", posture_outcome.digest_part()),
        ]);
        Self {
            source_requirement_record_digest: record.record_digest().to_string(),
            source_catalog_record_digest: record.source_catalog_record_digest().to_string(),
            query_family_name: record.query_family_name().to_string(),
            query_family_digest_seed: record.query_family_digest_seed().to_string(),
            touched_authority_input: record.touched_authority_input().to_string(),
            read_family_target: record.read_family_target().to_string(),
            posture_outcome,
            record_digest,
        }
    }

    pub fn source_requirement_record_digest(&self) -> &str {
        &self.source_requirement_record_digest
    }

    pub fn source_catalog_record_digest(&self) -> &str {
        &self.source_catalog_record_digest
    }

    pub fn query_family_name(&self) -> &str {
        &self.query_family_name
    }

    pub fn query_family_digest_seed(&self) -> &str {
        &self.query_family_digest_seed
    }

    pub fn touched_authority_input(&self) -> &str {
        &self.touched_authority_input
    }

    pub fn read_family_target(&self) -> &str {
        &self.read_family_target
    }

    pub fn posture_outcome(&self) -> &WorthGraphReadAccessAdmissionPostureOutcome {
        &self.posture_outcome
    }

    pub fn record_digest(&self) -> &str {
        &self.record_digest
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        false
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        false
    }
}
