use std::sync::{Arc, Mutex};

use crate::domain_computation::WorthQueryConvergenceAssessment;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FixtureObservedIncumbent {
    occurrence_identity: String,
    state_identity: String,
    report_evidence_identity: String,
}

impl FixtureObservedIncumbent {
    pub(crate) fn occurrence_identity(&self) -> &str {
        &self.occurrence_identity
    }

    pub(crate) fn state_identity(&self) -> &str {
        &self.state_identity
    }

    pub(crate) fn report_evidence_identity(&self) -> &str {
        &self.report_evidence_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FixtureReportHistoryObservation {
    iteration_ordinal: usize,
    provider_receipt_identity: String,
    graph_evidence_identity: String,
    incumbents: Vec<FixtureObservedIncumbent>,
}

impl FixtureReportHistoryObservation {
    pub(crate) const fn iteration_ordinal(&self) -> usize {
        self.iteration_ordinal
    }

    pub(crate) fn provider_receipt_identity(&self) -> &str {
        &self.provider_receipt_identity
    }

    pub(crate) fn graph_evidence_identity(&self) -> &str {
        &self.graph_evidence_identity
    }

    pub(crate) fn incumbents(&self) -> &[FixtureObservedIncumbent] {
        &self.incumbents
    }
}

#[derive(Clone, Default)]
pub(crate) struct FixtureReportHistoryProbe {
    observations: Arc<Mutex<Vec<FixtureReportHistoryObservation>>>,
}

impl FixtureReportHistoryProbe {
    pub(super) fn observe(&self, assessment: &WorthQueryConvergenceAssessment<'_>) {
        let receipt = assessment.receipt();
        let incumbents = assessment
            .incumbents()
            .iter()
            .map(|incumbent| FixtureObservedIncumbent {
                occurrence_identity: incumbent.occurrence_identity().to_owned(),
                state_identity: incumbent.state_identity().to_owned(),
                report_evidence_identity: incumbent.report_evidence_identity().to_owned(),
            })
            .collect();
        self.observations
            .lock()
            .expect("report history probe lock must remain available")
            .push(FixtureReportHistoryObservation {
                iteration_ordinal: assessment.iteration_ordinal(),
                provider_receipt_identity: receipt.provider_receipt().to_owned(),
                graph_evidence_identity: receipt.evidence_identity().to_owned(),
                incumbents,
            });
    }

    pub(crate) fn observations(&self) -> Vec<FixtureReportHistoryObservation> {
        self.observations
            .lock()
            .expect("report history probe lock must remain available")
            .clone()
    }
}
