use crate::graph_read_access_declarations::WorthGraphReadAdmissionPostureRecord;

use super::super::proof_digest::stable_digest;
use super::requirement_row_digest::WorthGraphReadRequirementRowDigestProjection;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthGraphReadRequirementEvidenceSummary {
    requirement_row_evidence: Vec<WorthGraphReadRequirementRowDigestProjection>,
    requirement_summary_digest: String,
}

impl WorthGraphReadRequirementEvidenceSummary {
    pub(crate) fn from_posture_records(records: &[WorthGraphReadAdmissionPostureRecord]) -> Self {
        let requirement_row_evidence = records
            .iter()
            .map(WorthGraphReadRequirementRowDigestProjection::from_posture_record)
            .collect::<Vec<_>>();
        let requirement_summary_digest = stable_digest(
            &requirement_row_evidence
                .iter()
                .map(|row| format!("requirement_row:{}", row.requirement_row_digest()))
                .collect::<Vec<_>>(),
        );
        Self {
            requirement_row_evidence,
            requirement_summary_digest,
        }
    }

    pub(crate) fn requirement_row_evidence(
        &self,
    ) -> &[WorthGraphReadRequirementRowDigestProjection] {
        &self.requirement_row_evidence
    }

    pub(crate) fn requirement_summary_digest(&self) -> &str {
        &self.requirement_summary_digest
    }
}
