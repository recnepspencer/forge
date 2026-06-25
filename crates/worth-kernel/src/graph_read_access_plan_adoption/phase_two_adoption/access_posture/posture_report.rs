use super::super::query_admission::WorthGraphReadAccessPlanAdoptionAttempt;
use super::super::stable_digest;
use super::posture_row::{
    WorthGraphReadAccessPlanAdoptionPostureKind, WorthGraphReadAccessPlanAdoptionPostureRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionPostureReport {
    posture_rows: Vec<WorthGraphReadAccessPlanAdoptionPostureRow>,
    admitted_plan_candidate_count: usize,
    required_or_denied_posture_count: usize,
    missing_query_read_family_artifact_count: usize,
    report_digest: String,
}

impl WorthGraphReadAccessPlanAdoptionPostureReport {
    pub(in crate::graph_read_access_plan_adoption::phase_two_adoption) fn from_adoption_attempts(
        attempts: &[WorthGraphReadAccessPlanAdoptionAttempt],
    ) -> Self {
        let posture_rows = attempts
            .iter()
            .map(WorthGraphReadAccessPlanAdoptionPostureRow::from_adoption_attempt)
            .collect::<Vec<_>>();

        let mut admitted_plan_candidate_count = 0;
        let mut required_or_denied_posture_count = 0;
        let mut missing_query_read_family_artifact_count = 0;
        for row in &posture_rows {
            match row.posture_kind() {
                WorthGraphReadAccessPlanAdoptionPostureKind::AdmittedPlanCandidate
                | WorthGraphReadAccessPlanAdoptionPostureKind::InlineIndexedAdmitted
                | WorthGraphReadAccessPlanAdoptionPostureKind::BoundedEphemeralIndexAdmitted
                | WorthGraphReadAccessPlanAdoptionPostureKind::PagedStreamingAdmitted => {
                    admitted_plan_candidate_count += 1;
                }
                WorthGraphReadAccessPlanAdoptionPostureKind::RequiredSupportPosture
                | WorthGraphReadAccessPlanAdoptionPostureKind::PagedStreamingRequired
                | WorthGraphReadAccessPlanAdoptionPostureKind::PersistentIndexRequired
                | WorthGraphReadAccessPlanAdoptionPostureKind::AsyncMaterializationRequired
                | WorthGraphReadAccessPlanAdoptionPostureKind::StoreBackedCapabilityRequired
                | WorthGraphReadAccessPlanAdoptionPostureKind::AccessCapabilityRegistrationRequired
                | WorthGraphReadAccessPlanAdoptionPostureKind::Denied
                | WorthGraphReadAccessPlanAdoptionPostureKind::CarriedCapabilityGap => {
                    required_or_denied_posture_count += 1;
                }
                WorthGraphReadAccessPlanAdoptionPostureKind::MissingQueryReadFamilyArtifact => {
                    required_or_denied_posture_count += 1;
                    missing_query_read_family_artifact_count += 1;
                }
            }
        }

        let mut digest_parts = vec![
            "worth_graph_read_access_plan_adoption_posture_report_v1".to_string(),
            format!("row_count:{}", posture_rows.len()),
            format!("admitted_count:{admitted_plan_candidate_count}"),
            format!("required_or_denied_count:{required_or_denied_posture_count}"),
            format!(
                "missing_query_read_family_artifact_count:{missing_query_read_family_artifact_count}"
            ),
        ];
        digest_parts.extend(
            posture_rows
                .iter()
                .map(|row| format!("posture_row:{}", row.row_digest())),
        );

        Self {
            posture_rows,
            admitted_plan_candidate_count,
            required_or_denied_posture_count,
            missing_query_read_family_artifact_count,
            report_digest: stable_digest(&digest_parts),
        }
    }

    #[cfg(test)]
    pub(crate) fn from_adoption_attempts_for_tests(
        attempts: &[WorthGraphReadAccessPlanAdoptionAttempt],
    ) -> Self {
        Self::from_adoption_attempts(attempts)
    }

    pub fn posture_rows(&self) -> &[WorthGraphReadAccessPlanAdoptionPostureRow] {
        &self.posture_rows
    }

    pub const fn admitted_plan_candidate_count(&self) -> usize {
        self.admitted_plan_candidate_count
    }

    pub const fn required_or_denied_posture_count(&self) -> usize {
        self.required_or_denied_posture_count
    }

    pub const fn missing_query_read_family_artifact_count(&self) -> usize {
        self.missing_query_read_family_artifact_count
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}
