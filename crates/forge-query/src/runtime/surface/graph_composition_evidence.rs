use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryGraphCompositionAssumptionSummary, ForgeQueryGraphCompositionBreadth,
    ForgeQueryGraphCompositionLifecycleOutcomes, ForgeQueryGraphCompositionLineageSummary,
    ForgeQueryGraphCompositionResolutionMap, ForgeQueryWriteReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionEvidence {
    graph_composition_digest: String,
    graph_symbolic_resolution_digest: String,
    graph_assumption_digest: Option<String>,
    graph_lineage_digest: Option<String>,
    counter_snapshot: String,
    lifecycle_counter_snapshot: String,
    symbolic_resolution_count: usize,
    affected_live_view_count: usize,
    affected_derived_view_count: usize,
    considered_computed_view_count: usize,
    assumption_summary: Option<ForgeQueryGraphCompositionAssumptionSummary>,
    lineage_summary: Option<ForgeQueryGraphCompositionLineageSummary>,
}

impl ForgeQueryGraphCompositionEvidence {
    pub(crate) fn derive(
        write_receipts: &[ForgeQueryWriteReceipt],
        breadth: &ForgeQueryGraphCompositionBreadth,
        lifecycle_outcomes: &ForgeQueryGraphCompositionLifecycleOutcomes,
        resolution_map: &ForgeQueryGraphCompositionResolutionMap,
        affected_live_view_count: usize,
        affected_derived_view_count: usize,
        considered_computed_view_count: usize,
    ) -> Option<Self> {
        if breadth.component_count() == 0 {
            return None;
        }
        let assumption_summary =
            ForgeQueryGraphCompositionAssumptionSummary::derive(write_receipts);
        let lineage_summary = ForgeQueryGraphCompositionLineageSummary::derive(write_receipts);
        let symbolic_resolution_count = resolution_map.len();
        let graph_symbolic_resolution_digest = hash_parts(
            &std::iter::once("forge_query_graph_symbolic_resolution_digest_v1".to_string())
                .chain(resolution_map.entries().iter().map(|entry| {
                    format!(
                        "{}:{}:{}:{}:{}",
                        entry.component_index(),
                        entry.aspect_path().unwrap_or("target"),
                        entry.symbol(),
                        entry.resolved_entity_identity(),
                        entry.target_collection().unwrap_or("none")
                    )
                }))
                .collect::<Vec<_>>(),
        );
        let counter_snapshot = format!(
            "components={};symbolic_entities={};symbolic_relations={};symbolic_resolutions={};affected_live_views={};affected_derived_views={};considered_computed_views={};{}",
            breadth.component_count(),
            breadth.symbolic_entity_declaration_count(),
            breadth.symbolic_relation_declaration_count(),
            symbolic_resolution_count,
            affected_live_view_count,
            affected_derived_view_count,
            considered_computed_view_count,
            lifecycle_outcomes.counter_snapshot()
        );
        let lifecycle_counter_snapshot = lifecycle_outcomes.counter_snapshot().to_string();
        let graph_composition_digest = hash_parts(&[
            "forge_query_graph_composition_digest_v1".to_string(),
            format!("breadth:{}", breadth.breadth_digest()),
            format!("lifecycle:{}", lifecycle_outcomes.lifecycle_digest()),
            format!("symbolic-resolution:{graph_symbolic_resolution_digest}"),
            format!(
                "assumptions:{}",
                assumption_summary
                    .as_ref()
                    .map_or("none", |summary| summary.assumption_summary_digest())
            ),
            format!(
                "lineage:{}",
                lineage_summary
                    .as_ref()
                    .map_or("none", |summary| summary.lineage_summary_digest())
            ),
            format!("counters:{counter_snapshot}"),
        ]);
        Some(Self {
            graph_composition_digest,
            graph_symbolic_resolution_digest,
            graph_assumption_digest: assumption_summary
                .as_ref()
                .map(|summary| summary.assumption_summary_digest().to_string()),
            graph_lineage_digest: lineage_summary
                .as_ref()
                .map(|summary| summary.lineage_summary_digest().to_string()),
            counter_snapshot,
            lifecycle_counter_snapshot,
            symbolic_resolution_count,
            affected_live_view_count,
            affected_derived_view_count,
            considered_computed_view_count,
            assumption_summary,
            lineage_summary,
        })
    }

    pub fn graph_composition_digest(&self) -> &str {
        &self.graph_composition_digest
    }

    pub fn graph_symbolic_resolution_digest(&self) -> &str {
        &self.graph_symbolic_resolution_digest
    }

    pub fn graph_assumption_digest(&self) -> Option<&str> {
        self.graph_assumption_digest.as_deref()
    }

    pub fn graph_lineage_digest(&self) -> Option<&str> {
        self.graph_lineage_digest.as_deref()
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }

    pub fn symbolic_resolution_count(&self) -> usize {
        self.symbolic_resolution_count
    }

    pub fn lifecycle_counter_snapshot(&self) -> &str {
        &self.lifecycle_counter_snapshot
    }

    pub fn affected_live_view_count(&self) -> usize {
        self.affected_live_view_count
    }

    pub fn affected_derived_view_count(&self) -> usize {
        self.affected_derived_view_count
    }

    pub fn considered_computed_view_count(&self) -> usize {
        self.considered_computed_view_count
    }

    pub fn assumption_summary(&self) -> Option<&ForgeQueryGraphCompositionAssumptionSummary> {
        self.assumption_summary.as_ref()
    }

    pub fn lineage_summary(&self) -> Option<&ForgeQueryGraphCompositionLineageSummary> {
        self.lineage_summary.as_ref()
    }
}
