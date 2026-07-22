use super::*;

impl PoolInner {
    pub(super) fn reserve_operation(
        self: &Arc<Self>,
        scope: OperationAllocationScope,
        bytes: u64,
    ) -> Result<(), PhysicalResidencyDenial> {
        let mut state = self.lock();
        if !state.accepting {
            return Err(Self::deny(&mut state, PhysicalResidencyDenial::PoolClosed));
        }
        let Some(next) = state.counters.active_operation_bytes.checked_add(bytes) else {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::OperationBudgetExceeded,
            ));
        };
        if next > self.limits.operation_bytes() {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::OperationBudgetExceeded,
            ));
        }
        state.counters.active_operation_bytes = next;
        state.counters.peak_operation_bytes = state.counters.peak_operation_bytes.max(next);
        state.counters.operation_scope_bytes[scope.index()] += bytes;
        state.counters.peak_operation_scope_bytes[scope.index()] =
            state.counters.peak_operation_scope_bytes[scope.index()]
                .max(state.counters.operation_scope_bytes[scope.index()]);
        self.observe_admitted_peak(&mut state);
        Ok(())
    }

    pub(crate) fn release_operation(&self, scope: OperationAllocationScope, bytes: u64) {
        let mut state = self.lock();
        state.counters.active_operation_bytes -= bytes;
        state.counters.operation_scope_bytes[scope.index()] -= bytes;
        self.changed.notify_all();
    }

    pub(crate) fn record_copy(&self, bytes: u64) {
        let mut state = self.lock();
        if state.closed {
            return;
        }
        state.counters.copy_operations += 1;
        state.counters.copied_bytes = state.counters.copied_bytes.saturating_add(bytes);
    }

    pub(super) fn reserve_speculative(
        self: &Arc<Self>,
        kind: crate::SpeculativePhysicalWorkKind,
        frames: u32,
    ) -> Result<(), PhysicalResidencyDenial> {
        use super::super::observation::speculative_index;
        let mut state = self.lock();
        let index = speculative_index(kind);
        state.counters.speculative_attempts[index] += 1;
        if !state.accepting {
            return Err(Self::deny(&mut state, PhysicalResidencyDenial::PoolClosed));
        }
        if frames == 0 {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::SpeculativeFrameBudgetExceeded,
            ));
        }
        let limit = match kind {
            crate::SpeculativePhysicalWorkKind::Prefetch => self.limits.prefetch_frames(),
            crate::SpeculativePhysicalWorkKind::ReadAhead => self.limits.read_ahead_frames(),
            crate::SpeculativePhysicalWorkKind::WriteBehind => self.limits.write_back_frames(),
        };
        let Some(next) = state.counters.active_speculative_frames[index].checked_add(frames) else {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::SpeculativeFrameBudgetExceeded,
            ));
        };
        if next > limit {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::SpeculativeFrameBudgetExceeded,
            ));
        }
        if kind == crate::SpeculativePhysicalWorkKind::WriteBehind
            && next > state.counters.dirty_frames
        {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::WriteBackExceedsDirtyPosture,
            ));
        }
        state.counters.active_speculative_frames[index] = next;
        state.counters.peak_speculative_frames[index] =
            state.counters.peak_speculative_frames[index].max(next);
        state.counters.speculative_admissions[index] += 1;
        Ok(())
    }

    pub(crate) fn release_speculative(
        &self,
        kind: crate::SpeculativePhysicalWorkKind,
        frames: u32,
    ) {
        use super::super::observation::speculative_index;
        let mut state = self.lock();
        if state.closed {
            return;
        }
        state.counters.active_speculative_frames[speculative_index(kind)] -= frames;
        self.changed.notify_all();
    }

    pub(super) fn claim_writeback(
        self: &Arc<Self>,
        frames: &[PhysicalFrameKey],
    ) -> Result<Vec<Arc<Vec<u8>>>, PhysicalResidencyDenial> {
        if frames.is_empty() {
            return Err(self.record_denial(PhysicalResidencyDenial::WriteBackExceedsDirtyPosture));
        }
        let mut unique = std::collections::HashSet::new();
        unique
            .try_reserve(frames.len())
            .map_err(|_| self.record_denial(PhysicalResidencyDenial::AllocationFailed))?;
        for frame in frames {
            if let Err(reason) = self.validate_key(*frame) {
                return Err(self.record_denial(reason));
            }
            if !unique.insert(*frame) {
                return Err(
                    self.record_denial(PhysicalResidencyDenial::WriteBackFrameAlreadyClaimed)
                );
            }
        }
        let count = u32::try_from(frames.len()).map_err(|_| {
            self.record_denial(PhysicalResidencyDenial::WriteBackExceedsDirtyPosture)
        })?;
        let mut resident_bytes = Vec::new();
        resident_bytes
            .try_reserve_exact(frames.len())
            .map_err(|_| self.record_denial(PhysicalResidencyDenial::AllocationFailed))?;
        let mut state = self.lock();
        if !state.accepting {
            return Err(Self::deny(&mut state, PhysicalResidencyDenial::PoolClosed));
        }
        let index = super::super::observation::speculative_index(
            crate::SpeculativePhysicalWorkKind::WriteBehind,
        );
        let next = state.counters.active_speculative_frames[index]
            .checked_add(count)
            .ok_or_else(|| {
                Self::deny(
                    &mut state,
                    PhysicalResidencyDenial::SpeculativeFrameBudgetExceeded,
                )
            })?;
        if next > self.limits.write_back_frames() {
            return Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::SpeculativeFrameBudgetExceeded,
            ));
        }
        for frame in frames {
            let Some(entry) = state.frames.get(&frame.coordinate) else {
                return Err(Self::deny(
                    &mut state,
                    PhysicalResidencyDenial::WriteBackFrameNotDirty,
                ));
            };
            if !entry.dirty {
                return Err(Self::deny(
                    &mut state,
                    PhysicalResidencyDenial::WriteBackFrameNotDirty,
                ));
            }
            if entry.writeback_claimed {
                return Err(Self::deny(
                    &mut state,
                    PhysicalResidencyDenial::WriteBackFrameAlreadyClaimed,
                ));
            }
            let FrameState::Resident(bytes) = &entry.state else {
                return Err(Self::deny(
                    &mut state,
                    PhysicalResidencyDenial::WriteBackFrameNotDirty,
                ));
            };
            resident_bytes.push(Arc::clone(bytes));
        }
        for frame in frames {
            state
                .frames
                .get_mut(&frame.coordinate)
                .expect("validated dirty frame remains resident")
                .writeback_claimed = true;
        }
        state.counters.active_speculative_frames[index] = next;
        state.counters.peak_speculative_frames[index] =
            state.counters.peak_speculative_frames[index].max(next);
        state.counters.speculative_attempts[index] += 1;
        state.counters.speculative_admissions[index] += 1;
        Ok(resident_bytes)
    }

    pub(crate) fn complete_writeback_claim(
        &self,
        frames: &[PhysicalFrameKey],
    ) -> Result<(), PhysicalResidencyDenial> {
        let mut state = self.lock();
        if !state.accepting {
            return Err(Self::deny(&mut state, PhysicalResidencyDenial::PoolClosed));
        }
        for frame in frames {
            let Some(entry) = state.frames.get(&frame.coordinate) else {
                return Err(Self::deny(
                    &mut state,
                    PhysicalResidencyDenial::WriteBackFrameNotDirty,
                ));
            };
            if !entry.dirty || !entry.writeback_claimed {
                return Err(Self::deny(
                    &mut state,
                    PhysicalResidencyDenial::WriteBackFrameNotDirty,
                ));
            }
        }
        let mut published_candidates = 0_u64;
        for frame in frames {
            let entry = state
                .frames
                .get_mut(&frame.coordinate)
                .expect("validated writeback frame remains resident");
            entry.dirty = false;
            entry.writeback_claimed = false;
            if entry.origin == FrameOrigin::Candidate {
                entry.origin = FrameOrigin::Fault;
                published_candidates += 1;
            }
        }
        let count = frames.len() as u32;
        state.counters.dirty_frames -= count;
        state.counters.candidate_frames -= published_candidates as u32;
        state.counters.candidate_publications += published_candidates;
        state.counters.writebacks += frames.len() as u64;
        let index = super::super::observation::speculative_index(
            crate::SpeculativePhysicalWorkKind::WriteBehind,
        );
        state.counters.active_speculative_frames[index] -= count;
        for frame in frames {
            let becomes_evictable = state
                .frames
                .get(&frame.coordinate)
                .is_some_and(FrameEntry::is_evictable);
            if becomes_evictable {
                state.append_evictable(frame.coordinate);
            }
        }
        self.changed.notify_all();
        Ok(())
    }

    pub(crate) fn release_writeback_claim(&self, frames: &[PhysicalFrameKey]) {
        let mut state = self.lock();
        if state.closed {
            return;
        }
        for frame in frames {
            if let Some(entry) = state.frames.get_mut(&frame.coordinate) {
                entry.writeback_claimed = false;
            }
        }
        let index = super::super::observation::speculative_index(
            crate::SpeculativePhysicalWorkKind::WriteBehind,
        );
        state.counters.active_speculative_frames[index] =
            state.counters.active_speculative_frames[index].saturating_sub(frames.len() as u32);
        self.changed.notify_all();
    }
}
