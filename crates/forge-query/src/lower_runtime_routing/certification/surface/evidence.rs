use std::collections::BTreeMap;

use crate::identity::hash_parts;
use crate::lower_runtime_routing::ForgeQueryLowerRuntimeSeamKey;
use crate::lower_runtime_routing::{
    forge_query_lower_runtime_closeout_registry, forge_query_lower_runtime_crossing_inventory,
    inspect_lower_runtime_boundary, inspect_lower_runtime_closeout,
    summarize_lower_runtime_boundary, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRoutePlan,
};

use super::fixtures::{
    hostile_parity_divergence_digest, normalized_parity_digest,
    representative_basis_subscription_readmission_row,
    representative_basis_truth_view_readmission_row,
    representative_causal_bridge_materialization_row, representative_compose_read_row,
    representative_compose_read_with_invariant_pack_row,
    representative_effect_bridge_writeback_row, representative_effect_relational_merge_row,
    representative_effect_relational_mutation_row,
    representative_execute_read_family_in_basis_context_row,
    representative_execute_read_family_row, representative_frontier_evidence_row,
    representative_historical_bridge_lowering_row, representative_intent_runtime_execution_row,
    representative_live_view_schema_row, representative_live_view_source_row,
    representative_preview_basis_row, representative_projection_bridge_row,
    representative_projection_query_receipts_row, representative_projection_relational_row,
    representative_public_live_view_declaration_row,
    representative_runtime_basis_context_read_graph_row,
    representative_runtime_current_read_graph_row, representative_runtime_intent_authority_row,
    representative_runtime_live_installation_orchestration_row,
    representative_signal_invalidation_row, representative_subscription_activation_row,
    representative_subscription_continuity_row, representative_write_authority_row,
    synthetic_inventory_row, RepresentativeArtifacts,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ForgeQueryLowerRuntimeRepresentativeEvidenceSource {
    RuntimeBackedFixture,
    InventorySynthesized,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeRepresentativeSurface {
    requests: Vec<ForgeQueryLowerRuntimeCapabilityRequest>,
    eligibilities: Vec<ForgeQueryLowerRuntimeCapabilityEligibility>,
    route_plans: Vec<ForgeQueryLowerRuntimeRoutePlan>,
    boundary_receipts: Vec<ForgeQueryLowerRuntimeBoundaryExecutionReceipt>,
    envelopes: Vec<ForgeQueryLowerRuntimeBoundaryEnvelope>,
    dx_digest: String,
    golden_transcript_digest: String,
    query_digest: String,
    route_parity_digest: String,
    evidence_sources: BTreeMap<&'static str, ForgeQueryLowerRuntimeRepresentativeEvidenceSource>,
    requests_by_seam: BTreeMap<&'static str, ForgeQueryLowerRuntimeCapabilityRequest>,
    route_plans_by_seam: BTreeMap<&'static str, ForgeQueryLowerRuntimeRoutePlan>,
    receipts_by_seam: BTreeMap<&'static str, ForgeQueryLowerRuntimeBoundaryExecutionReceipt>,
    envelopes_by_seam: BTreeMap<&'static str, ForgeQueryLowerRuntimeBoundaryEnvelope>,
}

impl ForgeQueryLowerRuntimeRepresentativeSurface {
    pub fn requests(&self) -> &[ForgeQueryLowerRuntimeCapabilityRequest] {
        &self.requests
    }

    pub fn eligibilities(&self) -> &[ForgeQueryLowerRuntimeCapabilityEligibility] {
        &self.eligibilities
    }

    pub fn route_plans(&self) -> &[ForgeQueryLowerRuntimeRoutePlan] {
        &self.route_plans
    }

    pub fn boundary_receipts(&self) -> &[ForgeQueryLowerRuntimeBoundaryExecutionReceipt] {
        &self.boundary_receipts
    }

    pub fn envelopes(&self) -> &[ForgeQueryLowerRuntimeBoundaryEnvelope] {
        &self.envelopes
    }

    pub fn dx_digest(&self) -> &str {
        &self.dx_digest
    }

    pub fn golden_transcript_digest(&self) -> &str {
        &self.golden_transcript_digest
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn route_parity_digest(&self) -> &str {
        &self.route_parity_digest
    }

    pub fn concrete_surface_width(&self) -> usize {
        self.evidence_source_count(
            ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
        )
    }

    pub fn synthetic_surface_width(&self) -> usize {
        self.evidence_source_count(
            ForgeQueryLowerRuntimeRepresentativeEvidenceSource::InventorySynthesized,
        )
    }

    pub fn concrete_surface_digest(&self) -> String {
        self.evidence_source_digest(
            ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
        )
    }

    pub fn synthetic_surface_digest(&self) -> String {
        self.evidence_source_digest(
            ForgeQueryLowerRuntimeRepresentativeEvidenceSource::InventorySynthesized,
        )
    }

    pub(crate) fn synthetic_surface_seams(&self) -> Vec<&'static str> {
        self.evidence_sources
            .iter()
            .filter_map(|(seam_key, source)| {
                (*source
                    == ForgeQueryLowerRuntimeRepresentativeEvidenceSource::InventorySynthesized)
                    .then_some(*seam_key)
            })
            .collect()
    }

    fn evidence_source_digest(
        &self,
        source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource,
    ) -> String {
        hash_parts(
            &self
                .evidence_sources
                .iter()
                .filter_map(|(seam_key, row_source)| {
                    (*row_source == source).then_some((*seam_key).to_string())
                })
                .collect::<Vec<_>>(),
        )
    }

    fn evidence_source_count(
        &self,
        source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource,
    ) -> usize {
        self.evidence_sources
            .values()
            .filter(|row_source| **row_source == source)
            .count()
    }

    pub(crate) fn evidence_source_for(
        &self,
        seam_key: ForgeQueryLowerRuntimeSeamKey,
    ) -> Option<ForgeQueryLowerRuntimeRepresentativeEvidenceSource> {
        self.evidence_sources.get(seam_key.as_str()).copied()
    }

    #[cfg(test)]
    pub(super) fn with_evidence_source_override(
        mut self,
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource,
    ) -> Self {
        self.evidence_sources.insert(seam_key.as_str(), source);
        self
    }

    #[cfg(test)]
    pub(super) fn request_for(
        &self,
        seam_key: ForgeQueryLowerRuntimeSeamKey,
    ) -> Option<&ForgeQueryLowerRuntimeCapabilityRequest> {
        self.requests_by_seam.get(seam_key.as_str())
    }

    #[cfg(test)]
    pub(super) fn route_plan_for(
        &self,
        seam_key: ForgeQueryLowerRuntimeSeamKey,
    ) -> Option<&ForgeQueryLowerRuntimeRoutePlan> {
        self.route_plans_by_seam.get(seam_key.as_str())
    }

    #[cfg(test)]
    pub(super) fn boundary_receipt_for(
        &self,
        seam_key: ForgeQueryLowerRuntimeSeamKey,
    ) -> Option<&ForgeQueryLowerRuntimeBoundaryExecutionReceipt> {
        self.receipts_by_seam.get(seam_key.as_str())
    }

    #[cfg(test)]
    pub(super) fn envelope_for(
        &self,
        seam_key: ForgeQueryLowerRuntimeSeamKey,
    ) -> Option<&ForgeQueryLowerRuntimeBoundaryEnvelope> {
        self.envelopes_by_seam.get(seam_key.as_str())
    }
}

pub fn forge_query_lower_runtime_representative_surface(
) -> ForgeQueryLowerRuntimeRepresentativeSurface {
    let concrete_rows = vec![
        representative_compose_read_row(),
        representative_compose_read_with_invariant_pack_row(),
        representative_execute_read_family_row(),
        representative_execute_read_family_in_basis_context_row(),
        representative_runtime_current_read_graph_row(),
        representative_runtime_basis_context_read_graph_row(),
        representative_live_view_schema_row(),
        representative_live_view_source_row(),
        representative_public_live_view_declaration_row(),
        representative_runtime_live_installation_orchestration_row(),
        representative_subscription_activation_row(),
        representative_subscription_continuity_row(),
        representative_preview_basis_row(),
        representative_basis_truth_view_readmission_row(),
        representative_basis_subscription_readmission_row(),
        representative_historical_bridge_lowering_row(),
        representative_effect_relational_mutation_row(),
        representative_effect_relational_merge_row(),
        representative_effect_bridge_writeback_row(),
        representative_write_authority_row(),
        representative_signal_invalidation_row(),
        representative_runtime_intent_authority_row(),
        representative_intent_runtime_execution_row(),
        representative_projection_query_receipts_row(),
        representative_projection_relational_row(),
        representative_projection_bridge_row(),
        representative_causal_bridge_materialization_row(),
        representative_frontier_evidence_row(),
    ];
    let concrete_seams = concrete_rows
        .iter()
        .map(|row| row.seam_key.as_str())
        .collect::<Vec<_>>();
    let mut rows = concrete_rows;

    for row in forge_query_lower_runtime_crossing_inventory().rows() {
        if concrete_seams.contains(&row.seam_key().as_str()) {
            continue;
        }
        rows.push(synthetic_inventory_row(row));
    }

    let requests = rows
        .iter()
        .map(|row| row.request.clone())
        .collect::<Vec<_>>();
    let eligibilities = rows
        .iter()
        .map(|row| row.eligibility.clone())
        .collect::<Vec<_>>();
    let route_plans = rows
        .iter()
        .filter_map(|row| row.route_plan.clone())
        .collect::<Vec<_>>();
    let boundary_receipts = rows
        .iter()
        .map(|row| row.boundary_receipt.clone())
        .collect::<Vec<_>>();
    let envelopes = rows
        .iter()
        .map(|row| row.envelope.clone())
        .collect::<Vec<_>>();
    let dx_digest = digest_surface_summaries(&envelopes);
    let golden_transcript_digest = digest_surface_transcripts(&envelopes);
    let query_digest = digest_query_subjects(&requests);
    let route_parity_digest = hash_parts(&[
        normalized_parity_digest("compose-read", &envelopes),
        normalized_parity_digest("basis-readmission", &envelopes),
        hostile_parity_divergence_digest(&envelopes),
    ]);

    ForgeQueryLowerRuntimeRepresentativeSurface {
        requests,
        eligibilities,
        route_plans,
        boundary_receipts,
        envelopes,
        dx_digest,
        golden_transcript_digest,
        query_digest,
        route_parity_digest,
        evidence_sources: collect_surface_map(&rows, |row| row.evidence_source),
        requests_by_seam: collect_surface_map(&rows, |row| row.request.clone()),
        route_plans_by_seam: rows
            .iter()
            .filter_map(|row| {
                row.route_plan
                    .clone()
                    .map(|plan| (row.seam_key.as_str(), plan))
            })
            .collect(),
        receipts_by_seam: collect_surface_map(&rows, |row| row.boundary_receipt.clone()),
        envelopes_by_seam: collect_surface_map(&rows, |row| row.envelope.clone()),
    }
}

fn digest_surface_summaries(envelopes: &[ForgeQueryLowerRuntimeBoundaryEnvelope]) -> String {
    hash_parts(
        &envelopes
            .iter()
            .map(|envelope| {
                summarize_lower_runtime_boundary(envelope)
                    .summary_digest()
                    .to_string()
            })
            .collect::<Vec<_>>(),
    )
}

fn digest_surface_transcripts(envelopes: &[ForgeQueryLowerRuntimeBoundaryEnvelope]) -> String {
    let mut transcripts = envelopes
        .iter()
        .map(|envelope| {
            let inspection = inspect_lower_runtime_boundary(envelope);
            format!("{}|{}", inspection.headline(), inspection.detail())
        })
        .collect::<Vec<_>>();
    transcripts.extend(
        forge_query_lower_runtime_closeout_registry()
            .rows()
            .iter()
            .map(|row| {
                let inspection = inspect_lower_runtime_closeout(row);
                format!("{}|{}", inspection.headline(), inspection.detail())
            }),
    );
    hash_parts(&transcripts)
}

fn digest_query_subjects(requests: &[ForgeQueryLowerRuntimeCapabilityRequest]) -> String {
    hash_parts(
        &requests
            .iter()
            .map(|request| request.subject_digest().to_string())
            .collect::<Vec<_>>(),
    )
}

fn collect_surface_map<T: Clone>(
    rows: &[RepresentativeArtifacts],
    project: impl Fn(&RepresentativeArtifacts) -> T,
) -> BTreeMap<&'static str, T> {
    rows.iter()
        .map(|row| (row.seam_key.as_str(), project(row)))
        .collect()
}
