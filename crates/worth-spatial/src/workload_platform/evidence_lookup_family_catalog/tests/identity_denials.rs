use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;

use super::super::{
    current_evidence_lookup_family_catalog, EvidenceLookupDiagnosticWitnessShape,
    EvidenceLookupEvidenceClass, EvidenceLookupEvidenceClassSet,
    EvidenceLookupFamilyCatalogErrorKind, EvidenceLookupFamilyDeclaration,
    EvidenceLookupFamilyIndexPosture, EvidenceLookupFamilyQueryPosture,
    EvidenceLookupFamilySourceInventoryPressure, EvidenceLookupFamilyStageSelection,
    EvidenceLookupProductPosture, EvidenceLookupSpatialTouchAuthorityRequirement,
    EvidenceLookupStageApplicability, EvidenceLookupStageReceiptFamilyIdentity,
    EvidenceLookupTopologyInputPosture, TestCatalogCloseout,
};

#[test]
fn family_catalog_rejects_duplicate_family_identity() {
    let closeout = current_evidence_lookup_family_catalog().expect("family catalog closes");
    let family = closeout
        .family_by_identity("spatial-touch.boolean.overlap-evidence.v1")
        .expect("overlap family exists")
        .clone();

    let error = TestCatalogCloseout::from_declarations(vec![family.clone(), family])
        .expect_err("duplicate identity is rejected");

    assert_eq!(
        error.kind(),
        EvidenceLookupFamilyCatalogErrorKind::DuplicateFamilyIdentity
    );
}

#[test]
fn family_declaration_builder_requires_every_authoritative_field() {
    assert_missing_field(
        EvidenceLookupFamilyDeclaration::builder().build(),
        EvidenceLookupFamilyCatalogErrorKind::MissingFamilyIdentity,
    );
    assert_missing_field(
        valid_builder_without_spatial_touch_authority().build(),
        EvidenceLookupFamilyCatalogErrorKind::MissingSpatialTouchAuthority,
    );
    assert_missing_field(
        valid_builder_without_topology_input_posture().build(),
        EvidenceLookupFamilyCatalogErrorKind::MissingTopologyInputPosture,
    );
    assert_missing_field(
        valid_builder_without_stage_applicability().build(),
        EvidenceLookupFamilyCatalogErrorKind::MissingStageApplicability,
    );
    assert_missing_field(
        valid_builder_without_evidence_classes().build(),
        EvidenceLookupFamilyCatalogErrorKind::MissingEvidenceClass,
    );
    assert_missing_field(
        valid_builder_without_lookup_product_posture().build(),
        EvidenceLookupFamilyCatalogErrorKind::MissingLookupProductPosture,
    );
    assert_missing_field(
        valid_builder_without_index_posture().build(),
        EvidenceLookupFamilyCatalogErrorKind::MissingIndexPosture,
    );
    assert_missing_field(
        valid_builder_without_query_posture().build(),
        EvidenceLookupFamilyCatalogErrorKind::MissingQueryPosture,
    );
    assert_missing_field(
        valid_builder_without_diagnostic_witness().build(),
        EvidenceLookupFamilyCatalogErrorKind::MissingDiagnosticWitness,
    );
    assert_missing_field(
        valid_builder_without_source_inventory_pressure().build(),
        EvidenceLookupFamilyCatalogErrorKind::MissingSourceInventoryPressure,
    );
}

#[test]
fn stage_applicability_rejects_empty_and_duplicate_stage_sets() {
    assert_missing_field(
        EvidenceLookupStageApplicability::matching_stages(
            Vec::new(),
            EvidenceLookupStageReceiptFamilyIdentity::boolean_common_plane(),
        ),
        EvidenceLookupFamilyCatalogErrorKind::EmptyStageApplicability,
    );
    assert_missing_field(
        EvidenceLookupStageApplicability::matching_stages(
            vec![
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
            ],
            EvidenceLookupStageReceiptFamilyIdentity::boolean_common_plane(),
        ),
        EvidenceLookupFamilyCatalogErrorKind::DuplicateStageApplicability,
    );
}

#[test]
fn evidence_class_sets_reject_empty_and_duplicate_classes() {
    assert_missing_field(
        EvidenceLookupEvidenceClassSet::new(Vec::new()),
        EvidenceLookupFamilyCatalogErrorKind::EmptyEvidenceClassSet,
    );
    assert_missing_field(
        EvidenceLookupEvidenceClassSet::new(vec![
            EvidenceLookupEvidenceClass::BooleanStageReceipt,
            EvidenceLookupEvidenceClass::BooleanStageReceipt,
        ]),
        EvidenceLookupFamilyCatalogErrorKind::DuplicateEvidenceClass,
    );
}

