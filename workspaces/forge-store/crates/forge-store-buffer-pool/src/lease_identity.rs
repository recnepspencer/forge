use crate::ResidentFrameIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeaseScope {
    identity: ResidentFrameIdentity,
}

impl LeaseScope {
    pub(crate) const fn new(identity: ResidentFrameIdentity) -> Self {
        Self { identity }
    }

    pub const fn resident_frame_identity(self) -> ResidentFrameIdentity {
        self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LeaseEpoch(u64);

impl LeaseEpoch {
    pub(crate) const fn initial() -> Self {
        Self(1)
    }

    pub(crate) fn next(self) -> Option<Self> {
        self.0.checked_add(1).map(Self)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageLeaseId {
    scope: LeaseScope,
    epoch: LeaseEpoch,
}

impl PageLeaseId {
    pub(crate) const fn new(scope: LeaseScope, epoch: LeaseEpoch) -> Self {
        Self { scope, epoch }
    }

    pub const fn scope(self) -> LeaseScope {
        self.scope
    }

    pub const fn epoch(self) -> LeaseEpoch {
        self.epoch
    }
}
