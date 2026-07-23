use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc,
        Arc, Mutex,
    },
    thread::JoinHandle,
};

use worth_signal::facade::RuntimeClockBasis;

use crate::physical_runtime::work::{PhysicalSignalAspectBindingDigest, PhysicalSignalAspectBindingSet};

use super::{
    graph::PhysicalSignalGraph, PhysicalSignalAdmissionStatus, PhysicalSignalConstructionFailure,
    route::{
        PhysicalSignalRouteCommand, PhysicalSignalRouteMailbox, PhysicalSignalRouteOwner,
        ROUTE_COMMAND_CAPACITY,
    },
    wake::PhysicalSignalWorkerWake,
};

pub(super) struct PhysicalSignalGraphWorker {
    routes: Box<[PhysicalSignalRouteOwner]>,
    worker: Option<JoinHandle<()>>,
    shared: PhysicalSignalWorkerShared,
}

#[derive(Clone)]
struct PhysicalSignalWorkerShared {
    stopping: Arc<AtomicBool>,
    wake: Arc<PhysicalSignalWorkerWake>,
    clock: Arc<Mutex<RuntimeClockBasis>>,
    admission: PhysicalSignalAdmissionStatus,
}

struct PhysicalSignalWorkerBoot {
    bindings: Arc<PhysicalSignalAspectBindingSet>,
    mailboxes: Box<[Arc<PhysicalSignalRouteMailbox>]>,
    shared: PhysicalSignalWorkerShared,
    ready: mpsc::SyncSender<Result<(), PhysicalSignalConstructionFailure>>,
}

struct PhysicalSignalWorkerLivenessGuard {
    admission: PhysicalSignalAdmissionStatus,
    mailboxes: Box<[Arc<PhysicalSignalRouteMailbox>]>,
}

impl PhysicalSignalGraphWorker {
    pub(super) fn spawn(
        bindings: Arc<PhysicalSignalAspectBindingSet>,
        admission: PhysicalSignalAdmissionStatus,
    ) -> Result<Self, PhysicalSignalConstructionFailure> {
        let route_ids = bindings
            .bindings()
            .iter()
            .map(|binding| binding.digest())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let shared = PhysicalSignalWorkerShared::new(admission);
        let mailboxes = route_mailboxes(route_ids.len(), &shared);
        let (ready_sender, ready_receiver) = mpsc::sync_channel(0);
        let worker = spawn_graph_worker(PhysicalSignalWorkerBoot {
            bindings,
            mailboxes: mailboxes.clone(),
            shared: shared.clone(),
            ready: ready_sender,
        })?;
        let worker = await_graph_readiness(&ready_receiver, worker)?;
        Ok(Self {
            routes: bind_routes(&route_ids, &mailboxes),
            worker: Some(worker),
            shared,
        })
    }

    pub(super) fn len(&self) -> usize {
        self.routes.len()
    }

    pub(super) fn route(
        &self,
        route: PhysicalSignalAspectBindingDigest,
    ) -> Option<&PhysicalSignalRouteOwner> {
        self.routes.iter().find(|owner| owner.route == route)
    }

    pub(super) fn clock_basis(&self) -> Option<RuntimeClockBasis> {
        self.shared.admission.is_available().then(|| {
            *self
                .shared
                .clock
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
        })
    }

    pub(super) fn stop(&mut self) {
        self.shared.admission.revoke();
        self.shared.stopping.store(true, Ordering::Release);
        self.shared.wake.signal();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }

    #[cfg(feature = "certification-test-authority")]
    pub(super) fn fail_for_certification(&self) {
        if let Some(route) = self.routes.first() {
            route.fail_for_certification();
        } else {
            self.shared.admission.revoke();
        }
    }
}

impl Drop for PhysicalSignalGraphWorker {
    fn drop(&mut self) {
        self.stop();
    }
}

impl PhysicalSignalWorkerShared {
    fn new(admission: PhysicalSignalAdmissionStatus) -> Self {
        Self {
            stopping: Arc::new(AtomicBool::new(false)),
            wake: Arc::new(PhysicalSignalWorkerWake::new()),
            clock: Arc::new(Mutex::new(RuntimeClockBasis::default())),
            admission,
        }
    }
}

