//! The bounded two-branch Supply Chain court every owner-phase locality proof
//! runs in.
//!
//! It forks the unrelated branches from one root, parks one of them at a real
//! production interruption boundary, and runs the other branch's full ordinary
//! commit under a bound. Every wait here is bounded on purpose: a phase that
//! never parks, or an unrelated commit that serializes behind the park, must
//! convict by name inside the court's budget instead of hanging the lane.

use std::collections::BTreeSet;
use std::sync::mpsc::{sync_channel, RecvTimeoutError};
use std::sync::{Arc, Barrier};
use std::time::Duration;

use super::invariant_oracle_expectations::expected_supply_chain_branch;
use super::world::supply_chain::{
    commit_branch_batch_with_result, compare, fork_supply_chain_branch_from_main,
    head_for_supply_chain_branch, lower_supply_chain_production_delta,
    observe_supply_chain_snapshot, BranchLabel, DeltaId, ProductionSeededSupplyChainWorld,
};
use worth_relational::facade::branch::RelationalBranchReferenceState;
use worth_relational::facade::history::{BranchId, RelationalCommitReceipt};
use worth_relational::facade::mvcc::{
    BranchBoundRelationalTransaction, RelationalInterruptionBoundary, RelationalOperationControl,
    RelationalTransactionIntent,
};
use worth_relational::facade::runtime::RelationalRuntime;
use worth_relational::facade::transactions::{CommitResult, WorkerIntentBatch};

/// Bound on every court wait; it only decides how fast serialization convicts.
pub(crate) const OWNER_PHASE_COURT_TIMEOUT: Duration = Duration::from_secs(30);

/// The unrelated Supply Chain branches every locality court forks from one root.
pub(crate) struct OwnerPhaseCourtBranches {
    pub(crate) storm: BranchId,
    pub(crate) maintenance: BranchId,
}

/// Owner-state evidence captured while the storm branch is paused.
pub(crate) struct PausedCourtObservation {
    pub(crate) storm_reference: RelationalBranchReferenceState,
    pub(crate) storm_coordination: BranchCoordination,
    maintenance_reference: RelationalBranchReferenceState,
    maintenance_head: RelationalCommitReceipt,
}

/// Coordination accounting the public sharing observation reports for a branch.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BranchCoordination {
    pub(crate) contacts: u64,
    pub(crate) waits: u64,
}

impl OwnerPhaseCourtBranches {
    pub(crate) fn fork(world: &mut ProductionSeededSupplyChainWorld) -> Self {
        let storm = BranchId("storm".to_owned());
        let maintenance = BranchId("maintenance".to_owned());
        fork_supply_chain_branch_from_main(&mut world.runtime, storm.clone());
        fork_supply_chain_branch_from_main(&mut world.runtime, maintenance.clone());
        Self { storm, maintenance }
    }

    pub(crate) fn capture(&self, runtime: &RelationalRuntime) -> PausedCourtObservation {
        PausedCourtObservation {
            storm_reference: branch_reference_state(runtime, &self.storm),
            storm_coordination: coordination_counters(runtime, &self.storm),
            maintenance_reference: branch_reference_state(runtime, &self.maintenance),
            maintenance_head: head_for_supply_chain_branch(runtime, &self.maintenance),
        }
    }
}

impl PausedCourtObservation {
    pub(crate) fn assert_branch_locality(
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
pub(crate) struct OwnerPhasePause {
    reached: Arc<Barrier>,
    release: Arc<Barrier>,
}

impl OwnerPhasePause {
    pub(crate) fn new() -> Self {
        Self {
            reached: Arc::new(Barrier::new(2)),
            release: Arc::new(Barrier::new(2)),
        }
    }

    pub(crate) fn control(
        &self,
        boundary: RelationalInterruptionBoundary,
    ) -> RelationalOperationControl {
        RelationalOperationControl::uninterrupted().with_boundary_pause(
            boundary,
            Arc::clone(&self.reached),
            Arc::clone(&self.release),
        )
    }

    pub(crate) fn await_arrival(&self, phase: &str) {
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

    pub(crate) fn open(&self) {
        self.release.wait();
    }
}

/// Run one full ordinary Supply Chain commit on the unrelated branch while the
/// storm branch holds an owner-phase park. A worker runs it so the court can
/// bound it: only a thread outside the commit can hold the completion receiver
/// and convict serialization by name instead of hanging. The worker is scoped
/// because the commit facade still takes the exclusive runtime receiver the
/// court needs back; every failing exit opens the park before scope teardown.
pub(crate) fn commit_unrelated_branch_while_paused(
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

pub(crate) fn assert_unrelated_commit_matches_oracle(
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

pub(crate) fn lower_court_delta(
    world: &mut ProductionSeededSupplyChainWorld,
    branch: &BranchId,
    delta: DeltaId,
) -> WorkerIntentBatch {
    lower_supply_chain_production_delta(
        &mut world.runtime,
        &world.program,
        &world.handles,
        branch,
        &BTreeSet::new(),
        delta,
    )
    .expect("the production lowerer accepts the court branch's actual observed state")
}

pub(crate) fn begin_court_transaction(
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

pub(crate) fn branch_reference_state(
    runtime: &RelationalRuntime,
    branch: &BranchId,
) -> RelationalBranchReferenceState {
    runtime
        .branch_reference_state(branch)
        .expect("every court branch keeps a live owner reference cell")
}

pub(crate) fn coordination_counters(
    runtime: &RelationalRuntime,
    branch: &BranchId,
) -> BranchCoordination {
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
