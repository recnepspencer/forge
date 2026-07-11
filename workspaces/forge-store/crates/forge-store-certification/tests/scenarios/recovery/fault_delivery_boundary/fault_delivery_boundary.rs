#[path = "../../../support/recovery/fault_delivery_boundary/support.rs"]
mod support;

use forge_foundational::FoundationalBoundaryEvidenceLocality;
use forge_store_physical_backend::ProductionStorageBoundarySeam;
use forge_store_physical_certification::{
    lower_physical_simulation_plan, AdmittedDriverContractSet, CrashEvent,
    ExpectedFaultLocalization, FaultDeliveryDenial, FaultDeliveryPlan, FaultObservedBoundaryKind,
    NoFaultProductionBoundaryParity, ObservedFaultBoundary, PhysicalArtifactKind,
    PhysicalBoundarySeam, PhysicalBoundaryYieldpoint, PhysicalFaultEvent, PhysicalFaultEventKind,
    PhysicalFaultFieldKind,
};
use forge_store_test_support::harness::test_authority::{
    ambiguous_locus_fault_attempt_fixture, arbitrary_byte_scribble_fault_attempt_fixture,
    crash_recovery_fault_locus, observed_checksum_mismatch_boundary,
    post_decode_corruption_fault_attempt_fixture, private_mutation_fault_attempt_fixture,
    same_process_crash_fault_attempt_fixture, wal_frame_payload_fault_locus,
};
use support::{
    alternate_production_trace, complete_context, deliver_storage_event,
    developer_smoke_production_trace, fresh_runtime_crash_evidence,
    production_driver_with_all_seams, readiness_scenario, readiness_scenario_scheduled_at,
    recovery_scenario, storage_fault_delivery_cases,
};

#[test]
fn storage_fault_events_deliver_through_declared_boundary_matrix() {
    for case in storage_fault_delivery_cases() {
        let receipt = deliver_storage_event(&case).unwrap();
        assert_eq!(receipt.event_kind(), case.event_kind);
        assert_eq!(receipt.seam(), case.expected_seam);
        assert_eq!(receipt.yieldpoint(), &case.yieldpoint);
        assert_eq!(
            receipt.expected_localization(),
            Some(case.expected_localization)
        );
        assert_eq!(receipt.actual_boundary(), &case.actual_boundary);

        let locus = receipt.locus().expect("phase 6 event carries a locus");
        assert_eq!(locus.artifact_kind(), case.artifact_kind);
        assert_eq!(locus.field_kind(), case.field_kind);
        if case.requires_offset {
            assert!(locus.offset().is_some());
        }
    }
}

#[test]
fn crash_event_delivers_through_fresh_runtime_boundary_plan() {
    let missing = CrashEvent::missing_fresh_runtime_evidence(
        PhysicalBoundaryYieldpoint::fresh_runtime_replay_open(),
    )
    .unwrap_err();
    let event = PhysicalFaultEvent::crash(
        CrashEvent::fresh_runtime_recovery(
            PhysicalBoundaryYieldpoint::fresh_runtime_replay_open(),
            crash_recovery_fault_locus(),
            fresh_runtime_crash_evidence(),
        )
        .unwrap(),
    );
    let plan = lower_physical_simulation_plan(recovery_scenario(), complete_context()).unwrap();
    let lowered = FaultDeliveryPlan::lower(
        event,
        plan.yieldpoint_binding(),
        PhysicalBoundaryYieldpoint::fresh_runtime_replay_open(),
    )
    .unwrap();
    assert_eq!(
        lowered.payload().proof_event_kind(),
        PhysicalFaultEventKind::Crash
    );
    assert_eq!(
        lowered.payload().proof_yieldpoint(),
        &PhysicalBoundaryYieldpoint::fresh_runtime_replay_open()
    );
    let ready = FaultDeliveryPlan::admit_execution_ready(
        lowered,
        ObservedFaultBoundary::fresh_runtime_crash_recovery(&fresh_runtime_crash_evidence()),
    )
    .unwrap();
    let receipt =
        FaultDeliveryPlan::receipt_from_executed(FaultDeliveryPlan::execute_ready(ready)).unwrap();

    assert_eq!(missing, FaultDeliveryDenial::MissingFreshRuntimeEvidence);
    assert_eq!(receipt.event_kind(), PhysicalFaultEventKind::Crash);
    assert_eq!(receipt.seam(), PhysicalBoundarySeam::FreshRuntimeRecovery);
    assert_eq!(
        receipt.expected_localization(),
        Some(ExpectedFaultLocalization::FreshRuntimeRecoveryBoundary)
    );
    assert_eq!(
        receipt.actual_boundary().locality(),
        FoundationalBoundaryEvidenceLocality::RestoredReadmitted
    );
    let locus = receipt.locus().expect("crash receipt records a locus");
    assert_eq!(
        locus.artifact_kind(),
        PhysicalArtifactKind::CrashRecoveryRuntime
    );
    assert_eq!(locus.field_kind(), PhysicalFaultFieldKind::RuntimeIsolation);
}

