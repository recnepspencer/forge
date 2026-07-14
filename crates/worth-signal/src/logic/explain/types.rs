use serde::{Deserialize, Serialize};

use std::fmt;

use crate::data::aspect::{Aspect, AspectMask};
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::handle::NodeId;
use crate::data::node::{ContextRequirement, EvaluationCondition, NodeState};
use crate::data::output::{
    ChangedRegion, MemoizedResultOrigin, OutputChange, OutputIdentity, PartitionSubscription,
};
use crate::data::reuse::{ReuseBasis, ReuseCertificationRecord};
use crate::data::trace::{CausalityMetadata, HistoricalArtifactRecord, TraceSummary};
use crate::diagnostics::policy::DiagnosticsAvailability;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeaningfulChangeReason {
    ExactDifference,
    Tolerance { epsilon: u64 },
    OutputIdentity,
    CustomComparator { key: String },
    InheritedComparator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionDecision {
    Deferred,
    RevertedClean,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CausalDisposition {
    Semantic,
    Suppressed,
    Ignored,
    Conservative,
    Topology,
    Lifecycle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScopeProvenanceKind {
    None,
    Direct,
    Translated,
    Discarded,
    InsufficientEvidence,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CausalLinkKind {
    Changed,
    SkippedByComparator,
    ConditionDeferred {
        condition: EvaluationCondition,
        decision: ConditionDecision,
    },
    ScopeUntouched,
    Clean,
    MissingSnapshot,
    DependencyAdded,
    DependencyRemoved,
}

impl fmt::Display for CausalLinkKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Changed => write!(f, "Changed"),
            Self::SkippedByComparator => write!(f, "SkippedByComparator"),
            Self::ConditionDeferred {
                condition,
                decision,
            } => write!(f, "ConditionDeferred::{condition:?}/{decision:?}"),
            Self::ScopeUntouched => write!(f, "ScopeUntouched"),
            Self::Clean => write!(f, "Clean"),
            Self::MissingSnapshot => write!(f, "MissingSnapshot"),
            Self::DependencyAdded => write!(f, "DependencyAdded"),
            Self::DependencyRemoved => write!(f, "DependencyRemoved"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeProvenance {
    pub source_scope: Option<PartitionSubscription>,
    pub validation_scope: Option<PartitionSubscription>,
    pub kind: ScopeProvenanceKind,
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CausalLink {
    pub source: Option<NodeId>,
    pub aspect: Option<Aspect>,
    pub disposition: CausalDisposition,
    pub kind: CausalLinkKind,
    pub scope: ScopeProvenance,
    pub cached_version: Option<u64>,
    pub current_version: Option<u64>,
    pub comparator: Option<VersionComparatorPolicy>,
    pub reason: Option<MeaningfulChangeReason>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewiringDependency {
    pub source: NodeId,
    pub aspect: Aspect,
    pub subscription: Option<PartitionSubscription>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewiringSummary {
    pub added: Vec<RewiringDependency>,
    pub removed: Vec<RewiringDependency>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeExplanation {
    pub node: NodeId,
    #[serde(default)]
    pub materialization_mode: DiagnosticsAvailability,
    pub state: NodeState,
    pub dirty_aspects: AspectMask,
    pub contract_reads: AspectMask,
    pub contract_produces: AspectMask,
    pub contract_partition_scope: Option<Vec<PartitionSubscription>>,
    pub required_context: ContextRequirement,
    pub condition: EvaluationCondition,
    pub historical_artifact_record: Option<HistoricalArtifactRecord>,
    pub execution_record_id: Option<u64>,
    pub semantic_segment_id: Option<u64>,
    pub output_identity: Option<OutputIdentity>,
    pub output_change: Option<OutputChange>,
    pub changed_regions: Vec<ChangedRegion>,
    pub propagation_suppressed: bool,
    pub memoized_origin: Option<MemoizedResultOrigin>,
    pub reuse_basis: Option<ReuseBasis>,
    pub reuse_origin: Option<crate::data::reuse::ReuseOrigin>,
    pub reuse_certification: Option<ReuseCertificationRecord>,
    pub upstream: Vec<UpstreamCause>,
    #[serde(default)]
    pub causal_links: Vec<CausalLink>,
    #[serde(default)]
    pub rewiring: Option<RewiringSummary>,
    pub causality: Option<CausalityMetadata>,
}

impl NodeExplanation {
    pub fn materialized_trace_summary(&self) -> Option<TraceSummary> {
        self.historical_artifact_record
            .as_ref()
            .map(TraceSummary::from_record)
    }
}

impl fmt::Display for NodeExplanation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Node {} state={:?} condition={:?}",
            self.node, self.state, self.condition
        )?;
        writeln!(
            f,
            "Contract: reads={:?} produces={:?} required_context={:?} partition_scopes={}",
            self.contract_reads,
            self.contract_produces,
            self.required_context,
            self.contract_partition_scope
                .as_ref()
                .map(|scopes| scopes.len())
                .unwrap_or(0)
        )?;
        writeln!(f, "Materialization: {:?}", self.materialization_mode)?;
        if !self.dirty_aspects.is_empty() {
            writeln!(f, "Dirty aspects: {:?}", self.dirty_aspects)?;
        }
        if let Some(trace) = self.materialized_trace_summary() {
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
            writeln!(f, "Reuse basis: {:?}", trace.reuse_basis)?;
            writeln!(f, "Reuse origin: {:?}", trace.reuse_origin)?;
            if let Some(certification) = &self.reuse_certification {
                writeln!(
                    f,
                    "Reuse certification proofs: {}",
                    certification.proofs.len()
                )?;
            }
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
        if let Some(rewiring) = &self.rewiring {
            writeln!(
                f,
                "Rewiring: +{} / -{}",
                rewiring.added.len(),
                rewiring.removed.len()
            )?;
        }
        for link in &self.causal_links {
            writeln!(f, "{}", format_causal_link(link))?;
        }
        for cause in &self.upstream {
            writeln!(f, "{}", format_upstream_cause(cause))?;
        }
        Ok(())
    }
}

fn format_causal_link(link: &CausalLink) -> String {
    format!(
        "  cause {:?}/{:?} <- {:?} aspect {:?} scope {:?} note {:?}",
        link.disposition,
        link.kind,
        link.source,
        link.aspect.map(|aspect| aspect.index()),
        link.scope.validation_scope,
        link.note
    )
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
