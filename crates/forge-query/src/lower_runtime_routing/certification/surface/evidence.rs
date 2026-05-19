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
    representative_causal_bridge_materialization_row, representative_frontier_evidence_row,
    representative_live_view_schema_row, representative_live_view_source_row,
    representative_preview_basis_row, representative_projection_bridge_row,
    representative_projection_query_receipts_row, representative_projection_relational_row,
    representative_signal_invalidation_row, representative_subscription_activation_row,
    representative_write_authority_row, synthetic_inventory_row, RepresentativeArtifacts,
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
    fn request_for(
        &self,
        seam_key: ForgeQueryLowerRuntimeSeamKey,
    ) -> Option<&ForgeQueryLowerRuntimeCapabilityRequest> {
        self.requests_by_seam.get(seam_key.as_str())
    }

    #[cfg(test)]
    fn route_plan_for(
        &self,
        seam_key: ForgeQueryLowerRuntimeSeamKey,
    ) -> Option<&ForgeQueryLowerRuntimeRoutePlan> {
        self.route_plans_by_seam.get(seam_key.as_str())
    }

    #[cfg(test)]
    fn boundary_receipt_for(
        &self,
        seam_key: ForgeQueryLowerRuntimeSeamKey,
    ) -> Option<&ForgeQueryLowerRuntimeBoundaryExecutionReceipt> {
        self.receipts_by_seam.get(seam_key.as_str())
    }

    #[cfg(test)]
    fn envelope_for(
        &self,
        seam_key: ForgeQueryLowerRuntimeSeamKey,
    ) -> Option<&ForgeQueryLowerRuntimeBoundaryEnvelope> {
        self.envelopes_by_seam.get(seam_key.as_str())
    }
}

