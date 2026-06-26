use crate::graph_read_access_declarations::WorthGraphReadAdmissionPostureRecord;

use super::super::proof_digest::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadRequirementRowDigestProjection {
    source_requirement_record_digest: String,
    source_catalog_record_digest: String,
    query_family_digest_seed: String,
    requirement_row_digest: String,
}

impl WorthGraphReadRequirementRowDigestProjection {
    pub(crate) fn from_posture_record(record: &WorthGraphReadAdmissionPostureRecord) -> Self {
        let source_requirement_record_digest =
            record.source_requirement_record_digest().to_string();
        let source_catalog_record_digest = record.source_catalog_record_digest().to_string();
        let query_family_digest_seed = record.query_family_digest_seed().to_string();
        let requirement_row_digest = stable_digest(&[
            "worth_graph_read_requirement_row_digest_projection_v1".to_string(),
            format!("requirement_record:{source_requirement_record_digest}"),
            format!("catalog_record:{source_catalog_record_digest}"),
            format!("query_family:{query_family_digest_seed}"),
        ]);
        Self {
            source_requirement_record_digest,
            source_catalog_record_digest,
            query_family_digest_seed,
            requirement_row_digest,
        }
    }

    pub fn source_requirement_record_digest(&self) -> &str {
        &self.source_requirement_record_digest
    }

    pub fn source_catalog_record_digest(&self) -> &str {
        &self.source_catalog_record_digest
    }

    pub fn query_family_digest_seed(&self) -> &str {
        &self.query_family_digest_seed
    }

    pub fn requirement_row_digest(&self) -> &str {
        &self.requirement_row_digest
    }

    #[cfg(test)]
    pub(crate) fn for_access_plan_adoption_test(
        source_requirement_record_digest: impl Into<String>,
        source_catalog_record_digest: impl Into<String>,
        query_family_digest_seed: impl Into<String>,
    ) -> Self {
        let source_requirement_record_digest = source_requirement_record_digest.into();
        let source_catalog_record_digest = source_catalog_record_digest.into();
        let query_family_digest_seed = query_family_digest_seed.into();
        let requirement_row_digest = stable_digest(&[
            "worth_graph_read_requirement_row_digest_projection_v1".to_string(),
            format!("requirement_record:{source_requirement_record_digest}"),
            format!("catalog_record:{source_catalog_record_digest}"),
            format!("query_family:{query_family_digest_seed}"),
        ]);
        Self {
            source_requirement_record_digest,
            source_catalog_record_digest,
            query_family_digest_seed,
            requirement_row_digest,
        }
    }
}
