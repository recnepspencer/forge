use super::*;

mod clean_publication;
mod frame_space;

#[derive(Clone, Copy)]
enum FrameAccessPosture {
    Loading(PhysicalFrameLoadingIdentity),
    LoadFailed(PhysicalFrameLoadTerminal),
    CandidateReserved,
    Resident,
    Absent,
}

enum LoadingJoinState {
    Loading,
    Failed(PhysicalFrameLoadTerminal),
    Resident(Arc<Vec<u8>>),
    Other,
}

impl FrameState {
    fn clone_for_loading_join(&self) -> LoadingJoinState {
        match self {
            Self::Loading => LoadingJoinState::Loading,
            Self::LoadFailed(terminal) => LoadingJoinState::Failed(*terminal),
            Self::Resident(bytes) => LoadingJoinState::Resident(Arc::clone(bytes)),
            Self::CandidateReserved => LoadingJoinState::Other,
        }
    }
}

impl PoolInner {
    pub(super) fn access_frame(
        self: &Arc<Self>,
        scope: PhysicalOperationAllocationScope,
        key: PhysicalFrameKey,
    ) -> Result<PhysicalFrameAccess, PhysicalResidencyDenial> {
        let mut state = self.lock();
        loop {
            if !state.accepting {
                return Err(Self::deny(&mut state, PhysicalResidencyDenial::PoolClosed));
            }
            match Self::frame_access_posture(&state, key) {
                FrameAccessPosture::Loading(identity) => {
                    return self.attach_loading_waiter(&mut state, scope, key, identity);
                }
                FrameAccessPosture::LoadFailed(terminal) => {
                    return Err(Self::deny(
                        &mut state,
                        PhysicalResidencyDenial::FrameLoadTerminated(terminal),
                    ));
                }
                FrameAccessPosture::CandidateReserved => {
                    state = self
                        .changed
                        .wait(state)
                        .unwrap_or_else(|poisoned| poisoned.into_inner());
                }
                FrameAccessPosture::Resident => {
                    return self
                        .pin_resident_frame(&mut state, scope, key)
                        .map(PhysicalFrameAccess::Hit);
                }
                FrameAccessPosture::Absent => {
                    return self
                        .reserve_loading(&mut state, scope, key)
                        .map(PhysicalFrameAccess::Fault);
                }
            }
        }
    }

    fn frame_access_posture(state: &PoolState, key: PhysicalFrameKey) -> FrameAccessPosture {
        let Some(entry) = state.frames.get(&key.coordinate) else {
            return FrameAccessPosture::Absent;
        };
        match &entry.state {
            FrameState::Loading => FrameAccessPosture::Loading(
                entry
                    .loading_identity
                    .expect("a loading frame has one loading identity"),
            ),
            FrameState::LoadFailed(terminal) => FrameAccessPosture::LoadFailed(*terminal),
            FrameState::CandidateReserved => FrameAccessPosture::CandidateReserved,
            FrameState::Resident(_) => FrameAccessPosture::Resident,
        }
    }

