use crate::workload_platform::evidence_lookup_inventory::EvidenceLookupQuerySurface;
use crate::workload_platform::evidence_lookup_query_surface_matrix::{
    current_evidence_lookup_query_surface_matrix, EvidenceLookupQuerySurfaceTouchpoint,
};

use super::super::current_evidence_lookup_query_consumer_kit;

#[test]
fn lookup_query_consumption_uses_consumer_kit_not_local_reports() {
    let closeout = current_evidence_lookup_query_consumer_kit().expect("consumer kit closeout");
    let matrix = current_evidence_lookup_query_surface_matrix().expect("matrix closes");

    assert!(!closeout.query_surface_matrix_digest().is_empty());
    assert!(!closeout.support_snapshot_digest().is_empty());
    assert!(!closeout.support_pin_contract_digest().is_empty());
    assert!(!closeout.support_pin_report_digest().is_empty());
    assert!(!closeout.evidence_report_identity().is_empty());
    assert!(!closeout.evidence_digest_participation_identity().is_empty());
    assert!(!closeout.boundary_audit_coverage_identity().is_empty());
    assert!(!closeout.boundary_audit_report_identity().is_empty());
    assert!(!closeout.consumer_residue_report_identity().is_empty());
    assert!(!closeout
        .consumer_residue_source_inventory_digest()
        .is_empty());
    assert!(!closeout.closeout_digest().is_empty());

    assert_eq!(
        closeout.binding_rows().len(),
        matrix
            .rows()
            .iter()
            .filter(|row| row.query_surface() != EvidenceLookupQuerySurface::NotQuery)
            .count()
    );

    for matrix_row in matrix
        .rows()
        .iter()
        .filter(|row| row.query_surface() != EvidenceLookupQuerySurface::NotQuery)
    {
        let binding_row = closeout
            .require_binding_row(
                matrix_row.family_identity(),
                matrix_row.stage(),
                matrix_row.touchpoint(),
            )
            .expect("every query matrix row must bind into the consumer closeout");
        assert_eq!(binding_row.query_surface(), matrix_row.query_surface());
        assert_eq!(binding_row.matrix_row_digest(), matrix_row.row_digest());
        assert_eq!(
            binding_row.query_surface_proof_digest(),
            matrix_row.proof_digest()
        );
        assert!(binding_row.query_surface_proof_digest().is_some());
        if matrix_row.query_surface() == EvidenceLookupQuerySurface::SupportPinning {
            assert!(binding_row.requires_support_pin_linkage());
            assert_eq!(
                binding_row.support_pin_report_digest(),
                Some(closeout.support_pin_report_digest())
            );
        } else {
            assert_eq!(binding_row.support_pin_report_digest(), None);
        }
    }

    assert_eq!(
        closeout
            .binding_rows_for_touchpoint(EvidenceLookupQuerySurfaceTouchpoint::PublicCloseoutProof)
            .len(),
        matrix
            .rows_for_touchpoint(EvidenceLookupQuerySurfaceTouchpoint::PublicCloseoutProof)
            .into_iter()
            .filter(|row| row.query_surface() != EvidenceLookupQuerySurface::NotQuery)
            .count()
    );
}
