use super::*;

impl PoolInner {
    pub(super) fn reserve_candidate_frames(
        self: &Arc<Self>,
        keys: &[PhysicalFrameKey],
    ) -> Result<PhysicalCandidateBatchReservation, PhysicalResidencyDenial> {
        let admitted = self.validate_candidate_set(keys)?;
        let mut state = self.lock();
        self.admit_candidate_set(&mut state, keys)?;
        Ok(PhysicalCandidateBatchReservation {
            owner: Arc::clone(self),
            keys: admitted,
            armed: true,
        })
    }

    fn validate_candidate_set(
        &self,
        keys: &[PhysicalFrameKey],
    ) -> Result<std::collections::VecDeque<PhysicalFrameKey>, PhysicalResidencyDenial> {
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
        Ok(admitted)
    }

    fn admit_candidate_set(
        &self,
        state: &mut PoolState,
        keys: &[PhysicalFrameKey],
    ) -> Result<(), PhysicalResidencyDenial> {
        if !state.accepting {
            return Err(Self::deny(state, PhysicalResidencyDenial::PoolClosed));
        }
        if state.active_candidate_publications >= self.limits.pin_leases() {
            return Err(Self::deny(
                state,
                PhysicalResidencyDenial::PinnedFrameBudgetExceeded,
            ));
        }
        if keys
            .iter()
            .any(|key| state.frames.contains_key(&key.coordinate))
        {
            return Err(Self::deny(
                state,
                PhysicalResidencyDenial::FrameAlreadyResident,
            ));
        }
        self.validate_candidate_capacity(state, keys)?;
        state.active_candidate_publications += 1;
        Ok(())
    }

    fn validate_candidate_capacity(
        &self,
        state: &mut PoolState,
        keys: &[PhysicalFrameKey],
    ) -> Result<(), PhysicalResidencyDenial> {
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
                state,
                PhysicalResidencyDenial::ResidentBudgetExhausted,
            ));
        }
        if fixed_frames.saturating_add(1) > self.limits.frame_entries() as usize {
            return Err(Self::deny(
                state,
                PhysicalResidencyDenial::FrameEntryBudgetExhausted,
            ));
        }
        if state.counters.dirty_frames >= self.limits.dirty_frames() {
            return Err(Self::deny(
                state,
                PhysicalResidencyDenial::DirtyFrameBudgetExceeded,
            ));
        }
        if state.counters.pinned_frames >= self.limits.pinned_frames()
            || state.counters.pin_leases >= self.limits.pin_leases()
        {
            return Err(Self::deny(
                state,
                PhysicalResidencyDenial::PinnedFrameBudgetExceeded,
            ));
        }
        Ok(())
    }

    pub(crate) fn reserve_next_candidate(
        &self,
        key: PhysicalFrameKey,
    ) -> Result<(), PhysicalResidencyDenial> {
        let mut state = self.lock();
        if state.frames.contains_key(&key.coordinate) {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::FrameAlreadyResident,
            ));
        }
        if state.counters.dirty_frames >= self.limits.dirty_frames() {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::DirtyFrameBudgetExceeded,
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
        state.active_candidate_publications = state.active_candidate_publications.saturating_sub(1);
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

    pub(crate) fn discard_dirty_candidate(
        &self,
        key: PhysicalFrameKey,
    ) -> Result<(), PhysicalResidencyDenial> {
        let mut state = self.lock();
        let Some(entry) = state.frames.get(&key.coordinate) else {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::FrameNotResident,
            ));
        };
        if !entry.dirty {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::FrameNotDirty,
            ));
        }
        if entry.origin != FrameOrigin::Candidate
            || entry.pins != 1
            || entry.writeback_claimed
            || !matches!(entry.state, FrameState::Resident(_))
        {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::CandidatePublicationActive,
            ));
        }
        let removed = state
            .frames
            .remove(&key.coordinate)
            .expect("validated candidate frame remains resident");
        state.counters.resident_bytes -= removed.bytes;
        state.counters.pinned_frames -= 1;
        state.counters.pin_leases -= 1;
        state.counters.dirty_frames -= 1;
        state.counters.candidate_frames -= 1;
        state.counters.administrative_drains += 1;
        self.changed.notify_all();
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
