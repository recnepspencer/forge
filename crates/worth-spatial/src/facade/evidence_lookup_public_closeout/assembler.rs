use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_ledger::SpatialEvidenceSurfaceCloseoutPosture;

use super::closeout::EvidenceLookupPublicCloseout;
use super::counters::EvidenceLookupPublicCloseoutCounters;
use super::error::EvidenceLookupPublicCloseoutError;
use super::family_stage_row::EvidenceLookupPublicCloseoutDisposition;
use super::input::EvidenceLookupPublicCloseoutRouteInput;
use super::milestone_twelve_seed_lowering::lower_milestone_twelve_seed;

impl EvidenceLookupPublicCloseout {
    pub(crate) fn assemble_from_route_input(
        route_input: &EvidenceLookupPublicCloseoutRouteInput,
    ) -> Result<Self, EvidenceLookupPublicCloseoutError> {
        let input = route_input.admitted_assembly_input().assembly_input();
        let counters = EvidenceLookupPublicCloseoutCounters::new(
            input.family_stage_rows().len(),
            input
                .family_stage_rows()
                .iter()
                .filter(|row| {
                    matches!(
                        row.disposition(),
                        EvidenceLookupPublicCloseoutDisposition::ReceiptProof { .. }
                    )
                })
                .count(),
            input
                .family_stage_rows()
                .iter()
                .filter(|row| {
                    matches!(
                        row.disposition(),
                        EvidenceLookupPublicCloseoutDisposition::NonOrdinaryResidue { .. }
                    )
                })
                .count(),
            input.query_surface_matrix().rows().len(),
            input.query_consumer_kit().binding_rows().len(),
            input.query_consumer_kit().support_rows().len(),
            input.spatial_deletion_ledger_rows().len(),
            input
                .spatial_deletion_ledger_rows()
                .iter()
                .filter(|row| {
                    matches!(
                        row.closeout_posture(),
                        SpatialEvidenceSurfaceCloseoutPosture::CertificationOnly
                            | SpatialEvidenceSurfaceCloseoutPosture::CappedResidue
                    )
                })
                .count(),
            input.query_consumer_kit().query_residue_rows().len(),
            input
                .source_firewall_report()
                .counters()
                .forbidden_row_count(),
            input
                .source_firewall_report()
                .counters()
                .allowed_exception_row_count(),
        );
        let family_coverage_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &input
                .family_stage_rows()
                .iter()
                .map(|row| format!("family-row:{}", row.row_digest()))
                .collect::<Vec<_>>(),
        );
        let spatial_deletion_ledger_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &input
                .spatial_deletion_ledger_rows()
                .iter()
                .map(|row| {
                    format!(
                        "deletion:{}:{}:{:?}",
                        row.surface_name(),
                        row.source_path(),
                        row.closeout_posture()
                    )
                })
                .collect::<Vec<_>>(),
        );
        let residue_audit_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &input
                .family_stage_rows()
                .iter()
                .filter_map(|row| match row.disposition() {
                    EvidenceLookupPublicCloseoutDisposition::NonOrdinaryResidue { .. } => {
                        Some(format!("family-residue:{}", row.row_digest()))
                    }
                    EvidenceLookupPublicCloseoutDisposition::ReceiptProof { .. } => None,
                })
                .chain(
                    input
                        .query_consumer_kit()
                        .query_residue_rows()
                        .iter()
                        .map(|row| format!("query-residue:{}", row.row_digest())),
                )
                .chain(
                    input
                        .spatial_deletion_ledger_rows()
                        .iter()
                        .filter_map(|row| {
                            matches!(
                                row.closeout_posture(),
                                SpatialEvidenceSurfaceCloseoutPosture::CertificationOnly
                                    | SpatialEvidenceSurfaceCloseoutPosture::CappedResidue
                            )
                            .then(|| {
                                format!(
                                    "spatial-residue:{}:{}",
                                    row.surface_name(),
                                    row.source_path()
                                )
                            })
                        }),
                )
                .collect::<Vec<_>>(),
        );
        let closeout_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:evidence-lookup-public-closeout:v1".to_string(),
                format!(
                    "spatial-compiled-product-family:{}",
                    input.spatial_compiled_product_family_digest()
                ),
                format!("family-coverage:{family_coverage_digest}"),
                format!(
                    "query-matrix:{}",
                    input.query_surface_matrix().matrix_digest()
                ),
                format!(
                    "consumer-kit:{}",
                    input.query_consumer_kit().closeout_digest()
                ),
                format!(
                    "query-boundary-support:{}",
                    input.query_boundary_support_digest()
                ),
                format!(
                    "source-firewall:{}",
                    input.source_firewall_report().firewall_digest()
                ),
                format!("deletion-ledger:{spatial_deletion_ledger_digest}"),
                format!("residue-audit:{residue_audit_digest}"),
                format!("family-stage-rows:{}", counters.family_stage_row_count()),
                format!("receipt-proof-rows:{}", counters.receipt_proof_row_count()),
                format!(
                    "non-ordinary-residue-rows:{}",
                    counters.non_ordinary_residue_row_count()
                ),
            ],
        );
        let milestone_twelve_seed = lower_milestone_twelve_seed(
            &closeout_digest,
            route_input.selected_route_family_identity(),
            route_input.selected_compiled_product_identity_digest(),
            route_input.selected_equivalence_family_identity(),
            route_input.selected_reuse_basis_identity_digest(),
            input.query_surface_matrix().matrix_digest(),
            input.query_consumer_kit().closeout_digest(),
            input.source_firewall_report().firewall_digest(),
            &residue_audit_digest,
            &family_coverage_digest,
            input.family_stage_rows(),
            &counters,
        );

        Ok(Self {
            spatial_compiled_product_family_digest: input
                .spatial_compiled_product_family_digest()
                .to_string(),
            family_stage_rows: input.family_stage_rows().to_vec(),
            query_surface_matrix: input.query_surface_matrix().clone(),
            query_consumer_kit: input.query_consumer_kit().clone(),
            query_boundary_support_digest: input.query_boundary_support_digest().to_string(),
            source_firewall_report: input.source_firewall_report().clone(),
            spatial_deletion_ledger_rows: input.spatial_deletion_ledger_rows().to_vec(),
            counters,
            family_coverage_digest,
            spatial_deletion_ledger_digest,
            residue_audit_digest,
            milestone_twelve_seed,
            closeout_digest,
        })
    }
}