    fn reserve_loading(
        self: &Arc<Self>,
        state: &mut PoolState,
        scope: PhysicalOperationAllocationScope,
        key: PhysicalFrameKey,
    ) -> Result<PhysicalFrameFaultOwner, PhysicalResidencyDenial> {
        let ordinal = state.next_loading_ordinal;
        let next_ordinal = ordinal
            .checked_add(1)
            .ok_or_else(|| Self::deny(state, PhysicalResidencyDenial::AllocationFailed))?;
        self.reserve_frame_space(state, scope, key.coordinate.length() as u64)?;
        state.next_loading_ordinal = next_ordinal;
        let identity = PhysicalFrameLoadingIdentity::new(self.incarnation, ordinal);
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
                loading_identity: Some(identity),
                loading_waiters: 0,
                artifact_posture: FrameArtifactPosture::Fragment,
            },
        );
        state
            .accounting
            .admit_frame(u64::from(key.coordinate.length()), false, false);
        state.loading_frames += 1;
        Ok(PhysicalFrameFaultOwner {
            owner: Arc::clone(self),
            key,
            identity,
            armed: true,
        })
    }

    fn attach_loading_waiter(
        self: &Arc<Self>,
        state: &mut PoolState,
        scope: PhysicalOperationAllocationScope,
        key: PhysicalFrameKey,
        identity: PhysicalFrameLoadingIdentity,
    ) -> Result<PhysicalFrameAccess, PhysicalResidencyDenial> {
        if state.accounting.pin_leases() >= self.limits.pin_leases() {
            let current = u64::from(state.accounting.pin_leases());
            return Err(self.pressure(
                state,
                PhysicalResidencyPressureDemand {
                    dimension: PhysicalResidencyDimension::PinLeases,
                    scope,
                    requested: 1,
                    current,
                    limit: u64::from(self.limits.pin_leases()),
                },
            ));
        }
        let entry = state
            .frames
            .get_mut(&key.coordinate)
            .expect("the classified loading frame remains indexed");
        if entry.loading_identity != Some(identity) || !matches!(entry.state, FrameState::Loading) {
            return Err(Self::deny(
                state,
                PhysicalResidencyDenial::FrameLoadTerminated(PhysicalFrameLoadTerminal::new(
                    identity,
                    PhysicalFrameLoadTerminalKind::FaultOwnerAbandoned,
                )),
            ));
        }
        entry.pins += 1;
        entry.loading_waiters += 1;
        state.accounting.attach_loading_waiter();
        Ok(PhysicalFrameAccess::Coalesced(PhysicalFrameFaultWaiter {
            owner: Arc::clone(self),
            key,
            identity,
            armed: true,
        }))
    }

    pub(crate) fn finish_loading(
        self: &Arc<Self>,
        key: PhysicalFrameKey,
        identity: PhysicalFrameLoadingIdentity,
        bytes: Arc<Vec<u8>>,
    ) -> Result<PhysicalFrameLease, (PhysicalResidencyDenial, PhysicalFrameLoadTerminal)> {
        let mut state = self.lock();
        if !state.accepting {
            let terminal = Self::fail_loading_state(
                &mut state,
                key,
                identity,
                PhysicalFrameLoadTerminalKind::PoolClosed,
            );
            let denial = Self::deny(&mut state, PhysicalResidencyDenial::PoolClosed);
            self.changed.notify_all();
            return Err((denial, terminal));
        }
        let matches_identity = state.frames.get(&key.coordinate).is_some_and(|entry| {
            matches!(entry.state, FrameState::Loading) && entry.loading_identity == Some(identity)
        });
        if !matches_identity {
            let terminal = PhysicalFrameLoadTerminal::new(
                identity,
                PhysicalFrameLoadTerminalKind::FaultOwnerAbandoned,
            );
            let denial = Self::deny(
                &mut state,
                PhysicalResidencyDenial::FrameLoadTerminated(terminal),
            );
            return Err((denial, terminal));
        }
        state
            .frames
            .get_mut(&key.coordinate)
            .expect("loading reservation exists")
            .state = FrameState::Resident(Arc::clone(&bytes));
        state.loading_frames -= 1;
        state.accounting.finish_loading();
        self.changed.notify_all();
        Ok(PhysicalFrameLease {
            owner: Arc::clone(self),
            key,
            bytes,
        })
    }

    pub(crate) fn fail_loading(
        &self,
        key: PhysicalFrameKey,
        identity: PhysicalFrameLoadingIdentity,
        kind: PhysicalFrameLoadTerminalKind,
    ) -> PhysicalFrameLoadTerminal {
        let mut state = self.lock();
        let terminal = Self::fail_loading_state(&mut state, key, identity, kind);
        self.changed.notify_all();
        terminal
    }

    pub(crate) fn abandon_loading(
        &self,
        key: PhysicalFrameKey,
        identity: PhysicalFrameLoadingIdentity,
    ) {
        self.fail_loading(
            key,
            identity,
            PhysicalFrameLoadTerminalKind::FaultOwnerAbandoned,
        );
    }

    fn fail_loading_state(
        state: &mut PoolState,
        key: PhysicalFrameKey,
        identity: PhysicalFrameLoadingIdentity,
        kind: PhysicalFrameLoadTerminalKind,
    ) -> PhysicalFrameLoadTerminal {
        let terminal = PhysicalFrameLoadTerminal::new(identity, kind);
        let Some(entry) = state.frames.get(&key.coordinate) else {
            return terminal;
        };
        if !matches!(entry.state, FrameState::Loading) || entry.loading_identity != Some(identity) {
            return terminal;
        }
        let bytes = entry.bytes;
        let pins = entry.pins;
        let waiters = entry.loading_waiters;
        state.loading_frames -= 1;
        state.accounting.fail_loading(bytes, pins, waiters > 0);
        if waiters == 0 {
            state.frames.remove(&key.coordinate);
        } else {
            let entry = state
                .frames
                .get_mut(&key.coordinate)
                .expect("retained failed loading identity remains indexed");
            entry.state = FrameState::LoadFailed(terminal);
            entry.pins = 0;
        }
        terminal
    }

    pub(crate) fn join_loading(
        self: &Arc<Self>,
        key: PhysicalFrameKey,
        identity: PhysicalFrameLoadingIdentity,
    ) -> Result<PhysicalFrameLease, PhysicalFrameLoadTerminal> {
        let mut state = self.lock();
        loop {
            let posture = state.frames.get(&key.coordinate).map(|entry| {
                (
                    entry.state.clone_for_loading_join(),
                    entry.loading_identity,
                    entry.loading_waiters,
                )
            });
            match posture {
                Some((LoadingJoinState::Loading, Some(found), _)) if found == identity => {
                    state = self
                        .changed
                        .wait(state)
                        .unwrap_or_else(|poisoned| poisoned.into_inner());
                }
                Some((LoadingJoinState::Resident(bytes), Some(found), waiters))
                    if found == identity && waiters > 0 =>
                {
                    let entry = state
                        .frames
                        .get_mut(&key.coordinate)
                        .expect("joined resident frame remains indexed");
                    entry.loading_waiters -= 1;
                    if entry.loading_waiters == 0 {
                        entry.loading_identity = None;
                    }
                    return Ok(PhysicalFrameLease {
                        owner: Arc::clone(self),
                        key,
                        bytes,
                    });
                }
                Some((LoadingJoinState::Failed(terminal), Some(found), waiters))
                    if found == identity && waiters > 0 =>
                {
                    Self::release_failed_waiter(&mut state, key);
                    self.changed.notify_all();
                    return Err(terminal);
                }
                _ => {
                    return Err(PhysicalFrameLoadTerminal::new(
                        identity,
                        PhysicalFrameLoadTerminalKind::FaultOwnerAbandoned,
                    ));
                }
            }
        }
    }

    pub(crate) fn release_loading_waiter(
        &self,
        key: PhysicalFrameKey,
        identity: PhysicalFrameLoadingIdentity,
    ) {
        let mut state = self.lock();
        let Some(entry) = state.frames.get(&key.coordinate) else {
            return;
        };
        if entry.loading_identity != Some(identity) || entry.loading_waiters == 0 {
            return;
        }
        match &entry.state {
            FrameState::LoadFailed(_) => Self::release_failed_waiter(&mut state, key),
            FrameState::Loading => {
                let entry = state
                    .frames
                    .get_mut(&key.coordinate)
                    .expect("loading waiter frame remains indexed");
                entry.loading_waiters -= 1;
                entry.pins -= 1;
                state.accounting.unpin(false);
            }
            FrameState::Resident(_) => {
                let (became_unpinned, became_evictable) = {
                    let entry = state
                        .frames
                        .get_mut(&key.coordinate)
                        .expect("resident waiter frame remains indexed");
                    entry.loading_waiters -= 1;
                    entry.pins -= 1;
                    let became_unpinned = entry.pins == 0;
                    if entry.loading_waiters == 0 {
                        entry.loading_identity = None;
                    }
                    (
                        became_unpinned,
                        became_unpinned && !entry.dirty && !entry.writeback_claimed,
                    )
                };
                state.accounting.unpin(became_unpinned);
                if became_evictable {
                    state.append_evictable(key.coordinate);
                    if state.closed {
                        state.drain_all_legal_clean_frames();
                    }
                }
            }
            FrameState::CandidateReserved => {}
        }
        self.changed.notify_all();
    }

    fn release_failed_waiter(state: &mut PoolState, key: PhysicalFrameKey) {
        let entry = state
            .frames
            .get_mut(&key.coordinate)
            .expect("failed loading waiter frame remains indexed");
        entry.loading_waiters -= 1;
        if entry.loading_waiters == 0 {
            state.frames.remove(&key.coordinate);
            state.accounting.release_failed_loading_identity();
        }
    }
}
