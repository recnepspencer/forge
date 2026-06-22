use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryGraphCompositionAssumptionSummary, ForgeQueryGraphCompositionBreadth,
    ForgeQueryGraphCompositionLifecycleOutcomes, ForgeQueryGraphCompositionLineageSummary,
    ForgeQueryGraphCompositionResolutionMap, ForgeQueryWriteReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionEvidence {
    graph_composition_digest: ForgeQueryEvidenceIdentity,
    graph_symbolic_resolution_digest: ForgeQueryEvidenceIdentity,
    graph_assumption_digest: Option<ForgeQueryEvidenceIdentity>,
    graph_lineage_digest: Option<ForgeQueryEvidenceIdentity>,
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
        let graph_symbolic_resolution_entries = resolution_map
            .entries()
            .iter()
            .map(|entry| {
                let aspect_digest = entry
                    .aspect_touch()
                    .map(crate::runtime::ForgeQueryAspectTouch::admitted_touch_digest_part);
                let mut entry_identity = forge_query_evidence_identity(
                    ForgeQueryEvidenceScope::BatchWriteReceiptGraphResolution,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("component_index"),
                    entry.component_index(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("aspect_path"),
                    aspect_digest.as_deref(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("symbol"),
                    entry.symbol().evidence_identity(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("resolved_entity_identity"),
                    &entry.resolved_entity_identity().evidence_identity(),
                );
                if let Some(collection) = entry.target_collection() {
                    entry_identity = entry_identity.field_evidence_identity(
                        ForgeQueryEvidenceTag::new("target_collection"),
                        collection.evidence_identity(),
                    );
                }
                entry_identity.seal()
            })
            .collect::<Vec<_>>();
        let graph_symbolic_resolution_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "graph-symbolic-resolution",
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("entry"),
                    graph_symbolic_resolution_entries.iter(),
                )
                .seal();
        let counter_snapshot = diagnostic_counter_snapshot_with_tail(
            &[
                ("components", breadth.component_count()),
                (
                    "symbolic_entities",
                    breadth.symbolic_entity_declaration_count(),
                ),
                (
                    "symbolic_relations",
                    breadth.symbolic_relation_declaration_count(),
                ),
                ("symbolic_resolutions", symbolic_resolution_count),
                ("affected_live_views", affected_live_view_count),
                ("affected_derived_views", affected_derived_view_count),
                ("considered_computed_views", considered_computed_view_count),
            ],
            lifecycle_outcomes.counter_snapshot(),
        );
        let lifecycle_counter_snapshot = lifecycle_outcomes.counter_snapshot().to_string();
        let graph_composition_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(ForgeQueryEvidenceTag::new("role"), "graph-composition")
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("breadth"),
                    breadth.breadth_evidence_digest(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("lifecycle"),
                    lifecycle_outcomes.lifecycle_evidence_digest(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("symbolic_resolution"),
                    &graph_symbolic_resolution_digest,
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("assumptions"),
                    assumption_summary
                        .as_ref()
                        .map(|summary| summary.assumption_summary_evidence_digest()),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("lineage"),
                    lineage_summary
                        .as_ref()
                        .map(|summary| summary.lineage_summary_evidence_digest()),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("component_count"),
                    breadth.component_count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("symbolic_entity_count"),
                    breadth.symbolic_entity_declaration_count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("symbolic_relation_count"),
                    breadth.symbolic_relation_declaration_count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("symbolic_resolution_count"),
                    symbolic_resolution_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("affected_live_view_count"),
                    affected_live_view_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("affected_derived_view_count"),
                    affected_derived_view_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("considered_computed_view_count"),
                    considered_computed_view_count,
                )
                .seal();
        Some(Self {
            graph_composition_digest,
            graph_symbolic_resolution_digest,
            graph_assumption_digest: assumption_summary
                .as_ref()
                .map(|summary| summary.assumption_summary_evidence_digest().clone()),
            graph_lineage_digest: lineage_summary
                .as_ref()
                .map(|summary| summary.lineage_summary_evidence_digest().clone()),
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
        self.graph_composition_digest.as_str()
    }

    pub fn graph_composition_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.graph_composition_digest
    }

    pub fn graph_symbolic_resolution_digest(&self) -> &str {
        self.graph_symbolic_resolution_digest.as_str()
    }

    pub fn graph_symbolic_resolution_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.graph_symbolic_resolution_digest
    }

    pub fn graph_assumption_digest(&self) -> Option<&str> {
        self.graph_assumption_digest
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn graph_assumption_evidence_digest(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        self.graph_assumption_digest.as_ref()
    }

    pub fn graph_lineage_digest(&self) -> Option<&str> {
        self.graph_lineage_digest
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn graph_lineage_evidence_digest(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        self.graph_lineage_digest.as_ref()
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

fn diagnostic_counter_snapshot_with_tail(fields: &[(&str, usize)], tail: &str) -> String {
    let mut snapshot = diagnostic_counter_snapshot(fields);
    if !snapshot.is_empty() && !tail.is_empty() {
        snapshot.push(';');
    }
    snapshot.push_str(tail);
    snapshot
}

fn diagnostic_counter_snapshot(fields: &[(&str, usize)]) -> String {
    let mut snapshot = String::new();
    for (index, (label, value)) in fields.iter().enumerate() {
        if index > 0 {
            snapshot.push(';');
        }
        snapshot.push_str(label);
        snapshot.push('=');
        snapshot.push_str(&value.to_string());
    }
    snapshot
}