#[test]
fn stage_selection_deduplicates_by_rejected_authoring_not_query_result_cleanup() {
    let closeout = current_evidence_lookup_family_catalog().expect("family catalog closes");
    let selection: EvidenceLookupFamilyStageSelection = closeout.families_for_stage(
        WorkloadEvidenceStage::BooleanEventLedger,
        &EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
    );

    assert_eq!(selection.family_count(), 1);
    assert_eq!(selection.counters().stage_match_count(), 1);
}

fn assert_missing_field<T>(
    result: Result<T, super::super::EvidenceLookupFamilyCatalogError>,
    expected: EvidenceLookupFamilyCatalogErrorKind,
) {
    let error = match result {
        Ok(_) => panic!("invalid catalog construction must fail"),
        Err(error) => error,
    };
    assert_eq!(error.kind(), expected);
}

fn valid_builder_without_spatial_touch_authority(
) -> super::super::EvidenceLookupFamilyDeclarationBuilder {
    builder_without("spatial_touch_authority")
}

fn valid_builder_without_topology_input_posture(
) -> super::super::EvidenceLookupFamilyDeclarationBuilder {
    builder_without("topology_input_posture")
}

fn valid_builder_without_stage_applicability(
) -> super::super::EvidenceLookupFamilyDeclarationBuilder {
    builder_without("stage_applicability")
}

fn valid_builder_without_evidence_classes() -> super::super::EvidenceLookupFamilyDeclarationBuilder
{
    builder_without("evidence_classes")
}

fn valid_builder_without_lookup_product_posture(
) -> super::super::EvidenceLookupFamilyDeclarationBuilder {
    builder_without("lookup_product_posture")
}

fn valid_builder_without_index_posture() -> super::super::EvidenceLookupFamilyDeclarationBuilder {
    builder_without("index_posture")
}

fn valid_builder_without_query_posture() -> super::super::EvidenceLookupFamilyDeclarationBuilder {
    builder_without("query_posture")
}

fn valid_builder_without_diagnostic_witness() -> super::super::EvidenceLookupFamilyDeclarationBuilder
{
    builder_without("diagnostic_witness")
}

fn valid_builder_without_source_inventory_pressure(
) -> super::super::EvidenceLookupFamilyDeclarationBuilder {
    builder_without("source_inventory_pressure")
}

fn builder_without(field: &str) -> super::super::EvidenceLookupFamilyDeclarationBuilder {
    let mut builder = EvidenceLookupFamilyDeclaration::builder().identity(
        super::super::EvidenceLookupFamilyIdentity::declared("test.family"),
    );
    if field != "spatial_touch_authority" {
        builder = builder.spatial_touch_authority(
            EvidenceLookupSpatialTouchAuthorityRequirement::SealedSpatialTouchAuthorityRequired,
        );
    }
    if field != "topology_input_posture" {
        builder =
            builder.topology_input_posture(EvidenceLookupTopologyInputPosture::not_required());
    }
    if field != "stage_applicability" {
        builder = builder.stage_applicability(valid_stage_applicability());
    }
    if field != "evidence_classes" {
        builder = builder.evidence_classes(valid_evidence_classes());
    }
    if field != "lookup_product_posture" {
        builder = builder
            .lookup_product_posture(EvidenceLookupProductPosture::DeclarationOnlySelectionRequired);
    }
    if field != "index_posture" {
        builder =
            builder.index_posture(EvidenceLookupFamilyIndexPosture::sparse_lookup_plan_required());
    }
    if field != "query_posture" {
        builder = builder.query_posture(EvidenceLookupFamilyQueryPosture::not_required());
    }
    if field != "diagnostic_witness" {
        builder = builder.diagnostic_witness(
            EvidenceLookupDiagnosticWitnessShape::spatial_touch_stage_receipt_only(),
        );
    }
    if field != "source_inventory_pressure" {
        builder = builder.source_inventory_pressure(valid_source_pressure());
    }
    builder
}

fn valid_stage_applicability() -> EvidenceLookupStageApplicability {
    EvidenceLookupStageApplicability::matching_stages(
        vec![WorkloadEvidenceStage::BooleanEventLedger],
        EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
    )
    .expect("valid stage applicability")
}

fn valid_evidence_classes() -> EvidenceLookupEvidenceClassSet {
    EvidenceLookupEvidenceClassSet::new(vec![EvidenceLookupEvidenceClass::BooleanStageReceipt])
        .expect("valid evidence class set")
}

fn valid_source_pressure() -> EvidenceLookupFamilySourceInventoryPressure {
    EvidenceLookupFamilySourceInventoryPressure::phase_two_family_catalog(1, "test-inventory")
}
