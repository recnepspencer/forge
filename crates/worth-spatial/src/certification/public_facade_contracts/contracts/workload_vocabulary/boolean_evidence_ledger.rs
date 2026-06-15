use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceReceipt, BooleanEvidenceStageKind, CompleteWorkloadEvidenceLedger,
    WorkloadEvidenceLedger, WorkloadEvidenceLedgerError, WorkloadEvidenceRow,
    WorkloadEvidenceStage, WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};

use super::evidence_ledger_receipts::counter_backed_rows;

#[test]
fn boolean_evidence_ledger_rejects_missing_or_mismatched_boolean_stage_rows() {
    run_with_large_stack(|| {
        let declaration = contract_boolean_receipt(
            BooleanEvidenceStageKind::DeclarationEntry,
            "boolean-declaration:real",
            WorkloadEvidenceSupport::Admitted,
            WorkloadEvidenceStageCounters::boolean_declaration(),
        );

        let missing = complete_ledger(vec![]);
        assert_eq!(
            missing
                .require_boolean_receipt(&declaration)
                .expect_err("missing boolean declaration row must deny at the ledger boundary"),
            WorkloadEvidenceLedgerError::MissingBooleanStage(
                WorkloadEvidenceStage::BooleanDeclarationEntry
            )
        );

        let mismatched = complete_ledger(vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
            &contract_boolean_receipt(
                BooleanEvidenceStageKind::DeclarationEntry,
                "boolean-declaration:foreign",
                WorkloadEvidenceSupport::Admitted,
                WorkloadEvidenceStageCounters::boolean_declaration(),
            ),
        )]);
        assert_eq!(
            mismatched.require_boolean_receipt(&declaration).expect_err(
                "foreign boolean declaration identity must not satisfy the required row"
            ),
            WorkloadEvidenceLedgerError::MismatchedBooleanStage(
                WorkloadEvidenceStage::BooleanDeclarationEntry
            )
        );

        let manual = complete_ledger(vec![WorkloadEvidenceRow::new(
            WorkloadEvidenceStage::BooleanDeclarationEntry,
            declaration.evidence_identity(),
        )]);
        assert_eq!(
            manual
                .require_boolean_receipt(&declaration)
                .expect_err("hand-filled boolean declaration rows must not satisfy ledger proof"),
            WorkloadEvidenceLedgerError::ManualBooleanStage(
                WorkloadEvidenceStage::BooleanDeclarationEntry
            )
        );

        let counterless =
            complete_ledger(vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &BorrowedIdentityBooleanReceipt {
                    stage: BooleanEvidenceStageKind::DeclarationEntry,
                    identity: declaration.evidence_identity().to_string(),
                    support: WorkloadEvidenceSupport::Admitted,
                    counters: WorkloadEvidenceStageCounters::default(),
                },
            )]);
        assert_eq!(
            counterless
                .require_boolean_receipt(&declaration)
                .expect_err("counterless boolean declaration rows must not count as real evidence"),
            WorkloadEvidenceLedgerError::CounterlessBooleanStage(
                WorkloadEvidenceStage::BooleanDeclarationEntry
            )
        );
    });
}

#[test]
fn boolean_stage_counters_count_real_receipt_backed_boolean_rows_only() {
    run_with_large_stack(|| {
        let ledger = WorkloadEvidenceLedger::from_rows({
            let mut rows = counter_backed_rows("boolean-counter-ledger");
            rows.extend([
                WorkloadEvidenceRow::from_boolean_evidence_receipt(&contract_boolean_receipt(
                    BooleanEvidenceStageKind::DeclarationEntry,
                    "boolean-declaration:real",
                    WorkloadEvidenceSupport::Admitted,
                    WorkloadEvidenceStageCounters::boolean_declaration(),
                )),
                WorkloadEvidenceRow::new(
                    WorkloadEvidenceStage::BooleanRoutePlan,
                    "boolean-route:manual",
                ),
                WorkloadEvidenceRow::from_boolean_evidence_receipt(&contract_boolean_receipt(
                    BooleanEvidenceStageKind::OperandPairConstruction,
                    "boolean-construction:counterless",
                    WorkloadEvidenceSupport::Admitted,
                    WorkloadEvidenceStageCounters::default(),
                )),
            ]);
            rows
        })
        .expect("mixed boolean evidence rows should remain inspectable");

        assert_eq!(ledger.counters().boolean_rows(), 1);
    });
}

#[test]
fn boolean_evidence_ledger_rejects_rows_backed_by_the_wrong_boolean_counter_family() {
    run_with_large_stack(|| {
        let declaration = contract_boolean_receipt(
            BooleanEvidenceStageKind::DeclarationEntry,
            "boolean-declaration:real",
            WorkloadEvidenceSupport::Admitted,
            WorkloadEvidenceStageCounters::boolean_declaration(),
        );
        let wrong_counter_family =
            complete_ledger(vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &BorrowedIdentityBooleanReceipt {
                    stage: BooleanEvidenceStageKind::DeclarationEntry,
                    identity: declaration.evidence_identity().to_string(),
                    support: WorkloadEvidenceSupport::Admitted,
                    counters: WorkloadEvidenceStageCounters::boolean_route(),
                },
            )]);

        assert_eq!(
            wrong_counter_family
                .require_boolean_receipt(&declaration)
                .expect_err("a declaration row backed only by route counters must fail"),
            WorkloadEvidenceLedgerError::CounterlessBooleanStage(
                WorkloadEvidenceStage::BooleanDeclarationEntry
            )
        );
        assert_eq!(
            wrong_counter_family.counters().boolean_rows(),
            0,
            "boolean row counters must only count rows that carry stage-matching proof"
        );
    });
}

fn complete_ledger(boolean_rows: Vec<WorkloadEvidenceRow>) -> CompleteWorkloadEvidenceLedger {
    let mut rows = counter_backed_rows("boolean-evidence-ledger");
    rows.extend(boolean_rows);
    WorkloadEvidenceLedger::from_rows(rows)
        .expect("boolean evidence test ledger should stay inspectable")
        .certify_complete()
        .expect("authority stages should remain complete")
}

#[derive(Clone, Copy)]
struct ContractBooleanReceipt {
    stage: BooleanEvidenceStageKind,
    identity: &'static str,
    support: WorkloadEvidenceSupport,
    counters: WorkloadEvidenceStageCounters,
}

impl BooleanEvidenceReceipt for ContractBooleanReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        self.stage
    }

    fn evidence_identity(&self) -> &str {
        self.identity
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        self.support
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        self.counters
    }
}

fn contract_boolean_receipt(
    stage: BooleanEvidenceStageKind,
    identity: &'static str,
    support: WorkloadEvidenceSupport,
    counters: WorkloadEvidenceStageCounters,
) -> ContractBooleanReceipt {
    ContractBooleanReceipt {
        stage,
        identity,
        support,
        counters,
    }
}

#[derive(Clone)]
struct BorrowedIdentityBooleanReceipt {
    stage: BooleanEvidenceStageKind,
    identity: String,
    support: WorkloadEvidenceSupport,
    counters: WorkloadEvidenceStageCounters,
}

impl BooleanEvidenceReceipt for BorrowedIdentityBooleanReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        self.stage
    }

    fn evidence_identity(&self) -> &str {
        &self.identity
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        self.support
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        self.counters
    }
}

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("boolean-evidence-ledger".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("boolean evidence ledger contract thread should spawn")
        .join()
        .expect("boolean evidence ledger contract thread should finish");
}
