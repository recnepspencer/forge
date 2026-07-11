use forge_store_budgets::CounterEvidenceStrength;
use forge_store_tiering::ColdPlacementState;

use super::test_support::{
    alternate_movement_read_hold, cold_target, copied_scope_reservation, lifecycle_with_bytes,
    mismatched_streaming_read, missing_read_hold_request, movement_case, movement_read_hold,
    physical_execution_for_read_hold, plan_current, same_digest_wrong_identity_streaming_read,
    scoped_reservation_for_lifecycle, stale_request, unavailable_cold_request,
    violated_reservation_request,
};
use super::{
    BlobMovementReadPhase, BlobMovementVerifiedReadEvidence, BlobPlacementMovementAuthority,
    BlobPlacementMovementColdCapsuleOutcome, BlobPlacementMovementColdExportOutcome,
    BlobPlacementMovementColdMaterializationOutcome, BlobPlacementMovementColdOutcome,
    BlobPlacementMovementColdReadOutcome, BlobPlacementMovementDenial,
    BlobPlacementMovementRequest, BlobPlacementMovementRestartOutcome,
    StoreOwnedPlacementMovementExecution, StoreOwnedPlacementMovementPublication,
};

#[test]
fn read_before_during_and_after_move_preserve_blob_basis_and_verified_bytes() {
    let case = movement_case("phase17-parity");
    let read = case.read.clone();
    let plan = plan_current(case).expect("movement plan should admit");
    let before = plan
        .read_guard(BlobMovementReadPhase::BeforeMove)
        .admit_verified_read(read.clone())
        .expect("pre-move read should verify");
    let execution = StoreOwnedPlacementMovementExecution::store_owned()
        .execute_physical_movement(
            &plan,
            physical_execution_for_read_hold(&plan, plan.read_hold()),
        )
        .expect("lower physical execution should bind to plan");
    let executed = plan
        .execute_with_receipt(execution)
        .expect("movement execution receipt should bind to admitted plan");
    let during = executed
        .read_guard(BlobMovementReadPhase::DuringMove)
        .admit_verified_read(read.clone())
        .expect("during-move read should verify");
    let published =
        executed.publish_observation(StoreOwnedPlacementMovementPublication::store_owned());
    let after = published
        .read_guard()
        .admit_verified_read(read.clone())
        .expect("post-move read should verify");

    assert_eq!(before.object_id(), during.object_id());
    assert_eq!(during.object_id(), after.object_id());
    assert_eq!(before.security_metadata(), after.security_metadata());
    assert_eq!(before.stored_digest(), after.stored_digest());
    assert_eq!(before.verified_bytes(), 12);
    assert_eq!(during.verified_bytes(), 12);
    assert_eq!(after.verified_bytes(), 12);
    assert_eq!(
        published.counters().strength(),
        CounterEvidenceStrength::Exact
    );
    assert_eq!(published.counters().placement_moves(), 1);
    assert_eq!(published.counters().inline_reads(), 1);
    assert_eq!(published.counters().external_reads(), 1);
    assert_eq!(published.counters().published_observations(), 1);
}

#[test]
fn movement_denies_stale_missing_read_hold_unavailable_cold_and_reservation_violation() {
    assert!(matches!(
        BlobPlacementMovementAuthority::store_owned()
            .plan_movement(stale_request(movement_case("phase17-stale"))),
        Err(BlobPlacementMovementDenial::StaleMovementPlan { .. })
    ));
    assert!(matches!(
        BlobPlacementMovementAuthority::store_owned().plan_movement(missing_read_hold_request(
            movement_case("phase17-missing-read")
        )),
        Err(BlobPlacementMovementDenial::MissingMovementReadHold { .. })
    ));
    assert!(matches!(
        BlobPlacementMovementAuthority::store_owned().plan_movement(unavailable_cold_request(
            movement_case("phase17-cold-unavailable")
        )),
        Err(BlobPlacementMovementDenial::ColdPlacementUnavailable { .. })
    ));
    assert!(matches!(
        BlobPlacementMovementAuthority::store_owned().plan_movement(violated_reservation_request(
            movement_case("phase17-reservation")
        )),
        Err(BlobPlacementMovementDenial::ForegroundReservationViolated { .. })
    ));
}

#[test]
fn wrong_basis_verified_read_is_denied_before_bytes_are_exposed() {
    let case = movement_case("phase17-read-basis");
    let plan = plan_current(case).expect("movement plan should admit");
    let wrong_read = BlobMovementVerifiedReadEvidence::mismatched_for_certification_test(&plan);

    assert!(matches!(
        plan.read_guard(BlobMovementReadPhase::DuringMove)
            .admit_verified_read(wrong_read),
        Err(BlobPlacementMovementDenial::VerifiedReadBasisMismatch { .. })
    ));
}

#[test]
fn copied_lower_physical_execution_receipt_is_denied_before_publish() {
    let source_plan = plan_current(movement_case("phase17-copied-execution-source"))
        .expect("source movement plan should admit");
    let copied_physical_execution =
        physical_execution_for_read_hold(&source_plan, source_plan.read_hold());
    let copied_execution = StoreOwnedPlacementMovementExecution::store_owned()
        .execute_physical_movement(&source_plan, copied_physical_execution.clone())
        .expect("source physical execution should admit");
    let target_plan = plan_current(movement_case("phase17-copied-execution-target"))
        .expect("movement plan should admit");

    assert!(matches!(
        StoreOwnedPlacementMovementExecution::store_owned()
            .execute_physical_movement(&target_plan, copied_physical_execution),
        Err(BlobPlacementMovementDenial::MovementExecutionReceiptMismatch { .. })
    ));
    assert!(matches!(
        target_plan.execute_with_receipt(copied_execution),
        Err(BlobPlacementMovementDenial::MovementExecutionReceiptMismatch { .. })
    ));
}

