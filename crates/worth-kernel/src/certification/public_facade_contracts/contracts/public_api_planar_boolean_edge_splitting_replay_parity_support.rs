use super::edge_splitting_decision_log_support::{
    build_decision_log_products_for_metaboss, DecisionLogMetabossProducts,
};
use super::metaboss_support::MetabossEventExtractionSubject;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    CompareEdgeSplitReplayParity, PlanarBooleanEdgeSplitCloseout,
    PlanarBooleanEdgeSplitReplayExecutionMode, PlanarBooleanEdgeSplitReplayParityDenialKind,
    PlanarBooleanEdgeSplitReplayParityReport, PlanarBooleanEdgeSplitReplayParityRowKind,
    PlanarBooleanEdgeSplitReplayProduct, PlanarBooleanEdgeSplitReplayQueryDomain,
    PlanarBooleanEdgeSplitReplayQueryInput, PlanarBooleanSplitDecisionLogQueryDomain,
    PlanarBooleanSplitDecisionLogQueryInput, PlanarBooleanSplitDecisionLogQueryResult,
    PlanarBooleanSplitEdgeChainLedgerQueryDomain, PlanarBooleanSplitEdgeChainLedgerQueryInput,
    PlanarBooleanSplitEdgeChainLedgerQueryResult, PlanarBooleanSplitReplayClosureRowKind,
    ValidatePlanarBooleanReplayParity,
};
use worth_spatial::facade::planar_boolean_events::PlanarBooleanSourceIntervalSense;
use worth_spatial::facade::retained_replay_workload::ReplayReceiptSet;

pub(crate) struct EdgeSplitReplayParitySubject {
    pub(crate) original_products: DecisionLogMetabossProducts,
    pub(crate) replayed_products: DecisionLogMetabossProducts,
    pub(crate) original_decision_log: PlanarBooleanSplitDecisionLogQueryResult,
    pub(crate) replayed_decision_log: PlanarBooleanSplitDecisionLogQueryResult,
    pub(crate) original_ledger: PlanarBooleanSplitEdgeChainLedgerQueryResult,
    pub(crate) replayed_ledger: PlanarBooleanSplitEdgeChainLedgerQueryResult,
    pub(crate) replay_receipts: ReplayReceiptSet,
}

pub(crate) fn build_edge_split_replay_parity_subject(
    subject: &MetabossEventExtractionSubject,
) -> EdgeSplitReplayParitySubject {
    let original_products = build_decision_log_products_for_metaboss(subject);
    let replayed_products = build_decision_log_products_for_metaboss(subject);
    let original_decision_log = decision_log_for(&original_products);
    let replayed_decision_log = decision_log_for(&replayed_products);
    let original_ledger = ledger_for(&original_products, &original_decision_log);
    let replayed_ledger = ledger_for(&replayed_products, &replayed_decision_log);
    let replay_receipts = subject
        .pair()
        .left()
        .replay_receipts()
        .expect("metaboss left operand should expose same-workload retained replay receipts")
        .clone();

    EdgeSplitReplayParitySubject {
        original_products,
        replayed_products,
        original_decision_log,
        replayed_decision_log,
        original_ledger,
        replayed_ledger,
        replay_receipts,
    }
}

pub(crate) fn replay_parity_report(
    subject: &EdgeSplitReplayParitySubject,
) -> PlanarBooleanEdgeSplitReplayParityReport {
    let replay_product = replay_product_for(
        subject,
        PlanarBooleanEdgeSplitReplayExecutionMode::RetainedReplay,
    );
    let input = worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitReplayParityInput::from_replay_product(
        &replay_product,
    )
    .expect("metaboss split products and retained replay receipts should form parity input");
    CompareEdgeSplitReplayParity::compare(input)
        .expect("metaboss split replay parity should compare without mismatch")
}

