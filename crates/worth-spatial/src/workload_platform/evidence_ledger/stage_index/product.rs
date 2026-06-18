use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, WorkloadEvidenceLedgerError, WorkloadEvidenceRow,
    WorkloadEvidenceStage, WorkloadEvidenceStageLinkSet,
};

use super::super::stage_links::link_required_stages;
use super::counters::WorkloadEvidenceStageIndexCounters;
use super::identity::stage_index_identity;
use super::receipt_match::match_boolean_receipt;
use super::validation::{build_stage_offsets, stage_index_counters};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadEvidenceStageIndexProduct {
    rows: Vec<WorkloadEvidenceRow>,
    stage_offsets: [Option<usize>; WorkloadEvidenceStage::STAGE_COUNT],
    counters: WorkloadEvidenceStageIndexCounters,
    index_identity: String,
}

impl WorkloadEvidenceStageIndexProduct {
    pub(crate) fn new(rows: Vec<WorkloadEvidenceRow>) -> Result<Self, WorkloadEvidenceLedgerError> {
        if rows.is_empty() {
            return Err(WorkloadEvidenceLedgerError::EmptyLedger);
        }
        if rows
            .iter()
            .any(|row| row.evidence_identity().trim().is_empty())
        {
            return Err(WorkloadEvidenceLedgerError::MissingEvidenceIdentity);
        }
        let stage_offsets = build_stage_offsets(&rows)?;
        let counters = stage_index_counters(&rows, &stage_offsets);
        let index_identity = stage_index_identity(&rows, counters);
        Ok(Self {
            rows,
            stage_offsets,
            counters,
            index_identity,
        })
    }

    pub fn index_identity(&self) -> &str {
        &self.index_identity
    }

    pub fn rows(&self) -> &[WorkloadEvidenceRow] {
        &self.rows
    }

    pub fn counters(&self) -> WorkloadEvidenceStageIndexCounters {
        self.counters
    }

    pub fn row_for_stage(&self, stage: WorkloadEvidenceStage) -> Option<&WorkloadEvidenceRow> {
        self.stage_offsets
            .get(stage.index_slot())
            .and_then(|offset| offset.map(|row_index| &self.rows[row_index]))
    }

    pub fn evidence_for_stage(&self, stage: WorkloadEvidenceStage) -> Option<&str> {
        self.row_for_stage(stage)
            .map(WorkloadEvidenceRow::evidence_identity)
    }

    pub fn missing_authority_stage(&self) -> Option<WorkloadEvidenceStage> {
        WorkloadEvidenceStage::AUTHORITY_STAGES
            .iter()
            .copied()
            .find(|stage| self.row_for_stage(*stage).is_none())
    }

    pub fn first_manual_authority_stage(&self) -> Option<WorkloadEvidenceStage> {
        WorkloadEvidenceStage::AUTHORITY_STAGES
            .iter()
            .copied()
            .find(|stage| {
                self.row_for_stage(*stage)
                    .is_some_and(|row| !row.is_receipt_backed())
            })
    }

    pub fn first_unadmitted_authority_stage(&self) -> Option<WorkloadEvidenceStage> {
        WorkloadEvidenceStage::AUTHORITY_STAGES
            .iter()
            .copied()
            .find(|stage| {
                self.row_for_stage(*stage)
                    .is_some_and(|row| row.is_receipt_backed() && !row.is_admitted())
            })
    }

