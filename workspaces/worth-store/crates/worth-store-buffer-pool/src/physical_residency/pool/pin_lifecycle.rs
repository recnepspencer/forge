use super::*;

impl PoolInner {
    pub(super) fn try_pin_resident(
        self: &Arc<Self>,
        key: PhysicalFrameKey,
    ) -> Result<Option<PhysicalFrameLease>, PhysicalResidencyDenial> {
        let mut state = self.lock();
        loop {
            if !state.accepting {
                return Err(Self::deny(&mut state, PhysicalResidencyDenial::PoolClosed));
            }
            if state.candidate_publication_active {
                return Err(Self::deny(
                    &mut state,
                    PhysicalResidencyDenial::CandidatePublicationActive,
                ));
            }
            match state.frames.get(&key.coordinate).map(|entry| &entry.state) {
                Some(FrameState::Loading | FrameState::CandidateReserved) => {
                    state = self
                        .changed
                        .wait(state)
                        .unwrap_or_else(|poisoned| poisoned.into_inner());
                }
                Some(FrameState::Resident(bytes)) => {
                    if state.counters.pin_leases >= self.limits.pin_leases() {
                        return Err(Self::deny(
                            &mut state,
                            PhysicalResidencyDenial::PinLeaseBudgetExceeded,
                        ));
                    }
                    let bytes = Arc::clone(bytes);
                    let was_unpinned = state.frames[&key.coordinate].pins == 0;
                    if was_unpinned {
                        state.detach_evictable(key.coordinate);
                        if state.counters.pinned_frames >= self.limits.pinned_frames() {
                            state.append_evictable(key.coordinate);
                            return Err(Self::deny(
                                &mut state,
                                PhysicalResidencyDenial::PinnedFrameBudgetExceeded,
                            ));
                        }
                    }
                    let entry = state.frames.get_mut(&key.coordinate).unwrap();
                    entry.pins += 1;
                    if was_unpinned {
                        state.counters.pinned_frames += 1;
                    }
                    state.counters.pin_leases += 1;
                    state.counters.peak_pinned_frames = state
                        .counters
                        .peak_pinned_frames
                        .max(state.counters.pinned_frames);
                    state.counters.peak_pin_leases = state
                        .counters
                        .peak_pin_leases
                        .max(state.counters.pin_leases);
                    state.counters.hits += 1;
                    return Ok(Some(PhysicalFrameLease {
                        owner: Arc::clone(self),
                        key,
                        bytes,
                    }));
                }
                None => return Ok(None),
            }
        }
    }

    pub(crate) fn release_pin(&self, key: PhysicalFrameKey) {
        let mut state = self.lock();
        if state.closed {
            return;
        }
        let Some(entry) = state.frames.get_mut(&key.coordinate) else {
            return;
        };
        if entry.pins == 0 {
            return;
        }
        entry.pins -= 1;
        let became_unpinned = entry.pins == 0;
        let became_evictable =
            became_unpinned && !entry.dirty && matches!(entry.state, FrameState::Resident(_));
        state.counters.pin_leases -= 1;
        if became_unpinned {
            state.counters.pinned_frames -= 1;
        }
        if became_evictable {
            state.append_evictable(key.coordinate);
        }
        self.changed.notify_all();
    }
}
