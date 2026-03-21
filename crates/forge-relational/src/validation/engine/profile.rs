use crate::validation::data::{InvariantExecutionPoint, InvariantGroup, InvariantGroupSet};

use super::observation::InvariantObservationKind;

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

    pub(crate) fn consumed_groups(self) -> InvariantGroupSet {
        match self {
            Self::CommitBoundary => InvariantGroupSet::of(InvariantGroup::StorageCoherence)
                .union(InvariantGroupSet::of(InvariantGroup::IdentityCoherence))
                .union(InvariantGroupSet::of(InvariantGroup::SchemaCompliance))
                .union(InvariantGroupSet::of(InvariantGroup::RelationIntegrity))
                .union(InvariantGroupSet::of(InvariantGroup::LineageIntegrity))
                .union(InvariantGroupSet::of(InvariantGroup::PublicationCoherence)),
            Self::MutationSensitive => InvariantGroupSet::of(InvariantGroup::StorageCoherence)
                .union(InvariantGroupSet::of(InvariantGroup::IdentityCoherence))
                .union(InvariantGroupSet::of(InvariantGroup::SchemaCompliance))
                .union(InvariantGroupSet::of(InvariantGroup::RelationIntegrity))
                .union(InvariantGroupSet::of(InvariantGroup::AdjacencyIntegrity))
                .union(InvariantGroupSet::of(InvariantGroup::LineageIntegrity)),
            Self::SnapshotPublication => InvariantGroupSet::of(InvariantGroup::VersionVisibility)
                .union(InvariantGroupSet::of(InvariantGroup::PublicationCoherence)),
            Self::HarnessAudit => InvariantGroupSet::all(),
        }
    }

    pub(crate) const fn supports_observation(self, observation: InvariantObservationKind) -> bool {
        match self {
            Self::CommitBoundary | Self::HarnessAudit => {
                matches!(observation, InvariantObservationKind::Committed)
            }
            Self::MutationSensitive | Self::SnapshotPublication => true,
        }
    }
}
