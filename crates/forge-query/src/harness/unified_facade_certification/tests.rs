use super::row_catalog::{
    UNIFIED_FACADE_REQUIRED_CANONICAL_ROW_NAMES,
    UNIFIED_FACADE_REQUIRED_COMPILE_FAIL_BOUNDARY_NAMES,
    UNIFIED_FACADE_REQUIRED_REJECTION_ROW_NAMES,
};
use super::{MilestoneFivePointSixUnifiedFacadeCertificationAdapter, UnifiedFacadeFailureClass};
use crate::harness::certification::{contains_row, milestone_five_point_six_requirements};
use std::path::Path;

#[test]
fn unified_facade_certification_matrix_contains_required_rows() {
    let matrix = MilestoneFivePointSixUnifiedFacadeCertificationAdapter::
        unified_facade_and_configuration_boundary_test();

    assert_eq!(
        matrix.suite_name,
        "Unified Facade And Configuration Boundary Test"
    );
    for row_name in UNIFIED_FACADE_REQUIRED_CANONICAL_ROW_NAMES {
        assert!(contains_row(&matrix, row_name));
    }
    for row_name in UNIFIED_FACADE_REQUIRED_REJECTION_ROW_NAMES {
        assert!(contains_row(&matrix, row_name));
    }

    let requirements = milestone_five_point_six_requirements();
    assert_eq!(requirements.suite_name, matrix.suite_name);
    for row in &matrix.rows {
        assert!(!row.control_lane.support_report_digest.is_empty());
        assert_eq!(
            row.control_lane.support_report_digest,
            row.parity_lane.support_report_digest
        );
        assert_eq!(row.control_lane.support_report_generation_count, 1);
        assert_eq!(row.control_lane.capability_lookup_count, 1);
        assert_eq!(row.control_lane.configuration_section_resolution_count, 1);
        assert_eq!(row.control_lane.unsupported_composition_denial_count, 0);
        assert_eq!(row.control_lane.deferred_capability_denial_count, 0);
    }
    let support_sync = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "capability-support-metadata-sync")
        .expect("support metadata sync row should exist");
    assert_eq!(
        support_sync.control_lane.capability_family,
        "historical_evaluation"
    );

    let preview = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "unified-preview-capability")
        .expect("preview row should exist");
    assert_eq!(preview.control_lane.capability_family, "preview_session");

    let workflow = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "unified-workflow-capability")
        .expect("workflow row should exist");
    assert_eq!(
        workflow.control_lane.capability_family,
        "workflow_orchestration"
    );

    let historical = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "unified-historical-capability")
        .expect("historical row should exist");
    assert_eq!(
        historical.control_lane.capability_family,
        "historical_evaluation"
    );

    let config_section = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "unified-config-section-explicitness")
        .expect("config section row should exist");
    assert_eq!(config_section.control_lane.capability_lookup_count, 1);
    assert_eq!(
        config_section.hostile_lane.capability_lookup_count, 0,
        "section resolution row should prove section work without capability lookup"
    );
    assert_eq!(
        config_section
            .hostile_lane
            .configuration_section_resolution_count,
        1
    );
}

#[test]
fn unified_facade_rejections_preserve_failure_class_honesty() {
    let matrix = MilestoneFivePointSixUnifiedFacadeCertificationAdapter::
        unified_facade_and_configuration_boundary_test();

    let unsupported = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "missing-owning-live-section")
        .expect("missing owning live section rejection row should exist");
    assert_eq!(
        unsupported.hostile_lane.failure_class,
        UnifiedFacadeFailureClass::MissingOwningSection
    );
    assert_eq!(unsupported.hostile_lane.capability_lookup_count, 1);
    assert_eq!(
        unsupported
            .hostile_lane
            .unsupported_composition_denial_count,
        1
    );
    assert_eq!(unsupported.hostile_lane.deferred_capability_denial_count, 0);
    assert_eq!(unsupported.hostile_lane.config_validation_denial_count, 0);

    let invalid_support = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "invalid-workflow-support-posture")
        .expect("invalid workflow support posture rejection row should exist");
    assert_eq!(
        invalid_support.hostile_lane.failure_class,
        UnifiedFacadeFailureClass::InvalidComposedSupportPosture
    );
    assert_eq!(invalid_support.hostile_lane.capability_lookup_count, 1);
    assert_eq!(
        invalid_support
            .hostile_lane
            .unsupported_composition_denial_count,
        1
    );
    assert_eq!(
        invalid_support
            .hostile_lane
            .deferred_capability_denial_count,
        0
    );
    assert_eq!(
        invalid_support.hostile_lane.config_validation_denial_count,
        0
    );

    let deferred = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "deferred-durable-artifacts")
        .expect("deferred durability rejection row should exist");
    assert_eq!(
        deferred.hostile_lane.failure_class,
        UnifiedFacadeFailureClass::DeferredCapability
    );
    assert_eq!(deferred.hostile_lane.capability_lookup_count, 1);
    assert_eq!(
        deferred.hostile_lane.unsupported_composition_denial_count,
        0
    );
    assert_eq!(deferred.hostile_lane.deferred_capability_denial_count, 1);
    assert_eq!(deferred.hostile_lane.config_validation_denial_count, 0);

    let invalid_config = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "invalid-unified-configuration")
        .expect("invalid unified config rejection row should exist");
    assert_eq!(
        invalid_config.hostile_lane.failure_class,
        UnifiedFacadeFailureClass::InvalidConfiguration
    );
    assert_eq!(invalid_config.hostile_lane.capability_lookup_count, 0);
    assert_eq!(
        invalid_config
            .hostile_lane
            .configuration_section_resolution_count,
        5
    );
    assert_eq!(
        invalid_config.hostile_lane.config_validation_denial_count,
        1
    );
    assert_eq!(
        invalid_config
            .hostile_lane
            .unsupported_composition_denial_count,
        0
    );
    assert_eq!(
        invalid_config.hostile_lane.deferred_capability_denial_count,
        0
    );
}

#[test]
fn unified_facade_compile_fail_boundaries_cover_legacy_shortcuts() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let ui = root.join("tests").join("ui");

    for boundary_name in UNIFIED_FACADE_REQUIRED_COMPILE_FAIL_BOUNDARY_NAMES {
        assert!(
            ui.join(format!("{boundary_name}.rs")).exists(),
            "missing compile-fail fixture {boundary_name}.rs"
        );
        assert!(
            ui.join(format!("{boundary_name}.stderr")).exists(),
            "missing compile-fail fixture {boundary_name}.stderr"
        );
    }
}
