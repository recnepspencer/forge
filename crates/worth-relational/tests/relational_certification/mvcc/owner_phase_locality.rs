//! Production-boundary locality courts for the Relational owner phases.
//!
//! Each court parks one Supply Chain branch inside a real owner phase and
//! requires an unrelated branch to complete a full ordinary commit through the
//! public facade while that park is held. A whole-runtime exclusive borrow
//! convicts at compile time, never here; what these courts convict is a runtime
//! gate, a global lock or shared cell, that serializes independent branches.
//! Preparation parks at the first `CandidatePreparation` observation near the
//! top of the phase, so it cannot speak for a gate taken later in it;
//! publication parks at `BeforeCriticalSection`, inside the entered phase and
//! before the branch coordination cell, which is the stronger position.

use super::mvcc_owner_phase_locality_court::{
    assert_unrelated_commit_matches_oracle, begin_court_transaction, branch_reference_state,
    commit_unrelated_branch_while_paused, coordination_counters, lower_court_delta,
    BranchCoordination, OwnerPhaseCourtBranches, OwnerPhasePause,
};
use super::world::supply_chain::{
    assert_oracle_matches, certified_supply_chain_world, DeltaId, SupplyChainScale,
};
use worth_relational::facade::mvcc::{
    RelationalInterruptionBoundary, RelationalPublicationOutcome,
};

#[test]
fn paused_supply_chain_preparation_leaves_an_unrelated_branch_commit_unblocked() {
    let (mut world, expected) = certified_supply_chain_world(SupplyChainScale::court());
    assert_oracle_matches(&world, &expected);
    let court = OwnerPhaseCourtBranches::fork(&mut world);
    let storm_batch = lower_court_delta(&mut world, &court.storm, DeltaId::StormRerouteAurora);
    let maintenance_batch =
        lower_court_delta(&mut world, &court.maintenance, DeltaId::MaintainAtlasBerth);

    let pause = OwnerPhasePause::new();
    let storm_transaction = begin_court_transaction(
        &world.runtime,
        &court.storm,
        pause.control(RelationalInterruptionBoundary::CandidatePreparation),
        storm_batch,
    );
    let preparation = world.runtime.preparation_port();
    let storm_worker =
        std::thread::spawn(move || preparation.prepare_branch_transaction(storm_transaction));
    pause.await_arrival("candidate preparation");

    let before = court.capture(&world.runtime);
    let committed = commit_unrelated_branch_while_paused(
        &mut world,
        &court,
        maintenance_batch,
        &pause,
        "candidate preparation",
    );
    assert!(
        !storm_worker.is_finished(),
        "the storm branch must stay parked in candidate preparation for the whole unrelated commit"
    );
    before.assert_branch_locality(&court, &world.runtime, &committed);

    pause.open();
    let storm_candidate = storm_worker
        .join()
        .expect("the storm preparation worker joins")
        .expect("the storm branch prepares once its owner-phase pause opens");
    let preparation = world.runtime.preparation_port();
    preparation
        .discard_prepared_candidate(storm_candidate)
        .expect("the released storm candidate discards through its own owner port");
    assert_unrelated_commit_matches_oracle(&mut world, &committed);
    assert_oracle_matches(&world, &expected);
}

#[test]
fn paused_supply_chain_publication_leaves_an_unrelated_branch_commit_unblocked() {
    let (mut world, expected) = certified_supply_chain_world(SupplyChainScale::court());
    assert_oracle_matches(&world, &expected);
    let court = OwnerPhaseCourtBranches::fork(&mut world);
    let storm_batch = lower_court_delta(&mut world, &court.storm, DeltaId::StormRerouteAurora);
    let maintenance_batch =
        lower_court_delta(&mut world, &court.maintenance, DeltaId::MaintainAtlasBerth);

    let pause = OwnerPhasePause::new();
    let storm_transaction = begin_court_transaction(
        &world.runtime,
        &court.storm,
        pause.control(RelationalInterruptionBoundary::BeforeCriticalSection),
        storm_batch,
    );
    let storm_candidate = world
        .runtime
        .preparation_port()
        .prepare_branch_transaction(storm_transaction)
        .expect("the storm candidate prepares before the publication court opens");
    let publication = world.runtime.publication_port();
    let storm_worker = std::thread::spawn(move || publication.compare_and_publish(storm_candidate));
    pause.await_arrival("publication critical-section entry");

    let before = court.capture(&world.runtime);
    let committed = commit_unrelated_branch_while_paused(
        &mut world,
        &court,
        maintenance_batch,
        &pause,
        "publication critical-section entry",
    );
    assert!(
        !storm_worker.is_finished(),
        "the storm branch must stay parked at its critical section for the whole unrelated commit"
    );
    before.assert_branch_locality(&court, &world.runtime, &committed);

    pause.open();
    let performed = match storm_worker.join().expect("the storm publisher joins") {
        RelationalPublicationOutcome::Performed(performed) => performed,
        outcome => panic!("the released storm publication must perform: {outcome:?}"),
    };
    let storm_committed = world
        .runtime
        .settle_performed_publication(performed)
        .expect("the released storm publication settles through its owner runtime");
    world
        .runtime
        .snapshots()
        .release_snapshot(&storm_committed.snapshot)
        .expect("the exact storm settlement snapshot closes");
    assert_ne!(
        branch_reference_state(&world.runtime, &court.storm),
        before.storm_reference,
        "the released storm publication moves its own branch reference"
    );
    assert_eq!(
        coordination_counters(&world.runtime, &court.storm),
        BranchCoordination {
            contacts: before.storm_coordination.contacts + 1,
            waits: before.storm_coordination.waits,
        },
        "storm's own publication charges exactly one uncontended storm contact, so the zero-delta claim measures a live counter"
    );
    assert_unrelated_commit_matches_oracle(&mut world, &committed);
    assert_oracle_matches(&world, &expected);
}
