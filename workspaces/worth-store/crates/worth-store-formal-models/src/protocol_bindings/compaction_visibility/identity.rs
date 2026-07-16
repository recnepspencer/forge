use crate::protocols::compaction_visibility::CompactionVisibilityAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompactionVisibilityOwnerCaseFamily {
    LsmMembership,
    LsmExecution,
    LsmMaintenance,
    PhysicalCompaction,
}

impl CompactionVisibilityOwnerCaseFamily {
    pub const fn all() -> [Self; 4] {
        [
            Self::LsmMembership,
            Self::LsmExecution,
            Self::LsmMaintenance,
            Self::PhysicalCompaction,
        ]
    }

    pub(crate) const fn index(self) -> usize {
        match self {
            Self::LsmMembership => 0,
            Self::LsmExecution => 1,
            Self::LsmMaintenance => 2,
            Self::PhysicalCompaction => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompactionVisibilityOwnerCase {
    LsmMembership(worth_store_lsm_authority::LsmMembershipOwnerCaseId),
    LsmExecution(worth_store_layout_indexes::LsmExecutionOwnerCaseId),
    LsmMaintenance(worth_store_layout_indexes::LsmMaintenanceOwnerCaseId),
    PhysicalCompaction(worth_store_physical_isolation::CompactionOwnerCaseId),
}

impl CompactionVisibilityOwnerCase {
    pub const fn family(self) -> CompactionVisibilityOwnerCaseFamily {
        match self {
            Self::LsmMembership(_) => CompactionVisibilityOwnerCaseFamily::LsmMembership,
            Self::LsmExecution(_) => CompactionVisibilityOwnerCaseFamily::LsmExecution,
            Self::LsmMaintenance(_) => CompactionVisibilityOwnerCaseFamily::LsmMaintenance,
            Self::PhysicalCompaction(_) => CompactionVisibilityOwnerCaseFamily::PhysicalCompaction,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompactionVisibilityMappedOwnerCase {
    owner_case: CompactionVisibilityOwnerCase,
    action: CompactionVisibilityAction,
}

impl CompactionVisibilityMappedOwnerCase {
    pub(crate) const fn new(
        owner_case: CompactionVisibilityOwnerCase,
        action: CompactionVisibilityAction,
    ) -> Self {
        Self { owner_case, action }
    }

    pub const fn owner_case(self) -> CompactionVisibilityOwnerCase {
        self.owner_case
    }

    pub const fn action(self) -> CompactionVisibilityAction {
        self.action
    }
}
