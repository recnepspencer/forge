use std::fmt;

use super::{CausalLink, NodeExplanation, UpstreamCause};

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
                writeln!(f, "Execution record: {execution_record_id}")?;
            }
            if let Some(semantic_segment_id) = self.semantic_segment_id {
                writeln!(f, "Semantic segment: {semantic_segment_id}")?;
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
