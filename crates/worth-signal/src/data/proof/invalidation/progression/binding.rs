use crate::data::graph::storage::invalidation_causes::PendingCauseSetId;
use crate::data::handle::NodeId;

use super::super::binding::{DependencyRevision, OutputCommitOrdinal};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum InvalidationOriginBinding {
    SourceAdmission {
        generation: u64,
    },
    DependencyCommit {
        cause_set: PendingCauseSetId,
        producer_commit_ordinals: Vec<OutputCommitOrdinal>,
    },
    StructuralMutation {
        ordinal: u64,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct InvalidationReadinessEpoch(pub(crate) u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct InvalidationStageOrder {
    pub(crate) stage: u32,
    pub(crate) order: u32,
}

worth_proof::binding_axes! {
    pub(crate) struct InvalidationOriginBindingAxes {
        pub(crate) graph_instance: u64 => GraphInstance,
        pub(crate) target: NodeId => Target,
        pub(crate) dependency_revision: DependencyRevision => DependencyRevision,
        pub(crate) origin: InvalidationOriginBinding => Origin,
    }
    drift pub(crate) enum InvalidationOriginBindingDrift;
}

worth_proof::binding_axes! {
    pub(crate) struct InvalidationWorkBindingAxes {
        pub(crate) graph_instance: u64 => GraphInstance,
        pub(crate) target: NodeId => Target,
        pub(crate) dependency_revision: DependencyRevision => DependencyRevision,
        pub(crate) origin: InvalidationOriginBinding => Origin,
        pub(crate) readiness_epoch: InvalidationReadinessEpoch => ReadinessEpoch,
        pub(crate) stage_order: InvalidationStageOrder => StageOrder,
    }
    drift pub(crate) enum InvalidationWorkBindingDrift;
}

impl InvalidationOriginBindingAxes {
    pub(super) fn into_work_binding(
        self,
        readiness_epoch: InvalidationReadinessEpoch,
        stage_order: InvalidationStageOrder,
    ) -> InvalidationWorkBindingAxes {
        InvalidationWorkBindingAxes {
            graph_instance: self.graph_instance,
            target: self.target,
            dependency_revision: self.dependency_revision,
            origin: self.origin,
            readiness_epoch,
            stage_order,
        }
    }
}

pub(super) fn dependency_origin_binding(
    cause_set: PendingCauseSetId,
    causes: &[super::super::binding::ResolvedDependencyCause],
) -> InvalidationOriginBinding {
    let mut producer_commit_ordinals = causes
        .iter()
        .map(|cause| cause.binding_axes.output_commit_ordinal)
        .collect::<Vec<_>>();
    producer_commit_ordinals.sort_unstable();
    producer_commit_ordinals.dedup();
    InvalidationOriginBinding::DependencyCommit {
        cause_set,
        producer_commit_ordinals,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(index: u32) -> NodeId {
        NodeId::new(index, 0)
    }

    worth_proof::binding_axis_drift_certification! {
        binding: InvalidationWorkBindingAxes,
        drift: InvalidationWorkBindingDrift,
        base: InvalidationWorkBindingAxes {
            graph_instance: 1,
            target: node(2),
            dependency_revision: DependencyRevision(3),
            origin: InvalidationOriginBinding::SourceAdmission { generation: 4 },
            readiness_epoch: InvalidationReadinessEpoch(5),
            stage_order: InvalidationStageOrder { stage: 6, order: 7 },
        },
        twins: {
            graph_instance => GraphInstance = 2,
            target => Target = node(3),
            dependency_revision => DependencyRevision = DependencyRevision(4),
            origin => Origin = InvalidationOriginBinding::StructuralMutation { ordinal: 9 },
            readiness_epoch => ReadinessEpoch = InvalidationReadinessEpoch(6),
            stage_order => StageOrder = InvalidationStageOrder { stage: 6, order: 8 },
        }
    }
}