#[test]
fn hostile_fault_authoring_attempts_deny_before_delivery() {
    assert_eq!(
        private_mutation_fault_attempt_fixture()
            .admit()
            .unwrap_err(),
        FaultDeliveryDenial::PrivateMutationDenied
    );
    assert_eq!(
        arbitrary_byte_scribble_fault_attempt_fixture()
            .admit()
            .unwrap_err(),
        FaultDeliveryDenial::ArbitraryByteScribbleDenied
    );
    assert_eq!(
        same_process_crash_fault_attempt_fixture()
            .admit()
            .unwrap_err(),
        FaultDeliveryDenial::SameProcessCrashDenied
    );
    assert_eq!(
        post_decode_corruption_fault_attempt_fixture()
            .admit()
            .unwrap_err(),
        FaultDeliveryDenial::PostDecodeCorruptionDenied
    );
    assert_eq!(
        ambiguous_locus_fault_attempt_fixture().admit().unwrap_err(),
        FaultDeliveryDenial::AmbiguousFaultLocusDenied
    );
}

#[test]
fn fault_delivery_rejects_unbound_and_wrong_seam_yieldpoints() {
    let plan = lower_physical_simulation_plan(readiness_scenario(), complete_context()).unwrap();
    let event = PhysicalFaultEvent::byte_corruption(
        ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
        wal_frame_payload_fault_locus(),
    )
    .unwrap();
    let lowered = FaultDeliveryPlan::lower(
        event.clone(),
        plan.yieldpoint_binding(),
        PhysicalBoundaryYieldpoint::root_publication_before_observe(),
    )
    .unwrap();

    assert_eq!(
        FaultDeliveryPlan::receipt_from_executed(FaultDeliveryPlan::execute_ready(
            FaultDeliveryPlan::admit_execution_ready(
                lowered,
                observed_checksum_mismatch_boundary(),
            )
            .unwrap()
        ))
        .unwrap()
        .event_kind(),
        PhysicalFaultEventKind::ByteCorruption
    );
    let unbound = event
        .clone()
        .deliver_through(
            plan.yieldpoint_binding(),
            PhysicalBoundaryYieldpoint::wal_append_before_flush(),
            observed_checksum_mismatch_boundary(),
        )
        .unwrap_err();
    let wal_plan = lower_physical_simulation_plan(
        readiness_scenario_scheduled_at("wal-append-before-flush"),
        complete_context().with_driver_contracts(
            AdmittedDriverContractSet::from_drivers([production_driver_with_all_seams()]).unwrap(),
        ),
    )
    .unwrap();
    let wrong_seam = event
        .deliver_through(
            wal_plan.yieldpoint_binding(),
            PhysicalBoundaryYieldpoint::wal_append_before_flush(),
            observed_checksum_mismatch_boundary(),
        )
        .unwrap_err();

    assert_eq!(
        unbound,
        FaultDeliveryDenial::UnboundFaultYieldpoint {
            scheduled_yieldpoint: "root-publication-before-observe".to_owned(),
            delivery_yieldpoint: "wal-append-before-flush".to_owned(),
        }
    );
    assert_eq!(
        wrong_seam,
        FaultDeliveryDenial::FaultYieldpointSeamMismatch {
            expected: PhysicalBoundarySeam::ProductionStorage(
                ProductionStorageBoundarySeam::RootPublicationBeforeObserve
            ),
            actual: PhysicalBoundarySeam::ProductionStorage(
                ProductionStorageBoundarySeam::WalAppendBeforeFlush
            ),
        }
    );
}