impl Drop for PhysicalSignalWorkerLivenessGuard {
    fn drop(&mut self) {
        self.admission.revoke();
        for mailbox in &self.mailboxes {
            mailbox.clear();
        }
    }
}

fn route_mailboxes(
    route_count: usize,
    shared: &PhysicalSignalWorkerShared,
) -> Box<[Arc<PhysicalSignalRouteMailbox>]> {
    (0..route_count)
        .map(|_| {
            Arc::new(PhysicalSignalRouteMailbox::new(
                Arc::clone(&shared.wake),
                shared.admission.clone(),
                ROUTE_COMMAND_CAPACITY,
            ))
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn bind_routes(
    routes: &[PhysicalSignalAspectBindingDigest],
    mailboxes: &[Arc<PhysicalSignalRouteMailbox>],
) -> Box<[PhysicalSignalRouteOwner]> {
    routes
        .iter()
        .copied()
        .zip(mailboxes.iter())
        .map(|(route, mailbox)| PhysicalSignalRouteOwner {
            route,
            mailbox: Arc::clone(mailbox),
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn spawn_graph_worker(
    boot: PhysicalSignalWorkerBoot,
) -> Result<JoinHandle<()>, PhysicalSignalConstructionFailure> {
    std::thread::Builder::new()
        .name("worth-store-physical-signal-graph".to_owned())
        .spawn(move || run_graph_worker_boot(boot))
        .map_err(|_| PhysicalSignalConstructionFailure::WorkerSpawnRejected)
}

fn run_graph_worker_boot(boot: PhysicalSignalWorkerBoot) {
    let liveness = PhysicalSignalWorkerLivenessGuard {
        admission: boot.shared.admission.clone(),
        mailboxes: boot.mailboxes,
    };
    let result = PhysicalSignalGraph::build(boot.bindings);
    let Ok(mut graph) = result else {
        let _ = boot.ready.send(result.map(|_| ()));
        return;
    };
    *boot
        .shared
        .clock
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = graph.clock_basis();
    if boot.ready.send(Ok(())).is_err() {
        return;
    }
    run_graph_worker(&mut graph, &liveness, &boot.shared);
}

fn await_graph_readiness(
    ready: &mpsc::Receiver<Result<(), PhysicalSignalConstructionFailure>>,
    worker: JoinHandle<()>,
) -> Result<JoinHandle<()>, PhysicalSignalConstructionFailure> {
    match ready.recv() {
        Ok(Ok(())) => Ok(worker),
        Ok(Err(failure)) => {
            let _ = worker.join();
            Err(failure)
        }
        Err(_) => {
            let _ = worker.join();
            Err(PhysicalSignalConstructionFailure::WorkerReadinessLost)
        }
    }
}

fn run_graph_worker(
    graph: &mut PhysicalSignalGraph,
    liveness: &PhysicalSignalWorkerLivenessGuard,
    shared: &PhysicalSignalWorkerShared,
) {
    let mut next_route = 0;
    loop {
        if shared.stopping.load(Ordering::Acquire) {
            break;
        }
        if let Some((route_slot, command)) = next_command(&liveness.mailboxes, next_route) {
            next_route = route_slot.saturating_add(1);
            if !graph.apply_command(route_slot, command, &liveness.admission) {
                break;
            }
            *shared
                .clock
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner()) = graph.clock_basis();
            continue;
        }
        shared.wake.wait(&shared.stopping);
    }
}

fn next_command(
    mailboxes: &[Arc<PhysicalSignalRouteMailbox>],
    next_route: usize,
) -> Option<(usize, PhysicalSignalRouteCommand)> {
    if mailboxes.is_empty() {
        return None;
    }
    for offset in 0..mailboxes.len() {
        let slot = next_route.wrapping_add(offset) % mailboxes.len();
        if let Some(command) = mailboxes[slot].pop() {
            return Some((slot, command));
        }
    }
    None
}
