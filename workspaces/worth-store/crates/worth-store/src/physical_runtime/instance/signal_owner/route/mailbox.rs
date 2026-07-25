use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

use super::super::{wake::PhysicalSignalWorkerWake, PhysicalSignalAdmissionStatus};
use super::PhysicalSignalRouteCommand;

pub(in crate::physical_runtime::instance::signal_owner) const ROUTE_COMMAND_CAPACITY: usize = 8;

pub(in crate::physical_runtime::instance::signal_owner) struct PhysicalSignalRouteMailbox {
    commands: Mutex<VecDeque<PhysicalSignalRouteCommand>>,
    space_available: Condvar,
    wake: Arc<PhysicalSignalWorkerWake>,
    admission: PhysicalSignalAdmissionStatus,
    capacity: usize,
}

impl PhysicalSignalRouteMailbox {
    pub(in crate::physical_runtime::instance::signal_owner) fn new(
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

    pub(super) fn enqueue(&self, command: PhysicalSignalRouteCommand) -> Result<(), ()> {
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

    pub(in crate::physical_runtime::instance::signal_owner) fn pop(
        &self,
    ) -> Option<PhysicalSignalRouteCommand> {
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

    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime::instance::signal_owner) fn len(&self) -> usize {
        self.commands
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .len()
    }

    pub(in crate::physical_runtime::instance::signal_owner) fn clear(&self) {
        self.commands
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clear();
        self.space_available.notify_all();
    }
}
