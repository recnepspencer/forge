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

use std::collections::BTreeSet;
use std::sync::mpsc::{sync_channel, RecvTimeoutError};
use std::sync::{Arc, Barrier};
use std::time::Duration;

use super::invariant_oracle_expectations::expected_supply_chain_branch;
use super::world::supply_chain::{
    assert_oracle_matches, certified_supply_chain_world, commit_branch_batch_with_result, compare,
    fork_supply_chain_branch_from_main, head_for_supply_chain_branch,
    lower_supply_chain_production_delta, observe_supply_chain_snapshot, BranchLabel, DeltaId,
    ProductionSeededSupplyChainWorld, SupplyChainScale,
};
use worth_relational::facade::branch::RelationalBranchReferenceState;
use worth_relational::facade::history::{BranchId, RelationalCommitReceipt};
use worth_relational::facade::mvcc::{
    BranchBoundRelationalTransaction, RelationalInterruptionBoundary, RelationalOperationControl,
    RelationalPublicationOutcome, RelationalTransactionIntent,
};
use worth_relational::facade::runtime::RelationalRuntime;
use worth_relational::facade::transactions::{CommitResult, WorkerIntentBatch};

/// Bound on every court wait; it only decides how fast serialization convicts.
const OWNER_PHASE_COURT_TIMEOUT: Duration = Duration::from_secs(30);

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

/// The unrelated Supply Chain branches every locality court forks from one root.
struct OwnerPhaseCourtBranches {
    storm: BranchId,
    maintenance: BranchId,
}

/// Owner-state evidence captured while the storm branch is paused.
struct PausedCourtObservation {
    storm_reference: RelationalBranchReferenceState,
    storm_coordination: BranchCoordination,
    maintenance_reference: RelationalBranchReferenceState,
    maintenance_head: RelationalCommitReceipt,
}

/// Coordination accounting the public sharing observation reports for a branch.
#[derive(Debug, PartialEq, Eq)]
struct BranchCoordination {
    contacts: u64,
    waits: u64,
}

impl OwnerPhaseCourtBranches {
    fn fork(world: &mut ProductionSeededSupplyChainWorld) -> Self {
        let storm = BranchId("storm".to_owned());
        let maintenance = BranchId("maintenance".to_owned());
        fork_supply_chain_branch_from_main(&world.runtime, storm.clone());
        fork_supply_chain_branch_from_main(&world.runtime, maintenance.clone());
        Self { storm, maintenance }
    }

    fn capture(&self, runtime: &RelationalRuntime) -> PausedCourtObservation {
        PausedCourtObservation {
            storm_reference: branch_reference_state(runtime, &self.storm),
            storm_coordination: coordination_counters(runtime, &self.storm),
            maintenance_reference: branch_reference_state(runtime, &self.maintenance),
            maintenance_head: head_for_supply_chain_branch(runtime, &self.maintenance),
        }
    }
}

impl PausedCourtObservation {
    fn assert_branch_locality(
        &self,
        court: &OwnerPhaseCourtBranches,
        runtime: &RelationalRuntime,
        committed: &CommitResult,
    ) {
        assert_eq!(
            branch_reference_state(runtime, &court.storm),
            self.storm_reference,
            "an unrelated ordinary commit must not move the paused storm reference"
        );
        assert_eq!(
            coordination_counters(runtime, &court.storm),
            self.storm_coordination,
            "an unrelated ordinary commit must add exactly zero storm contacts and waits"
        );
        assert_ne!(
            branch_reference_state(runtime, &court.maintenance),
            self.maintenance_reference,
            "the unrelated maintenance reference must move while storm stays paused"
        );
        assert_eq!(committed.commit.branch_id, court.maintenance);
        assert_eq!(
            committed.commit.parents,
            vec![self.maintenance_head.commit_id],
            "the unrelated maintenance head advances exactly one canonical commit"
        );
        assert_eq!(
            head_for_supply_chain_branch(runtime, &court.maintenance),
            committed.commit,
            "the observed maintenance head is exactly the returned canonical commit"
        );
    }
}

/// One owner-phase pause held by a worker branch. Arrival is observed through a
/// proxy thread and a bounded channel, so a phase that never parks convicts by
/// name instead of blocking the court on an unbounded barrier.
struct OwnerPhasePause {
    reached: Arc<Barrier>,
    release: Arc<Barrier>,
}

impl OwnerPhasePause {
    fn new() -> Self {
        Self {
            reached: Arc::new(Barrier::new(2)),
            release: Arc::new(Barrier::new(2)),
        }
    }

    fn control(&self, boundary: RelationalInterruptionBoundary) -> RelationalOperationControl {
        RelationalOperationControl::uninterrupted().with_boundary_pause(
            boundary,
            Arc::clone(&self.reached),
            Arc::clone(&self.release),
        )
    }

