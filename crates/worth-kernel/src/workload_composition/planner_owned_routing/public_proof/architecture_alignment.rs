use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_composition::deletion_closeout::{
    WorthTouchedGraphConflictDeletionCloseout, WorthTouchedGraphConflictDeletionDisposition,
};
use crate::workload_composition::planner_owned_routing::{
    WorthTouchedGraphConflictResidueDisposition, WorthTouchedGraphConflictSelectedRoutePacket,
};
use crate::workload_composition::{
    WorthTouchedGraphConflictResidueBoundaryPosture, WorthTouchedGraphConflictResidueChain,
};

use super::architecture_alignment_report::{
    WorthTouchedGraphConflictArchitectureAlignmentReport,
    WorthTouchedGraphConflictArchitectureAlignmentReportRow,
    WorthTouchedGraphConflictDeletionAlignmentRow,
};
use super::types::WorthTouchedGraphConflictPublicCloseoutError;

pub(crate) fn build_architecture_alignment_report(
    deletion_closeout: &WorthTouchedGraphConflictDeletionCloseout,
    residue_chain: &WorthTouchedGraphConflictResidueChain,
    selected_route_packet: &WorthTouchedGraphConflictSelectedRoutePacket,
) -> Result<
    WorthTouchedGraphConflictArchitectureAlignmentReport,
    WorthTouchedGraphConflictPublicCloseoutError,
> {
    let deleted_authority_rows = deletion_closeout
        .deletion_ledger()
        .rows()
        .iter()
        .filter(|row| {
            row.disposition() == WorthTouchedGraphConflictDeletionDisposition::DeletedAuthority
        })
        .map(WorthTouchedGraphConflictDeletionAlignmentRow::from_deletion_row)
        .collect::<Vec<_>>();
    let displaced_legacy_authority_rows = deletion_closeout
        .deletion_ledger()
        .rows()
        .iter()
        .map(WorthTouchedGraphConflictDeletionAlignmentRow::from_deletion_row)
        .collect::<Vec<_>>();
    let capped_deletion_rows = deletion_closeout
        .deletion_ledger()
        .rows()
        .iter()
        .filter(|row| {
            row.disposition() == WorthTouchedGraphConflictDeletionDisposition::CappedResidue
        })
        .map(WorthTouchedGraphConflictDeletionAlignmentRow::from_deletion_row)
        .collect::<Vec<_>>();
    let certification_only_fence_rows = deletion_closeout
        .deletion_ledger()
        .rows()
        .iter()
        .filter(|row| {
            row.disposition()
                == WorthTouchedGraphConflictDeletionDisposition::CertificationOnlyFence
        })
        .map(WorthTouchedGraphConflictDeletionAlignmentRow::from_deletion_row)
        .collect::<Vec<_>>();
    let capped_residue_rows = residue_chain
        .rows()
        .iter()
        .filter(|row| {
            row.disposition() == WorthTouchedGraphConflictResidueDisposition::ExplicitResidue
                && row.boundary_posture()
                    != WorthTouchedGraphConflictResidueBoundaryPosture::CoveredOrdinaryConsumerDependency
        })
        .map(WorthTouchedGraphConflictArchitectureAlignmentReportRow::from_residue_row)
        .collect::<Vec<_>>();
    let query_gap_support_rows = residue_chain
        .rows()
        .iter()
        .filter(|row| {
            row.disposition() == WorthTouchedGraphConflictResidueDisposition::QueryGap
                || row.boundary_posture()
                    == WorthTouchedGraphConflictResidueBoundaryPosture::QueryGapSupportGap
        })
        .map(WorthTouchedGraphConflictArchitectureAlignmentReportRow::from_residue_row)
        .collect::<Vec<_>>();
    let ordinary_second_ontology_blockers = residue_chain
        .rows()
        .iter()
        .filter(|row| {
            row.boundary_posture()
                == WorthTouchedGraphConflictResidueBoundaryPosture::CoveredOrdinaryConsumerDependency
        })
        .map(WorthTouchedGraphConflictArchitectureAlignmentReportRow::from_residue_row)
        .collect::<Vec<_>>();
    let report_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &deleted_authority_rows
            .iter()
            .map(|row| format!("deleted:{}:{}", row.source_path(), row.surface_name()))
            .chain(capped_deletion_rows.iter().map(|row| {
                format!(
                    "capped-deletion:{}:{}",
                    row.source_path(),
                    row.surface_name()
                )
            }))
            .chain(certification_only_fence_rows.iter().map(|row| {
                format!(
                    "certification-only:{}:{}",
                    row.source_path(),
                    row.surface_name()
                )
            }))
            .chain(
                capped_residue_rows
                    .iter()
                    .map(|row| format!("capped-residue:{}:{}", row.owner(), row.surface_name())),
            )
            .chain(query_gap_support_rows.iter().map(|row| {
                format!(
                    "query-gap-support:{}:{}:{}",
                    row.owner(),
                    row.surface_name(),
                    row.query_gap_kind()
                        .expect("query-gap rows must name exact blocker shape")
                        .as_str()
                )
            }))
            .chain(ordinary_second_ontology_blockers.iter().map(|row| {
                format!(
                    "second-ontology:{}:{}:{}",
                    row.owner(),
                    row.surface_name(),
                    row.mechanically_unreachable_from_ordinary_path()
                )
            }))
            .chain(std::iter::once(format!(
                "topology-compiled-product:{}",
                selected_route_packet.selected_product_identity_digest()
            )))
            .chain(std::iter::once(format!(
                "topology-equivalence-policy:{}",
                selected_route_packet.selected_equivalence_policy_identity_digest()
            )))
            .chain(std::iter::once(format!(
                "spatial-compiled-product:{}",
                selected_route_packet.spatial_selected_product_identity_digest()
            )))
            .chain(std::iter::once(format!(
                "spatial-equivalence-policy:{}",
                selected_route_packet.spatial_equivalence_policy_identity_digest()
            )))
            .chain(std::iter::once(format!(
                "reuse-decision:{}",
                selected_route_packet
                    .topology_reuse_decision_identity_digest()
                    .unwrap_or("not-applicable")
            )))
            .chain(std::iter::once(format!(
                "rebuild-denial:{}",
                selected_route_packet
                    .rebuild_denial_identity_digest()
                    .unwrap_or("not-applicable")
            )))
            .chain(std::iter::once(
                "worth-kernel:touched-graph-conflict-architecture-alignment-report:v1".to_string(),
            ))
            .collect::<Vec<_>>(),
    );
    Ok(
        WorthTouchedGraphConflictArchitectureAlignmentReport::from_parts(
            selected_route_packet
                .selected_product_identity_digest()
                .to_string(),
            selected_route_packet
                .selected_equivalence_policy_identity_digest()
                .to_string(),
            selected_route_packet
                .spatial_selected_product_identity_digest()
                .to_string(),
            selected_route_packet
                .spatial_equivalence_policy_identity_digest()
                .to_string(),
            selected_route_packet
                .topology_reuse_decision_identity_digest()
                .map(str::to_string),
            selected_route_packet
                .rebuild_denial_identity_digest()
                .map(str::to_string),
            displaced_legacy_authority_rows,
            deleted_authority_rows,
            capped_deletion_rows,
            certification_only_fence_rows,
            capped_residue_rows,
            query_gap_support_rows,
            ordinary_second_ontology_blockers,
            report_digest,
        ),
    )
}
