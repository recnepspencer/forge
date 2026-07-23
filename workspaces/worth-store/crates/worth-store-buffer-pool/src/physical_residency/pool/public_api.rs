use super::*;

impl PhysicalResidencyPool {
    pub(crate) fn bind_queue_frame(
        &self,
        key: PhysicalFrameKey,
    ) -> Result<
        (
            worth_store_physical_format::store_namespace::StableStoreIdentity,
            PhysicalResidencyIncarnation,
            worth_store_physical_format::RecordFrameCoordinate,
        ),
        PhysicalResidencyDenial,
    > {
        self.inner.validate_key(key)?;
        Ok((key.store(), self.inner.incarnation, key.coordinate()))
    }

    pub fn open(
        store: StableStoreIdentity,
        limits: PhysicalResidencyLimits,
    ) -> Result<Self, PhysicalResidencyDenial> {
        let frame_count = limits.frame_entries() as usize;
        let metadata_per_entry = std::mem::size_of::<RecordFrameCoordinate>()
            .saturating_add(std::mem::size_of::<FrameEntry>())
            .saturating_add(32);
        let minimum_metadata = frame_count
            .checked_mul(metadata_per_entry)
            .and_then(|bytes| bytes.checked_add(std::mem::size_of::<PoolState>()))
            .ok_or(PhysicalResidencyDenial::MetadataBudgetExceeded)?
            as u64;
        if minimum_metadata > limits.metadata_bytes() {
            return Err(PhysicalResidencyDenial::MetadataBudgetExceeded);
        }
        let mut frames = HashMap::new();
        frames
            .try_reserve(frame_count)
            .map_err(|_| PhysicalResidencyDenial::AllocationFailed)?;
        let metadata = frames
            .capacity()
            .checked_mul(metadata_per_entry)
            .and_then(|bytes| bytes.checked_add(std::mem::size_of::<PoolState>()))
            .ok_or(PhysicalResidencyDenial::MetadataBudgetExceeded)? as u64;
        if metadata > limits.metadata_bytes() {
            return Err(PhysicalResidencyDenial::MetadataBudgetExceeded);
        }
        Ok(Self {
            inner: Arc::new(PoolInner {
                store,
                incarnation: PhysicalResidencyIncarnation::next()
                    .ok_or(PhysicalResidencyDenial::AllocationFailed)?,
                limits,
                metadata_bytes: metadata,
                state: Mutex::new(PoolState {
                    frames,
                    counters: PhysicalResidencyCounters {
                        metadata_bytes: metadata,
                        peak_admitted_bytes: metadata,
                        ..PhysicalResidencyCounters::default()
                    },
                    evictable_head: None,
                    evictable_tail: None,
                    loading_frames: 0,
                    candidate_publication_active: false,
                    accepting: true,
                    closed: false,
                }),
                changed: Condvar::new(),
            }),
        })
    }

    pub fn store_identity(&self) -> StableStoreIdentity {
        self.inner.store
    }

    pub fn incarnation(&self) -> PhysicalResidencyIncarnation {
        self.inner.incarnation
    }

    pub fn load<E, F>(
        &self,
        key: PhysicalFrameKey,
        fill: F,
    ) -> Result<PhysicalFrameLease, PhysicalFrameLoadError<E>>
    where
        F: FnOnce(&mut [u8]) -> Result<(), E>,
    {
        self.inner.validate_key(key).map_err(|reason| {
            PhysicalFrameLoadError::Residency(self.inner.record_denial(reason))
        })?;
        loop {
            if let Some(lease) = self
                .inner
                .try_pin_resident(key)
                .map_err(PhysicalFrameLoadError::Residency)?
            {
                return Ok(lease);
            }
            match self.inner.reserve_loading(key) {
                Ok(()) => break,
                Err(PhysicalResidencyDenial::FrameAlreadyResident) => continue,
                Err(reason) => return Err(PhysicalFrameLoadError::Residency(reason)),
            }
        }
        let mut reservation = LoadingReservation::new(Arc::clone(&self.inner), key);
        let length = key.coordinate.length() as usize;
        let mut bytes = Vec::new();
        if bytes.try_reserve_exact(length).is_err() {
            return Err(PhysicalFrameLoadError::Residency(
                self.inner
                    .record_denial(PhysicalResidencyDenial::AllocationFailed),
            ));
        }
        bytes.resize(length, 0);
        if let Err(error) = fill(bytes.as_mut_slice()) {
            return Err(PhysicalFrameLoadError::Source(error));
        }
        self.inner.record_source_load();
        let lease = self
            .inner
            .finish_loading(key, Arc::new(bytes))
            .map_err(PhysicalFrameLoadError::Residency)?;
        reservation.disarm();
        Ok(lease)
    }

