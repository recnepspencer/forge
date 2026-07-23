use super::*;

impl PoolInner {
    pub(super) fn reserve_loading(
        &self,
        key: PhysicalFrameKey,
    ) -> Result<(), PhysicalResidencyDenial> {
        let mut state = self.lock();
        if state.candidate_publication_active {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::CandidatePublicationActive,
            ));
        }
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

    pub(super) fn reserve_candidate_frames(
        self: &Arc<Self>,
        keys: &[PhysicalFrameKey],
    ) -> Result<PhysicalCandidateBatchReservation, PhysicalResidencyDenial> {
        if keys.is_empty() {
            return Err(self.record_denial(PhysicalResidencyDenial::FrameLengthMismatch));
        }
        let mut unique = std::collections::HashSet::new();
        unique
            .try_reserve(keys.len())
            .map_err(|_| self.record_denial(PhysicalResidencyDenial::AllocationFailed))?;
        for key in keys {
            if let Err(reason) = self.validate_key(*key) {
                return Err(self.record_denial(reason));
            }
            if !unique.insert(*key) {
                return Err(self.record_denial(PhysicalResidencyDenial::FrameAlreadyResident));
            }
        }
        let mut admitted = std::collections::VecDeque::new();
        admitted
            .try_reserve_exact(keys.len())
            .map_err(|_| self.record_denial(PhysicalResidencyDenial::AllocationFailed))?;
        admitted.extend(keys.iter().copied());
        let mut state = self.lock();
        if !state.accepting {
            return Err(Self::deny(&mut state, PhysicalResidencyDenial::PoolClosed));
        }
        if state.candidate_publication_active {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::CandidatePublicationActive,
            ));
        }
        if keys
            .iter()
            .any(|key| state.frames.contains_key(&key.coordinate))
        {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::FrameAlreadyResident,
            ));
        }
        let max_frame = keys
            .iter()
            .map(|key| u64::from(key.coordinate.length()))
            .max()
            .expect("nonempty candidate set");
        let evictable_bytes = state
            .frames
            .values()
            .filter(|entry| entry.is_evictable())
            .map(|entry| entry.bytes)
            .sum::<u64>();
        let evictable_frames = state
            .frames
            .values()
            .filter(|entry| entry.is_evictable())
            .count();
        let fixed_bytes = state
            .counters
            .resident_bytes
            .saturating_sub(evictable_bytes);
        let fixed_frames = state.frames.len().saturating_sub(evictable_frames);
        if fixed_bytes.saturating_add(max_frame) > self.limits.resident_bytes() {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::ResidentBudgetExhausted,
            ));
        }
        if fixed_frames.saturating_add(1) > self.limits.frame_entries() as usize {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::FrameEntryBudgetExhausted,
            ));
        }
        if state.counters.dirty_frames >= self.limits.dirty_frames() {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::DirtyFrameBudgetExceeded,
            ));
        }
        if state.counters.pinned_frames >= self.limits.pinned_frames()
            || state.counters.pin_leases >= self.limits.pin_leases()
        {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::PinnedFrameBudgetExceeded,
            ));
        }
        state.candidate_publication_active = true;
        Ok(PhysicalCandidateBatchReservation {
            owner: Arc::clone(self),
            keys: admitted,
            armed: true,
        })
    }

    pub(crate) fn reserve_next_candidate(
        &self,
        key: PhysicalFrameKey,
    ) -> Result<(), PhysicalResidencyDenial> {
        let mut state = self.lock();
        if !state.candidate_publication_active {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::CandidatePublicationActive,
            ));
        }
        self.reserve_frame_space(&mut state, u64::from(key.coordinate.length()))?;
        state.frames.insert(
            key.coordinate,
            FrameEntry {
                state: FrameState::CandidateReserved,
                origin: FrameOrigin::Candidate,
                pins: 1,
                dirty: true,
                writeback_claimed: false,
                bytes: u64::from(key.coordinate.length()),
                older_evictable: None,
                newer_evictable: None,
            },
        );
        state.counters.resident_bytes += u64::from(key.coordinate.length());
        state.counters.pinned_frames += 1;
        state.counters.pin_leases += 1;
        state.counters.dirty_frames += 1;
        state.counters.candidate_frames += 1;
        state.loading_frames += 1;
        state.counters.active_loading_frames += 1;
        observe_candidate_reservation_peaks(self, &mut state);
        Ok(())
    }

    pub(crate) fn finish_candidate_batch(&self) {
        let mut state = self.lock();
        state.candidate_publication_active = false;
        self.changed.notify_all();
    }

    pub(crate) fn finish_candidate(
        self: &Arc<Self>,
        key: PhysicalFrameKey,
        bytes: Arc<Vec<u8>>,
    ) -> Result<DirtyPhysicalFrame, PhysicalResidencyDenial> {
        let mut state = self.lock();
        if !state.accepting {
            cancel_candidate_locked(&mut state, key.coordinate);
            self.changed.notify_all();
            return Err(Self::deny(&mut state, PhysicalResidencyDenial::PoolClosed));
        }
        let Some(entry) = state.frames.get_mut(&key.coordinate) else {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::FrameNotResident,
            ));
        };
        if !matches!(entry.state, FrameState::CandidateReserved) {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::FrameAlreadyResident,
            ));
        }
        entry.state = FrameState::Resident(Arc::clone(&bytes));
        state.loading_frames -= 1;
        state.counters.active_loading_frames -= 1;
        self.changed.notify_all();
        Ok(DirtyPhysicalFrame {
            lease: Some(PhysicalFrameLease {
                owner: Arc::clone(self),
                key,
                bytes,
            }),
        })
    }

    pub(crate) fn cancel_candidate(&self, key: PhysicalFrameKey) {
        let mut state = self.lock();
        cancel_candidate_locked(&mut state, key.coordinate);
        self.changed.notify_all();
    }

    fn reserve_frame_space(
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

fn observe_candidate_reservation_peaks(owner: &PoolInner, state: &mut PoolState) {
    state.counters.peak_resident_bytes = state
        .counters
        .peak_resident_bytes
        .max(state.counters.resident_bytes);
    state.counters.peak_pinned_frames = state
        .counters
        .peak_pinned_frames
        .max(state.counters.pinned_frames);
    state.counters.peak_pin_leases = state
        .counters
        .peak_pin_leases
        .max(state.counters.pin_leases);
    state.counters.peak_dirty_frames = state
        .counters
        .peak_dirty_frames
        .max(state.counters.dirty_frames);
    state.counters.peak_candidate_frames = state
        .counters
        .peak_candidate_frames
        .max(state.counters.candidate_frames);
    owner.observe_admitted_peak(state);
}

fn cancel_candidate_locked(state: &mut PoolState, coordinate: RecordFrameCoordinate) {
    if matches!(
        state.frames.get(&coordinate).map(|entry| &entry.state),
        Some(FrameState::CandidateReserved)
    ) {
        let entry = state
            .frames
            .remove(&coordinate)
            .expect("candidate reservation remains present");
        state.counters.resident_bytes -= entry.bytes;
        state.counters.pinned_frames -= 1;
        state.counters.pin_leases -= 1;
        state.counters.dirty_frames -= 1;
        state.counters.candidate_frames -= 1;
        state.loading_frames -= 1;
        state.counters.active_loading_frames -= 1;
    }
}
