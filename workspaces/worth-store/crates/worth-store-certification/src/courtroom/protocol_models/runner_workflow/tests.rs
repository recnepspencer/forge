use std::num::NonZeroU64;

use worth_store_formal_models::runner::{
    adjudicate_shared_frontier_trace, classify_receipt_loss, interpret_tlc_output,
    AbstractionFunctionIdentity, CanonicalProtocolAction, CanonicalProtocolTrace,
    CertificationLaneIdentity, CounterexampleLocalization, ProtocolCheckBounds,
    ProtocolCheckVerdict, ProtocolExecutionOutcome, ProtocolFrontierIdentity,
    ReceiptLossClassification,
};
use worth_store_formal_models::{
    current_protocol_binding_manifest, CompactionVisibilityAction, ModelActionFamily,
    OwnerEvidenceClass, OwnerObservationOmissionCause, ProtocolFamily, SharedFrontierAction,
};

#[test]
fn legal_owner_projection_and_checked_frontier_adjudicate_together() {
    let trace = CanonicalProtocolTrace::admit(
        ProtocolFamily::SharedFrontiers,
        ProtocolFrontierIdentity::Visibility,
        [
            SharedFrontierAction::LiveLeaseAcquired,
            SharedFrontierAction::RecoveryPrecedencePreserved,
            SharedFrontierAction::CompactionCutover,
            SharedFrontierAction::Crash,
            SharedFrontierAction::Reopen,
        ]
        .map(CanonicalProtocolAction::SharedFrontier),
    )
    .unwrap();
    let verdict = interpret_tlc_output(
        ProtocolFamily::SharedFrontiers,
        bounds(),
        checked_output(),
        true,
    )
    .unwrap();

    assert!(matches!(
        adjudicate_shared_frontier_trace(verdict, trace),
        ProtocolExecutionOutcome::LegalProtocolExecution { statistics, .. }
            if statistics.distinct_states() == 7_660
    ));
}

#[test]
fn illegal_runtime_edge_is_distinct_from_a_checker_counterexample() {
    let trace = CanonicalProtocolTrace::admit(
        ProtocolFamily::SharedFrontiers,
        ProtocolFrontierIdentity::Quarantine,
        [
            SharedFrontierAction::DurabilityAdmitted,
            SharedFrontierAction::QuarantineSealed,
            SharedFrontierAction::CheckpointPublicationRequested,
        ]
        .map(CanonicalProtocolAction::SharedFrontier),
    )
    .unwrap();
    let verdict = interpret_tlc_output(
        ProtocolFamily::SharedFrontiers,
        bounds(),
        checked_output(),
        true,
    )
    .unwrap();

    assert!(matches!(
        adjudicate_shared_frontier_trace(verdict, trace),
        ProtocolExecutionOutcome::IllegalRuntimeTransition {
            action_index: 2,
            ..
        }
    ));
}

#[test]
fn controlled_defect_localizes_to_owner_mapping_and_lane() {
    let ProtocolCheckVerdict::CounterexampleFound { counterexample, .. } = interpret_tlc_output(
        ProtocolFamily::CompactionVisibility,
        bounds(),
        weakened_transition_output(),
        false,
    )
    .unwrap() else {
        panic!("weakened transition must be a checker counterexample")
    };
    let binding = current_protocol_binding_manifest()
        .bindings()
        .find(|binding| {
            binding.protocol() == ProtocolFamily::CompactionVisibility
                && binding.model_action_family() == ModelActionFamily::PhysicalCompaction
        })
        .expect("physical compaction binding is declared");
    let trace = CanonicalProtocolTrace::admit(
        ProtocolFamily::CompactionVisibility,
        ProtocolFrontierIdentity::Visibility,
        [CanonicalProtocolAction::CompactionVisibility(
            CompactionVisibilityAction::PublishRewrite,
        )],
    )
    .unwrap();
    let lane = CertificationLaneIdentity::admit("protocol.compaction.visibility.mutant").unwrap();
    let localized = CounterexampleLocalization::localize(
        counterexample,
        binding,
        AbstractionFunctionIdentity::CompactionVisibilityOwnerMapping,
        lane,
        trace,
    )
    .unwrap();

    assert_eq!(localized.owner_binding().operation(), binding.operation());
    assert_eq!(
        localized.abstraction_function(),
        AbstractionFunctionIdentity::CompactionVisibilityOwnerMapping
    );
    assert_eq!(
        localized.failing_lane().as_str(),
        "protocol.compaction.visibility.mutant"
    );
    assert!(localized
        .counterexample()
        .state_edges()
        .any(|edge| edge.contains("PublishRewrite")));
}

#[test]
fn receipt_loss_causes_remain_operationally_distinct() {
    assert_eq!(
        classify_receipt_loss(
            OwnerEvidenceClass::EphemeralDiagnosticTrace,
            OwnerObservationOmissionCause::InstrumentationDidNotEmit,
        ),
        ReceiptLossClassification::DiagnosticOmissionDefect
    );
    assert_eq!(
        classify_receipt_loss(
            OwnerEvidenceClass::DurableAuthoritativeReceipt,
            OwnerObservationOmissionCause::LostAcrossCrash,
        ),
        ReceiptLossClassification::AuthoritativeReceiptOmissionDefect
    );
    assert_eq!(
        classify_receipt_loss(
            OwnerEvidenceClass::EphemeralDiagnosticTrace,
            OwnerObservationOmissionCause::LostAcrossCrash,
        ),
        ReceiptLossClassification::CrashLostNonAuthoritativeTrace
    );
}

fn bounds() -> ProtocolCheckBounds {
    ProtocolCheckBounds::new(
        NonZeroU64::new(10_000).unwrap(),
        NonZeroU64::new(32).unwrap(),
    )
}

fn checked_output() -> &'static str {
    "Finished computing initial states: 1 distinct state generated.\n\
     Model checking completed. No error has been found.\n\
     85886 states generated, 7660 distinct states found, 0 states left on queue.\n\
     The depth of the complete state graph search is 15."
}

fn weakened_transition_output() -> &'static str {
    "Finished computing initial states: 1 distinct state generated.\n\
     Error: Invariant PublicationRequiresCutover is violated.\n\
     Error: The behavior up to this point is:\n\
     State 1: <Initial predicate>\n\
     State 2: <PublishRewrite line 40, col 3 of module CompactionVisibility>\n\
     2 states generated, 2 distinct states found, 0 states left on queue.\n\
     The depth of the complete state graph search is 2."
}
