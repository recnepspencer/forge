use serde::{Deserialize, Serialize};

use super::execution::InvariantExecutionPoint;
use super::groups::{InvariantCostClass, InvariantGroup, InvariantGroupSet};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordKindTag {
    Entity,
    Relation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantRule {
    LiveRecordRequiresSidecar(RecordKindTag),
    MaxMergedIntents(usize),
    MaxSnapshotEntities(usize),
    UniqueEntityPayloadField(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct InvariantRuleMetadata {
    pub groups: InvariantGroupSet,
    pub cost: InvariantCostClass,
}

impl InvariantRule {
    pub(crate) fn metadata(&self) -> InvariantRuleMetadata {
        match self {
            Self::LiveRecordRequiresSidecar(_) => InvariantRuleMetadata {
                groups: InvariantGroupSet::of(InvariantGroup::StorageCoherence)
                    .union(InvariantGroupSet::of(InvariantGroup::IdentityCoherence)),
                cost: InvariantCostClass::Touched,
            },
            Self::MaxMergedIntents(_) => InvariantRuleMetadata {
                groups: InvariantGroupSet::of(InvariantGroup::PublicationCoherence),
                cost: InvariantCostClass::Touched,
            },
            Self::MaxSnapshotEntities(_) => InvariantRuleMetadata {
                groups: InvariantGroupSet::of(InvariantGroup::VersionVisibility)
                    .union(InvariantGroupSet::of(InvariantGroup::PublicationCoherence)),
                cost: InvariantCostClass::Global,
            },
            Self::UniqueEntityPayloadField(_) => InvariantRuleMetadata {
                groups: InvariantGroupSet::of(InvariantGroup::SchemaCompliance)
                    .union(InvariantGroupSet::of(InvariantGroup::IdentityCoherence)),
                cost: InvariantCostClass::Touched,
            },
        }
    }

    pub(crate) fn cost_class(&self) -> InvariantCostClass {
        self.metadata().cost
    }

    pub(crate) fn groups(&self) -> InvariantGroupSet {
        self.metadata().groups
    }

    pub(crate) fn supports_execution_point(&self, execution_point: InvariantExecutionPoint) -> bool {
        match self {
            Self::LiveRecordRequiresSidecar(_) => {
                execution_point == InvariantExecutionPoint::MutationSensitive
            }
            Self::MaxMergedIntents(_) => {
                execution_point == InvariantExecutionPoint::CommitBoundary
                    || execution_point == InvariantExecutionPoint::HarnessAudit
            }
            Self::UniqueEntityPayloadField(_) => {
                execution_point == InvariantExecutionPoint::MutationSensitive
                    || execution_point == InvariantExecutionPoint::CommitBoundary
                    || execution_point == InvariantExecutionPoint::HarnessAudit
            }
            Self::MaxSnapshotEntities(_) => {
                execution_point == InvariantExecutionPoint::SnapshotPublication
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn same_registration_kind(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::LiveRecordRequiresSidecar(left),
                Self::LiveRecordRequiresSidecar(right),
            ) => left == right,
            (Self::MaxMergedIntents(_), Self::MaxMergedIntents(_))
            | (Self::MaxSnapshotEntities(_), Self::MaxSnapshotEntities(_))
            | (
                Self::UniqueEntityPayloadField(_),
                Self::UniqueEntityPayloadField(_),
            ) => true,
            _ => false,
        }
    }
}
