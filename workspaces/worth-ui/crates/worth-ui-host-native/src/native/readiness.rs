const READINESS_CAPACITY: usize = 8;

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
    pending: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativeReadyWork {
    pub(crate) generation: u64,
    pub(crate) scale_factor_milli: u32,
    pub(crate) client_physical_size: [u32; 2],
}

pub(crate) struct UiNativeReadinessRegistry {
    slots: [Option<ReadinessSlot>; READINESS_CAPACITY],
    next_generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativeReadinessSignalDisposition {
    RedrawRequested,
    Coalesced,
}

pub(crate) fn signal_committed(
    registry: &mut UiNativeReadinessRegistry,
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

impl UiNativeReadinessRegistry {
    pub(crate) const fn new() -> Self {
        Self {
            slots: [None; READINESS_CAPACITY],
            next_generation: 1,
        }
    }

    pub(crate) fn register(&mut self) -> Result<UiNativeReadyOwner, ()> {
        let slot = self.slots.iter().position(Option::is_none).ok_or(())?;
        let generation = self.next_generation;
        self.next_generation = self.next_generation.checked_add(1).ok_or(())?;
        self.slots[slot] = Some(ReadinessSlot {
            owner_generation: generation,
            next_work_generation: 1,
            work: None,
            pending: false,
        });
        Ok(UiNativeReadyOwner { slot, generation })
    }

    pub(crate) fn commit_latest(
        &mut self,
        owner: UiNativeReadyOwner,
        scale_factor_milli: u32,
        client_physical_size: [u32; 2],
    ) -> Result<u64, ()> {
        let slot = self.slot_mut(owner)?;
        let generation = slot.next_work_generation;
        slot.next_work_generation = generation.checked_add(1).ok_or(())?;
        slot.work = Some(UiNativeReadyWork {
            generation,
            scale_factor_milli,
            client_physical_size,
        });
        Ok(generation)
    }

    pub(crate) fn signal(&mut self, owner: UiNativeReadyOwner) -> Result<bool, ()> {
        let slot = self.slot_mut(owner)?;
        if slot.work.is_none() {
            return Err(());
        }
        let queued = !slot.pending;
        slot.pending = true;
        Ok(queued)
    }

    pub(crate) fn take(&mut self, owner: UiNativeReadyOwner) -> Result<UiNativeReadyWork, ()> {
        let slot = self.slot_mut(owner)?;
        if !slot.pending {
            return Err(());
        }
        slot.pending = false;
        slot.work.take().ok_or(())
    }

    pub(crate) fn close(&mut self) -> usize {
        let live = self.slots.iter().flatten().count();
        self.slots.fill(None);
        live
    }

    fn slot_mut(&mut self, owner: UiNativeReadyOwner) -> Result<&mut ReadinessSlot, ()> {
        self.slots
            .get_mut(owner.slot)
            .and_then(Option::as_mut)
            .filter(|slot| slot.owner_generation == owner.generation)
            .ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        signal_committed, UiNativeReadinessRegistry, UiNativeReadinessSignalDisposition,
        READINESS_CAPACITY,
    };

    #[test]
    fn committed_readiness_requests_exactly_one_redraw_and_preserves_the_latest_generation() {
        let mut registry = UiNativeReadinessRegistry::new();
        let first = registry.register().unwrap();
        for _ in 1..READINESS_CAPACITY {
            registry.register().unwrap();
        }
        assert!(registry.register().is_err());
        assert!(registry.signal(first).is_err());
        assert_eq!(registry.commit_latest(first, 1_000, [160, 96]), Ok(1));
        let mut redraw_requests = 0;
        assert_eq!(
            signal_committed(&mut registry, first, || redraw_requests += 1),
            Ok(UiNativeReadinessSignalDisposition::RedrawRequested)
        );
        assert_eq!(registry.commit_latest(first, 1_500, [240, 144]), Ok(2));
        assert_eq!(
            signal_committed(&mut registry, first, || redraw_requests += 1),
            Ok(UiNativeReadinessSignalDisposition::Coalesced)
        );
        assert_eq!(redraw_requests, 1);
        let coalesced = registry.take(first).unwrap();
        assert_eq!(coalesced.generation, 2);
        assert_eq!(coalesced.scale_factor_milli, 1_500);
        assert_eq!(coalesced.client_physical_size, [240, 144]);
        assert!(registry.take(first).is_err());
        assert_eq!(registry.commit_latest(first, 2_000, [320, 192]), Ok(3));
        assert_eq!(
            signal_committed(&mut registry, first, || redraw_requests += 1),
            Ok(UiNativeReadinessSignalDisposition::RedrawRequested)
        );
        assert_eq!(redraw_requests, 2);
        let third = registry.take(first).unwrap();
        assert_eq!(third.generation, 3);
        assert_eq!(third.scale_factor_milli, 2_000);
        assert_eq!(third.client_physical_size, [320, 192]);
        assert_eq!(registry.close(), READINESS_CAPACITY);
        assert!(registry.signal(first).is_err());
    }
}
