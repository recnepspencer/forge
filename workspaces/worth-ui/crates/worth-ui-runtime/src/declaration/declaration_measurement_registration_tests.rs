use crate::declaration::{
    UiDeclarationSupportRowSchemaKind, UiDeclaredMeasurementBasisSource,
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementEvidenceRequirement,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementOwnershipPosture,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::facade::entry::WorthUi;
use crate::facade::registry::{
    MeasurementConstraint, MeasurementValue, MosaicChildRule, MosaicClippingPosture,
    MosaicFocusScopeKind, MosaicHitTestPosture, MosaicMeasurementAuthority, MosaicOverflowBehavior,
    MosaicParentGrowthBehavior, MosaicRegionKindDescriptor, MosaicRegionKindId,
    MosaicRegionPersistence, MosaicRegionRole, MosaicResizePermission, MosaicScrollOwnership,
    MosaicSizingBehavior, MosaicSizingContractDescriptor, MosaicSizingContractId, MosaicSizingKind,
    MosaicSizingPersistence, MosaicViewportConstraint, NamedMeasurementDefinition,
    NamedMeasurementToken, SurfacePlacementClass,
};
use crate::facade::WorthUiDslPackage;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};

fn expected_measurement_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        Some(UiDeclaredMeasurementBasisSource::ScrollViewport),
        Some(UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis),
        vec![
            UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics,
            UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent,
        ],
    )
    .expect("test posture should contain measurement meaning")
}

fn expected_portal_measurement_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        None,
        None,
        Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        Some(UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired),
        vec![UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics],
    )
    .expect("test posture should contain portal measurement meaning")
}

fn measurement_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.body"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/measurement_boundary.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:body"))
    .with_posture_token(UiDslPostureToken::new("measurement:mode:hug-height"))
    .with_posture_token(UiDslPostureToken::new("measurement:constraint:bounded"))
    .with_posture_token(UiDslPostureToken::new("measurement:scroll-owned"))
    .with_posture_token(UiDslPostureToken::new(
        "measurement:evidence:font-metrics-required",
    ))
}

fn portal_measurement_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.portal"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/measurement_boundary_portal.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:portal"))
    .with_posture_token(UiDslPostureToken::new("measurement:portal-anchored"))
}

fn registered_measurement_descriptor(id: &str) -> MosaicSizingContractDescriptor {
    MosaicSizingContractDescriptor::new(
        MosaicSizingContractId::new(id).expect("test sizing id should be valid"),
        MosaicSizingKind::hug(),
    )
    .with_measurement_authority(MosaicMeasurementAuthority::runtime_token())
    .with_resize_permission(MosaicResizePermission::user_resizable())
    .with_persistence(MosaicSizingPersistence::restorable())
    .with_overflow_behavior(MosaicOverflowBehavior::scroll_when_constrained())
    .with_parent_growth_behavior(MosaicParentGrowthBehavior::does_not_force_parent())
    .with_viewport_constraint(MosaicViewportConstraint::clamp_to_viewport())
    .with_named_measurement(NamedMeasurementDefinition::new(
        NamedMeasurementToken::new(id).expect("test measurement token should be valid"),
        MeasurementValue::logical_pixels(320),
        MeasurementConstraint::between(
            MeasurementValue::logical_pixels(200),
            MeasurementValue::logical_pixels(640),
        ),
    ))
}

fn hostile_sizing_contract_id() -> MosaicSizingContractId {
    MosaicSizingContractId::new("workspace.measurement.viewport_hostile")
        .expect("test sizing id should be valid")
}

fn hostile_scroll_region_id() -> MosaicRegionKindId {
    MosaicRegionKindId::new("workspace.region.hostile_scroll")
        .expect("test region id should be valid")
}

fn hostile_scroll_region_descriptor() -> MosaicRegionKindDescriptor {
    MosaicRegionKindDescriptor::new(hostile_scroll_region_id(), MosaicRegionRole::primary())
        .with_sizing_behavior(MosaicSizingBehavior::fills_available_space())
        .with_scroll_ownership(MosaicScrollOwnership::viewport_owned())
        .with_focus_scope(MosaicFocusScopeKind::active_surface_scope())
        .with_child_rule(MosaicChildRule::accepts_surfaces())
        .with_allowed_surface_class(SurfacePlacementClass::primary_region())
        .with_persistence(MosaicRegionPersistence::restorable())
        .with_clipping(MosaicClippingPosture::clip_to_region())
        .with_hit_test(MosaicHitTestPosture::participates())
}

