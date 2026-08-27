use super::{
    UiRecordedServiceProposalOwnerPort, UiReservedServiceProposal, UiServiceFamilyProposal,
    UiServiceProposalCandidate, UiServiceProposalCompiler, UiServiceProposalDemand,
    UiServiceProposalReservationOutcome, UiServiceProposalTerminalReason,
};
use crate::runtime::session::service_proposal::{
    fixture_service_family_participation, fixture_service_request_coherence,
    UiServiceProposalConflictDisposition, UiServiceProposalConflictPolicy,
    UiServiceProposalOccupancyDenial,
};

#[test]
fn supersession_and_cancellation_close_at_the_first_staging_effect() {
    let mut compiler = UiServiceProposalCompiler::new();
    let coherence = fixture_service_request_coherence(200);
    let incumbent = reserve(
        &mut compiler,
        &coherence,
        200,
        UiServiceProposalConflictPolicy::RejectOccupied,
    );
    let mut staging = compiler.begin_staging(incumbent).unwrap();
    record_first_portal_effect(&mut compiler, &mut staging, 200);
    let before = compiler.census();

    for (identity, policy) in [
        (201, UiServiceProposalConflictPolicy::SupersedeBeforeEffect),
        (202, UiServiceProposalConflictPolicy::CancelBeforeEffect),
    ] {
        let candidate = preflight(&mut compiler, &coherence, identity, policy, 1);
        assert!(matches!(
            compiler.reserve(candidate),
            Err(super::UiServiceProposalReservationDenial::Occupancy(
                UiServiceProposalOccupancyDenial::BeforeEffectWindowClosed(found)
            )) if found == super::UiServiceProposalIdentity::for_test(200)
        ));
        assert_eq!(compiler.census(), before);
    }

    let proposal = staging.identity();
    let mut teardown = compiler.shutdown_staging(staging);
    let owner = UiRecordedServiceProposalOwnerPort::recorded_fixture(
        crate::capability::UiRuntimeServiceFamily::Portal,
        super::super::UiServiceProposalOccupancyScopeIdentity::for_test(1),
    );
    compiler
        .acknowledge_terminal_owner(
            &mut teardown,
            owner.terminal_outcome(
                proposal,
                UiServiceProposalTerminalReason::AbandonedAtShutdown,
            ),
        )
        .unwrap();
    compiler.finish_teardown(teardown).unwrap();
    assert!(compiler.census().is_zero());
}

#[test]
fn cancel_before_effect_is_an_explicit_aba_safe_displacement() {
    let mut compiler = UiServiceProposalCompiler::new();
    let coherence = fixture_service_request_coherence(210);
    let incumbent = reserve(
        &mut compiler,
        &coherence,
        210,
        UiServiceProposalConflictPolicy::RejectOccupied,
    );
    let successor = reserve(
        &mut compiler,
        &coherence,
        211,
        UiServiceProposalConflictPolicy::CancelBeforeEffect,
    );
    assert_eq!(
        successor.displacement().unwrap().disposition(),
        UiServiceProposalConflictDisposition::CancelledBeforeEffect
    );
    assert!(compiler.cancel_before_effect(incumbent).is_err());
    compiler.cancel_before_effect(successor).unwrap();
    assert!(compiler.census().is_zero());
}

#[test]
fn coalesce_exact_rejects_a_non_equivalent_demand() {
    let mut compiler = UiServiceProposalCompiler::new();
    let coherence = fixture_service_request_coherence(220);
    let incumbent = reserve(
        &mut compiler,
        &coherence,
        220,
        UiServiceProposalConflictPolicy::RejectOccupied,
    );
    let candidate = preflight(
        &mut compiler,
        &coherence,
        221,
        UiServiceProposalConflictPolicy::CoalesceExact,
        2,
    );
    let before = compiler.census();
    assert!(matches!(
        compiler.reserve(candidate),
        Err(super::UiServiceProposalReservationDenial::Occupancy(
            UiServiceProposalOccupancyDenial::AmbiguousConflict
        ))
    ));
    assert_eq!(compiler.census(), before);
    compiler.cancel_before_effect(incumbent).unwrap();
}

#[test]
fn coalesce_exact_rejects_same_total_with_different_family_distribution() {
    let mut compiler = UiServiceProposalCompiler::new();
    let coherence = fixture_service_request_coherence(225);
    let participation = fixture_service_family_participation(2);
    let incumbent = UiServiceProposalCandidate::for_test(
        225,
        UiServiceProposalDemand::recorded_fixture(participation, 2, 3, 3),
        coherence.clone(),
        vec![
            UiServiceFamilyProposal::recorded_fixture(
                crate::capability::UiRuntimeServiceFamily::Portal,
                1,
                1,
                1,
                2,
            ),
            UiServiceFamilyProposal::recorded_fixture(
                crate::capability::UiRuntimeServiceFamily::Focus,
                1,
                1,
                2,
                1,
            ),
        ],
    );
    let incumbent = compiler
        .preflight(incumbent, &coherence, portal_and_focus_support())
        .unwrap();
    let incumbent = match compiler.reserve(incumbent).unwrap() {
        UiServiceProposalReservationOutcome::Reserved(reservation) => reservation,
        UiServiceProposalReservationOutcome::Coalesced { .. } => unreachable!(),
    };
    let candidate = UiServiceProposalCandidate::for_test(
        226,
        UiServiceProposalDemand::recorded_fixture(participation, 2, 3, 3),
        coherence.clone(),
        vec![
            UiServiceFamilyProposal::recorded_fixture(
                crate::capability::UiRuntimeServiceFamily::Portal,
                1,
                1,
                2,
                1,
            )
            .with_conflict_policy(UiServiceProposalConflictPolicy::CoalesceExact),
            UiServiceFamilyProposal::recorded_fixture(
                crate::capability::UiRuntimeServiceFamily::Focus,
                1,
                1,
                1,
                2,
            )
            .with_conflict_policy(UiServiceProposalConflictPolicy::CoalesceExact),
        ],
    );
    let candidate = compiler
        .preflight(candidate, &coherence, portal_and_focus_support())
        .unwrap();
    let before = compiler.census();

    assert!(matches!(
        compiler.reserve(candidate),
        Err(super::UiServiceProposalReservationDenial::Occupancy(
            UiServiceProposalOccupancyDenial::AmbiguousConflict
        ))
    ));
    assert_eq!(compiler.census(), before);
    compiler.cancel_before_effect(incumbent).unwrap();
    assert!(compiler.census().is_zero());
}

