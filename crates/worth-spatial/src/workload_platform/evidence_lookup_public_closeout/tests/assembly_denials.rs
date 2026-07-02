use crate::workload_platform::evidence_lookup_family_catalog::current_evidence_lookup_family_catalog;
use crate::workload_platform::evidence_lookup_public_closeout::{
    current_evidence_lookup_public_closeout_assembly_input,
    EvidenceLookupPublicCloseoutAssemblyInput, EvidenceLookupPublicCloseoutDisposition,
    EvidenceLookupPublicCloseoutErrorKind,
};
use crate::workload_platform::evidence_lookup_query_surface_matrix::EvidenceLookupQuerySurfaceMatrixCloseout;
use crate::workload_platform::evidence_lookup_query_surface_matrix::EvidenceLookupQuerySurfaceTouchpoint;
use crate::workload_platform::planner_owned_routing::public_closeout_route::admit_evidence_lookup_public_closeout_assembly_input;

#[test]
fn assembly_rejects_missing_public_closeout_query_row() {
    let input =
        current_evidence_lookup_public_closeout_assembly_input().expect("current assembly input");
    let removed_row = input
        .query_surface_matrix()
        .rows()
        .iter()
        .find(|row| {
            row.family_identity() == "spatial-touch.boolean.event-ledger-evidence.v1"
                && row.stage()
                    == crate::workload_platform::evidence_ledger::WorkloadEvidenceStage::BooleanEventLedger
                && row.touchpoint() == EvidenceLookupQuerySurfaceTouchpoint::PublicCloseoutProof
        })
        .expect("target query row");
    let filtered_matrix = EvidenceLookupQuerySurfaceMatrixCloseout::from_rows(
        input
            .query_surface_matrix()
            .rows()
            .iter()
            .filter(|row| row.row_digest() != removed_row.row_digest())
            .cloned()
            .collect(),
    )
    .expect("filtered matrix");
    let denied_input = EvidenceLookupPublicCloseoutAssemblyInput::admit(
        input.spatial_compiled_product_family_digest().to_string(),
        input.family_stage_rows().to_vec(),
        filtered_matrix,
        input.query_consumer_kit().clone(),
        input.query_boundary_support_digest().to_string(),
        input.source_firewall_report().clone(),
        input.spatial_deletion_ledger_rows().to_vec(),
    )
    .expect("assembly input");

    let catalog = current_evidence_lookup_family_catalog().expect("family catalog");
    let error =
        admit_evidence_lookup_public_closeout_assembly_input(denied_input, catalog.declarations())
            .expect_err("missing query row must deny");
    assert_eq!(
        error.kind(),
        EvidenceLookupPublicCloseoutErrorKind::MissingPublicCloseoutQueryRow
    );
}

#[test]
fn assembly_rejects_residue_without_topology_blocker() {
    let input =
        current_evidence_lookup_public_closeout_assembly_input().expect("current assembly input");
    let mutated_rows = input
        .family_stage_rows()
        .iter()
        .cloned()
        .map(|row| {
            if row.family_identity() == "spatial-touch.boolean.event-ledger-evidence.v1"
                && matches!(
                    row.disposition(),
                    EvidenceLookupPublicCloseoutDisposition::ReceiptProof { .. }
                )
            {
                row.with_test_disposition(
                    EvidenceLookupPublicCloseoutDisposition::NonOrdinaryResidue {
                        reason: "synthetic residue".to_string(),
                        removal_trigger: "none".to_string(),
                    },
                )
            } else {
                row
            }
        })
        .collect();
    let denied_input = EvidenceLookupPublicCloseoutAssemblyInput::admit(
        input.spatial_compiled_product_family_digest().to_string(),
        mutated_rows,
        input.query_surface_matrix().clone(),
        input.query_consumer_kit().clone(),
        input.query_boundary_support_digest().to_string(),
        input.source_firewall_report().clone(),
        input.spatial_deletion_ledger_rows().to_vec(),
    )
    .expect("assembly input");

    let catalog = current_evidence_lookup_family_catalog().expect("family catalog");
    let error =
        admit_evidence_lookup_public_closeout_assembly_input(denied_input, catalog.declarations())
            .expect_err("residue without topology blocker must deny");
    assert_eq!(
        error.kind(),
        EvidenceLookupPublicCloseoutErrorKind::ImpossibleResidueSuccessMix
    );
}
