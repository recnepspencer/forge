use super::public_api_query_obligation_selection_real_spatial_authority_support::{
    real_spatial_authority_case, real_spatial_selection_case, RealSpatialSelectionCase,
};
use forge_query::facade::consumer_kit::ForgeQueryGraphObligationResidueManifest;
use forge_query::facade::ForgeQueryGraphObligationSupportStatus;
use worth_kernel::query_obligation_selection::selection_substrate::QueryObligationSelectionAuthorityKind;
use worth_spatial::facade::query_adoption::spatial_query_graph_obligation_residue_manifest;
use worth_spatial::facade::workload_vocabulary::{
    deny_copied_receipt_fields_as_spatial_query_lowering_authority,
    deny_query_descriptor_as_spatial_query_lowering_authority,
    deny_raw_row_as_spatial_query_lowering_authority,
    deny_topology_touched_basis_as_spatial_query_lowering_authority,
    lower_spatial_touch_authority_to_query_descriptor, SpatialEvidenceQueryGapKind,
    SpatialEvidenceQueryLoweringDenialKind, SpatialEvidenceSurfaceOwner,
};

#[test]
fn spatial_touch_descriptor_selects_query_obligations_from_real_authority() {
    run_with_large_stack(|| {
        let selected_case = real_spatial_query_selection_case();
        let selected = selected_case.selected();
        let descriptor = selected_case.descriptor();
        let closeout = selected.closeout();
        let counters = closeout.selection_counters();
        let selected_obligations = selected
            .execution_proof()
            .selection_proof()
            .selected_obligations();

        assert_eq!(
            closeout.authority_kind(),
            QueryObligationSelectionAuthorityKind::SpatialQueryDescriptor
        );
        assert_eq!(
            closeout.authority_digest(),
            descriptor.product_digest().as_str()
        );
        assert_eq!(
            closeout.spatial_touch_digest(),
            Some(descriptor.spatial_touch_digest().as_str())
        );
        assert_eq!(
            closeout.spatial_lookup_product_digest(),
            Some(descriptor.lookup_product_digest().as_str())
        );
        assert_eq!(
            closeout.spatial_touch_digest(),
            Some(selected_case.authority().digest().as_str())
        );
        assert_eq!(
            closeout.spatial_lookup_product_digest(),
            Some(selected_case.lookup().product_digest().as_str())
        );
        assert_eq!(
            closeout.touch_descriptor_digest(),
            descriptor.touch_descriptor().descriptor_digest()
        );
        assert_eq!(
            closeout.operating_world_digest(),
            descriptor.operating_world().descriptor_digest()
        );
        assert_eq!(closeout.selected_obligation_count(), 1);
        assert_eq!(closeout.execution_row_count(), 1);
        assert_eq!(selected_obligations.len(), 1);
        assert_eq!(
            selected_obligations[0].support_status(),
            ForgeQueryGraphObligationSupportStatus::Supported
        );
        assert!(selected.execution_proof().has_real_executor_rows());
        assert_eq!(counters.matched_obligation_count(), 1);
        assert_eq!(counters.registration_full_scan_count(), 0);
        assert_eq!(
            counters.attempted_bucket_lookup_count(),
            counters.touch_lookup_key_count() * counters.operating_world_lookup_key_count()
        );
        assert!(counters.matched_bucket_count() > 0);
        assert!(
            counters.candidate_registration_count() >= counters.matched_obligation_count(),
            "candidate_registration_count reports the bucket candidate inventory, not only the selected obligation"
        );
        assert!(
            counters.deduplicated_candidate_count() >= counters.matched_obligation_count(),
            "deduplicated_candidate_count must still cover the selected spatial obligation"
        );
        assert!(!closeout.execution_proof_digest().is_empty());
        assert!(!closeout.adoption_manifest_digest().is_empty());
        assert!(!closeout.residue_manifest_digest().is_empty());
        assert_eq!(descriptor.counters().broad_ledger_scan_count(), 0);
    });
}

