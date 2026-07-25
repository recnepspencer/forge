use super::*;

impl PoolInner {
    pub(crate) fn replace_clean_lease_with_dirty(
        &self,
        key: PhysicalFrameKey,
        expected: &Arc<Vec<u8>>,
        replacement: Arc<Vec<u8>>,
    ) -> Result<(), PhysicalResidencyDenial> {
        if let Err(reason) = self.validate_key(key) {
            return Err(self.record_denial(reason));
        }
        let mut state = self.lock();
        validate_transition(self, &mut state, key, expected)?;
        let entry = state
            .frames
            .get_mut(&key.coordinate)
            .expect("validated clean frame remains resident");
        let was_candidate = entry.origin == FrameOrigin::Candidate;
        entry.state = FrameState::Resident(replacement);
        entry.origin = FrameOrigin::Candidate;
        entry.dirty = true;
        state.counters.dirty_frames += 1;
        if !was_candidate {
            state.counters.candidate_frames += 1;
        }
        state.counters.peak_dirty_frames = state
            .counters
            .peak_dirty_frames
            .max(state.counters.dirty_frames);
        state.counters.peak_candidate_frames = state
            .counters
            .peak_candidate_frames
            .max(state.counters.candidate_frames);
        Ok(())
    }
}

fn validate_transition(
    owner: &PoolInner,
    state: &mut PoolState,
    key: PhysicalFrameKey,
    expected: &Arc<Vec<u8>>,
) -> Result<(), PhysicalResidencyDenial> {
    if !state.accepting {
        return Err(PoolInner::deny(state, PhysicalResidencyDenial::PoolClosed));
    }
    if state.counters.dirty_frames >= owner.limits.dirty_frames() {
        return Err(PoolInner::deny(
            state,
            PhysicalResidencyDenial::DirtyFrameBudgetExceeded,
        ));
    }
    let Some(entry) = state.frames.get(&key.coordinate) else {
        return Err(PoolInner::deny(
            state,
            PhysicalResidencyDenial::FrameNotResident,
        ));
    };
    if entry.pins != 1 {
        return Err(PoolInner::deny(state, PhysicalResidencyDenial::FramePinned));
    }
    if entry.dirty {
        return Err(PoolInner::deny(state, PhysicalResidencyDenial::FrameDirty));
    }
    if entry.writeback_claimed {
        return Err(PoolInner::deny(
            state,
            PhysicalResidencyDenial::WriteBackFrameAlreadyClaimed,
        ));
    }
    match &entry.state {
        FrameState::Resident(current) if Arc::ptr_eq(current, expected) => Ok(()),
        _ => Err(PoolInner::deny(
            state,
            PhysicalResidencyDenial::FrameNotResident,
        )),
    }
}
