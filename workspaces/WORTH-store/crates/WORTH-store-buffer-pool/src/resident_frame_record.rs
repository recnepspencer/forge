use crate::{
    DirtyPageAccessOrigin, EvictionProtectionReason, EvictionProtectionSummary, LeaseEpoch,
    ResidentFrameBytes, ResidentFrameDenial, ResidentFrameDenialKind, ResidentFrameIdentity,
    ResidentFrameLoadRequest,
};

use crate::dirty_publication::DirtyPublicationEpoch;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResidentFrameRecord {
    identity: ResidentFrameIdentity,
    request: ResidentFrameLoadRequest,
    pin_state: ResidentFramePinState,
    eviction_protection: ResidentFrameEvictionProtection,
    dirty_state: ResidentFrameDirtyState,
    dirty_epoch: DirtyPublicationEpoch,
    bytes: Option<ResidentFrameBytes>,
}

impl ResidentFrameRecord {
    pub(crate) const fn new(
        identity: ResidentFrameIdentity,
        request: ResidentFrameLoadRequest,
        bytes: Option<ResidentFrameBytes>,
    ) -> Self {
        Self {
            identity,
            request,
            pin_state: ResidentFramePinState::unpinned(),
            eviction_protection: ResidentFrameEvictionProtection::unprotected(),
            dirty_state: ResidentFrameDirtyState::Clean,
            dirty_epoch: DirtyPublicationEpoch::initial(),
            bytes,
        }
    }

    pub(crate) const fn identity(&self) -> ResidentFrameIdentity {
        self.identity
    }

    pub(crate) const fn request(&self) -> ResidentFrameLoadRequest {
        self.request
    }

    pub(crate) const fn bytes(&self) -> Option<&ResidentFrameBytes> {
        self.bytes.as_ref()
    }

    pub(crate) fn attach_resident_bytes(&mut self, bytes: ResidentFrameBytes) {
        self.bytes = Some(bytes);
    }

    pub(crate) const fn has_active_pin(&self) -> bool {
        self.pin_state.active_pin_count > 0
    }

    pub(crate) const fn has_resident_dirty_delta(&self) -> bool {
        self.dirty_state.has_resident_dirty_delta()
    }

    pub(crate) const fn has_unflushed_dirty_state(&self) -> bool {
        self.dirty_state.has_unflushed_dirty_state()
    }

    pub(crate) const fn eviction_protection_summary(&self) -> EvictionProtectionSummary {
        let mut summary = EvictionProtectionSummary::empty();
        if self.has_active_pin() {
            summary = summary.with_reason(EvictionProtectionReason::Pinned);
        }
        if self.has_unflushed_dirty_state() {
            summary = summary.with_reason(EvictionProtectionReason::DirtyUnpublished);
        }
        if self.eviction_protection.verifier_protected {
            summary = summary.with_reason(EvictionProtectionReason::VerifierProtected);
        }
        if self.eviction_protection.recovery_protected {
            summary = summary.with_reason(EvictionProtectionReason::RecoveryProtected);
        }
        if self.eviction_protection.streaming_protected {
            summary = summary.with_reason(EvictionProtectionReason::StreamingProtected);
        }
        summary
    }

    pub(crate) const fn active_pin_count(&self) -> u64 {
        self.pin_state.active_pin_count
    }

    pub(crate) const fn lease_epoch(&self) -> LeaseEpoch {
        self.pin_state.lease_epoch
    }

    pub(crate) const fn dirty_publication_epoch(&self) -> DirtyPublicationEpoch {
        self.dirty_epoch
    }

    pub(crate) fn mark_pinned(&mut self) {
        self.pin_state.active_pin_count += 1;
    }

