use super::*;

impl PoolInner {
    pub(super) fn invalidate_clean(
        &self,
        key: PhysicalFrameKey,
    ) -> Result<(), PhysicalResidencyDenial> {
        if let Err(reason) = self.validate_key(key) {
            return Err(self.record_denial(reason));
        }
        let mut state = self.lock();
        if !state.accepting {
            return Err(Self::deny(&mut state, PhysicalResidencyDenial::PoolClosed));
        }
        let Some(entry) = state.frames.get(&key.coordinate) else {
            return Ok(());
        };
        if entry.pins != 0 {
            return Err(Self::deny(&mut state, PhysicalResidencyDenial::FramePinned));
        }
        if entry.dirty {
            return Err(Self::deny(&mut state, PhysicalResidencyDenial::FrameDirty));
        }
        state.detach_evictable(key.coordinate);
        let removed = state
            .frames
            .remove(&key.coordinate)
            .expect("validated clean frame remains present");
        state.counters.resident_bytes -= removed.bytes;
        if removed.origin == FrameOrigin::Candidate {
            state.counters.candidate_frames -= 1;
        }
        Ok(())
    }

    pub(super) fn promote_clean_identity(
        &self,
        source: PhysicalFrameKey,
        target: PhysicalFrameKey,
    ) -> Result<(), PhysicalResidencyDenial> {
        if let Err(reason) = self.validate_key(source) {
            return Err(self.record_denial(reason));
        }
        if let Err(reason) = self.validate_key(target) {
            return Err(self.record_denial(reason));
        }
        if source.coordinate.length() != target.coordinate.length() {
            return Err(self.record_denial(PhysicalResidencyDenial::FrameLengthMismatch));
        }
        if source == target {
            return Err(self.record_denial(PhysicalResidencyDenial::IdentityAlreadyCurrent));
        }
        let mut state = self.lock();
        if !state.accepting {
            return Err(Self::deny(&mut state, PhysicalResidencyDenial::PoolClosed));
        }
        let Some(source_entry) = state.frames.get(&source.coordinate) else {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::FrameNotResident,
            ));
        };
        if source_entry.pins != 0 {
            return Err(Self::deny(&mut state, PhysicalResidencyDenial::FramePinned));
        }
        if source_entry.dirty {
            return Err(Self::deny(&mut state, PhysicalResidencyDenial::FrameDirty));
        }
        if !matches!(source_entry.state, FrameState::Resident(_)) {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::FrameNotResident,
            ));
        }
        if let Some(target_entry) = state.frames.get(&target.coordinate) {
            if target_entry.pins != 0 {
                return Err(Self::deny(&mut state, PhysicalResidencyDenial::FramePinned));
            }
            if target_entry.dirty {
                return Err(Self::deny(&mut state, PhysicalResidencyDenial::FrameDirty));
            }
        }
        state.detach_evictable(source.coordinate);
        if state.frames.contains_key(&target.coordinate) {
            state.detach_evictable(target.coordinate);
            let removed = state
                .frames
                .remove(&target.coordinate)
                .expect("validated target remains resident");
            state.counters.resident_bytes -= removed.bytes;
            if removed.origin == FrameOrigin::Candidate {
                state.counters.candidate_frames -= 1;
            }
        }
        let entry = state
            .frames
            .remove(&source.coordinate)
            .expect("validated source remains resident");
        state.frames.insert(target.coordinate, entry);
        state.append_evictable(target.coordinate);
        state.counters.identity_transitions += 1;
        Ok(())
    }
}
