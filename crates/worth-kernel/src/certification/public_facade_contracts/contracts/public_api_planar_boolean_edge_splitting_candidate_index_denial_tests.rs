#[test]
fn candidate_index_consumption_gate_rejects_missing_metaboss_event_ledger_evidence() {
    assert_metaboss_candidate_index_consumption_denial(
        "phase7.3 missing event ledger evidence",
        |subject| {
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &subject.inputs().pair_worklist,
            )]
        },
        PlanarBooleanCandidateIndexConsumptionDenialKind::MissingEventLedgerEvidence,
        "split consumption must require receipt-backed event-ledger evidence",
    );
}

#[test]
fn candidate_index_consumption_gate_rejects_missing_metaboss_segment_pair_evidence() {
    assert_metaboss_candidate_index_consumption_denial(
        "phase7.3 missing segment pair evidence",
        |subject| {
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                subject.ledger(),
            )]
        },
        PlanarBooleanCandidateIndexConsumptionDenialKind::MissingSegmentPairEnumerationEvidence,
        "split consumption must require receipt-backed segment-pair evidence",
    );
}

#[test]
fn candidate_index_consumption_gate_rejects_manual_metaboss_segment_pair_evidence() {
    assert_metaboss_candidate_index_consumption_denial(
        "phase7.3 manual segment pair evidence",
        |subject| {
            let segment_pairs = &subject.inputs().pair_worklist;
            vec![
                WorkloadEvidenceRow::new(
                    WorkloadEvidenceStage::BooleanSegmentPairEnumeration,
                    segment_pairs.segment_pair_enumeration_identity(),
                ),
                WorkloadEvidenceRow::from_boolean_evidence_receipt(subject.ledger()),
            ]
        },
        PlanarBooleanCandidateIndexConsumptionDenialKind::ManualSegmentPairEnumerationEvidence,
        "split consumption must reject manual segment-pair evidence rows",
    );
}

#[test]
fn candidate_index_consumption_gate_rejects_manual_metaboss_event_ledger_evidence() {
    assert_metaboss_candidate_index_consumption_denial(
        "phase7.3 manual event ledger evidence",
        |subject| {
            vec![
                WorkloadEvidenceRow::from_boolean_evidence_receipt(&subject.inputs().pair_worklist),
                WorkloadEvidenceRow::new(
                    WorkloadEvidenceStage::BooleanEventLedger,
                    subject.ledger().event_ledger_identity(),
                ),
            ]
        },
        PlanarBooleanCandidateIndexConsumptionDenialKind::ManualEventLedgerEvidence,
        "split consumption must reject manual event-ledger evidence rows",
    );
}