    fn await_arrival(&self, phase: &str) {
        let (arrived, arrival) = sync_channel(1);
        let reached = Arc::clone(&self.reached);
        let proxy = std::thread::spawn(move || {
            reached.wait();
            let _ = arrived.send(());
        });
        if arrival.recv_timeout(OWNER_PHASE_COURT_TIMEOUT).is_err() {
            panic!("the storm branch never reached its {phase} pause");
        }
        proxy.join().expect("the pause-arrival proxy joins");
    }

    fn open(&self) {
        self.release.wait();
    }
}

/// Run one full ordinary Supply Chain commit on the unrelated branch while the
/// storm branch holds an owner-phase park. A worker runs it so the court can
/// bound it: only a thread outside the commit can hold the completion receiver
/// and convict serialization by name instead of hanging. The worker is scoped
/// because the commit facade still takes the exclusive runtime receiver the
/// court needs back; every failing exit opens the park before scope teardown.
fn commit_unrelated_branch_while_paused(
    world: &mut ProductionSeededSupplyChainWorld,
    court: &OwnerPhaseCourtBranches,
    batch: WorkerIntentBatch,
    pause: &OwnerPhasePause,
    phase: &str,
) -> CommitResult {
    let (finished, completion) = sync_channel(1);
    let branch = court.maintenance.clone();
    std::thread::scope(|scope| {
        let runtime = &mut world.runtime;
        let committer = scope.spawn(move || {
            let committed = commit_branch_batch_with_result(runtime, branch, batch);
            finished
                .send(())
                .expect("the locality court still owns the completion receiver");
            committed
        });
        match completion.recv_timeout(OWNER_PHASE_COURT_TIMEOUT) {
            Ok(()) => {}
            Err(RecvTimeoutError::Timeout) => {
                pause.open();
                panic!(
                    "the unrelated maintenance commit serialized behind the paused storm {phase}"
                );
            }
            Err(RecvTimeoutError::Disconnected) => pause.open(),
        }
        match committer.join() {
            Ok(committed) => committed,
            Err(worker_panic) => std::panic::resume_unwind(worker_panic),
        }
    })
}

fn assert_unrelated_commit_matches_oracle(
    world: &mut ProductionSeededSupplyChainWorld,
    committed: &CommitResult,
) {
    let observed = observe_supply_chain_snapshot(
        &world.program,
        &world.handles.for_snapshot(committed.snapshot.clone()),
        &world.runtime,
        &committed.snapshot,
    )
    .expect("the performed maintenance root is observable through its exact snapshot");
    let expected = expected_supply_chain_branch(
        &world.program,
        BranchLabel::Maintenance,
        Some(DeltaId::MaintainAtlasBerth),
    );
    compare(&expected, &observed)
        .expect("the unrelated maintenance commit matches the independent Supply Chain oracle");
    world
        .runtime
        .snapshots()
        .release_snapshot(&committed.snapshot)
        .expect("the exact maintenance commit snapshot closes once comparison completes");
}

fn lower_court_delta(
    world: &mut ProductionSeededSupplyChainWorld,
    branch: &BranchId,
    delta: DeltaId,
) -> WorkerIntentBatch {
    lower_supply_chain_production_delta(
        &world.runtime,
        &world.program,
        &world.handles,
        branch,
        &BTreeSet::new(),
        delta,
    )
    .expect("the production lowerer accepts the court branch's actual observed state")
}

fn begin_court_transaction(
    runtime: &RelationalRuntime,
    branch: &BranchId,
    control: RelationalOperationControl,
    batch: WorkerIntentBatch,
) -> BranchBoundRelationalTransaction {
    let identity = runtime
        .branch_identity(branch)
        .expect("the court branch identity is owner-issued");
    let basis = runtime
        .admit_branch_basis(&identity)
        .expect("the court branch basis is owner-admitted");
    let mut transaction = runtime
        .begin_branch_transaction_with_control(
            &basis,
            RelationalTransactionIntent::ordinary(),
            control,
        )
        .expect("the controlled court transaction binds to its exact admitted basis");
    transaction
        .push_batch(batch)
        .expect("the lowered Supply Chain delta fits the configured transaction budget");
    transaction
}

fn branch_reference_state(
    runtime: &RelationalRuntime,
    branch: &BranchId,
) -> RelationalBranchReferenceState {
    runtime
        .branch_reference_state(branch)
        .expect("every court branch keeps a live owner reference cell")
}

fn coordination_counters(runtime: &RelationalRuntime, branch: &BranchId) -> BranchCoordination {
    let identity = runtime
        .branch_identity(branch)
        .expect("the court branch identity is owner-issued");
    let observation = runtime
        .observe_branch_sharing(std::slice::from_ref(&identity))
        .expect("the paused court branch remains inspectable through public sharing");
    BranchCoordination {
        contacts: observation.coordination_contacts(),
        waits: observation.coordination_waits(),
    }
}
