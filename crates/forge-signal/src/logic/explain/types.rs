use std::fmt;

use crate::data::aspect::{Aspect, AspectMask};
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::handle::NodeId;
use crate::data::node::{EvaluationCondition, NodeState};
use crate::data::output::{
    ChangedRegion, MemoizedResultOrigin, OutputChange, OutputIdentity, PartitionSubscription,
};
use crate::data::trace::{CausalityMetadata, TraceSummary};

#[derive(Debug, Clone, PartialEq)]
pub enum MeaningfulChangeReason {
    ExactDifference,
    Tolerance { epsilon: u64 },
    OutputIdentity,
    CustomComparator { key: String },
    InheritedComparator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionDecision {
    Deferred,
    RevertedClean,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpstreamCause {
    Changed {
        source: NodeId,
        aspect: Aspect,
        subscription: Option<PartitionSubscription>,
        cached_version: u64,
        current_version: u64,
        comparator: VersionComparatorPolicy,
        reason: MeaningfulChangeReason,
    },
    SkippedByComparator {
        source: NodeId,
        aspect: Aspect,
        subscription: Option<PartitionSubscription>,
        cached_version: u64,
        current_version: u64,
        comparator: VersionComparatorPolicy,
        reason: MeaningfulChangeReason,
    },
    ConditionDeferred {
        source: NodeId,
        aspect: Aspect,
        subscription: Option<PartitionSubscription>,
        cached_version: u64,
        current_version: u64,
        condition: EvaluationCondition,
        decision: ConditionDecision,
    },
    Clean {
        source: NodeId,
        aspect: Aspect,
        subscription: Option<PartitionSubscription>,
        cached_version: u64,
        current_version: u64,
    },
    MissingSnapshot {
        source: NodeId,
        aspect: Aspect,
        subscription: Option<PartitionSubscription>,
        current_version: Option<u64>,
    },
    DependencyRemoved {
        source: NodeId,
        aspect: Aspect,
        subscription: Option<PartitionSubscription>,
        cached_version: u64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeExplanation {
    pub node: NodeId,
    pub state: NodeState,
    pub dirty_aspects: AspectMask,
    pub condition: EvaluationCondition,
    pub trace_summary: Option<TraceSummary>,
    pub execution_record_id: Option<u64>,
    pub semantic_segment_id: Option<u64>,
    pub output_identity: Option<OutputIdentity>,
    pub output_change: Option<OutputChange>,
    pub changed_regions: Vec<ChangedRegion>,
    pub propagation_suppressed: bool,
    pub memoized_origin: Option<MemoizedResultOrigin>,
    pub upstream: Vec<UpstreamCause>,
    pub causality: Option<CausalityMetadata>,
}

impl fmt::Display for NodeExplanation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Node {} state={:?} condition={:?}",
            self.node, self.state, self.condition
        )?;
        if !self.dirty_aspects.is_empty() {
            writeln!(f, "Dirty aspects: {:?}", self.dirty_aspects)?;
        }
        if let Some(trace) = &self.trace_summary {
            writeln!(
                f,
                "Trace: recomputed={} dependency_count={} meaningful_input_changes={} output_hash={}",
                trace.recomputed,
                trace.dependency_count,
                trace.meaningful_input_changes,
                trace.output_hash
            )?;
            writeln!(
                f,
                "Output: identity={:?} change={:?} propagation_suppressed={} memoized_origin={:?}",
                trace.output_identity,
                trace.output_change,
                trace.propagation_suppressed,
                trace.memoized_origin
            )?;
            if let Some(execution_record_id) = self.execution_record_id {
                writeln!(f, "Execution record: {}", execution_record_id)?;
            }
            if let Some(semantic_segment_id) = self.semantic_segment_id {
                writeln!(f, "Semantic segment: {}", semantic_segment_id)?;
            }
        }
        if let Some(causality) = &self.causality {
            writeln!(f, "Causality: {}", causality.kind)?;
        }
        if !self.changed_regions.is_empty() {
            writeln!(f, "Changed regions: {}", self.changed_regions.len())?;
        }
        for cause in &self.upstream {
            writeln!(f, "{}", format_upstream_cause(cause))?;
        }
        Ok(())
    }
}

fn format_upstream_cause(cause: &UpstreamCause) -> String {
    match cause {
        UpstreamCause::Changed {
            source,
            aspect,
            subscription,
            cached_version,
            current_version,
            ..
        } => format!(
            "  changed <- {} aspect {} scope {:?} ({} -> {})",
            source,
            aspect.index(),
            subscription,
            cached_version,
            current_version
        ),
        UpstreamCause::SkippedByComparator {
            source,
            aspect,
            subscription,
            cached_version,
            current_version,
            ..
        } => format!(
            "  skipped by comparator <- {} aspect {} scope {:?} ({} -> {})",
            source,
            aspect.index(),
            subscription,
            cached_version,
            current_version
        ),
        UpstreamCause::ConditionDeferred {
            source,
            aspect,
            subscription,
            cached_version,
            current_version,
            condition,
            decision,
        } => format!(
            "  condition {:?}/{:?} <- {} aspect {} scope {:?} ({} -> {})",
            condition,
            decision,
            source,
            aspect.index(),
            subscription,
            cached_version,
            current_version
        ),
        UpstreamCause::Clean {
            source,
            aspect,
            subscription,
            cached_version,
            current_version,
        } => format!(
            "  clean <- {} aspect {} scope {:?} ({} == {})",
            source,
            aspect.index(),
            subscription,
            cached_version,
            current_version
        ),
        UpstreamCause::MissingSnapshot {
            source,
            aspect,
            subscription,
            current_version,
        } => format!(
            "  missing snapshot <- {} aspect {} scope {:?} current={:?}",
            source,
            aspect.index(),
            subscription,
            current_version
        ),
        UpstreamCause::DependencyRemoved {
            source,
            aspect,
            subscription,
            cached_version,
        } => format!(
            "  dependency removed <- {} aspect {} scope {:?} cached={}",
            source,
            aspect.index(),
            subscription,
            cached_version
        ),
    }
}

pub(super) fn reason_for_policy(
    policy: &VersionComparatorPolicy,
    explicit: bool,
) -> MeaningfulChangeReason {
    match policy {
        VersionComparatorPolicy::Exact => {
            if explicit {
                MeaningfulChangeReason::ExactDifference
            } else {
                MeaningfulChangeReason::InheritedComparator
            }
        }
        VersionComparatorPolicy::Tolerance { epsilon } => {
            MeaningfulChangeReason::Tolerance { epsilon: *epsilon }
        }
        VersionComparatorPolicy::OutputIdentity => MeaningfulChangeReason::OutputIdentity,
        VersionComparatorPolicy::Custom { key } => {
            MeaningfulChangeReason::CustomComparator { key: key.clone() }
        }
    }
}