    pub fn require_boolean_receipt<T: BooleanEvidenceReceipt + 'static>(
        &self,
        receipt: &T,
    ) -> Result<(), WorkloadEvidenceLedgerError> {
        match_boolean_receipt(self, receipt)
    }

    pub fn link_required_stages(
        &self,
        stages: &[WorkloadEvidenceStage],
    ) -> Result<WorkloadEvidenceStageLinkSet, WorkloadEvidenceLedgerError> {
        link_required_stages(self, stages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workload_platform::evidence_ledger::{
        BooleanEvidenceRowAuthority, BooleanEvidenceStageKind, WorkloadEvidenceStageBinding,
        WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
    };

    #[test]
    fn stage_index_rejects_duplicate_stages_before_ledger_construction() {
        let denial = WorkloadEvidenceStageIndexProduct::new(vec![
            WorkloadEvidenceRow::new(WorkloadEvidenceStage::Topology, "topology-a"),
            WorkloadEvidenceRow::new(WorkloadEvidenceStage::Topology, "topology-b"),
        ])
        .expect_err("stage index construction must deny duplicate stage rows");

        assert_eq!(
            denial,
            WorkloadEvidenceLedgerError::DuplicateEvidenceStage(WorkloadEvidenceStage::Topology)
        );
    }

    #[test]
    fn stage_index_counts_lookup_postures_without_runtime_scans() {
        let product = WorkloadEvidenceStageIndexProduct::new(vec![
            WorkloadEvidenceRow::receipt_backed(
                WorkloadEvidenceStage::Topology,
                "topology",
                WorkloadEvidenceStageCounters::topology(2, 1, 1),
            ),
            WorkloadEvidenceRow::new(WorkloadEvidenceStage::BooleanSplit, "manual split"),
            WorkloadEvidenceRow::receipt_backed_with_support(
                WorkloadEvidenceStage::SurfaceSupport,
                "unsupported support",
                WorkloadEvidenceSupport::Unsupported,
                WorkloadEvidenceStageCounters::surface_support(1),
            ),
        ])
        .expect("distinct stage rows should index");

        let counters = product.counters();
        assert_eq!(counters.row_count(), 3);
        assert_eq!(counters.indexed_stage_count(), 3);
        assert_eq!(counters.manual_row_count(), 1);
        assert_eq!(counters.unadmitted_row_count(), 1);
        assert_eq!(counters.boolean_row_count(), 1);
        assert_eq!(counters.counterless_boolean_row_count(), 1);
        assert_eq!(
            product.evidence_for_stage(WorkloadEvidenceStage::Topology),
            Some("topology")
        );
    }

    #[test]
    fn stage_index_rejects_duplicate_required_stage_links() {
        let product =
            WorkloadEvidenceStageIndexProduct::new(vec![WorkloadEvidenceRow::receipt_backed(
                WorkloadEvidenceStage::Projection,
                "projection",
                WorkloadEvidenceStageCounters::projection(1, 1),
            )])
            .expect("single projection row should index");

        let denial = product
            .link_required_stages(&[
                WorkloadEvidenceStage::Projection,
                WorkloadEvidenceStage::Projection,
            ])
            .expect_err("required stage links must not accept duplicate stage inputs");

        assert_eq!(
            denial,
            WorkloadEvidenceLedgerError::DuplicateEvidenceStage(WorkloadEvidenceStage::Projection)
        );
    }

    #[test]
    fn stage_index_rejects_foreign_upstream_stage_binding_links() {
        let product = WorkloadEvidenceStageIndexProduct::new(vec![
            WorkloadEvidenceRow::receipt_backed(
                WorkloadEvidenceStage::Projection,
                "projection-a",
                WorkloadEvidenceStageCounters::projection(1, 1),
            ),
            WorkloadEvidenceRow::receipt_backed_with_stage_binding(
                WorkloadEvidenceStage::Transform,
                "transform-b",
                WorkloadEvidenceStageCounters::transform(1, 1, 0),
                WorkloadEvidenceStageBinding::new(
                    WorkloadEvidenceStage::Projection,
                    "projection-b",
                ),
            ),
        ])
        .expect("foreign transform binding remains inspectable until stage-link admission");

        let denial = product
            .link_required_stages(&[
                WorkloadEvidenceStage::Projection,
                WorkloadEvidenceStage::Transform,
            ])
            .expect_err("stage links must deny foreign upstream receipt binding");

        assert_eq!(
            denial,
            WorkloadEvidenceLedgerError::MismatchedAuthorityStageBinding(
                WorkloadEvidenceStage::Transform,
                WorkloadEvidenceStage::Projection
            )
        );
    }

    #[test]
    fn stage_index_rejects_counterless_boolean_receipt_rows() {
        let receipt =
            FakeBooleanReceipt::new("split", WorkloadEvidenceStageCounters::boolean_split());
        let product =
            WorkloadEvidenceStageIndexProduct::new(vec![WorkloadEvidenceRow::receipt_backed(
                WorkloadEvidenceStage::BooleanSplit,
                "split",
                WorkloadEvidenceStageCounters::default(),
            )])
            .expect("counterless row remains indexable but cannot match a receipt");

        let denial = product
            .require_boolean_receipt(&receipt)
            .expect_err("boolean receipt matching must reject counterless indexed rows");

        assert_eq!(
            denial,
            WorkloadEvidenceLedgerError::CounterlessBooleanStage(
                WorkloadEvidenceStage::BooleanSplit
            )
        );
    }

    #[test]
    fn stage_index_rejects_mismatched_boolean_receipt_identity() {
        let receipt = FakeBooleanReceipt::new(
            "expected split",
            WorkloadEvidenceStageCounters::boolean_split(),
        );
        let product =
            WorkloadEvidenceStageIndexProduct::new(vec![WorkloadEvidenceRow::receipt_backed(
                WorkloadEvidenceStage::BooleanSplit,
                "foreign split",
                WorkloadEvidenceStageCounters::boolean_split(),
            )])
            .expect("row with foreign identity still indexes for mismatch denial");

        let denial = product
            .require_boolean_receipt(&receipt)
            .expect_err("boolean receipt matching must reject foreign indexed rows");

        assert_eq!(
            denial,
            WorkloadEvidenceLedgerError::MismatchedBooleanStage(
                WorkloadEvidenceStage::BooleanSplit
            )
        );
    }

    #[test]
    fn stage_index_rejects_same_identity_boolean_receipt_family_substitution() {
        let admitted =
            FakeBooleanReceipt::new("split", WorkloadEvidenceStageCounters::boolean_split());
        let substituted = SubstitutedFakeBooleanReceipt::new(
            "split",
            WorkloadEvidenceStageCounters::boolean_split(),
        );
        let product = WorkloadEvidenceStageIndexProduct::new(vec![
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&substituted),
        ])
        .expect("substituted row remains indexable for family-mismatch denial");

        let denial = product
            .require_boolean_receipt(&admitted)
            .expect_err("same identity and counters must not spoof a different receipt family");

        assert_eq!(
            denial,
            WorkloadEvidenceLedgerError::MismatchedBooleanStage(
                WorkloadEvidenceStage::BooleanSplit
            )
        );
    }

    struct FakeBooleanReceipt {
        identity: &'static str,
        counters: WorkloadEvidenceStageCounters,
    }

    impl FakeBooleanReceipt {
        fn new(identity: &'static str, counters: WorkloadEvidenceStageCounters) -> Self {
            Self { identity, counters }
        }
    }

    impl BooleanEvidenceReceipt for FakeBooleanReceipt {
        fn boolean_stage(&self) -> BooleanEvidenceStageKind {
            BooleanEvidenceStageKind::Split
        }

        fn evidence_identity(&self) -> &str {
            self.identity
        }

        fn evidence_support(&self) -> WorkloadEvidenceSupport {
            WorkloadEvidenceSupport::Admitted
        }

        fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
            self.counters
        }
    }

    impl BooleanEvidenceRowAuthority for FakeBooleanReceipt {}

    struct SubstitutedFakeBooleanReceipt {
        identity: &'static str,
        counters: WorkloadEvidenceStageCounters,
    }

    impl SubstitutedFakeBooleanReceipt {
        fn new(identity: &'static str, counters: WorkloadEvidenceStageCounters) -> Self {
            Self { identity, counters }
        }
    }

    impl BooleanEvidenceReceipt for SubstitutedFakeBooleanReceipt {
        fn boolean_stage(&self) -> BooleanEvidenceStageKind {
            BooleanEvidenceStageKind::Split
        }

        fn evidence_identity(&self) -> &str {
            self.identity
        }

        fn evidence_support(&self) -> WorkloadEvidenceSupport {
            WorkloadEvidenceSupport::Admitted
        }

        fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
            self.counters
        }
    }

    impl BooleanEvidenceRowAuthority for SubstitutedFakeBooleanReceipt {}
}
