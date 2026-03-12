use crate::validation::data::{InvariantCostClass, InvariantExecutionPoint, InvariantGroup, InvariantGroupSet};

use super::policy::InvariantExecutionPolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InvariantRequestProfile {
    CommitBoundary,
    MutationSensitive,
    SnapshotPublication,
    HarnessAudit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessAuditMode {
    Disabled,
    Full,
}

impl HarnessAuditMode {
    pub(crate) const fn request_profile(self) -> Option<InvariantRequestProfile> {
        match self {
            Self::Disabled => None,
            Self::Full => Some(InvariantRequestProfile::HarnessAudit),
        }
    }
}

impl InvariantRequestProfile {
    pub(crate) fn execution_point(self) -> InvariantExecutionPoint {
        match self {
            Self::CommitBoundary => InvariantExecutionPoint::CommitBoundary,
            Self::MutationSensitive => InvariantExecutionPoint::MutationSensitive,
            Self::SnapshotPublication => InvariantExecutionPoint::SnapshotPublication,
            Self::HarnessAudit => InvariantExecutionPoint::HarnessAudit,
        }
    }

    pub(crate) fn groups(self) -> InvariantGroupSet {
        match self {
            Self::CommitBoundary => InvariantGroupSet::of(InvariantGroup::Mutation)
                .union(InvariantGroupSet::of(InvariantGroup::Uniqueness))
                .union(InvariantGroupSet::of(InvariantGroup::History)),
            Self::MutationSensitive => {
                InvariantGroupSet::of(InvariantGroup::Structural)
                    .union(InvariantGroupSet::of(InvariantGroup::Mutation))
                    .union(InvariantGroupSet::of(InvariantGroup::Uniqueness))
            }
            Self::SnapshotPublication => {
                InvariantGroupSet::of(InvariantGroup::Snapshot)
                    .union(InvariantGroupSet::of(InvariantGroup::Publication))
            }
            Self::HarnessAudit => InvariantGroupSet::all(),
        }
    }

    pub(crate) fn policy(self) -> InvariantExecutionPolicy {
        match self {
            Self::CommitBoundary | Self::SnapshotPublication => {
                InvariantExecutionPolicy::MaxCost(InvariantCostClass::FullScan)
            }
            Self::MutationSensitive => {
                InvariantExecutionPolicy::MaxCost(InvariantCostClass::TargetedScan)
            }
            Self::HarnessAudit => InvariantExecutionPolicy::AllowAll,
        }
    }

    pub(crate) fn requires_plan(self) -> bool {
        matches!(self, Self::CommitBoundary)
    }
}