#[test]
fn lower_physical_execution_receipt_must_match_physical_isolation_interlock() {
    let plan = plan_current(movement_case("phase17-copied-interlock"))
        .expect("movement plan should admit");

    assert!(matches!(
        StoreOwnedPlacementMovementExecution::store_owned().execute_physical_movement(
            &plan,
            physical_execution_for_read_hold(&plan, alternate_movement_read_hold()),
        ),
        Err(BlobPlacementMovementDenial::MovementExecutionReceiptMismatch { .. })
    ));
}

#[test]
fn streaming_read_proof_must_match_movement_basis() {
    let plan = plan_current(movement_case("phase17-streaming-read-proof"))
        .expect("movement plan should admit");
    let wrong_streaming_read = mismatched_streaming_read(plan.read_hold().guarded_bytes());

    assert!(matches!(
        BlobMovementVerifiedReadEvidence::from_streaming_verified_read(
            &plan,
            plan.read_hold(),
            &wrong_streaming_read
        ),
        Err(BlobPlacementMovementDenial::VerifiedReadBasisMismatch { .. })
    ));
}

#[test]
fn same_digest_streaming_read_proof_must_match_blob_identity() {
    let plan = plan_current(movement_case("phase17-streaming-read-identity"))
        .expect("movement plan should admit");
    let wrong_identity_read =
        same_digest_wrong_identity_streaming_read(&plan, plan.read_hold().guarded_bytes());

    assert!(matches!(
        BlobMovementVerifiedReadEvidence::from_streaming_verified_read(
            &plan,
            plan.read_hold(),
            &wrong_identity_read
        ),
        Err(BlobPlacementMovementDenial::VerifiedReadBasisMismatch { .. })
    ));
}

#[test]
fn target_placement_from_another_lifecycle_basis_is_denied() {
    let mut case = movement_case("phase17-target-basis");
    let wrong_lifecycle = lifecycle_with_bytes("phase17-target-wrong", b"different-basis");
    case.target = crate::placement::admission::test_support::admit_external_placement(
        wrong_lifecycle.reachability(),
    );
    let reservation = scoped_reservation_for_lifecycle(&case.lifecycle);

    assert!(matches!(
        BlobPlacementMovementAuthority::store_owned().plan_movement(
            BlobPlacementMovementRequest::new(
                case.lifecycle,
                case.source,
                case.target,
                movement_read_hold(),
                reservation.into(),
                BlobPlacementMovementColdOutcome::from_state(ColdPlacementState::HotAvailable),
                super::BlobPlacementMovementFreshness::Current,
            )
        ),
        Err(BlobPlacementMovementDenial::LifecycleTargetPlacementBasisMismatch { .. })
    ));
}

#[test]
fn copied_foreground_reservation_scope_is_denied_before_movement_planning() {
    let case = movement_case("phase17-copied-reservation-scope");

    assert!(matches!(
        BlobPlacementMovementAuthority::store_owned().plan_movement(
            BlobPlacementMovementRequest::new(
                case.lifecycle,
                case.source,
                case.target,
                movement_read_hold(),
                copied_scope_reservation().into(),
                BlobPlacementMovementColdOutcome::from_state(ColdPlacementState::HotAvailable),
                super::BlobPlacementMovementFreshness::Current,
            )
        ),
        Err(BlobPlacementMovementDenial::ForegroundReservationScopeMismatch { .. })
    ));
}

#[test]
fn crash_restart_resumes_executed_receipt_or_localizes_residue_without_mixed_publish() {
    let plan = plan_current(movement_case("phase17-crash")).expect("movement plan should admit");
    let execution = StoreOwnedPlacementMovementExecution::store_owned()
        .execute_physical_movement(
            &plan,
            physical_execution_for_read_hold(&plan, plan.read_hold()),
        )
        .expect("lower physical execution should bind to plan");
    let executed = plan
        .execute_with_receipt(execution)
        .expect("movement execution receipt should bind");
    let localized = BlobPlacementMovementRestartOutcome::localize_residue(&executed);
    let resumed = BlobPlacementMovementRestartOutcome::resume_from_receipt(executed);

    assert!(!localized.publishes_mixed_placement());
    assert!(!resumed.publishes_mixed_placement());
}

#[test]
fn cold_lane_states_have_distinct_surface_outcomes() {
    assert_eq!(
        BlobPlacementMovementColdOutcome::from_state(ColdPlacementState::ColdUnavailable)
            .read_outcome(),
        BlobPlacementMovementColdReadOutcome::DeniedUnavailable
    );
    assert_eq!(
        BlobPlacementMovementColdOutcome::from_state(ColdPlacementState::ColdStale)
            .capsule_outcome(),
        BlobPlacementMovementColdCapsuleOutcome::DeniedStale
    );
    assert_eq!(
        BlobPlacementMovementColdOutcome::from_state(ColdPlacementState::ColdScopeDenied)
            .materialization_outcome(),
        BlobPlacementMovementColdMaterializationOutcome::DeniedScope
    );
    assert_eq!(
        BlobPlacementMovementColdOutcome::from_state(ColdPlacementState::ColdRebindRequired)
            .export_outcome(),
        BlobPlacementMovementColdExportOutcome::RebindRequired
    );
}

#[test]
fn cold_target_move_reports_cold_fetch_counter() {
    let mut case = movement_case("phase17-cold-target");
    case.target = cold_target(&case);
    let plan = plan_current(case).expect("cold target movement should admit");
    assert_eq!(plan.counters().cold_fetches(), 1);
}
