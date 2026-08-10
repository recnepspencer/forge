use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::reuse::{ReuseBasis, ReuseCrossing, ReuseSource};

use super::super::reporting::classify_task_record;
use super::super::types::{
    EligibleTask, ExecutionRecordId, SemanticSegmentId, TaskExecutionOutcome, TaskReason,
};

#[test]
fn task_record_classification_uses_reuse_basis_as_authoritative_truth() {
    let task = EligibleTask {
        node: NodeId::new(7, 0),
        request_mode: crate::logic::evaluation::EvaluationRequestMode::Default,
        direct_request: false,
        reason: TaskReason::MemoValidation,
        admission: crate::logic::planner::EligibleTaskAdmission {
            node_state_at_admission: Some(NodeState::Dirty),
            dirty_partition_scopes_present: false,
            maybe_stale: None,
        },
    };

    let record = classify_task_record(
        ExecutionRecordId(1),
        SemanticSegmentId(1),
        &task,
        NodeState::Dirty,
        NodeState::Clean,
        None,
        None,
        crate::logic::evaluation::EvaluationVerdict::Suppressed {
            reason: crate::logic::evaluation::SuppressionReason::ComparatorMatch,
        },
        None,
        crate::data::output::MemoizedResultOrigin::DirectCompute,
        ReuseBasis::strategy(
            crate::data::reuse::ReuseStrategy::MemoizedArtifactReuse,
            ReuseSource::MemoizedArtifact,
            ReuseCrossing::None,
        ),
    );

    assert_eq!(record.record.outcome, TaskExecutionOutcome::MemoizedReuse);
}