    pub(crate) fn mark_unpinned(&mut self) -> Result<(), ResidentFrameDenial> {
        if self.pin_state.active_pin_count == 0 {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::PageLeaseNotPinned,
            ));
        }
        self.pin_state.active_pin_count -= 1;
        Ok(())
    }

    pub(crate) fn clear_pin_after_abnormal_close(&mut self) {
        self.pin_state.active_pin_count = 0;
    }

    pub(crate) fn mark_unreported_leak(&mut self) -> u64 {
        if self.pin_state.active_pin_count == 0 || self.pin_state.leak_reported {
            return 0;
        }
        self.pin_state.leak_reported = true;
        self.pin_state.active_pin_count
    }

    pub(crate) fn mark_eviction_protected(&mut self, reason: EvictionProtectionReason) {
        self.eviction_protection.mark(reason);
    }

    pub(crate) const fn dirty_mark_transition(&self) -> ResidentFrameDirtyMarkTransition {
        self.dirty_state.dirty_mark_transition()
    }

    pub(crate) const fn dirty_access_origin(&self) -> DirtyPageAccessOrigin {
        self.dirty_state.access_origin()
    }

    pub(crate) const fn write_schedule_transition(
        &self,
    ) -> Option<ResidentFrameWriteScheduleTransition> {
        self.dirty_state.write_schedule_transition()
    }

    pub(crate) fn mark_dirty(&mut self) -> ResidentFrameDirtyMarkTransition {
        self.mark_dirty_from_origin(DirtyPageAccessOrigin::StoreBuffer)
    }

    pub(crate) fn mark_mmap_dirty(&mut self) -> ResidentFrameDirtyMarkTransition {
        self.mark_dirty_from_origin(DirtyPageAccessOrigin::Mmap)
    }

    fn mark_dirty_from_origin(
        &mut self,
        origin: DirtyPageAccessOrigin,
    ) -> ResidentFrameDirtyMarkTransition {
        let transition = self.dirty_mark_transition();
        if matches!(
            transition,
            ResidentFrameDirtyMarkTransition::NewlyDirty
                | ResidentFrameDirtyMarkTransition::NewlyDirtyBehindScheduledWrite
        ) {
            self.dirty_epoch = self.dirty_epoch.next();
        }
        self.dirty_state = match self.dirty_state {
            ResidentFrameDirtyState::Clean => ResidentFrameDirtyState::ResidentDirty { origin },
            ResidentFrameDirtyState::ResidentDirty {
                origin: existing_origin,
            } => ResidentFrameDirtyState::ResidentDirty {
                origin: existing_origin.merge(origin),
            },
            ResidentFrameDirtyState::WriteScheduledNotDurable
            | ResidentFrameDirtyState::WriteScheduledAndResidentDirty { origin: _ } => {
                let origin = self.dirty_state.access_origin().merge(origin);
                ResidentFrameDirtyState::WriteScheduledAndResidentDirty { origin }
            }
        };
        transition
    }

    pub(crate) fn mark_write_scheduled_not_durable(
        &mut self,
    ) -> Option<ResidentFrameWriteScheduleTransition> {
        let transition = self.write_schedule_transition()?;
        self.dirty_state = match self.dirty_state {
            ResidentFrameDirtyState::ResidentDirty { .. } => {
                ResidentFrameDirtyState::WriteScheduledNotDurable
            }
            ResidentFrameDirtyState::WriteScheduledAndResidentDirty { .. } => {
                ResidentFrameDirtyState::WriteScheduledNotDurable
            }
            ResidentFrameDirtyState::Clean | ResidentFrameDirtyState::WriteScheduledNotDurable => {
                return None;
            }
        };
        Some(transition)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResidentFrameDirtyMarkTransition {
    NewlyDirty,
    AlreadyResidentDirty,
    NewlyDirtyBehindScheduledWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResidentFrameWriteScheduleTransition {
    FirstScheduledWrite,
    AdditionalScheduledWriteBehindPendingWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResidentFrameDirtyState {
    Clean,
    ResidentDirty { origin: DirtyPageAccessOrigin },
    WriteScheduledNotDurable,
    WriteScheduledAndResidentDirty { origin: DirtyPageAccessOrigin },
}

impl DirtyPageAccessOrigin {
    const fn merge(self, next: Self) -> Self {
        match (self, next) {
            (Self::Mmap, _) | (_, Self::Mmap) => Self::Mmap,
            (Self::StoreBuffer, Self::StoreBuffer) => Self::StoreBuffer,
        }
    }
}

impl ResidentFrameDirtyState {
    const fn has_resident_dirty_delta(self) -> bool {
        matches!(
            self,
            Self::ResidentDirty { .. } | Self::WriteScheduledAndResidentDirty { .. }
        )
    }

    const fn has_unflushed_dirty_state(self) -> bool {
        !matches!(self, Self::Clean)
    }

    const fn dirty_mark_transition(self) -> ResidentFrameDirtyMarkTransition {
        match self {
            Self::Clean => ResidentFrameDirtyMarkTransition::NewlyDirty,
            Self::ResidentDirty { .. } | Self::WriteScheduledAndResidentDirty { .. } => {
                ResidentFrameDirtyMarkTransition::AlreadyResidentDirty
            }
            Self::WriteScheduledNotDurable => {
                ResidentFrameDirtyMarkTransition::NewlyDirtyBehindScheduledWrite
            }
        }
    }

    const fn write_schedule_transition(self) -> Option<ResidentFrameWriteScheduleTransition> {
        match self {
            Self::ResidentDirty { .. } => {
                Some(ResidentFrameWriteScheduleTransition::FirstScheduledWrite)
            }
            Self::WriteScheduledAndResidentDirty { .. } => Some(
                ResidentFrameWriteScheduleTransition::AdditionalScheduledWriteBehindPendingWrite,
            ),
            Self::Clean | Self::WriteScheduledNotDurable => None,
        }
    }

    const fn access_origin(self) -> DirtyPageAccessOrigin {
        match self {
            Self::ResidentDirty { origin } | Self::WriteScheduledAndResidentDirty { origin } => {
                origin
            }
            Self::Clean | Self::WriteScheduledNotDurable => DirtyPageAccessOrigin::StoreBuffer,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ResidentFramePinState {
    active_pin_count: u64,
    lease_epoch: LeaseEpoch,
    leak_reported: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ResidentFrameEvictionProtection {
    verifier_protected: bool,
    recovery_protected: bool,
    streaming_protected: bool,
}

impl ResidentFrameEvictionProtection {
    const fn unprotected() -> Self {
        Self {
            verifier_protected: false,
            recovery_protected: false,
            streaming_protected: false,
        }
    }

    fn mark(&mut self, reason: EvictionProtectionReason) {
        match reason {
            EvictionProtectionReason::Pinned | EvictionProtectionReason::DirtyUnpublished => {}
            EvictionProtectionReason::VerifierProtected => self.verifier_protected = true,
            EvictionProtectionReason::RecoveryProtected => self.recovery_protected = true,
            EvictionProtectionReason::StreamingProtected => self.streaming_protected = true,
        }
    }
}

impl ResidentFramePinState {
    const fn unpinned() -> Self {
        Self {
            active_pin_count: 0,
            lease_epoch: LeaseEpoch::initial(),
            leak_reported: false,
        }
    }
}
