use super::{dominant, UiNativeRecoveryCause};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativePhysicalRecoveryPreparation {
    epoch: u64,
    cause: UiNativeRecoveryCause,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativePhysicalRecoveryFact {
    epoch: u64,
    cause: UiNativeRecoveryCause,
    device_generation: u64,
    surface_generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UiNativePhysicalRecoveryState {
    Pending(UiNativePhysicalRecoveryPreparation),
    Prepared(UiNativePhysicalRecoveryFact),
}

#[derive(Clone, Copy)]
pub(super) struct UiNativePhysicalRecoveryAdmission {
    epoch: u64,
    replaces_references: bool,
}

pub(super) struct UiNativePhysicalRecoveryOwner {
    next_epoch: u64,
    state: Option<UiNativePhysicalRecoveryState>,
}

impl Default for UiNativePhysicalRecoveryOwner {
    fn default() -> Self {
        Self {
            next_epoch: 1,
            state: None,
        }
    }
}

impl UiNativePhysicalRecoveryOwner {
    pub(super) fn require(
        &mut self,
        cause: UiNativeRecoveryCause,
    ) -> UiNativePhysicalRecoveryAdmission {
        match self.state {
            None => self.issue(cause),
            Some(UiNativePhysicalRecoveryState::Pending(mut pending)) => {
                pending.cause = dominant(pending.cause, cause);
                self.state = Some(UiNativePhysicalRecoveryState::Pending(pending));
                UiNativePhysicalRecoveryAdmission {
                    epoch: pending.epoch,
                    replaces_references: false,
                }
            }
            Some(UiNativePhysicalRecoveryState::Prepared(_)) => self.issue(cause),
        }
    }

    pub(super) fn preparation(&self, epoch: u64) -> Option<UiNativePhysicalRecoveryPreparation> {
        match self.state {
            Some(UiNativePhysicalRecoveryState::Pending(preparation))
                if preparation.epoch == epoch =>
            {
                Some(preparation)
            }
            _ => None,
        }
    }

    pub(super) fn commit(
        &mut self,
        preparation: UiNativePhysicalRecoveryPreparation,
        device_generation: u64,
        surface_generation: u64,
    ) -> bool {
        if self.state != Some(UiNativePhysicalRecoveryState::Pending(preparation)) {
            return false;
        }
        self.state = Some(UiNativePhysicalRecoveryState::Prepared(
            UiNativePhysicalRecoveryFact {
                epoch: preparation.epoch,
                cause: preparation.cause,
                device_generation,
                surface_generation,
            },
        ));
        true
    }

    pub(super) fn fact(&self, epoch: u64) -> Option<UiNativePhysicalRecoveryFact> {
        match self.state {
            Some(UiNativePhysicalRecoveryState::Prepared(fact)) if fact.epoch == epoch => {
                Some(fact)
            }
            _ => None,
        }
    }

    pub(super) fn epoch(&self) -> Option<u64> {
        self.state.map(|state| match state {
            UiNativePhysicalRecoveryState::Pending(preparation) => preparation.epoch,
            UiNativePhysicalRecoveryState::Prepared(fact) => fact.epoch,
        })
    }

    pub(super) fn cause(&self) -> Option<UiNativeRecoveryCause> {
        self.state.map(|state| match state {
            UiNativePhysicalRecoveryState::Pending(preparation) => preparation.cause,
            UiNativePhysicalRecoveryState::Prepared(fact) => fact.cause,
        })
    }

    pub(super) fn clear(&mut self) {
        self.state = None;
    }

    fn issue(&mut self, cause: UiNativeRecoveryCause) -> UiNativePhysicalRecoveryAdmission {
        let epoch = self.next_epoch;
        self.next_epoch = self.next_epoch.saturating_add(1);
        self.state = Some(UiNativePhysicalRecoveryState::Pending(
            UiNativePhysicalRecoveryPreparation { epoch, cause },
        ));
        UiNativePhysicalRecoveryAdmission {
            epoch,
            replaces_references: true,
        }
    }
}

impl UiNativePhysicalRecoveryAdmission {
    pub(super) const fn epoch(self) -> u64 {
        self.epoch
    }

    pub(super) const fn replaces_references(self) -> bool {
        self.replaces_references
    }
}

impl UiNativePhysicalRecoveryPreparation {
    pub(crate) const fn cause(self) -> UiNativeRecoveryCause {
        self.cause
    }
}

impl UiNativePhysicalRecoveryFact {
    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) const fn generations(self) -> [u64; 2] {
        [self.device_generation, self.surface_generation]
    }
}
