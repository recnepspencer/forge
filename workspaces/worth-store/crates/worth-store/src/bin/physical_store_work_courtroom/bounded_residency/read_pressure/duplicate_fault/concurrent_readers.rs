use std::sync::mpsc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::bounded_residency::schedule::{EquivalentContenderIdentity, GateReleaseOrder};
use worth_store::physical_runtime::certification::CertificationPhysicalExecutionCheckpoint;
use worth_store::physical_runtime::{
    PhysicalRecordChunkBasis, PhysicalResidencyCounterSnapshot, RecordReadObservation,
    RecordReadSession, ServingPhysicalRuntime,
};

const WAIT_TIMEOUT: Duration = Duration::from_secs(10);

pub(super) struct ConcurrentReadEvidence {
    pub(super) held: PhysicalResidencyCounterSnapshot,
    pub(super) first_work: u64,
    pub(super) second_work: u64,
    pub(super) same_frame: bool,
    pub(super) same_prefix: bool,
    pub(super) waiter_created_work: bool,
    pub(super) release_order: GateReleaseOrder,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReaderRole {
    First,
    Second,
}

#[derive(Clone, Copy)]
struct HeldLeaseObservation {
    role: ReaderRole,
    basis: PhysicalRecordChunkBasis,
    prefix: [u8; 8],
    work: u64,
}

type ReaderThread = JoinHandle<Result<RecordReadObservation, String>>;

struct RunningReaders {
    gate: Option<
        worth_store::physical_runtime::certification::CertificationPhysicalExecutionPauseGate,
    >,
    evidence: mpsc::Receiver<Result<HeldLeaseObservation, String>>,
    owner_release: Option<mpsc::Sender<()>>,
    waiter_release: Option<mpsc::Sender<()>>,
    owner_thread: Option<ReaderThread>,
    waiter_thread: Option<ReaderThread>,
    waiter_created_work: bool,
}

pub(super) fn execute(
    serving: &ServingPhysicalRuntime,
    owner: RecordReadSession,
    waiter: RecordReadSession,
    before: PhysicalResidencyCounterSnapshot,
    contender_identity: EquivalentContenderIdentity,
    release_order: GateReleaseOrder,
) -> Result<ConcurrentReadEvidence, String> {
    let running = start(serving, owner, waiter, before, contender_identity)?;
    finish(serving, running, release_order)
}

fn start(
    serving: &ServingPhysicalRuntime,
    owner: RecordReadSession,
    waiter: RecordReadSession,
    before: PhysicalResidencyCounterSnapshot,
    contender_identity: EquivalentContenderIdentity,
) -> Result<RunningReaders, String> {
    let (owner_role, owner, waiter_role, waiter) = match contender_identity {
        EquivalentContenderIdentity::FirstOwner => {
            (ReaderRole::First, owner, ReaderRole::Second, waiter)
        }
        EquivalentContenderIdentity::SecondOwner => {
            (ReaderRole::Second, waiter, ReaderRole::First, owner)
        }
    };
    let gate = serving.certification_pause_physical_execution_at(
        CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch,
    );
    let (evidence_tx, evidence) = mpsc::channel();
    let (owner_release, owner_release_rx) = mpsc::channel();
    let owner_thread = spawn_reader(owner_role, owner, evidence_tx.clone(), owner_release_rx);
    let mut running = RunningReaders {
        gate: Some(gate),
        evidence,
        owner_release: Some(owner_release),
        waiter_release: None,
        owner_thread: Some(owner_thread),
        waiter_thread: None,
        waiter_created_work: false,
    };
    if !running.gate().await_arrival() {
        return Err("duplicate-fault owner never reached backend dispatch".to_owned());
    }

    let work_before = serving.physical_work_counters();
    let media_before = serving.media_counters();
    let signal_before = serving.physical_signal_observation();
    let (waiter_release, waiter_release_rx) = mpsc::channel();
    let waiter_thread = spawn_reader(waiter_role, waiter, evidence_tx, waiter_release_rx);
    running.waiter_release = Some(waiter_release);
    running.waiter_thread = Some(waiter_thread);
    await_waiter(serving, before.coalesced_waiters())?;
    running.waiter_created_work = serving.physical_work_counters() != work_before
        || serving.media_counters() != media_before
        || serving.physical_signal_observation() != signal_before;
    running.gate().release();
    Ok(running)
}

fn finish(
    serving: &ServingPhysicalRuntime,
    mut running: RunningReaders,
    release_order: GateReleaseOrder,
) -> Result<ConcurrentReadEvidence, String> {
    let first = receive(&running.evidence)?;
    let second = receive(&running.evidence)?;
    let held = serving.residency_observation().counters();
    let (owner_observation, waiter_observation) = match release_order {
        GateReleaseOrder::OwnerThenWaiter => {
            let owner = running.release_owner();
            let waiter = running.release_waiter();
            (owner?, waiter?)
        }
        GateReleaseOrder::WaiterThenOwner => {
            let waiter = running.release_waiter();
            let owner = running.release_owner();
            (owner?, waiter?)
        }
    };
    if owner_observation.explicit_copy_count() != 0 || waiter_observation.explicit_copy_count() != 0
    {
        return Err("duplicate-fault borrowed views performed explicit copies".to_owned());
    }
    let (first, second) = order(first, second)?;
    Ok(ConcurrentReadEvidence {
        held,
        first_work: first.work,
        second_work: second.work,
        same_frame: first.basis == second.basis,
        same_prefix: first.prefix == second.prefix,
        waiter_created_work: running.waiter_created_work,
        release_order,
    })
}

impl RunningReaders {
    fn gate(
        &self,
    ) -> &worth_store::physical_runtime::certification::CertificationPhysicalExecutionPauseGate
    {
        self.gate
            .as_ref()
            .expect("running duplicate-fault readers own their executor gate")
    }