#[test]
fn raw_row_lookup_product_and_topology_basis_cannot_select_spatial_obligations() {
    let query_descriptor =
        forge_query::facade::runtime::ForgeQueryGraphTouchDescriptor::read_family(
            "worth.spatial.evidence_touch",
            [forge_query::facade::runtime::ForgeQueryGraphTouchReadVerb::ObservesCollection],
        )
        .expect("query descriptor fixture should build for denial assertion");

    assert_eq!(
        deny_raw_row_as_spatial_query_lowering_authority("raw spatial row").kind(),
        SpatialEvidenceQueryLoweringDenialKind::RawRowSubstitution
    );
    assert_eq!(
        deny_copied_receipt_fields_as_spatial_query_lowering_authority("copied receipt fields")
            .kind(),
        SpatialEvidenceQueryLoweringDenialKind::CopiedReceiptSubstitution
    );
    assert_eq!(
        deny_query_descriptor_as_spatial_query_lowering_authority(&query_descriptor).kind(),
        SpatialEvidenceQueryLoweringDenialKind::QueryDescriptorSubstitution
    );
    assert_eq!(
        deny_topology_touched_basis_as_spatial_query_lowering_authority("topology touched basis")
            .kind(),
        SpatialEvidenceQueryLoweringDenialKind::TopologyTouchedBasisSubstitution
    );

    let first = real_spatial_query_selection_case();
    let second = real_spatial_authority_case("phase4-spatial-selection-mismatch-b");
    let mismatch =
        lower_spatial_touch_authority_to_query_descriptor(second.authority(), first.lookup())
            .expect_err("lookup product from a different spatial authority must not lower");
    assert_eq!(
        mismatch.kind(),
        SpatialEvidenceQueryLoweringDenialKind::LookupProductMismatch
    );
}

#[test]
fn spatial_descriptor_selection_records_query_gaps_without_claiming_milestone_six() {
    let selected_case = real_spatial_query_selection_case();
    let selected = selected_case.selected();
    let closeout = selected.closeout();
    let descriptor = selected_case.descriptor();
    let gap_rows = descriptor.gap_rows();

    assert_eq!(gap_rows.len(), 1);
    assert_eq!(gap_rows.len(), descriptor.counters().query_gap_count());
    assert_eq!(closeout.spatial_query_gap_rows(), gap_rows);
    assert_eq!(descriptor.counters().query_descriptor_count(), 1);
    assert_eq!(descriptor.counters().operating_world_descriptor_count(), 1);
    assert!(!descriptor.claims_milestone_five_selection_closeout());
    assert!(!closeout.graph_read_access_planning_claimed());

    let gap = &gap_rows[0];
    assert_eq!(
        gap.kind(),
        SpatialEvidenceQueryGapKind::DeclaredMutationCollectionNotExpressed
    );
    assert_eq!(gap.owner(), SpatialEvidenceSurfaceOwner::WorthSpatial);
    assert_eq!(
        gap.cap(),
        "declared mutation collection selector is capped because this phase lowers spatial evidence as Query read-family touch only"
    );
    assert_eq!(
        gap.blocker(),
        "Spatial evidence touch authority is read-family evidence, not graph mutation meaning."
    );
    assert_eq!(
        gap.removal_trigger(),
        "Milestone 5 introduces a Query-owned obligation selection lane that needs declared mutation semantics for spatial evidence."
    );
    assert!(!gap.gap_digest().is_empty());

    let residue = spatial_query_graph_obligation_residue_manifest().expect("spatial residue");
    let support_projection = residue
        .rows()
        .iter()
        .find(|row| row.class() == "worth-spatial-runtime-facade-support-projection")
        .expect("support projection residue remains explicit if not deleted");

    assert_eq!(support_projection.owner(), "worth-spatial");
    assert_eq!(support_projection.current_count(), 1);
    assert_eq!(support_projection.must_not_exceed_count(), 1);
    assert_eq!(
        support_projection.blocker(),
        "public facade still exposes current_spatial_workload_support_pin_rows for older query-native closeout consumers"
    );
    assert_eq!(
        support_projection.removal_trigger(),
        "delete support_projection.rs facade export after Milestone 6.5 consumes graph-obligation adoption status directly"
    );
    let certification =
        ForgeQueryGraphObligationResidueManifest::certify_candidate_against_previous(
            &residue,
            selected.residue_manifest(),
        )
        .expect("spatial residue manifest must not grow or drift during selection");
    assert_eq!(certification.certified_row_count(), residue.rows().len());
    assert_eq!(
        certification.previous_manifest_digest(),
        residue.manifest_digest()
    );
    assert_eq!(
        certification.candidate_manifest_digest(),
        selected.residue_manifest().manifest_digest()
    );
    assert!(!certification.certification_digest().is_empty());
}

fn real_spatial_query_selection_case() -> &'static RealSpatialSelectionCase {
    static REAL_SPATIAL_QUERY_SELECTION_CASE: OnceLock<RealSpatialSelectionCase> = OnceLock::new();

    REAL_SPATIAL_QUERY_SELECTION_CASE
        .get_or_init(|| real_spatial_selection_case("phase4-spatial-selection"))
}

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("query-obligation-selection-spatial-descriptor-contract".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("query obligation selection spatial contract thread should spawn")
        .join()
        .expect("query obligation selection spatial contract thread should finish");
}
use std::sync::OnceLock;
