use forge_query::facade::runtime::{
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportMatrix,
};
use worth_spatial::facade::query_adoption::{
    current_spatial_query_consumer_kit_adoption_status,
    spatial_query_graph_obligation_adoption_proof, spatial_query_graph_obligation_residue_manifest,
};
use worth_spatial::facade::workload_vocabulary::{
    lower_spatial_touch_authority_to_query_descriptor, SpatialGeometryEvidenceTouchRequest,
    WorkloadEvidenceSupport,
};

use super::spatial_touch_admission::{
    executable_spatial_touch_contract_subject, run_with_large_stack,
};

#[test]
fn public_facade_accessors_match_internal_spatial_touch_query_and_residue_products() {
    run_with_large_stack(|| {
        let (workload, receipt) = executable_spatial_touch_contract_subject();
        let authority = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&receipt)
            .with_complete_ledger(workload.evidence_ledger())
            .admit()
            .expect("public facade request must admit the real receipt and complete ledger");
        let lookup = authority
            .spatial_evidence_lookup(workload.evidence_ledger())
            .expect("public facade lookup must derive from admitted spatial authority");
        let direct_descriptor = authority
            .query_touch_descriptor(&lookup)
            .expect("authority method must lower to Query descriptor");
        let facade_descriptor =
            lower_spatial_touch_authority_to_query_descriptor(&authority, &lookup)
                .expect("facade lowering function must use the same proof product");

        assert_eq!(
            lookup.lookup_key().boolean_stage(),
            authority.boolean_stage()
        );
        assert_eq!(
            lookup.lookup_key().evidence_stage(),
            authority.evidence_stage()
        );
        assert_eq!(
            lookup.lookup_key().evidence_identity(),
            authority.evidence_identity()
        );
        assert_eq!(
            lookup.lookup_key().stage_index_identity(),
            authority.stage_index_identity()
        );
        assert_eq!(lookup.support(), WorkloadEvidenceSupport::Admitted);
        assert_eq!(lookup.counters(), authority.evidence_counters());
        assert_eq!(lookup.lookup_counters(), authority.lookup_counters());
        assert_eq!(
            lookup.product_digest().spatial_touch_digest(),
            authority.digest()
        );
        assert_eq!(
            direct_descriptor.product_digest(),
            facade_descriptor.product_digest()
        );
        assert_eq!(
            direct_descriptor.spatial_touch_digest(),
            facade_descriptor.spatial_touch_digest()
        );
        assert_eq!(
            direct_descriptor.lookup_product_digest(),
            lookup.product_digest()
        );
        assert_eq!(
            direct_descriptor.touch_descriptor().descriptor_digest(),
            facade_descriptor.touch_descriptor().descriptor_digest()
        );
        assert_eq!(
            direct_descriptor.operating_world().descriptor_digest(),
            facade_descriptor.operating_world().descriptor_digest()
        );
        assert!(!direct_descriptor.claims_milestone_five_selection_closeout());

        let status = current_spatial_query_consumer_kit_adoption_status()
            .expect("public Query adoption status must be execution-backed");
        let proof = spatial_query_graph_obligation_adoption_proof()
            .expect("internal Query adoption proof must be execution-backed");
        let residue_manifest =
            spatial_query_graph_obligation_residue_manifest().expect("residue manifest");
        let support_pin = proof.support_pin();
        let support_matrix =
            ForgeQueryGraphObligationSupportMatrix::assembly_selection_foundation();
        let audit = proof.local_ceremony_audit();
        let execution_proof = proof.execution_proof();
        let selection_counters = execution_proof.selection_proof().selection_counters();
        let selected_obligation_count = execution_proof.selected_obligation_count();
        let execution_row_count = execution_proof.rows().len();

        assert_eq!(status.support_pin_report_digest(), support_pin.pin_digest());
        assert_eq!(status.support_requirement_count(), support_pin.row_count());
        assert_eq!(
            status.support_observed_row_count(),
            support_matrix
                .rows_for_lane(ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection)
                .count()
        );
        assert_eq!(
            status.support_matched_required_count(),
            support_pin.row_count()
        );
        assert_eq!(
            status.support_snapshot_row_count(),
            support_matrix.rows().len()
        );
        assert_eq!(
            status.support_blocking_finding_count(),
            support_pin.findings(&support_matrix).len()
        );
        assert_eq!(
            status.evidence_report_identity(),
            proof.manifest().manifest_digest()
        );
        assert_eq!(
            status.evidence_digest_participation_identity(),
            proof.adoption_proof().manifest().manifest_digest()
        );
        assert_eq!(
            status.boundary_audit_report_identity(),
            audit.audit_digest()
        );
        assert_eq!(
            status.boundary_audit_source_count(),
            audit.evaluated_source_count()
        );
        assert_eq!(
            status.boundary_audit_coverage_row_count(),
            audit.findings().len()
        );
        assert_eq!(
            status.workload_support_pin_row_count(),
            support_pin.row_count()
        );
        assert_eq!(status.hard_prohibition_audit_clean(), audit.is_clean());
        assert_eq!(
            status.adoption_manifest_digest(),
            proof.manifest().manifest_digest()
        );
        assert_eq!(
            status.execution_proof_digest(),
            execution_proof.proof_digest()
        );
        assert_eq!(
            status.selected_obligation_count(),
            selected_obligation_count
        );
        assert_eq!(status.execution_row_count(), execution_row_count);
        assert_eq!(
            status.attempted_bucket_lookup_count(),
            selection_counters.attempted_bucket_lookup_count()
        );
        assert_eq!(
            status.candidate_registration_count(),
            selection_counters.candidate_registration_count()
        );
        assert_eq!(
            status.denied_row_count(),
            selected_obligation_count.saturating_sub(execution_row_count)
        );
        assert_eq!(
            status.full_scan_count(),
            selection_counters.registration_full_scan_count()
        );
        assert_eq!(status.residue_row_count(), residue_manifest.rows().len());
        assert_eq!(
            status.residue_row_count(),
            proof.residue_manifest().rows().len()
        );
        assert_eq!(status.residue_row_count(), 2);
    });
}