    fn release_owner(&mut self) -> Result<RecordReadObservation, String> {
        release_and_join(
            self.owner_release
                .take()
                .expect("owner release is consumed exactly once"),
            self.owner_thread
                .take()
                .expect("owner thread is joined exactly once"),
            "owner",
        )
    }

    fn release_waiter(&mut self) -> Result<RecordReadObservation, String> {
        release_and_join(
            self.waiter_release
                .take()
                .expect("waiter release is consumed exactly once"),
            self.waiter_thread
                .take()
                .expect("waiter thread is joined exactly once"),
            "waiter",
        )
    }
}

impl Drop for RunningReaders {
    fn drop(&mut self) {
        drop(self.gate.take());
        if let Some(release) = self.owner_release.take() {
            let _ = release.send(());
        }
        if let Some(release) = self.waiter_release.take() {
            let _ = release.send(());
        }
        if let Some(thread) = self.owner_thread.take() {
            let _ = thread.join();
        }
        if let Some(thread) = self.waiter_thread.take() {
            let _ = thread.join();
        }
    }
}

fn release_and_join<T>(
    sender: mpsc::Sender<()>,
    thread: JoinHandle<Result<T, String>>,
    role: &str,
) -> Result<T, String> {
    let release = sender
        .send(())
        .map_err(|_| format!("duplicate-fault {role} release failed"));
    let joined = join(thread, role);
    release?;
    joined
}

fn await_waiter(
    serving: &ServingPhysicalRuntime,
    coalesced_waiters_before: u64,
) -> Result<(), String> {
    let deadline = Instant::now() + WAIT_TIMEOUT;
    while serving
        .residency_observation()
        .counters()
        .coalesced_waiters()
        == coalesced_waiters_before
    {
        if Instant::now() >= deadline {
            return Err("duplicate-fault waiter never joined the loading frame".to_owned());
        }
        std::thread::yield_now();
    }
    Ok(())
}

fn receive(
    evidence: &mpsc::Receiver<Result<HeldLeaseObservation, String>>,
) -> Result<HeldLeaseObservation, String> {
    evidence
        .recv_timeout(WAIT_TIMEOUT)
        .map_err(|_| "duplicate-fault owner/waiter evidence timed out".to_owned())?
}

fn join<T>(thread: JoinHandle<Result<T, String>>, role: &str) -> Result<T, String> {
    thread
        .join()
        .map_err(|_| format!("duplicate-fault {role} panicked"))?
}

fn spawn_reader(
    role: ReaderRole,
    mut session: RecordReadSession,
    evidence: mpsc::Sender<Result<HeldLeaseObservation, String>>,
    release: mpsc::Receiver<()>,
) -> ReaderThread {
    std::thread::spawn(move || {
        let before = session.observation().physical_work_count();
        let observed = observe_held_lease(&mut session, role, before);
        evidence
            .send(observed.clone())
            .map_err(|_| "duplicate-fault evidence receiver disappeared".to_owned())?;
        release
            .recv()
            .map_err(|_| "duplicate-fault release sender disappeared".to_owned())?;
        observed.map(|_| session.observation())
    })
}

fn observe_held_lease(
    session: &mut RecordReadSession,
    role: ReaderRole,
    work_before: u64,
) -> Result<HeldLeaseObservation, String> {
    let (prefix, basis) = {
        let view = session
            .next_chunk()
            .map_err(|failure| format!("duplicate-fault view failed: {failure:?}"))?
            .ok_or_else(|| "duplicate-fault view found no payload".to_owned())?;
        let prefix = view
            .bytes()
            .get(..8)
            .ok_or_else(|| "duplicate-fault view omitted workload prefix".to_owned())?
            .try_into()
            .expect("an eight-byte slice has exact array width");
        (prefix, view.basis())
    };
    let work = session
        .observation()
        .physical_work_count()
        .checked_sub(work_before)
        .ok_or_else(|| "duplicate-fault work counter regressed".to_owned())?;
    Ok(HeldLeaseObservation {
        role,
        basis,
        prefix,
        work,
    })
}

fn order(
    first: HeldLeaseObservation,
    second: HeldLeaseObservation,
) -> Result<(HeldLeaseObservation, HeldLeaseObservation), String> {
    match (first.role, second.role) {
        (ReaderRole::First, ReaderRole::Second) => Ok((first, second)),
        (ReaderRole::Second, ReaderRole::First) => Ok((second, first)),
        _ => Err("duplicate-fault evidence duplicated a reader role".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{mpsc, Arc};
    use std::time::Duration;

    use super::release_and_join;

    #[test]
    fn failed_release_still_joins_the_started_worker() {
        let (release, receiver) = mpsc::channel();
        let (receiver_dropped, dropped) = mpsc::channel();
        let completed = Arc::new(AtomicBool::new(false));
        let worker_completed = Arc::clone(&completed);
        let worker = std::thread::spawn(move || {
            drop(receiver);
            receiver_dropped.send(()).unwrap();
            std::thread::sleep(Duration::from_millis(20));
            worker_completed.store(true, Ordering::Release);
            Ok(())
        });
        dropped.recv_timeout(Duration::from_secs(1)).unwrap();

        let failure = release_and_join(release, worker, "test").unwrap_err();
        if !completed.load(Ordering::Acquire) {
            panic!("MUTANT_PREDICATE:schedule-release-error-detaches-worker");
        }
        assert_eq!(failure, "duplicate-fault test release failed");
    }
}
