use super::super::edge_splitting_replay_parity_support::{
    build_edge_split_replay_parity_subject, replay_parity_report, EdgeSplitReplayParitySubject,
};
use super::super::metaboss_support::MetabossEventExtractionSubject;
use super::completed_split_handoff_for;
use worth_kernel::workload_composition::CompletedBooleanSplitHandoff;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanDownstreamSplitConsumption, PlanarBooleanDownstreamSplitConsumptionDenialKind,
    PlanarBooleanDownstreamSplitConsumptionInput, PlanarBooleanEdgeSplitReplayParityReport,
};
use worth_spatial::facade::workload_vocabulary::SpatialGeometryEvidenceTouchAuthority;

pub(crate) fn assert_foreign_split_authorities_are_rejected(
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_split_handoff: &CompletedBooleanSplitHandoff,
) {
    let foreign_subject =
        MetabossEventExtractionSubject::certify("phase7.3 foreign downstream split consumption");
    let foreign_replay_subject = build_edge_split_replay_parity_subject(&foreign_subject);
    let foreign_replay_report = replay_parity_report(&foreign_replay_subject);
    let foreign_completed_split_handoff =
        completed_split_handoff_for(&foreign_subject, &foreign_replay_subject);

    assert_foreign_decision_log_is_rejected(
        replay_subject,
        replay_report,
        completed_split_handoff,
        &foreign_replay_subject,
    );
    assert_foreign_validation_receipt_is_rejected(
        replay_subject,
        replay_report,
        completed_split_handoff,
        &foreign_replay_subject,
    );
    assert_foreign_persistent_naming_receipt_is_rejected(
        replay_subject,
        replay_report,
        completed_split_handoff,
        &foreign_replay_subject,
    );
    assert_foreign_replay_parity_receipt_is_rejected(
        replay_subject,
        &foreign_replay_report,
        completed_split_handoff,
    );
    assert_foreign_spatial_touch_authority_is_rejected(
        replay_subject,
        replay_report,
        &foreign_completed_split_handoff,
    );
}

fn assert_foreign_decision_log_is_rejected(
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_split_handoff: &CompletedBooleanSplitHandoff,
    foreign_replay_subject: &EdgeSplitReplayParitySubject,
) {
    let authority = spatial_touch_authority(
        completed_split_handoff,
        replay_subject.original_ledger.receipt(),
    );
    let denial = PlanarBooleanDownstreamSplitConsumption::admit(
        PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(
            replay_subject.original_ledger.receipt(),
            foreign_replay_subject.original_decision_log.receipt(),
            &replay_subject.original_products.validation,
            &replay_subject.original_products.naming,
            replay_report.receipt(),
            &authority,
        ),
    )
    .expect_err("foreign decision log must not certify downstream split consumption");
    assert_eq!(
        denial.kind(),
        PlanarBooleanDownstreamSplitConsumptionDenialKind::ForeignDecisionLogReceipt
    );
    assert_eq!(denial.counters().foreign_receipts_rejected(), 1);
}

fn assert_foreign_validation_receipt_is_rejected(
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_split_handoff: &CompletedBooleanSplitHandoff,
    foreign_replay_subject: &EdgeSplitReplayParitySubject,
) {
    let authority = spatial_touch_authority(
        completed_split_handoff,
        replay_subject.original_ledger.receipt(),
    );
    let denial = PlanarBooleanDownstreamSplitConsumption::admit(
        PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(
            replay_subject.original_ledger.receipt(),
            replay_subject.original_decision_log.receipt(),
            &foreign_replay_subject.original_products.validation,
            &replay_subject.original_products.naming,
            replay_report.receipt(),
            &authority,
        ),
    )
    .expect_err("foreign validation receipt must not certify downstream split consumption");
    assert_eq!(
        denial.kind(),
        PlanarBooleanDownstreamSplitConsumptionDenialKind::ForeignValidationReceipt
    );
}

fn assert_foreign_persistent_naming_receipt_is_rejected(
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_split_handoff: &CompletedBooleanSplitHandoff,
    foreign_replay_subject: &EdgeSplitReplayParitySubject,
) {
    let authority = spatial_touch_authority(
        completed_split_handoff,
        replay_subject.original_ledger.receipt(),
    );
    let denial = PlanarBooleanDownstreamSplitConsumption::admit(
        PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(
            replay_subject.original_ledger.receipt(),
            replay_subject.original_decision_log.receipt(),
            &replay_subject.original_products.validation,
            &foreign_replay_subject.original_products.naming,
            replay_report.receipt(),
            &authority,
        ),
    )
    .expect_err("foreign persistent naming receipt must not certify downstream consumption");
    assert_eq!(
        denial.kind(),
        PlanarBooleanDownstreamSplitConsumptionDenialKind::ForeignPersistentNamingReceipt
    );
}

fn assert_foreign_replay_parity_receipt_is_rejected(
    replay_subject: &EdgeSplitReplayParitySubject,
    foreign_replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_split_handoff: &CompletedBooleanSplitHandoff,
) {
    let authority = spatial_touch_authority(
        completed_split_handoff,
        replay_subject.original_ledger.receipt(),
    );
    let denial = PlanarBooleanDownstreamSplitConsumption::admit(
        PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(
            replay_subject.original_ledger.receipt(),
            replay_subject.original_decision_log.receipt(),
            &replay_subject.original_products.validation,
            &replay_subject.original_products.naming,
            foreign_replay_report.receipt(),
            &authority,
        ),
    )
    .expect_err("foreign replay parity receipt must not certify downstream split consumption");
    assert_eq!(
        denial.kind(),
        PlanarBooleanDownstreamSplitConsumptionDenialKind::ForeignReplayParityReceipt
    );
}

fn assert_foreign_spatial_touch_authority_is_rejected(
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    foreign_completed_split_handoff: &CompletedBooleanSplitHandoff,
) {
    let foreign_authority = spatial_touch_authority(
        foreign_completed_split_handoff,
        foreign_completed_split_handoff.split_ledger_receipt(),
    );
    let denial = PlanarBooleanDownstreamSplitConsumption::admit(
        PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(
            replay_subject.original_ledger.receipt(),
            replay_subject.original_decision_log.receipt(),
            &replay_subject.original_products.validation,
            &replay_subject.original_products.naming,
            replay_report.receipt(),
            &foreign_authority,
        ),
    )
    .expect_err("foreign spatial touch authority must not certify downstream split consumption");
    assert_eq!(
        denial.kind(),
        PlanarBooleanDownstreamSplitConsumptionDenialKind::ForeignSpatialTouchAuthority
    );
}

fn spatial_touch_authority(
    handoff: &CompletedBooleanSplitHandoff,
    receipt: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeChainLedgerReceipt,
) -> SpatialGeometryEvidenceTouchAuthority {
    handoff
        .completed_workload()
        .admit_spatial_geometry_evidence_touch(receipt)
        .expect("completed split handoff should admit spatial touch authority")
}
