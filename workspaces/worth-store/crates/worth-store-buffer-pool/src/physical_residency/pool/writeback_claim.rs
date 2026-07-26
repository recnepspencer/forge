use super::*;

impl PoolInner {
    pub(super) fn claim_writeback(
        self: &Arc<Self>,
        frames: &[PhysicalFrameKey],
    ) -> Result<Vec<Arc<Vec<u8>>>, PhysicalResidencyDenial> {
        let mut request = self.prepare_writeback_claim(frames)?;
        let mut state = self.lock();
        if !state.accepting {
            return Err(Self::deny(&mut state, PhysicalResidencyDenial::PoolClosed));
        }
        self.collect_writeback_residents(&mut state, frames, &mut request.resident_bytes)?;
        self.admit_writeback_capacity(&mut state, request.count)?;
        Self::apply_writeback_claim(&mut state, frames, request.count);
        Ok(request.resident_bytes)
    }

    fn prepare_writeback_claim(
        &self,
        frames: &[PhysicalFrameKey],
    ) -> Result<PreparedWritebackClaim, PhysicalResidencyDenial> {
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
        Ok(PreparedWritebackClaim {
            count,
            resident_bytes,
        })
    }

    fn admit_writeback_capacity(
        &self,
        state: &mut PoolState,
        count: u32,
    ) -> Result<(), PhysicalResidencyDenial> {
        let kind = crate::PhysicalSpeculativeWorkKind::WriteBehind;
        state.accounting.attempt_speculative(kind);
        let current = state.accounting.active_speculative_frames(kind);
        let next = current.checked_add(count).ok_or_else(|| {
            state.accounting.record_speculative_denial(kind);
            self.pressure(
                state,
                PhysicalResidencyPressureDemand {
                    dimension: PhysicalResidencyDimension::SpeculativeFrames(kind),
                    scope: PhysicalOperationAllocationScope::ForegroundWrite,
                    requested: u64::from(count),
                    current: u64::from(current),
                    limit: u64::from(self.limits.speculative_frames(kind)),
                },
            )
        })?;
        if next > self.limits.speculative_frames(kind) {
            state.accounting.record_speculative_denial(kind);
            return Err(self.pressure(
                state,
                PhysicalResidencyPressureDemand {
                    dimension: PhysicalResidencyDimension::SpeculativeFrames(kind),
                    scope: PhysicalOperationAllocationScope::ForegroundWrite,
                    requested: u64::from(count),
                    current: u64::from(current),
                    limit: u64::from(self.limits.speculative_frames(kind)),
                },
            ));
        }
        Ok(())
    }

    fn collect_writeback_residents(
        &self,
        state: &mut PoolState,
        frames: &[PhysicalFrameKey],
        resident_bytes: &mut Vec<Arc<Vec<u8>>>,
    ) -> Result<(), PhysicalResidencyDenial> {
        for frame in frames {
            let Some(entry) = state.frames.get(&frame.coordinate) else {
                return Err(Self::deny(
                    state,
                    PhysicalResidencyDenial::WriteBackFrameNotDirty,
                ));
            };
            if !entry.dirty {
                return Err(Self::deny(
                    state,
                    PhysicalResidencyDenial::WriteBackFrameNotDirty,
                ));
            }
            if entry.writeback_claimed {
                return Err(Self::deny(
                    state,
                    PhysicalResidencyDenial::WriteBackFrameAlreadyClaimed,
                ));
            }
            let FrameState::Resident(bytes) = &entry.state else {
                return Err(Self::deny(
                    state,
                    PhysicalResidencyDenial::WriteBackFrameNotDirty,
                ));
            };
            resident_bytes.push(Arc::clone(bytes));
        }
        Ok(())
    }

    fn apply_writeback_claim(state: &mut PoolState, frames: &[PhysicalFrameKey], count: u32) {
        let kind = crate::PhysicalSpeculativeWorkKind::WriteBehind;
        for frame in frames {
            state
                .frames
                .get_mut(&frame.coordinate)
                .expect("validated dirty frame remains resident")
                .writeback_claimed = true;
        }
        state.accounting.admit_speculative(kind, count);
        state.accounting.claim_writeback(count);
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
        for _ in frames {
            let candidate_removed = published_candidates > 0;
            state.accounting.mark_clean(candidate_removed, true);
            published_candidates = published_candidates.saturating_sub(1);
        }
        state
            .accounting
            .release_speculative(crate::PhysicalSpeculativeWorkKind::WriteBehind, count);
        state.accounting.release_writeback(count);
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
        for frame in frames {
            if let Some(entry) = state.frames.get_mut(&frame.coordinate) {
                entry.writeback_claimed = false;
            }
        }
        let count = frames.len() as u32;
        state
            .accounting
            .release_speculative(crate::PhysicalSpeculativeWorkKind::WriteBehind, count);
        state.accounting.release_writeback(count);
        self.changed.notify_all();
    }
}

struct PreparedWritebackClaim {
    count: u32,
    resident_bytes: Vec<Arc<Vec<u8>>>,
}
