use super::UiServiceProposalCompiler;

#[test]
fn compiler_owned_shutdown_drains_an_unclaimed_before_effect_reservation() {
    let mut compiler = UiServiceProposalCompiler::new();
    let coherence = super::super::fixture_service_request_coherence(17);
    let reservation = super::tests::reserve(
        &mut compiler,
        &coherence,
        17,
        super::super::UiServiceProposalConflictPolicy::RejectOccupied,
    );
    core::mem::forget(reservation);
    let receipt = compiler.shutdown_all_before_effect().unwrap();
    assert_eq!(receipt.abandoned_proposals(), 1);
    assert_eq!(receipt.abandoned_leases(), 1);
    assert!(receipt.final_census().is_zero());
    assert!(compiler.census().is_zero());
}

#[test]
fn compiler_owned_shutdown_drains_forgotten_zero_witness_staging() {
    let mut compiler = UiServiceProposalCompiler::new();
    let coherence = super::super::fixture_service_request_coherence(18);
    let reservation = super::tests::reserve(
        &mut compiler,
        &coherence,
        18,
        super::super::UiServiceProposalConflictPolicy::RejectOccupied,
    );
    let staging = compiler.begin_staging(reservation).unwrap();
    core::mem::forget(staging);

    let receipt = compiler.shutdown_all_before_effect().unwrap();
    assert_eq!(receipt.abandoned_proposals(), 1);
    assert_eq!(receipt.abandoned_leases(), 1);
    assert!(receipt.is_complete());
    assert!(compiler.census().is_zero());
}
