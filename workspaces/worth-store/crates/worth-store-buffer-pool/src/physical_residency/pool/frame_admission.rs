use super::*;

impl PoolInner {
    pub(super) fn reserve_loading(
        &self,
        key: PhysicalFrameKey,
    ) -> Result<(), PhysicalResidencyDenial> {
        let mut state = self.lock();
        if state.frames.contains_key(&key.coordinate) {
            return Err(PhysicalResidencyDenial::FrameAlreadyResident);
        }
        self.reserve_frame_space(&mut state, key.coordinate.length() as u64)?;
        state.frames.insert(
            key.coordinate,
            FrameEntry {
                state: FrameState::Loading,
                origin: FrameOrigin::Fault,
                pins: 1,
                dirty: false,
                writeback_claimed: false,
                bytes: key.coordinate.length() as u64,
                older_evictable: None,
                newer_evictable: None,
            },
        );
        state.counters.resident_bytes += key.coordinate.length() as u64;
        state.counters.peak_resident_bytes = state
            .counters
            .peak_resident_bytes
            .max(state.counters.resident_bytes);
        self.observe_admitted_peak(&mut state);
        state.counters.pinned_frames += 1;
        state.counters.pin_leases += 1;
        state.counters.peak_pinned_frames = state
            .counters
            .peak_pinned_frames
            .max(state.counters.pinned_frames);
        state.counters.peak_pin_leases = state
            .counters
            .peak_pin_leases
            .max(state.counters.pin_leases);
        state.counters.faults += 1;
        state.loading_frames += 1;
        state.counters.active_loading_frames += 1;
        Ok(())
    }

    pub(super) fn finish_loading(
        self: &Arc<Self>,
        key: PhysicalFrameKey,
        bytes: Arc<Vec<u8>>,
    ) -> Result<PhysicalFrameLease, PhysicalResidencyDenial> {
        let mut state = self.lock();
        if !state.accepting {
            if let Some(entry) = state.frames.remove(&key.coordinate) {
                state.counters.resident_bytes -= entry.bytes;
                state.counters.pinned_frames -= 1;
                state.counters.pin_leases -= entry.pins;
            }
            state.loading_frames -= 1;
            state.counters.active_loading_frames -= 1;
            self.changed.notify_all();
            return Err(Self::deny(&mut state, PhysicalResidencyDenial::PoolClosed));
        }
        state
            .frames
            .get_mut(&key.coordinate)
            .expect("loading reservation exists")
            .state = FrameState::Resident(Arc::clone(&bytes));
        state.loading_frames -= 1;
        state.counters.active_loading_frames -= 1;
        self.changed.notify_all();
        Ok(PhysicalFrameLease {
            owner: Arc::clone(self),
            key,
            bytes,
        })
    }

    pub(super) fn cancel_loading(&self, key: PhysicalFrameKey) {
        let mut state = self.lock();
        if let Some(entry) = state.frames.remove(&key.coordinate) {
            state.counters.resident_bytes -= entry.bytes;
            state.counters.pinned_frames -= 1;
            state.counters.pin_leases -= entry.pins;
            state.loading_frames -= 1;
            state.counters.active_loading_frames -= 1;
        }
        self.changed.notify_all();
    }

    pub(super) fn reserve_frame_space(
        &self,
        state: &mut PoolState,
        bytes: u64,
    ) -> Result<(), PhysicalResidencyDenial> {
        if !state.accepting {
            return Err(Self::deny(state, PhysicalResidencyDenial::PoolClosed));
        }
        if state.counters.pin_leases >= self.limits.pin_leases()
            || state.counters.pinned_frames >= self.limits.pinned_frames()
        {
            return Err(Self::deny(
                state,
                PhysicalResidencyDenial::PinnedFrameBudgetExceeded,
            ));
        }
        while state.counters.resident_bytes.saturating_add(bytes) > self.limits.resident_bytes()
            || state.frames.len() >= self.limits.frame_entries() as usize
        {
            state.counters.eviction_candidate_inspections += 1;
            let Some(coordinate) = state.pop_oldest_evictable() else {
                let denial = if state.frames.len() >= self.limits.frame_entries() as usize {
                    PhysicalResidencyDenial::FrameEntryBudgetExhausted
                } else {
                    PhysicalResidencyDenial::ResidentBudgetExhausted
                };
                return Err(Self::deny(state, denial));
            };
            let removed = state.frames.remove(&coordinate).expect("candidate exists");
            state.counters.resident_bytes -= removed.bytes;
            if removed.origin == FrameOrigin::Candidate {
                state.counters.candidate_frames -= 1;
            }
            state.counters.evictions += 1;
        }
        Ok(())
    }

    pub(crate) fn publish_clean(
        &self,
        key: PhysicalFrameKey,
    ) -> Result<(), PhysicalResidencyDenial> {
        let mut state = self.lock();
        if !state.accepting {
            return Err(Self::deny(&mut state, PhysicalResidencyDenial::PoolClosed));
        }
        let Some(entry) = state.frames.get_mut(&key.coordinate) else {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::FrameNotDirty,
            ));
        };
        if !entry.dirty {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::FrameNotDirty,
            ));
        }
        entry.dirty = false;
        let was_candidate = entry.origin == FrameOrigin::Candidate;
        if was_candidate {
            entry.origin = FrameOrigin::Fault;
        }
        state.counters.dirty_frames -= 1;
        if was_candidate {
            state.counters.candidate_frames -= 1;
        }
        state.counters.candidate_publications += 1;
        Ok(())
    }
}