pub(crate) fn assert_replay_parity_certifies_split_products(
    subject: &EdgeSplitReplayParitySubject,
    report: &PlanarBooleanEdgeSplitReplayParityReport,
) {
    let receipt = report.receipt();
    assert!(receipt.certifies_planar_boolean_replay_parity());
    assert_eq!(
        receipt.original_split_ledger_receipt_identity(),
        subject.original_ledger.receipt().receipt_identity()
    );
    assert_eq!(
        receipt.replayed_split_ledger_receipt_identity(),
        subject.replayed_ledger.receipt().receipt_identity()
    );
    assert_eq!(
        receipt.original_downstream_consumption_identity(),
        receipt.replayed_downstream_consumption_identity()
    );
    assert_eq!(
        receipt.replay_checkpoint_identity(),
        subject.replay_receipts.replay_checkpoint_identity()
    );
    assert_eq!(
        receipt.replay_evidence_identity(),
        subject.replay_receipts.replay_evidence_identity()
    );
    assert_eq!(
        receipt.original_split_request_identity(),
        subject.original_products.request.split_request_identity()
    );
    assert_eq!(
        receipt.replayed_split_request_identity(),
        subject.replayed_products.request.split_request_identity()
    );
    assert_eq!(
        subject
            .original_products
            .request
            .retained_replay_stage_identity()
            .map(ToString::to_string),
        Some(subject.replay_receipts.stage_identity().receipt_identity())
    );
    assert!(receipt.parity_rows().iter().any(|row| {
        row.kind() == PlanarBooleanEdgeSplitReplayParityRowKind::OperationalTruthDigest
    }));
    assert!(receipt
        .parity_rows()
        .iter()
        .any(|row| { row.kind() == PlanarBooleanEdgeSplitReplayParityRowKind::ReplayProduct }));
    assert!(receipt.parity_rows().iter().any(|row| {
        row.kind() == PlanarBooleanEdgeSplitReplayParityRowKind::ReplayClosureManifest
    }));
    assert!(receipt.counters().replay_closure_rows_compared() >= 20);
    assert_eq!(receipt.counters().event_extraction_reexecutions(), 0);
    assert_eq!(receipt.counters().candidate_index_reexecutions(), 0);
}

pub(crate) fn assert_reversed_source_sense_is_covered(
    subject: &EdgeSplitReplayParitySubject,
    report: &PlanarBooleanEdgeSplitReplayParityReport,
) {
    assert!(report.receipt().parity_rows().iter().any(|row| {
        row.kind() == PlanarBooleanEdgeSplitReplayParityRowKind::ReversedSourceSenseCanonicalization
    }));
    assert!(subject
        .original_products
        .fragments
        .fragments()
        .any(|fragment| fragment
            .source_senses()
            .contains(&PlanarBooleanSourceIntervalSense::Reversed)));
    assert!(subject
        .original_products
        .chains
        .chains()
        .iter()
        .flat_map(|chain| chain.members())
        .any(|member| member.source_sense() == PlanarBooleanSourceIntervalSense::Reversed));
    let replay_product = replay_product_for(
        subject,
        PlanarBooleanEdgeSplitReplayExecutionMode::ReversedSourceSenseVariant,
    );
    assert_eq!(
        replay_product.execution_mode(),
        PlanarBooleanEdgeSplitReplayExecutionMode::ReversedSourceSenseVariant
    );
    assert!(replay_product
        .closure_manifest()
        .rows()
        .iter()
        .any(|row| row.kind() == PlanarBooleanSplitReplayClosureRowKind::OverlapChainIdentity));
}

pub(crate) fn assert_checkpoint_parity_is_retained_replay_backed(
    subject: &EdgeSplitReplayParitySubject,
    report: &PlanarBooleanEdgeSplitReplayParityReport,
) {
    let receipt = report.receipt();
    assert!(receipt.parity_rows().iter().any(|row| {
        row.kind() == PlanarBooleanEdgeSplitReplayParityRowKind::RetainedReplayCheckpoint
    }));
    assert!(subject.replay_receipts.counters().retained_artifact_rows() > 0);
    assert!(subject.replay_receipts.counters().replay_rows() > 0);
    let validator = ValidatePlanarBooleanReplayParity::validate(receipt);
    assert!(validator.certifies_replay_parity_validation());
    assert_eq!(
        validator.parity_receipt_identity(),
        receipt.receipt_identity()
    );
    let checkpointed = replay_product_for(
        subject,
        PlanarBooleanEdgeSplitReplayExecutionMode::CheckpointedReplay,
    );
    assert_eq!(checkpointed.counters().checkpoint_rows_read(), 1);
    assert!(checkpointed.certifies_query_owned_replay_product());
}