#[test]
fn observed_boundary_mismatches_deny_typed() {
    let plan = lower_physical_simulation_plan(readiness_scenario(), complete_context()).unwrap();
    let event = PhysicalFaultEvent::byte_corruption(
        ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
        wal_frame_payload_fault_locus(),
    )
    .unwrap();
    let mismatch = event
        .deliver_through(
            plan.yieldpoint_binding(),
            PhysicalBoundaryYieldpoint::root_publication_before_observe(),
            ObservedFaultBoundary::fresh_runtime_crash_recovery(&fresh_runtime_crash_evidence()),
        )
        .unwrap_err();
    assert_eq!(
        mismatch,
        FaultDeliveryDenial::ObservedFaultBoundaryMismatch {
            expected: ExpectedFaultLocalization::PreDecodeBoundary,
            actual: FaultObservedBoundaryKind::FreshRuntimeCrashRecovery,
        }
    );

    let crash_plan =
        lower_physical_simulation_plan(recovery_scenario(), complete_context()).unwrap();
    let crash = PhysicalFaultEvent::crash(
        CrashEvent::fresh_runtime_recovery(
            PhysicalBoundaryYieldpoint::fresh_runtime_replay_open(),
            crash_recovery_fault_locus(),
            fresh_runtime_crash_evidence(),
        )
        .unwrap(),
    );
    assert_eq!(
        crash
            .deliver_through(
                crash_plan.yieldpoint_binding(),
                PhysicalBoundaryYieldpoint::fresh_runtime_replay_open(),
                observed_checksum_mismatch_boundary(),
            )
            .unwrap_err(),
        FaultDeliveryDenial::ObservedFaultBoundaryMismatch {
            expected: ExpectedFaultLocalization::FreshRuntimeRecoveryBoundary,
            actual: FaultObservedBoundaryKind::PreDecodeIntegrityDenial,
        }
    );
}

#[test]
fn no_fault_control_requires_matching_production_boundary_trace_parity() {
    let trace = developer_smoke_production_trace();
    let parity =
        NoFaultProductionBoundaryParity::from_traces(trace.clone(), trace.clone()).unwrap();
    let event = PhysicalFaultEvent::no_fault_control(
        ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
        wal_frame_payload_fault_locus(),
    )
    .unwrap();
    let receipt = event
        .deliver_through(
            lower_physical_simulation_plan(readiness_scenario(), complete_context())
                .unwrap()
                .yieldpoint_binding(),
            PhysicalBoundaryYieldpoint::root_publication_before_observe(),
            ObservedFaultBoundary::no_fault_production_boundary(parity),
        )
        .unwrap();

    assert_eq!(receipt.event_kind(), PhysicalFaultEventKind::NoFaultControl);
    assert!(receipt.actual_boundary().no_fault_parity().is_some());
    assert_eq!(
        NoFaultProductionBoundaryParity::from_traces(trace, alternate_production_trace())
            .unwrap_err(),
        FaultDeliveryDenial::NoFaultParityMismatch
    );
}
