use topology::derived_invalidation_milestone_ten_closeout::DerivedInvalidationMilestoneElevenSeed;
use worth_spatial::facade::evidence_lookup_family_catalog::{
    current_evidence_lookup_family_catalog, EvidenceLookupFamilyCatalogCloseout,
    EvidenceLookupFamilyCatalogError, EvidenceLookupFamilyIndexPostureKind,
    EvidenceLookupFamilyQueryPostureKind, EvidenceLookupProjectionConsumptionSurface,
    EvidenceLookupProjectionFactFamily, EvidenceLookupQueryImportEvidence,
    EvidenceLookupStageReceiptFamilyIdentity, EvidenceLookupTopologyInputState,
    EvidenceLookupTopologyRequirementReport,
};
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStage;

#[test]
fn spatial_public_api_exports_read_only_family_catalog_closeout() {
    let closeout = current_evidence_lookup_family_catalog().expect("public family catalog closes");
    let counters = closeout.counters();

    assert_eq!(counters.family_count(), 3);
    assert_eq!(counters.family_count(), closeout.declarations().len());
    assert_eq!(counters.query_required_family_count(), 2);
    assert_eq!(counters.topology_required_family_count(), 1);
    assert_eq!(counters.sparse_index_family_count(), 2);
    assert_eq!(counters.bounded_dense_index_family_count(), 1);
    assert_eq!(counters.declare_once_multi_stage_family_count(), 3);
    assert!(counters.source_inventory_migrate_row_count() > 0);
    assert!(!closeout.claims_lookup_execution_authority());
    assert!(!closeout.claims_family_selection());
    assert!(!closeout.claims_index_construction());
    assert!(!closeout.claims_query_support_authority());
    assert!(!closeout.catalog_digest().is_empty());
}

#[test]
fn spatial_public_api_exposes_overlap_family_posture_without_construction_hooks() {
    let closeout = current_evidence_lookup_family_catalog().expect("public family catalog closes");
    let family = closeout
        .family_by_identity("spatial-touch.boolean.overlap-evidence.v1")
        .expect("overlap evidence family is declared");

    assert_eq!(
        family.topology_input_posture().state(),
        EvidenceLookupTopologyInputState::DerivedProductReceiptRequired
    );
    assert_eq!(
        family.query_posture().kind(),
        EvidenceLookupFamilyQueryPostureKind::ImportedSupportPinRequired
    );
    assert_eq!(
        family.index_posture().kind(),
        EvidenceLookupFamilyIndexPostureKind::SparseLookupPlanRequired
    );
    assert!(family.query_posture().imported_evidence_digest().is_some());
    assert!(matches!(
        family.query_posture().imported_evidence(),
        Some(EvidenceLookupQueryImportEvidence::ConsumerKitSupportPin { .. })
    ));
    assert_eq!(
        family.stage_applicability().stage_receipt_family_identity(),
        &EvidenceLookupStageReceiptFamilyIdentity::boolean_common_plane()
    );
    assert!(family
        .stage_applicability()
        .declares_multiple_matching_stages());
    assert!(!family.declaration_digest().is_empty());
}

#[test]
fn spatial_public_api_routes_families_by_stage_and_receipt_identity() {
    let closeout = current_evidence_lookup_family_catalog().expect("public family catalog closes");
    let selection = closeout.families_for_stage(
        WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
        &EvidenceLookupStageReceiptFamilyIdentity::boolean_operand_projection_consumption(),
    );

    assert_eq!(selection.family_count(), 1);
    assert_eq!(
        selection.family_identities(),
        &["spatial-touch.boolean.projection-consumption-evidence.v1".to_string()]
    );
    assert_eq!(selection.counters().candidate_family_count(), 3);
    assert_eq!(selection.counters().receipt_family_match_count(), 1);
    assert_eq!(selection.counters().stage_match_count(), 1);
}

#[test]
fn spatial_public_api_exposes_typed_projection_query_import() {
    let closeout = current_evidence_lookup_family_catalog().expect("public family catalog closes");
    let family = closeout
        .family_by_identity("spatial-touch.boolean.projection-consumption-evidence.v1")
        .expect("projection consumption family is declared");
    let import = family
        .query_posture()
        .imported_evidence()
        .expect("projection family imports Query evidence");

    assert_eq!(
        import.projection_consumption_surface(),
        Some(EvidenceLookupProjectionConsumptionSurface::ForgeQueryProjectionConsumptionReceipt)
    );
    assert_eq!(
        import.projection_fact_family(),
        Some(EvidenceLookupProjectionFactFamily::SpatialTouchOperandProjection)
    );
}

#[test]
fn spatial_public_api_exposes_topology_seed_validation_boundary() {
    let _: fn(
        &EvidenceLookupFamilyCatalogCloseout,
        &DerivedInvalidationMilestoneElevenSeed,
    )
        -> Result<EvidenceLookupTopologyRequirementReport, EvidenceLookupFamilyCatalogError> =
        EvidenceLookupFamilyCatalogCloseout::validate_topology_requirements_against_seed;
}
