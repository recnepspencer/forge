use std::{
    collections::VecDeque,
    sync::{mpsc, Arc, Condvar, Mutex},
};

use worth_signal::facade::ResourceRequestHandle;

use crate::physical_runtime::work::{
    AdmittedPhysicalWork, PhysicalSignalAspectBindingDigest, PhysicalWorkAspectDelta,
    PhysicalWorkPreEffectDenial, PhysicalWorkReadiness,
};

use super::{
    wake::PhysicalSignalWorkerWake, PhysicalSignalAdmissionStatus,
    PhysicalSignalDeltaApplicationFailure,
};

pub(super) const ROUTE_COMMAND_CAPACITY: usize = 8;

pub(super) enum PhysicalSignalRouteCommand {
    Apply(
        PhysicalWorkAspectDelta,
        mpsc::SyncSender<Result<(), PhysicalSignalDeltaApplicationFailure>>,
    ),
    Request(
        AdmittedPhysicalWork,
        mpsc::SyncSender<Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial>>,
    ),
    Revalidate(
        AdmittedPhysicalWork,
        ResourceRequestHandle,
        mpsc::SyncSender<Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial>>,
    ),
    #[cfg(feature = "certification-test-authority")]
    FailForCertification(mpsc::SyncSender<()>),
}

pub(super) struct PhysicalSignalRouteOwner {
    pub(super) route: PhysicalSignalAspectBindingDigest,
    pub(super) mailbox: Arc<PhysicalSignalRouteMailbox>,
}

pub(super) struct PhysicalSignalRouteMailbox {
    commands: Mutex<VecDeque<PhysicalSignalRouteCommand>>,
    space_available: Condvar,
    wake: Arc<PhysicalSignalWorkerWake>,
    admission: PhysicalSignalAdmissionStatus,
    capacity: usize,
}

impl PhysicalSignalRouteOwner {
    pub(super) fn apply_delta(
        &self,
        delta: PhysicalWorkAspectDelta,
    ) -> Result<(), PhysicalSignalDeltaApplicationFailure> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.mailbox
            .enqueue(PhysicalSignalRouteCommand::Apply(delta, reply))
            .map_err(|_| PhysicalSignalDeltaApplicationFailure::OwnerUnavailable)?;
        observed
            .recv()
            .map_err(|_| PhysicalSignalDeltaApplicationFailure::OwnerUnavailable)?
    }

    pub(super) fn request(
        &self,
        admitted: AdmittedPhysicalWork,
    ) -> Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.mailbox
            .enqueue(PhysicalSignalRouteCommand::Request(admitted, reply))
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?;
        observed
            .recv()
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?
    }

    pub(super) fn revalidate(
        &self,
        admitted: AdmittedPhysicalWork,
        active: ResourceRequestHandle,
    ) -> Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.mailbox
            .enqueue(PhysicalSignalRouteCommand::Revalidate(
                admitted, active, reply,
            ))
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?;
        observed
            .recv()
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?
    }

    #[cfg(feature = "certification-test-authority")]
    pub(super) fn fail_for_certification(&self) {
        let (reply, observed) = mpsc::sync_channel(0);
        if self
            .mailbox
            .enqueue(PhysicalSignalRouteCommand::FailForCertification(reply))
            .is_ok()
        {
            let _ = observed.recv();
        }
    }
}

impl PhysicalSignalRouteMailbox {
    pub(super) fn new(
        wake: Arc<PhysicalSignalWorkerWake>,
        admission: PhysicalSignalAdmissionStatus,
        capacity: usize,
    ) -> Self {
        Self {
            commands: Mutex::new(VecDeque::with_capacity(capacity)),
            space_available: Condvar::new(),
            wake,
            admission,
            capacity,
        }
    }

    fn enqueue(&self, command: PhysicalSignalRouteCommand) -> Result<(), ()> {
        let mut commands = self
            .commands
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        while commands.len() == self.capacity && self.admission.is_available() {
            commands = self
                .space_available
                .wait(commands)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
        }
        if !self.admission.is_available() {
            return Err(());
        }
        commands.push_back(command);
        drop(commands);
        self.wake.signal();
        Ok(())
    }

    pub(super) fn pop(&self) -> Option<PhysicalSignalRouteCommand> {
        let command = self
            .commands
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .pop_front();
        if command.is_some() {
            self.space_available.notify_one();
        }
        command
    }

    pub(super) fn clear(&self) {
        self.commands
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clear();
        self.space_available.notify_all();
    }
}