pub(crate) fn assert_foreign_retained_replay_receipt_is_rejected(
    subject: &EdgeSplitReplayParitySubject,
    foreign_replay_receipts: &ReplayReceiptSet,
) {
    let original_closeout = closeout_for(
        &subject.original_products,
        &subject.original_decision_log,
        &subject.original_ledger,
    );
    let replayed_closeout = closeout_for(
        &subject.replayed_products,
        &subject.replayed_decision_log,
        &subject.replayed_ledger,
    );
    let denial_result = PlanarBooleanEdgeSplitReplayQueryDomain::declare(
        PlanarBooleanEdgeSplitReplayQueryInput::from_closeouts(
            original_closeout,
            replayed_closeout,
            foreign_replay_receipts,
            PlanarBooleanEdgeSplitReplayExecutionMode::RetainedReplay,
        ),
    )
    .and_then(|plan| plan.execute());
    let denial = match denial_result {
        Ok(_) => panic!("foreign retained replay receipts must not certify split replay parity"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        PlanarBooleanEdgeSplitReplayParityDenialKind::ForeignRetainedReplayReceipt
    );
}

fn replay_product_for<'a>(
    subject: &'a EdgeSplitReplayParitySubject,
    mode: PlanarBooleanEdgeSplitReplayExecutionMode,
) -> PlanarBooleanEdgeSplitReplayProduct<'a> {
    let original_closeout = closeout_for(
        &subject.original_products,
        &subject.original_decision_log,
        &subject.original_ledger,
    );
    let replayed_closeout = closeout_for(
        &subject.replayed_products,
        &subject.replayed_decision_log,
        &subject.replayed_ledger,
    );
    PlanarBooleanEdgeSplitReplayQueryDomain::declare(
        PlanarBooleanEdgeSplitReplayQueryInput::from_closeouts(
            original_closeout,
            replayed_closeout,
            &subject.replay_receipts,
            mode,
        ),
    )
    .expect("metaboss replay query should lower")
    .execute()
    .expect("metaboss closeouts should bind to retained replay product")
}

fn closeout_for<'a>(
    products: &'a DecisionLogMetabossProducts,
    decision_log: &'a PlanarBooleanSplitDecisionLogQueryResult,
    ledger: &'a PlanarBooleanSplitEdgeChainLedgerQueryResult,
) -> PlanarBooleanEdgeSplitCloseout<'a> {
    PlanarBooleanEdgeSplitCloseout::from_query_products(
        &products.request,
        &products.endpoint_boundary,
        &products.interval_subdivision,
        &products.vertices,
        &products.fragments,
        &products.chains,
        &products.validation,
        &products.naming,
        decision_log,
        ledger,
    )
}

fn decision_log_for(
    products: &DecisionLogMetabossProducts,
) -> PlanarBooleanSplitDecisionLogQueryResult {
    PlanarBooleanSplitDecisionLogQueryDomain::declare(PlanarBooleanSplitDecisionLogQueryInput::new(
        &products.request,
        &products.endpoint_boundary,
        &products.interval_subdivision,
        &products.vertices,
        &products.fragments,
        &products.validation,
        &products.naming,
    ))
    .expect("decision log declaration should lower for replay parity")
    .execute()
    .expect("decision log should execute for replay parity")
}

fn ledger_for(
    products: &DecisionLogMetabossProducts,
    decision_log: &PlanarBooleanSplitDecisionLogQueryResult,
) -> PlanarBooleanSplitEdgeChainLedgerQueryResult {
    PlanarBooleanSplitEdgeChainLedgerQueryDomain::declare(
        PlanarBooleanSplitEdgeChainLedgerQueryInput::new(
            &products.request,
            &products.endpoint_boundary,
            &products.interval_subdivision,
            &products.vertices,
            &products.fragments,
            &products.chains,
            &products.validation,
            &products.naming,
            decision_log,
        ),
    )
    .expect("split ledger declaration should lower for replay parity")
    .execute()
    .expect("split ledger should execute for replay parity")
}
