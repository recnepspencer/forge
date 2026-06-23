use worth_kernel::workload_composition::WorkloadCompositionError;
use worth_spatial::facade::workload_vocabulary::{WorkloadEvidenceRow, WorkloadEvidenceStage};

use super::reduced_pair_support;
use super::subject::MetabossEventExtractionSubject;

pub(crate) fn assert_split_handoff_requires_event_ledger_receipt(
    subject: &MetabossEventExtractionSubject,
) {
    let bare = subject.pair().left().workload().clone();
    assert_eq!(
        bare.require_boolean_event_ledger(subject.ledger())
            .expect_err("bare workload must not satisfy split handoff"),
        WorkloadCompositionError::MissingEvidenceStage(WorkloadEvidenceStage::BooleanEventLedger)
    );
    let admitted = reduced_pair_support::rebuild_left_workload(
        subject.pair(),
        vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
            subject.ledger(),
        )],
    );
    admitted
        .require_boolean_event_ledger(subject.ledger())
        .expect("event-ledger receipt is the admitted 7.3 handoff proof");
}

pub(crate) fn assert_public_contract_rejects_synthetic_event_ledger_rows(
    subject: &MetabossEventExtractionSubject,
) {
    let manual = reduced_pair_support::rebuild_left_workload(
        subject.pair(),
        vec![WorkloadEvidenceRow::new(
            WorkloadEvidenceStage::BooleanEventLedger,
            subject.ledger().event_ledger_identity(),
        )],
    );
    assert_eq!(
        manual
            .require_boolean_event_ledger(subject.ledger())
            .expect_err("manual ledger rows must not stand in for receipt evidence"),
        WorkloadCompositionError::ManualEvidenceStage(WorkloadEvidenceStage::BooleanEventLedger)
    );
}