pub fn forge_query_lower_runtime_representative_surface(
) -> ForgeQueryLowerRuntimeRepresentativeSurface {
    let concrete_rows = vec![
        representative_live_view_schema_row(),
        representative_live_view_source_row(),
        representative_subscription_activation_row(),
        representative_preview_basis_row(),
        representative_write_authority_row(),
        representative_signal_invalidation_row(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn representative_surface_covers_every_crossing_row_once() {
        let surface = forge_query_lower_runtime_representative_surface();
        let crossing_count = forge_query_lower_runtime_crossing_inventory().rows().len();

        assert_eq!(surface.requests().len(), crossing_count);
        assert_eq!(surface.eligibilities().len(), crossing_count);
        assert_eq!(surface.boundary_receipts().len(), crossing_count);
        assert_eq!(surface.envelopes().len(), crossing_count);
        assert!(!surface.route_parity_digest().is_empty());
    }

    #[test]
    fn representative_surface_uses_runtime_backed_fixtures_for_named_phase_six_seams() {
        let surface = forge_query_lower_runtime_representative_surface();

        assert_eq!(
            surface.evidence_source_for(ForgeQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission),
            Some(ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture)
        );
        assert_eq!(
            surface.evidence_source_for(ForgeQueryLowerRuntimeSeamKey::LiveViewSourceDeclaration),
            Some(ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture)
        );
        assert_eq!(
            surface.evidence_source_for(ForgeQueryLowerRuntimeSeamKey::PreviewBasisAdmission),
            Some(ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture)
        );
        assert_eq!(
            surface.evidence_source_for(ForgeQueryLowerRuntimeSeamKey::SubscriptionActivation),
            Some(ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture)
        );
        assert_eq!(
            surface
                .evidence_source_for(ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution),
            Some(ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture)
        );
        assert_eq!(
            surface.evidence_source_for(ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting),
            Some(ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture)
        );
        assert_eq!(
            surface.evidence_source_for(
                ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromQueryReceipts
            ),
            Some(ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture)
        );
        assert_eq!(
            surface.evidence_source_for(
                ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromRelationalArtifacts
            ),
            Some(ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture)
        );
        assert_eq!(
            surface.evidence_source_for(
                ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromBridgeArtifacts
            ),
            Some(ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture)
        );
        assert_eq!(
            surface.evidence_source_for(ForgeQueryLowerRuntimeSeamKey::CausalBridgeMaterialization),
            Some(ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture)
        );
        assert_eq!(
            surface.evidence_source_for(ForgeQueryLowerRuntimeSeamKey::FrontierEvidenceIntake),
            Some(ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture)
        );
    }

    #[test]
    fn representative_surface_runtime_backed_seams_match_real_boundary_artifact_constructors() {
        let surface = forge_query_lower_runtime_representative_surface();
        let live_row = representative_live_view_schema_row();
        let source_row = representative_live_view_source_row();
        let activation_row = representative_subscription_activation_row();
        let preview_row = representative_preview_basis_row();
        let write_row = representative_write_authority_row();
        let signal_row = representative_signal_invalidation_row();
        let query_receipt_row = representative_projection_query_receipts_row();
        let relational_row = representative_projection_relational_row();
        let bridge_row = representative_projection_bridge_row();
        let causal_row = representative_causal_bridge_materialization_row();
        let frontier_row = representative_frontier_evidence_row();

        assert_eq!(
            surface
                .request_for(ForgeQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission)
                .expect("live schema request should exist")
                .request_digest(),
            live_row.request.request_digest()
        );
        assert_eq!(
            surface
                .boundary_receipt_for(ForgeQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission)
                .expect("live schema receipt should exist")
                .boundary_execution_digest(),
            live_row.boundary_receipt.boundary_execution_digest()
        );
        assert_eq!(
            surface
                .route_plan_for(ForgeQueryLowerRuntimeSeamKey::LiveViewSourceDeclaration)
                .expect("live source plan should exist")
                .route_digest(),
            source_row
                .route_plan
                .as_ref()
                .expect("live source route plan")
                .route_digest()
        );
        assert_eq!(
            surface
                .route_plan_for(ForgeQueryLowerRuntimeSeamKey::SubscriptionActivation)
                .expect("subscription route plan should exist")
                .route_digest(),
            activation_row
                .route_plan
                .as_ref()
                .expect("subscription route plan")
                .route_digest()
        );
        assert_eq!(
            surface
                .boundary_receipt_for(ForgeQueryLowerRuntimeSeamKey::PreviewBasisAdmission)
                .expect("preview basis receipt should exist")
                .boundary_execution_digest(),
            preview_row.boundary_receipt.boundary_execution_digest()
        );
        assert_eq!(
            surface
                .route_plan_for(ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution)
                .expect("write route plan should exist")
                .route_digest(),
            write_row
                .route_plan
                .as_ref()
                .expect("write route plan")
                .route_digest()
        );
        assert_eq!(
            surface
                .boundary_receipt_for(ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution)
                .expect("write receipt should exist")
                .boundary_execution_digest(),
            write_row.boundary_receipt.boundary_execution_digest()
        );
        assert_eq!(
            surface
                .route_plan_for(ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting)
                .expect("signal route plan should exist")
                .route_digest(),
            signal_row
                .route_plan
                .as_ref()
                .expect("signal route plan")
                .route_digest()
        );
        assert_eq!(
            surface
                .boundary_receipt_for(
                    ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromQueryReceipts
                )
                .expect("query receipt source boundary should exist")
                .boundary_execution_digest(),
            query_receipt_row
                .boundary_receipt
                .boundary_execution_digest()
        );
        assert_eq!(
            surface
                .boundary_receipt_for(
                    ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromRelationalArtifacts
                )
                .expect("relational source boundary should exist")
                .boundary_execution_digest(),
            relational_row.boundary_receipt.boundary_execution_digest()
        );
        assert_eq!(
            surface
                .boundary_receipt_for(
                    ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromBridgeArtifacts
                )
                .expect("bridge source boundary should exist")
                .boundary_execution_digest(),
            bridge_row.boundary_receipt.boundary_execution_digest()
        );
        assert_eq!(
            surface
                .route_plan_for(ForgeQueryLowerRuntimeSeamKey::CausalBridgeMaterialization)
                .expect("causal route plan should exist")
                .route_digest(),
            causal_row
                .route_plan
                .as_ref()
                .expect("causal route plan")
                .route_digest()
        );
        assert_eq!(
            surface
                .route_plan_for(ForgeQueryLowerRuntimeSeamKey::FrontierEvidenceIntake)
                .expect("frontier route plan should exist")
                .route_digest(),
            frontier_row
                .route_plan
                .as_ref()
                .expect("frontier route plan")
                .route_digest()
        );
        assert_eq!(
            surface
                .envelope_for(ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting)
                .expect("signal envelope should exist")
                .envelope_digest(),
            signal_row.envelope.envelope_digest()
        );
    }

    #[test]
    fn representative_surface_reports_concrete_and_synthetic_coverage_widths() {
        let surface = forge_query_lower_runtime_representative_surface();

        assert_eq!(
            surface.concrete_surface_width() + surface.synthetic_surface_width(),
            surface.envelopes().len()
        );
        assert!(surface.concrete_surface_width() >= 11);
        assert!(!surface.concrete_surface_digest().is_empty());
        assert!(!surface.synthetic_surface_digest().is_empty());
    }
}