    pub fn admit_dirty(
        &self,
        key: PhysicalFrameKey,
        bytes: Vec<u8>,
    ) -> Result<DirtyPhysicalFrame, PhysicalResidencyDenial> {
        self.inner
            .validate_key(key)
            .map_err(|reason| self.inner.record_denial(reason))?;
        if bytes.len() != key.coordinate.length() as usize {
            return Err(self
                .inner
                .record_denial(PhysicalResidencyDenial::FrameLengthMismatch));
        }
        let mut batch = self.reserve_candidate_frames(&[key])?;
        batch.reserve_next(key)?.admit(bytes)
    }

    pub fn reserve_candidate_frames(
        &self,
        keys: &[PhysicalFrameKey],
    ) -> Result<PhysicalCandidateBatchReservation, PhysicalResidencyDenial> {
        self.inner.reserve_candidate_frames(keys)
    }

    pub fn invalidate_clean(&self, key: PhysicalFrameKey) -> Result<(), PhysicalResidencyDenial> {
        self.inner.invalidate_clean(key)
    }

    pub fn begin_operation(
        &self,
        scope: OperationAllocationScope,
        bytes: u64,
    ) -> Result<OperationAllocationGrant, PhysicalResidencyDenial> {
        self.inner.reserve_operation(scope, bytes)?;
        Ok(OperationAllocationGrant {
            owner: Arc::clone(&self.inner),
            scope,
            bytes,
        })
    }

    pub fn begin_speculative(
        &self,
        kind: crate::SpeculativePhysicalWorkKind,
        frames: u32,
    ) -> Result<SpeculativeResidencyGrant, PhysicalResidencyDenial> {
        self.inner.reserve_speculative(kind, frames)?;
        Ok(SpeculativeResidencyGrant {
            owner: Arc::clone(&self.inner),
            kind,
            frames,
        })
    }

    pub fn claim_writeback(
        &self,
        frames: Vec<PhysicalFrameKey>,
    ) -> Result<PhysicalWritebackClaim, PhysicalResidencyDenial> {
        let bytes = self.inner.claim_writeback(&frames)?;
        Ok(PhysicalWritebackClaim {
            owner: Arc::clone(&self.inner),
            frames,
            bytes,
            armed: true,
        })
    }

    pub fn promote_clean_identity(
        &self,
        source: PhysicalFrameKey,
        target: PhysicalFrameKey,
    ) -> Result<(), PhysicalResidencyDenial> {
        self.inner.promote_clean_identity(source, target)
    }

    pub fn counters(&self) -> PhysicalResidencyCounters {
        self.inner.lock().counters
    }

    pub fn close(&self) -> PhysicalResidencyShutdown {
        let mut state = self.inner.lock();
        if state.closed {
            return PhysicalResidencyShutdown::new(state.counters);
        }
        state.accepting = false;
        self.inner.changed.notify_all();
        while let Some(coordinate) = state.pop_oldest_evictable() {
            if let Some(entry) = state.frames.remove(&coordinate) {
                state.counters.resident_bytes -= entry.bytes;
                state.counters.administrative_drains += 1;
            }
        }
        state.closed = true;
        self.inner.changed.notify_all();
        PhysicalResidencyShutdown::new(state.counters)
    }

    pub fn drain_unpinned_clean_frames(&self) -> u64 {
        let mut state = self.inner.lock();
        if !state.accepting {
            return 0;
        }
        let mut drained = 0;
        loop {
            let coordinate = state.pop_oldest_evictable();
            let Some(coordinate) = coordinate else { break };
            if let Some(entry) = state.frames.remove(&coordinate) {
                state.counters.resident_bytes -= entry.bytes;
                if entry.origin == FrameOrigin::Candidate {
                    state.counters.candidate_frames -= 1;
                }
                state.counters.administrative_drains += 1;
                drained += 1;
            }
        }
        drained
    }
}
