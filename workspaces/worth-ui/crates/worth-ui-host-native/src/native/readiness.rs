use std::sync::{Arc, Mutex, MutexGuard};

const READINESS_CAPACITY: usize = 8;

mod application_ingress;

pub(crate) use application_ingress::UiNativeApplicationWake;
pub use application_ingress::{
    UiNativeApplicationReadinessPort, UiNativeApplicationReadinessSignalDenial,
    UiNativeApplicationReadinessSignalDisposition,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativeReadyOwner {
    slot: usize,
    generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ReadinessSlot {
    owner_generation: u64,
    next_work_generation: u64,
    work: Option<UiNativeReadyWork>,
    pending_generation: Option<u64>,
    pending: bool,
    level_only: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativeReadyWork {
    pub(crate) generation: u64,
    pub(crate) scale_factor_milli: u32,
    pub(crate) client_physical_size: [u32; 2],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativeLevelReadinessGrant {
    generation: u64,
}

impl UiNativeLevelReadinessGrant {
    pub(crate) const fn generation(self) -> u64 {
        self.generation
    }
}

#[derive(Clone)]
pub(crate) struct UiNativeReadinessRegistry {
    state: Arc<Mutex<UiNativeReadinessState>>,
}

struct UiNativeReadinessState {
    slots: [Option<ReadinessSlot>; READINESS_CAPACITY],
    next_generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativeReadinessClosureReceipt {
    queued_signals: usize,
    exact_owner_set: bool,
}

impl UiNativeReadinessClosureReceipt {
    pub(crate) const fn is_complete(self) -> bool {
        self.exact_owner_set
    }

    #[cfg(test)]
    pub(crate) const fn queued_signals(self) -> usize {
        self.queued_signals
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativeReadinessSignalDisposition {
    RedrawRequested,
    Coalesced,
    NoWork,
}

pub(crate) fn signal_committed(
    registry: &UiNativeReadinessRegistry,
    owner: UiNativeReadyOwner,
    mut request_redraw: impl FnMut(),
) -> Result<UiNativeReadinessSignalDisposition, ()> {
    if registry.signal(owner)? {
        request_redraw();
        Ok(UiNativeReadinessSignalDisposition::RedrawRequested)
    } else {
        Ok(UiNativeReadinessSignalDisposition::Coalesced)
    }
}

pub(crate) fn signal_level_ready(
    registry: &UiNativeReadinessRegistry,
    owner: UiNativeReadyOwner,
    has_ready_work: bool,
    mut request_redraw: impl FnMut(),
) -> Result<UiNativeReadinessSignalDisposition, ()> {
    if !has_ready_work {
        return Ok(UiNativeReadinessSignalDisposition::NoWork);
    }
    if registry.signal_level(owner)? {
        request_redraw();
        Ok(UiNativeReadinessSignalDisposition::RedrawRequested)
    } else {
        Ok(UiNativeReadinessSignalDisposition::Coalesced)
    }
}

impl UiNativeReadinessRegistry {
    pub(crate) fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(UiNativeReadinessState {
                slots: [None; READINESS_CAPACITY],
                next_generation: 1,
            })),
        }
    }

    pub(crate) fn register(&self) -> Result<UiNativeReadyOwner, ()> {
        self.register_slot(false)
    }

    pub(crate) fn register_level(&self) -> Result<UiNativeReadyOwner, ()> {
        self.register_slot(true)
    }

    fn register_slot(&self, level_only: bool) -> Result<UiNativeReadyOwner, ()> {
        let mut state = self.lock()?;
        let slot = state.slots.iter().position(Option::is_none).ok_or(())?;
        let generation = state.next_generation;
        state.next_generation = state.next_generation.checked_add(1).ok_or(())?;
        state.slots[slot] = Some(ReadinessSlot {
            owner_generation: generation,
            next_work_generation: 1,
            work: None,
            pending_generation: None,
            pending: false,
            level_only,
        });
        Ok(UiNativeReadyOwner { slot, generation })
    }

    pub(crate) fn commit_latest(
        &self,
        owner: UiNativeReadyOwner,
        scale_factor_milli: u32,
        client_physical_size: [u32; 2],
    ) -> Result<u64, ()> {
        let mut state = self.lock()?;
        let slot = state.slot_mut(owner)?;
        if slot.level_only {
            return Err(());
        }
        let generation = slot.next_work_generation;
        slot.next_work_generation = generation.checked_add(1).ok_or(())?;
        slot.work = Some(UiNativeReadyWork {
            generation,
            scale_factor_milli,
            client_physical_size,
        });
        Ok(generation)
    }

    pub(crate) fn signal(&self, owner: UiNativeReadyOwner) -> Result<bool, ()> {
        let mut state = self.lock()?;
        let slot = state.slot_mut(owner)?;
        if slot.level_only || slot.work.is_none() {
            return Err(());
        }
        let queued = !slot.pending;
        slot.pending = true;
        Ok(queued)
    }

    pub(crate) fn signal_level(&self, owner: UiNativeReadyOwner) -> Result<bool, ()> {
        let mut state = self.lock()?;
        let slot = state.slot_mut(owner)?;
        if !slot.level_only {
            return Err(());
        }
        let queued = !slot.pending;
        if queued {
            let generation = slot.next_work_generation;
            slot.next_work_generation = generation.checked_add(1).ok_or(())?;
            slot.pending_generation = Some(generation);
        }
        slot.pending = true;
        Ok(queued)
    }

    pub(super) fn cancel_level_signal(&self, owner: UiNativeReadyOwner) -> Result<(), ()> {
        let mut state = self.lock()?;
        let slot = state.slot_mut(owner)?;
        if !slot.level_only {
            return Err(());
        }
        slot.pending = false;
        slot.pending_generation = None;
        Ok(())
    }

    pub(crate) fn take(&self, owner: UiNativeReadyOwner) -> Result<UiNativeReadyWork, ()> {
        let mut state = self.lock()?;
        let slot = state.slot_mut(owner)?;
        if slot.level_only || !slot.pending {
            return Err(());
        }
        slot.pending = false;
        slot.work.take().ok_or(())
    }

    pub(crate) fn take_level(
        &self,
        owner: UiNativeReadyOwner,
    ) -> Result<UiNativeLevelReadinessGrant, ()> {
        let mut state = self.lock()?;
        let slot = state.slot_mut(owner)?;
        if !slot.level_only || !slot.pending {
            return Err(());
        }
        let generation = slot.pending_generation.take().ok_or(())?;
        slot.pending = false;
        Ok(UiNativeLevelReadinessGrant { generation })
    }

    pub(crate) fn close(&self) -> usize {
        let Ok(mut state) = self.lock() else {
            return 0;
        };
        let live = state.slots.iter().flatten().count();
        state.slots.fill(None);
        live
    }

    pub(crate) fn pending_signal_count(&self) -> usize {
        self.lock()
            .map(|state| {
                state
                    .slots
                    .iter()
                    .flatten()
                    .filter(|slot| slot.pending)
                    .count()
            })
            .unwrap_or_default()
    }

    pub(crate) fn close_exact(
        &self,
        expected: &[UiNativeReadyOwner],
    ) -> UiNativeReadinessClosureReceipt {
        let Ok(mut state) = self.lock() else {
            return UiNativeReadinessClosureReceipt {
                queued_signals: 0,
                exact_owner_set: false,
            };
        };
        let registered_owners = state.slots.iter().flatten().count();
        let queued_signals = state
            .slots
            .iter()
            .flatten()
            .filter(|slot| slot.pending)
            .count();
        let exact_owner_set = registered_owners == expected.len()
            && expected.iter().all(|owner| {
                state
                    .slots
                    .get(owner.slot)
                    .and_then(Option::as_ref)
                    .is_some_and(|slot| slot.owner_generation == owner.generation)
            });
        state.slots.fill(None);
        UiNativeReadinessClosureReceipt {
            queued_signals,
            exact_owner_set,
        }
    }

    fn lock(&self) -> Result<MutexGuard<'_, UiNativeReadinessState>, ()> {
        self.state.lock().map_err(|_| ())
    }
}

impl UiNativeReadinessState {
    fn slot_mut(&mut self, owner: UiNativeReadyOwner) -> Result<&mut ReadinessSlot, ()> {
        self.slots
            .get_mut(owner.slot)
            .and_then(Option::as_mut)
            .filter(|slot| slot.owner_generation == owner.generation)
            .ok_or(())
    }
}

#[cfg(test)]
#[path = "readiness/tests.rs"]
mod tests;
