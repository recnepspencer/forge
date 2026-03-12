use serde::{Deserialize, Serialize};

use super::contracts::InvariantPlanContract;
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

impl InvariantRule {
    pub fn groups(&self) -> InvariantGroupSet {
        match self {
            Self::LiveRecordRequiresSidecar(_) => InvariantGroupSet::of(InvariantGroup::Structural)
                .union(InvariantGroupSet::of(InvariantGroup::Mutation)),
            Self::MaxMergedIntents(_) => InvariantGroupSet::of(InvariantGroup::Mutation),
            Self::MaxSnapshotEntities(_) => InvariantGroupSet::of(InvariantGroup::Snapshot)
                .union(InvariantGroupSet::of(InvariantGroup::Publication)),
            Self::UniqueEntityPayloadField(_) => {
                InvariantGroupSet::of(InvariantGroup::Uniqueness)
                    .union(InvariantGroupSet::of(InvariantGroup::Mutation))
            }
        }
    }

    pub fn cost_class(&self) -> InvariantCostClass {
        match self {
            Self::LiveRecordRequiresSidecar(_) => InvariantCostClass::TargetedScan,
            Self::MaxMergedIntents(_) => InvariantCostClass::Constant,
            Self::MaxSnapshotEntities(_) => InvariantCostClass::FullScan,
            Self::UniqueEntityPayloadField(_) => InvariantCostClass::TargetedScan,
        }
    }

    pub fn supports_execution_point(&self, execution_point: InvariantExecutionPoint) -> bool {
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

    pub fn applies_to_contract(&self, contract: Option<InvariantPlanContract>) -> bool {
        let Some(contract) = contract else {
            return true;
        };
        if contract.is_empty() {
            return true;
        }
        match self {
            Self::LiveRecordRequiresSidecar(RecordKindTag::Entity) => {
                contract.touches_entity_existence || contract.touches_entity_payload
            }
            Self::LiveRecordRequiresSidecar(RecordKindTag::Relation) => {
                contract.touches_relation_existence || contract.touches_relation_payload
            }
            Self::MaxMergedIntents(_) => true,
            Self::MaxSnapshotEntities(_) => contract.touches_snapshot_surface,
            Self::UniqueEntityPayloadField(_) => contract.touches_uniqueness,
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