#[test]
fn mixed_shutdown_abandons_open_work_and_returns_typed_closed_remainder() {
    let mut compiler = UiServiceProposalCompiler::new();
    let closed_coherence = fixture_service_request_coherence(230);
    let closed = reserve(
        &mut compiler,
        &closed_coherence,
        230,
        UiServiceProposalConflictPolicy::RejectOccupied,
    );
    let mut staging = compiler.begin_staging(closed).unwrap();
    record_first_portal_effect(&mut compiler, &mut staging, 230);

    let open_coherence = fixture_service_request_coherence(231);
    let open = reserve(
        &mut compiler,
        &open_coherence,
        231,
        UiServiceProposalConflictPolicy::RejectOccupied,
    );
    core::mem::forget(open);
    core::mem::forget(staging);

    let denial = compiler.shutdown_all_before_effect().unwrap_err();
    let super::UiServiceProposalTeardownDenial::AwaitingOwnerSettlement(progress) = denial else {
        panic!("closed owner work must be the typed shutdown remainder");
    };
    assert_eq!(progress.abandoned_proposals(), 1);
    assert_eq!(progress.abandoned_leases(), 1);
    assert_eq!(
        progress.final_census().entries(),
        [
            ("proposals", 1),
            ("occupancy_leases", 1),
            ("cancellation_records", 1),
            ("stage_receipts", 1),
        ]
    );
    assert_eq!(compiler.live_occupancy_count(), 1);
    assert_eq!(compiler.live_cancellation_count(), 1);
}

fn record_first_portal_effect(
    compiler: &mut UiServiceProposalCompiler,
    staging: &mut super::UiServiceProposalStaging,
    identity: u64,
) {
    let family = crate::capability::UiRuntimeServiceFamily::Portal;
    let scope = super::super::UiServiceProposalOccupancyScopeIdentity::for_test(1);
    compiler
        .advance_staging(
            staging,
            super::UiServiceProposalStageReceipt::recorded_family_fixture(
                super::UiServiceProposalIdentity::for_test(identity),
                family,
                scope,
                vec![super::UiServiceProducedFactReference::recorded_fixture(
                    identity, family, scope,
                )],
                vec![super::UiServiceMountedWorkReference::recorded_fixture(
                    identity, family, scope,
                )],
            ),
        )
        .unwrap();
}

fn reserve(
    compiler: &mut UiServiceProposalCompiler,
    coherence: &super::super::UiServiceRequestCoherence,
    identity: u64,
    policy: UiServiceProposalConflictPolicy,
) -> UiReservedServiceProposal {
    let preflighted = preflight(compiler, coherence, identity, policy, 1);
    match compiler.reserve(preflighted).unwrap() {
        UiServiceProposalReservationOutcome::Reserved(reservation) => reservation,
        UiServiceProposalReservationOutcome::Coalesced { .. } => unreachable!(),
    }
}

fn preflight(
    compiler: &mut UiServiceProposalCompiler,
    coherence: &super::super::UiServiceRequestCoherence,
    identity: u64,
    policy: UiServiceProposalConflictPolicy,
    requirements: u8,
) -> super::UiPreflightedServiceProposal {
    let candidate = UiServiceProposalCandidate::for_test(
        identity,
        UiServiceProposalDemand::recorded_fixture(
            fixture_service_family_participation(1),
            requirements,
            1,
            1,
        ),
        coherence.clone(),
        vec![UiServiceFamilyProposal::recorded_fixture(
            crate::capability::UiRuntimeServiceFamily::Portal,
            1,
            requirements,
            1,
            1,
        )
        .with_conflict_policy(policy)],
    );
    compiler
        .preflight(
            candidate,
            coherence,
            crate::capability::UiRuntimeServiceSupport::none_installed()
                .with_installed(crate::capability::UiRuntimeServiceFamily::Portal),
        )
        .unwrap()
}

fn portal_and_focus_support() -> crate::capability::UiRuntimeServiceSupport {
    crate::capability::UiRuntimeServiceSupport::none_installed()
        .with_installed(crate::capability::UiRuntimeServiceFamily::Portal)
        .with_installed(crate::capability::UiRuntimeServiceFamily::Focus)
}
