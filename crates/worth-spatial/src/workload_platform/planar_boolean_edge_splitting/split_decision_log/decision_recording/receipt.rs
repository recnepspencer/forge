use super::artifact_decisions::PlanarBooleanSplitArtifactDecisionRows;
use super::counters::PlanarBooleanSplitDecisionLogCounters;
use super::denial::PlanarBooleanSplitDecisionLogDenial;
use super::diagnostic_report::PlanarBooleanStructuredEdgeSplitFailureReport;
use super::identity::receipt_identity;
use super::input::PlanarBooleanSplitDecisionLogInput;
use super::kind::PlanarBooleanSplitDecisionKind as Kind;
use super::localization::PlanarBooleanSplitFailureLocalization;
use super::lookup_index::PlanarBooleanSplitDecisionLookupIndex;
use super::row::PlanarBooleanSplitDecisionRow;
use super::row_recording::{
    push_coverage_rows, push_endpoint_rows, push_fragment_rows, push_interval_rows,
    push_persistent_name_rows, push_phase_stop_rows, push_query_declaration_row, push_vertex_rows,
};
use super::validation::validate_product_lineage;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitDecisionLogReceipt {
    receipt_identity: String,
    query_declaration_identity: String,
    lowered_plan_identity: String,
    split_chain_validation_receipt_identity: String,
    split_persistent_naming_receipt_identity: String,
    decision_rows: Vec<PlanarBooleanSplitDecisionRow>,
    lookup_index: PlanarBooleanSplitDecisionLookupIndex,
    counters: PlanarBooleanSplitDecisionLogCounters,
}

impl PlanarBooleanSplitDecisionLogReceipt {
    pub fn record_decisions(
        input: PlanarBooleanSplitDecisionLogInput<'_>,
    ) -> Result<Self, PlanarBooleanSplitDecisionLogDenial> {
        let mut counters = PlanarBooleanSplitDecisionLogCounters::default();
        validate_product_lineage(&input, &mut counters)?;
        let mut rows = Vec::new();
        push_query_declaration_row(&input, &mut rows, &mut counters);
        push_endpoint_rows(&input, &mut rows, &mut counters);
        push_interval_rows(&input, &mut rows, &mut counters);
        push_vertex_rows(&input, &mut rows, &mut counters);
        push_fragment_rows(&input, &mut rows, &mut counters);
        push_coverage_rows(&input, &mut rows, &mut counters);
        push_persistent_name_rows(&input, &mut rows, &mut counters);
        push_phase_stop_rows(&input, &mut rows, &mut counters);
        rows.sort_by(|left, right| left.decision_identity().cmp(right.decision_identity()));
        let lookup_index = PlanarBooleanSplitDecisionLookupIndex::build(&rows, &mut counters)?;
        let receipt_identity = receipt_identity(input.declaration().declaration_identity(), &rows);
        Ok(Self {
            receipt_identity,
            query_declaration_identity: input.declaration().declaration_identity().to_string(),
            lowered_plan_identity: input.declaration().lowered_plan_identity().to_string(),
            split_chain_validation_receipt_identity: input
                .split_chain_validation()
                .receipt_identity()
                .to_string(),
            split_persistent_naming_receipt_identity: input
                .split_persistent_names()
                .receipt_identity()
                .to_string(),
            decision_rows: rows,
            lookup_index,
            counters,
        })
    }

    pub fn receipt_identity(&self) -> &str {
        &self.receipt_identity
    }
    pub fn query_declaration_identity(&self) -> &str {
        &self.query_declaration_identity
    }
    pub fn lowered_plan_identity(&self) -> &str {
        &self.lowered_plan_identity
    }
    pub fn split_chain_validation_receipt_identity(&self) -> &str {
        &self.split_chain_validation_receipt_identity
    }
    pub fn split_persistent_naming_receipt_identity(&self) -> &str {
        &self.split_persistent_naming_receipt_identity
    }
    pub fn decision_rows(&self) -> &[PlanarBooleanSplitDecisionRow] {
        &self.decision_rows
    }
    pub fn counters(&self) -> PlanarBooleanSplitDecisionLogCounters {
        self.counters
    }

    pub fn decision_by_identity(
        &mut self,
        decision_identity: &str,
    ) -> Option<&PlanarBooleanSplitDecisionRow> {
        let Some(index) = self.lookup_index.decision_index(decision_identity) else {
            self.counters.recorded_lookup_miss();
            return None;
        };
        self.counters.recorded_lookup_hit();
        self.decision_rows.get(index)
    }

    pub fn decisions_for_artifact(
        &mut self,
        artifact_identity: &str,
    ) -> PlanarBooleanSplitArtifactDecisionRows<'_> {
        if !self.lookup_index.has_artifact_identity(artifact_identity) {
            self.counters.recorded_lookup_miss();
            return PlanarBooleanSplitArtifactDecisionRows::new(&self.decision_rows, &[]);
        }
        self.counters.recorded_lookup_hit();
        let indexes = self
            .lookup_index
            .artifact_indexes(artifact_identity)
            .expect("artifact identity was checked before lookup");
        PlanarBooleanSplitArtifactDecisionRows::new(&self.decision_rows, indexes)
    }

    pub fn localize_failure(
        &mut self,
        decision_identity: &str,
    ) -> Option<PlanarBooleanSplitFailureLocalization> {
        let Some(index) = self.lookup_index.decision_index(decision_identity) else {
            self.counters.recorded_lookup_miss();
            return None;
        };
        self.counters.recorded_lookup_hit();
        let row = self
            .decision_rows
            .get(index)
            .expect("decision index must point at a decision row");
        if !is_failure_localizable(row.kind()) {
            self.counters.rejected_non_failure_localization();
            return None;
        }
        Some(PlanarBooleanSplitFailureLocalization::from_row(row))
    }

    pub fn structured_failure_report(
        &mut self,
        localization: &PlanarBooleanSplitFailureLocalization,
    ) -> PlanarBooleanStructuredEdgeSplitFailureReport {
        self.counters.emitted_diagnostic_report();
        PlanarBooleanStructuredEdgeSplitFailureReport::from_localization(localization)
    }

    pub fn certifies_query_native_split_decision_log(&self) -> bool {
        self.counters.decision_rows() == self.decision_rows.len()
            && self.counters.lookup_index_entries() == self.decision_rows.len()
            && self.counters.affected_artifact_index_entries() > 0
            && self.counters.affected_artifact_index_entries() <= self.decision_rows.len()
            && self.counters.duplicate_decision_identities_rejected() == 0
            && self.counters.missing_coverage_rejected() == 0
            && self.counters.foreign_product_denials() == 0
            && !self.query_declaration_identity.is_empty()
            && !self.lowered_plan_identity.is_empty()
    }
}

fn is_failure_localizable(kind: Kind) -> bool {
    matches!(
        kind,
        Kind::MicroIntervalPolicyRequired | Kind::SplitPhaseDenied
    )
}
