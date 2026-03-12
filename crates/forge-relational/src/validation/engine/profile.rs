use crate::validation::data::{InvariantCostClass, InvariantExecutionPoint, InvariantGroup, InvariantGroupSet};

use super::policy::InvariantExecutionPolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvariantRequestProfile {
    CommitBoundary,
    MutationSensitive,
    MutationSensitiveState,
    SnapshotPublication,
    SnapshotPublicationState,
    HarnessAudit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessAuditMode {
    Disabled,
    Full,
}

impl InvariantRequestProfile {
    pub fn execution_point(self) -> InvariantExecutionPoint {
        match self {
            Self::CommitBoundary => InvariantExecutionPoint::CommitBoundary,
            Self::MutationSensitive | Self::MutationSensitiveState => {
                InvariantExecutionPoint::MutationSensitive
            }
            Self::SnapshotPublication | Self::SnapshotPublicationState => {
                InvariantExecutionPoint::SnapshotPublication
            }
            Self::HarnessAudit => InvariantExecutionPoint::HarnessAudit,
        }
    }

    pub fn groups(self) -> InvariantGroupSet {
        match self {
            Self::CommitBoundary => InvariantGroupSet::of(InvariantGroup::Mutation)
                .union(InvariantGroupSet::of(InvariantGroup::Uniqueness))
                .union(InvariantGroupSet::of(InvariantGroup::History)),
            Self::MutationSensitive | Self::MutationSensitiveState => {
                InvariantGroupSet::of(InvariantGroup::Structural)
                    .union(InvariantGroupSet::of(InvariantGroup::Mutation))
                    .union(InvariantGroupSet::of(InvariantGroup::Uniqueness))
            }
            Self::SnapshotPublication | Self::SnapshotPublicationState => {
                InvariantGroupSet::of(InvariantGroup::Snapshot)
                    .union(InvariantGroupSet::of(InvariantGroup::Publication))
            }
            Self::HarnessAudit => InvariantGroupSet::all(),
        }
    }

    pub fn policy(self) -> InvariantExecutionPolicy {
        match self {
            Self::CommitBoundary
            | Self::SnapshotPublication
            | Self::SnapshotPublicationState => {
                InvariantExecutionPolicy::MaxCost(InvariantCostClass::FullScan)
            }
            Self::MutationSensitive | Self::MutationSensitiveState => {
                InvariantExecutionPolicy::MaxCost(InvariantCostClass::TargetedScan)
            }
            Self::HarnessAudit => InvariantExecutionPolicy::AllowAll,
        }
    }

    pub fn requires_plan(self) -> bool {
        matches!(
            self,
            Self::CommitBoundary | Self::MutationSensitive | Self::SnapshotPublication
        )
    }
}
