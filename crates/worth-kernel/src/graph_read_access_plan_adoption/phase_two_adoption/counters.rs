use super::access_posture::WorthGraphReadAccessPlanAdoptionPostureReport;
use super::query_admission::WorthGraphReadAccessPlanAdoptionAttemptKind;
use super::read_family_adoption::WorthGraphReadAccessPlanAdoptionLedger;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionPhaseTwoCounters {
    structured_pairing_count: usize,
    adoption_attempt_count: usize,
    query_admission_inspected_count: usize,
    admitted_plan_count: usize,
    required_or_denied_posture_count: usize,
    missing_query_read_family_artifact_count: usize,
    carried_capability_gap_count: usize,
    duplicate_pairing_count: usize,
}

impl WorthGraphReadAccessPlanAdoptionPhaseTwoCounters {
    pub(crate) fn from_products(
        ledger: &WorthGraphReadAccessPlanAdoptionLedger,
        posture_report: &WorthGraphReadAccessPlanAdoptionPostureReport,
    ) -> Self {
        let mut query_admission_inspected_count = 0;

        for attempt in ledger.attempts() {
            match attempt.kind() {
                WorthGraphReadAccessPlanAdoptionAttemptKind::QueryAdmissionInspected => {
                    query_admission_inspected_count += 1;
                }
                WorthGraphReadAccessPlanAdoptionAttemptKind::AdmittedPlanCandidate
                | WorthGraphReadAccessPlanAdoptionAttemptKind::RequiredOrDeniedPosture
                | WorthGraphReadAccessPlanAdoptionAttemptKind::MissingQueryReadFamilyArtifact => {}
            }
        }

        Self {
            structured_pairing_count: ledger.pairings().len(),
            adoption_attempt_count: ledger.attempts().len(),
            query_admission_inspected_count,
            admitted_plan_count: posture_report.admitted_plan_candidate_count(),
            required_or_denied_posture_count: posture_report.required_or_denied_posture_count(),
            missing_query_read_family_artifact_count: posture_report
                .missing_query_read_family_artifact_count(),
            carried_capability_gap_count: ledger.carried_capability_gap_count(),
            duplicate_pairing_count: ledger.duplicate_pairing_count(),
        }
    }

    pub const fn structured_pairing_count(&self) -> usize {
        self.structured_pairing_count
    }

    pub const fn adoption_attempt_count(&self) -> usize {
        self.adoption_attempt_count
    }

    pub const fn query_admission_inspected_count(&self) -> usize {
        self.query_admission_inspected_count
    }

    pub const fn admitted_plan_count(&self) -> usize {
        self.admitted_plan_count
    }

    pub const fn required_or_denied_posture_count(&self) -> usize {
        self.required_or_denied_posture_count
    }

    pub const fn missing_query_read_family_artifact_count(&self) -> usize {
        self.missing_query_read_family_artifact_count
    }

    pub const fn carried_capability_gap_count(&self) -> usize {
        self.carried_capability_gap_count
    }

    pub const fn duplicate_pairing_count(&self) -> usize {
        self.duplicate_pairing_count
    }
}
