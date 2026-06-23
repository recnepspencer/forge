use super::counters::PlanarBooleanLoopDecisionLogCounters;
use super::denial::PlanarBooleanLoopDecisionLogDenial;
use super::diagnostics::{
    PlanarBooleanLoopDecisionLookupIndex, PlanarBooleanLoopFailureLocalization,
    PlanarBooleanStructuredLoopReconstructionFailureReport,
};
use super::identity::decision_log_identity;
use super::input::PlanarBooleanLoopDecisionLogInput;
use super::row::PlanarBooleanLoopDecisionRow;
use super::row_recording::record_rows;
use super::validation::validate_input;
use super::vocabulary::PlanarBooleanLoopDecisionKind as KindRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopDecisionLog {
    decision_log_identity: String,
    request_identity: String,
    split_ledger_receipt_identity: String,
    rows: Vec<PlanarBooleanLoopDecisionRow>,
    lookup_index: PlanarBooleanLoopDecisionLookupIndex,
    counters: PlanarBooleanLoopDecisionLogCounters,
}

impl PlanarBooleanLoopDecisionLog {
    pub fn record(
        input: PlanarBooleanLoopDecisionLogInput<'_>,
    ) -> Result<Self, PlanarBooleanLoopDecisionLogDenial> {
        let mut counters = PlanarBooleanLoopDecisionLogCounters::default();
        validate_input(input, &mut counters)?;
        let mut rows = record_rows(input, &mut counters)?;
        rows.sort_by(|left, right| left.decision_identity().cmp(right.decision_identity()));
        let lookup_index = PlanarBooleanLoopDecisionLookupIndex::build(&rows, &mut counters);
        let decision_log_identity = decision_log_identity(
            input.request().request_identity(),
            input.request().split_ledger_receipt_identity(),
            &rows,
        );
        Ok(Self {
            decision_log_identity,
            request_identity: input.request().request_identity().to_string(),
            split_ledger_receipt_identity: input
                .request()
                .split_ledger_receipt_identity()
                .to_string(),
            rows,
            lookup_index,
            counters,
        })
    }

    pub fn decision_log_identity(&self) -> &str {
        &self.decision_log_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn split_ledger_receipt_identity(&self) -> &str {
        &self.split_ledger_receipt_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanLoopDecisionRow] {
        &self.rows
    }

    pub fn counters(&self) -> PlanarBooleanLoopDecisionLogCounters {
        self.counters
    }

    pub fn decision_by_identity(
        &self,
        decision_identity: &str,
    ) -> Option<&PlanarBooleanLoopDecisionRow> {
        self.lookup_index
            .decision_index(decision_identity)
            .and_then(|index| self.rows.get(index))
    }

    pub fn decisions_for_artifact(
        &self,
        artifact_identity: &str,
    ) -> Vec<&PlanarBooleanLoopDecisionRow> {
        self.lookup_index
            .artifact_indexes(artifact_identity)
            .iter()
            .filter_map(|index| self.rows.get(*index))
            .collect()
    }

    pub fn localize_failure(
        &self,
        decision_identity: &str,
    ) -> Option<PlanarBooleanLoopFailureLocalization> {
        let row = self.decision_by_identity(decision_identity)?;
        if !matches!(row.kind(), KindRow::Denied | KindRow::PolicyRequired) {
            return None;
        }
        Some(PlanarBooleanLoopFailureLocalization::from_row(row))
    }

    pub fn structured_failure_report(
        &self,
        localization: &PlanarBooleanLoopFailureLocalization,
    ) -> PlanarBooleanStructuredLoopReconstructionFailureReport {
        let related_decision_identities = self
            .decisions_for_artifact(localization.affected_artifact_identity())
            .into_iter()
            .map(|row| row.decision_identity().to_string())
            .collect();
        PlanarBooleanStructuredLoopReconstructionFailureReport::from_localization(
            localization.clone(),
            related_decision_identities,
        )
    }

    #[cfg(test)]
    pub(crate) fn with_split_ledger_receipt_identity_for_tests(
        &self,
        split_ledger_receipt_identity: impl Into<String>,
    ) -> Self {
        let mut cloned = self.clone();
        cloned.split_ledger_receipt_identity = split_ledger_receipt_identity.into();
        cloned
    }
}
