use super::replay_catalog::ReplayFamilyIdentity;
use super::undo_catalog::UndoFamilyIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplayFamilyConsumerRequirement {
    required_family: ReplayFamilyIdentity,
}

impl ReplayFamilyConsumerRequirement {
    pub const fn new(required_family: ReplayFamilyIdentity) -> Self {
        Self { required_family }
    }

    pub const fn required_family(&self) -> ReplayFamilyIdentity {
        self.required_family
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UndoFamilyConsumerRequirement {
    required_family: UndoFamilyIdentity,
}

impl UndoFamilyConsumerRequirement {
    pub const fn new(required_family: UndoFamilyIdentity) -> Self {
        Self { required_family }
    }

    pub const fn required_family(&self) -> UndoFamilyIdentity {
        self.required_family
    }
}

pub fn retained_replay_workload_consumer_requirement() -> ReplayFamilyConsumerRequirement {
    ReplayFamilyConsumerRequirement::new(ReplayFamilyIdentity::SpatialBooleanEventLedgerReplay)
}

pub fn replay_public_closeout_consumer_requirement() -> ReplayFamilyConsumerRequirement {
    ReplayFamilyConsumerRequirement::new(ReplayFamilyIdentity::SpatialBooleanEventLedgerReplay)
}

pub fn transaction_boundary_undo_consumer_requirement() -> UndoFamilyConsumerRequirement {
    UndoFamilyConsumerRequirement::new(UndoFamilyIdentity::SpatialBooleanEventLedgerRollback)
}