fn declared_measurement_policies(
    app: &crate::facade::WorthUiApp,
) -> Vec<(String, usize, UiDeclaredMeasurementPolicyPosture)> {
    app.declaration_artifacts()
        .iter()
        .filter_map(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            artifact
                .support_snapshot()
                .ok()
                .and_then(|snapshot| {
                    snapshot.row(UiDeclarationSupportRowSchemaKind::MeasurementPolicy)
                })
                .and_then(|row| row.declared_measurement_policy_posture())
                .map(|posture| {
                    (
                        provenance.module_path().to_owned(),
                        provenance.declaration_index(),
                        posture.clone(),
                    )
                })
        })
        .filter(|(module_path, declaration_index, _)| {
            (module_path == "app/measurement_boundary.wui"
                || module_path == "app/measurement_boundary_portal.wui")
                && *declaration_index == 0
        })
        .collect()
}

fn assert_hostile_capabilities_are_admitted(app: &crate::facade::WorthUiApp) {
    let capability_index = app.capabilities().index();
    let sizing = capability_index
        .mosaic_sizing_contracts()
        .lookup(&hostile_sizing_contract_id());
    let region = capability_index
        .mosaic_regions()
        .lookup(&hostile_scroll_region_id());

    assert!(
        sizing.is_found(),
        "hostile sizing descriptor must survive registration freeze"
    );
    assert!(
        region.is_found(),
        "hostile region descriptor must survive registration freeze"
    );
    assert_eq!(
        sizing
            .value()
            .and_then(|descriptor| descriptor.viewport_constraint()),
        Some(&MosaicViewportConstraint::clamp_to_viewport()),
    );
    assert_eq!(
        region
            .value()
            .and_then(|descriptor| descriptor.scroll_ownership()),
        Some(&MosaicScrollOwnership::viewport_owned()),
    );
}

#[test]
fn registered_measurement_artifacts_do_not_change_declaration_measurement_posture() {
    let dsl_package = WorthUiDslPackage::named("worth-ui.runtime.measurement.registration-proof")
        .with_semantic_artifact_spec(measurement_control_spec())
        .with_semantic_artifact_spec(portal_measurement_control_spec());
    let baseline = WorthUi::app()
        .with_dsl_package(dsl_package.clone())
        .freeze();
    let with_registered_measurement = WorthUi::app()
        .with_dsl_package(dsl_package)
        .register_mosaic_sizing_contract(registered_measurement_descriptor(
            "workspace.measurement.sidebar",
        ))
        .register_mosaic_sizing_contract(registered_measurement_descriptor(
            hostile_sizing_contract_id().as_str(),
        ))
        .register_mosaic_region_kind(hostile_scroll_region_descriptor())
        .freeze();

    assert_hostile_capabilities_are_admitted(&with_registered_measurement);

    assert_eq!(
        baseline
            .declaration_artifacts()
            .iter()
            .find(
                |artifact| artifact.provenance().source_provenance().module_path()
                    == "app/measurement_boundary.wui"
            )
            .and_then(|artifact| artifact.support_snapshot().ok())
            .and_then(|snapshot| snapshot.row(UiDeclarationSupportRowSchemaKind::MeasurementPolicy))
            .and_then(|row| row.declared_measurement_policy_posture()),
        Some(&expected_measurement_policy())
    );
    assert_eq!(
        baseline
            .declaration_artifacts()
            .iter()
            .find(
                |artifact| artifact.provenance().source_provenance().module_path()
                    == "app/measurement_boundary_portal.wui"
            )
            .and_then(|artifact| artifact.support_snapshot().ok())
            .and_then(|snapshot| snapshot.row(UiDeclarationSupportRowSchemaKind::MeasurementPolicy))
            .and_then(|row| row.declared_measurement_policy_posture()),
        Some(&expected_portal_measurement_policy())
    );
    assert_eq!(
        declared_measurement_policies(&baseline),
        declared_measurement_policies(&with_registered_measurement)
    );
}
