use super::*;

mod completion;
mod failure;
mod join;

#[derive(Clone, Copy)]
enum BoundedAccessPosture {
    Loading {
        identity: PhysicalFrameLoadingIdentity,
        admitted_limit: u32,
    },
    LoadFailed(PhysicalFrameLoadTerminal),
    Resident(RecordFrameCoordinate),
    CandidatePublicationActive,
    Absent,
}

#[derive(Debug)]
pub(super) enum BoundedFrameEntry {
    Loading {
        identity: PhysicalFrameLoadingIdentity,
        admitted_limit: u32,
        waiters: u32,
    },
    LoadFailed {
        terminal: PhysicalFrameLoadTerminal,
        waiters: u32,
    },
    Resident {
        length: u32,
        frame: FrameEntry,
    },
}

impl BoundedFrameEntry {
    pub(super) fn resident_coordinate(
        &self,
        key: PhysicalBoundedFrameKey,
    ) -> Option<RecordFrameCoordinate> {
        match self {
            Self::Resident { length, .. } => RecordFrameCoordinate::new(key.artifact(), 0, *length),
            _ => None,
        }
    }

    pub(super) fn resident_frame(&self) -> Option<&FrameEntry> {
        match self {
            Self::Resident { frame, .. } => Some(frame),
            _ => None,
        }
    }

    pub(super) fn resident_frame_mut(&mut self) -> Option<&mut FrameEntry> {
        match self {
            Self::Resident { frame, .. } => Some(frame),
            _ => None,
        }
    }

    pub(super) fn into_resident_frame(self) -> Option<FrameEntry> {
        match self {
            Self::Resident { frame, .. } => Some(frame),
            _ => None,
        }
    }

    pub(super) fn resolve(&mut self, length: u32, frame: FrameEntry) {
        *self = Self::Resident { length, frame };
    }
}

