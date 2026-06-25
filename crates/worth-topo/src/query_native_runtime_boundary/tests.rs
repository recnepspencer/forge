use super::inventory::{
    WorthTopologyQueryNativeRuntimeBoundaryInventory,
    WorthTopologyQueryNativeRuntimeBoundaryInventoryError,
};
use super::inventory_row::WorthTopologyQueryNativeRuntimeBoundaryInventoryRow;
use super::residue_status::WorthTopologyQueryNativeRuntimeBoundaryResidueStatus;
use super::source_scan::stale_symbol_rows_from_source_pairs;
use super::stale_symbol::WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol;
use super::{
    WorthTopologyNativeAspectField, WorthTopologyNativeAspectValue,
    WorthTopologyNativeCarrierBoundaryError, WorthTopologyNativeFieldPath,
    WorthTopologyNativeSetAspectInput,
};
use crate::topology_operators::TopologyTouchedAspect;

#[test]
fn current_inventory_classifies_all_production_terminal_query_residue() {
    let inventory = WorthTopologyQueryNativeRuntimeBoundaryInventory::from_current_sources()
        .expect("Milestone 9.1 Phase 1 inventory should classify current stale Query residue");

    assert_eq!(inventory.unclassified_count(), 0);
    assert!(inventory.total_observed_occurrence_count() > 0);
    assert_eq!(
        inventory.ordinary_runtime_migration_row_count(),
        0,
        "ordinary runtime source should be clean after the hard-deletion cutover"
    );

    let symbol_counts = inventory.row_count_by_stale_symbol();
    assert!(symbol_counts
        .contains_key(&WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol::ExternalRowProjection));
    assert!(!symbol_counts
        .contains_key(&WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol::LegacyAspectValue));
    assert!(!symbol_counts.contains_key(
        &WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol::CallerBuiltWriteCommand
    ));

    let status_counts = inventory.row_count_by_status();
    assert!(!status_counts.contains_key(
        &WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::MigrateToNativeBoundary
    ));
    assert!(status_counts.contains_key(
        &WorthTopologyQueryNativeRuntimeBoundaryResidueStatus::CertificationSupportCutover
    ));
}

#[test]
fn native_aspect_field_lowers_topology_vocabulary_to_query_touch() {
    let field_path = WorthTopologyNativeFieldPath::from_segments(["kind", "name"])
        .expect("topology field path should be valid");
    let aspect_field = WorthTopologyNativeAspectField::from_touched_aspect_and_field_path(
        TopologyTouchedAspect::TopologyStructure,
        field_path,
    );

    let touch = aspect_field.query_aspect_touch();
    let native_path = touch
        .native_field_path()
        .expect("field-specific topology touch should carry native field path");

    assert_eq!(touch.native_aspect_key().as_str(), "topology.structure");
    assert_eq!(aspect_field.digest_part(), "topology.structure.kind.name");
    assert_eq!(
        native_path
            .fields()
            .iter()
            .map(|field| field.as_str())
            .collect::<Vec<_>>(),
        vec!["kind", "name"]
    );
}

#[test]
fn native_set_aspect_input_carries_query_touch_and_authored_value() {
    let field = WorthTopologyNativeAspectField::from_touched_aspect_and_field_path(
        TopologyTouchedAspect::NamingPersistentName,
        WorthTopologyNativeFieldPath::single("persistent_name")
            .expect("persistent name field path should be valid"),
    );
    let input = WorthTopologyNativeSetAspectInput::new(
        field,
        WorthTopologyNativeAspectValue::string("edge:17"),
    );

    assert_eq!(
        input.query_aspect_touch().native_aspect_key().as_str(),
        "naming.persistent_name"
    );
    assert!(matches!(
        input.foundational_value(),
        forge_foundational::facade::AspectValue::String(_)
    ));
    assert!(input.digest().contains("naming.persistent_name"));
    let _authored_value = input.query_authored_value();
}

#[test]
fn raw_terminal_field_path_segments_are_rejected_before_query_touch_creation() {
    let error = WorthTopologyNativeFieldPath::single("topology.kind")
        .expect_err("raw terminal path text must not become a native field key");

    assert_eq!(
        error,
        WorthTopologyNativeCarrierBoundaryError::InvalidFieldSegment("topology.kind".to_string())
    );
}

#[test]
fn inventory_validation_rejects_unclassified_residue_rows() {
    let row = WorthTopologyQueryNativeRuntimeBoundaryInventoryRow::unclassified_for_test(
        "projection/runtime_boundary/unknown_terminal_api.rs",
        WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol::ExternalRowProjection,
    );

    let error =
        WorthTopologyQueryNativeRuntimeBoundaryInventory::from_rows_for_validation(vec![row])
            .expect_err("unclassified terminal Query residue must block Phase 1 closeout");

    assert!(matches!(
        error,
        WorthTopologyQueryNativeRuntimeBoundaryInventoryError::UnclassifiedResidue { .. }
    ));
}

#[test]
fn compatibility_shim_residue_is_rejected_even_when_patterns_are_known() {
    let rows = stale_symbol_rows_from_source_pairs([(
        "projection/runtime_boundary/query_runtime/adapters/compat/external_row_compat.rs",
        "pub fn external_row() {}\nForgeQueryWriteCommand::placeholder();",
    )]);

    let error = WorthTopologyQueryNativeRuntimeBoundaryInventory::from_rows_for_validation(rows)
        .expect_err("compatibility shim residue must not be accepted as migration work");

    assert!(matches!(
        error,
        WorthTopologyQueryNativeRuntimeBoundaryInventoryError::CompatibilityShimResidue { .. }
    ));
}

#[test]
fn runtime_source_adapter_routes_live_views_by_query_targets_not_name_strings() {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/projection/runtime_boundary/query_runtime/adapters.rs"),
    )
    .expect("topology runtime adapter source should remain readable");

    for required in [
        "BTreeMap<ForgeQueryLiveArtifactTarget, ForgeQueryMutationTargetCollectionIdentity>",
        "ForgeQueryLiveArtifactTarget::from_source_adapter_declared_view_name",
        "fn live_entities_for_target(",
        "fn drain_live_patches_for_target(",
        "fn affected_live_view_targets(",
        ".same_target_collection_as(collection)",
    ] {
        assert!(
            source.contains(required),
            "Phase 6 live target routing should retain Query-native source adapter boundary `{required}`",
        );
    }
    for forbidden in [
        "BTreeMap<String, String>",
        "fn live_entities(&self, view_name",
        "fn drain_live_patches(&mut self, _view_name",
        "fn affected_live_view_ids",
        ".filter(move |(_, target)| *target == &delta.collection())",
    ] {
        assert!(
            !source.contains(forbidden),
            "Phase 6 live target routing must not revive view-name string authority `{forbidden}`",
        );
    }
}