impl PoolInner {
    pub(super) fn access_bounded_frame(
        self: &Arc<Self>,
        scope: PhysicalOperationAllocationScope,
        key: PhysicalBoundedFrameKey,
    ) -> Result<PhysicalBoundedFrameAccess, PhysicalResidencyDenial> {
        let mut state = self.lock();
        if !state.accepting {
            return Err(Self::deny(&mut state, PhysicalResidencyDenial::PoolClosed));
        }
        match Self::bounded_access_posture(&state, key) {
            BoundedAccessPosture::Loading {
                identity,
                admitted_limit,
            } => {
                if key.limit() != admitted_limit {
                    return Err(Self::deny(
                        &mut state,
                        PhysicalResidencyDenial::BoundedLoadLimitConflict {
                            active_limit: admitted_limit,
                            requested_limit: key.limit(),
                        },
                    ));
                }
                self.attach_bounded_waiter(&mut state, scope, key, identity)
            }
            BoundedAccessPosture::LoadFailed(terminal) => Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::FrameLoadTerminated(terminal),
            )),
            BoundedAccessPosture::Resident(coordinate) => {
                if coordinate.length() > key.limit() {
                    return Err(Self::deny(
                        &mut state,
                        PhysicalResidencyDenial::FrameLengthMismatch,
                    ));
                }
                let exact = PhysicalFrameKey::new(self.store, coordinate);
                self.pin_resident_frame(&mut state, scope, exact)
                    .map(PhysicalBoundedFrameAccess::Hit)
            }
            BoundedAccessPosture::CandidatePublicationActive => Err(Self::deny(
                &mut state,
                PhysicalResidencyDenial::CandidatePublicationActive,
            )),
            BoundedAccessPosture::Absent => self.reserve_bounded_loading(&mut state, scope, key),
        }
    }

    fn bounded_access_posture(
        state: &PoolState,
        key: PhysicalBoundedFrameKey,
    ) -> BoundedAccessPosture {
        if let Some(frame) = state.frames.get_complete_artifact(&key) {
            return match frame.artifact_posture {
                FrameArtifactPosture::CompleteCandidate => {
                    BoundedAccessPosture::CandidatePublicationActive
                }
                FrameArtifactPosture::CompleteResident => {
                    let length = u32::try_from(frame.bytes)
                        .expect("physical frame bytes originate in a u32 coordinate");
                    let coordinate = RecordFrameCoordinate::new(key.artifact(), 0, length)
                        .expect("a complete resident candidate has nonzero bytes");
                    BoundedAccessPosture::Resident(coordinate)
                }
                FrameArtifactPosture::Fragment => {
                    unreachable!("only complete exact frames have an artifact alias")
                }
            };
        }
        let Some(entry) = state.frames.get_bounded(&key) else {
            return BoundedAccessPosture::Absent;
        };
        match entry {
            BoundedFrameEntry::Loading {
                identity,
                admitted_limit,
                ..
            } => BoundedAccessPosture::Loading {
                identity: *identity,
                admitted_limit: *admitted_limit,
            },
            BoundedFrameEntry::LoadFailed { terminal, .. } => {
                BoundedAccessPosture::LoadFailed(*terminal)
            }
            BoundedFrameEntry::Resident { .. } => {
                let coordinate = entry
                    .resident_coordinate(key)
                    .expect("a resident bounded identity derives one exact coordinate");
                BoundedAccessPosture::Resident(coordinate)
            }
        }
    }

    fn reserve_bounded_loading(
        self: &Arc<Self>,
        state: &mut PoolState,
        scope: PhysicalOperationAllocationScope,
        key: PhysicalBoundedFrameKey,
    ) -> Result<PhysicalBoundedFrameAccess, PhysicalResidencyDenial> {
        let ordinal = state.next_loading_ordinal;
        let next_ordinal = ordinal
            .checked_add(1)
            .ok_or_else(|| Self::deny(state, PhysicalResidencyDenial::AllocationFailed))?;
        self.reserve_frame_space(state, scope, u64::from(key.limit()))?;
        state.next_loading_ordinal = next_ordinal;
        let identity = PhysicalFrameLoadingIdentity::new(self.incarnation, ordinal);
        state.frames.insert_bounded(
            key,
            BoundedFrameEntry::Loading {
                identity,
                admitted_limit: key.limit(),
                waiters: 0,
            },
        );
        state
            .accounting
            .admit_frame(u64::from(key.limit()), false, false);
        state.loading_frames += 1;
        Ok(PhysicalBoundedFrameAccess::Fault(
            PhysicalBoundedFrameFaultOwner {
                owner: Arc::clone(self),
                key,
                identity,
                armed: true,
            },
        ))
    }

    fn attach_bounded_waiter(
        self: &Arc<Self>,
        state: &mut PoolState,
        scope: PhysicalOperationAllocationScope,
        key: PhysicalBoundedFrameKey,
        identity: PhysicalFrameLoadingIdentity,
    ) -> Result<PhysicalBoundedFrameAccess, PhysicalResidencyDenial> {
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
            .get_bounded_mut(&key)
            .expect("the classified bounded loading frame remains indexed");
        let BoundedFrameEntry::Loading {
            identity: found,
            waiters,
            ..
        } = entry
        else {
            return Err(Self::deny(
                state,
                PhysicalResidencyDenial::FrameLoadTerminated(PhysicalFrameLoadTerminal::new(
                    identity,
                    PhysicalFrameLoadTerminalKind::FaultOwnerAbandoned,
                )),
            ));
        };
        if *found != identity {
            return Err(Self::deny(
                state,
                PhysicalResidencyDenial::FrameLoadTerminated(PhysicalFrameLoadTerminal::new(
                    identity,
                    PhysicalFrameLoadTerminalKind::FaultOwnerAbandoned,
                )),
            ));
        }
        *waiters += 1;
        state.accounting.attach_loading_waiter();
        Ok(PhysicalBoundedFrameAccess::Coalesced(
            PhysicalBoundedFrameFaultWaiter {
                owner: Arc::clone(self),
                key,
                identity,
                armed: true,
            },
        ))
    }
}
