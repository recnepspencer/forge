use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementConstraintModifier,
    UiDeclaredMeasurementEvidenceRequirement, UiDeclaredMeasurementMode,
    UiDeclaredMeasurementOwnershipPosture, UiDeclaredMeasurementPolicyPosture,
};
use crate::graph::UiGraphNodeIdentity;

use crate::evidence::measurement::projection::variant_test_support::display_field_plus_entity_identity_projection_context;
use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, display_field_projection_context, host_font_metrics_policy,
    host_result_font_metrics, host_result_portal_anchor, host_result_scroll_container_viewport,
    host_result_viewport_extent, scroll_viewport_policy, synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, consume_declared_measurement_projection_facts, MeasurementEvidenceInput,
    UiMeasurementDependencyLineageKind, UiMeasurementDependencyMapEntry,
    UiMeasurementNeighborhoodClassHint,
};

#[test]
fn basis_dependency_map_preserves_typed_lineage_families() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let declaration_identity = synthetic_declaration_identity("basis-dependency-map");
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("basis-dependency-map");
    let policy = UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        Some(UiDeclaredMeasurementBasisSource::ScrollViewport),
        Some(UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis),
        vec![
            UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics,
            UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent,
            UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics,
        ],
    )
    .expect("combined measurement policy should admit");
    let receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        generation,
        &policy,
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let capability_report = capability_report(77);
    let basis = admit_measurement_basis(
        declaration_identity,
        UiGraphNodeIdentity::new(101),
        world_profile,
        generation,
        &policy,
        &[
            MeasurementEvidenceInput::query_projection_fact(&receipt),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_font_metrics(
                1,
                &capability_report,
                generation,
            )),
            MeasurementEvidenceInput::host_measurement_result(&host_result_viewport_extent(
                2,
                &capability_report,
                generation,
            )),
            MeasurementEvidenceInput::host_measurement_result(&host_result_portal_anchor(
                3,
                &capability_report,
                generation,
            )),
            MeasurementEvidenceInput::host_measurement_result(
                &host_result_scroll_container_viewport(4, &capability_report, generation),
            ),
        ],
    );

    let entries = basis.dependency_map().entries();
    assert!(basis.is_admitted());
    assert_eq!(entries.len(), 5);
    assert_eq!(
        entry_class(
            entries,
            UiMeasurementDependencyLineageKind::QueryScrollContentExtent
        ),
        UiMeasurementNeighborhoodClassHint::LocalIntrinsicContentDependency
    );
    assert_eq!(
        entry_class(entries, UiMeasurementDependencyLineageKind::HostFontMetrics),
        UiMeasurementNeighborhoodClassHint::LocalIntrinsicContentDependency
    );
    assert_eq!(
        entry_class(
            entries,
            UiMeasurementDependencyLineageKind::HostViewportExtent
        ),
        UiMeasurementNeighborhoodClassHint::ViewportDependency
    );
    assert_eq!(
        entry_class(
            entries,
            UiMeasurementDependencyLineageKind::HostPortalAnchorRect
        ),
        UiMeasurementNeighborhoodClassHint::PortalAnchorDependency
    );
    assert_eq!(
        entry_class(
            entries,
            UiMeasurementDependencyLineageKind::HostScrollContainerViewport,
        ),
        UiMeasurementNeighborhoodClassHint::ScrollContainerDependency
    );
    assert_eq!(
        basis.neighborhood_class_hint(),
        UiMeasurementNeighborhoodClassHint::PortalAnchorDependency
    );
}

#[test]
fn local_measurement_inputs_classify_to_narrow_neighborhood_hints() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let capability_report = capability_report(77);

    let host_only_basis = admit_measurement_basis(
        synthetic_declaration_identity("basis-local-intrinsic"),
        UiGraphNodeIdentity::new(102),
        crate::graph::UiGraphWorldProfile::authoritative(),
        generation,
        &host_font_metrics_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_font_metrics(
                5,
                &capability_report,
                generation,
            )),
        ],
    );
    assert_eq!(
        host_only_basis.neighborhood_class_hint(),
        UiMeasurementNeighborhoodClassHint::LocalIntrinsicContentDependency
    );

    let declaration_identity = synthetic_declaration_identity("basis-viewport-hint");
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("basis-viewport-hint");
    let receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        generation,
        &scroll_viewport_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let viewport_basis = admit_measurement_basis(
        declaration_identity,
        UiGraphNodeIdentity::new(103),
        world_profile,
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::query_projection_fact(&receipt),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_font_metrics(
                6,
                &capability_report,
                generation,
            )),
            MeasurementEvidenceInput::host_measurement_result(&host_result_viewport_extent(
                7,
                &capability_report,
                generation,
            )),
        ],
    );
    assert_eq!(
        viewport_basis.neighborhood_class_hint(),
        UiMeasurementNeighborhoodClassHint::ViewportDependency
    );
}

#[test]
fn unrelated_query_projection_facts_do_not_widen_measurement_neighborhood() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let capability_report = capability_report(77);
    let declaration_identity = synthetic_declaration_identity("basis-query-separation");

    let (baseline_prerequisites, baseline_attempt, baseline_world_profile) =
        display_field_projection_context("basis-query-separation");
    let baseline_receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        generation,
        &scroll_viewport_policy(),
        baseline_prerequisites,
        &baseline_attempt,
    )
    .expect("baseline query receipt should admit");

    let (expanded_prerequisites, expanded_attempt, expanded_world_profile) =
        display_field_plus_entity_identity_projection_context("basis-query-separation");
    let expanded_receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        generation,
        &scroll_viewport_policy(),
        expanded_prerequisites,
        &expanded_attempt,
    )
    .expect("expanded query receipt should admit");

    assert_ne!(
        baseline_receipt.projection_fact_set_digest(),
        expanded_receipt.projection_fact_set_digest()
    );
    assert_ne!(
        baseline_receipt.projection_consumption_receipt_digest(),
        expanded_receipt.projection_consumption_receipt_digest()
    );

    let baseline_basis = admit_measurement_basis(
        declaration_identity.clone(),
        UiGraphNodeIdentity::new(104),
        baseline_world_profile,
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::query_projection_fact(&baseline_receipt),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_font_metrics(
                8,
                &capability_report,
                generation,
            )),
            MeasurementEvidenceInput::host_measurement_result(&host_result_viewport_extent(
                9,
                &capability_report,
                generation,
            )),
        ],
    );
    let expanded_basis = admit_measurement_basis(
        declaration_identity,
        UiGraphNodeIdentity::new(104),
        expanded_world_profile,
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::query_projection_fact(&expanded_receipt),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_font_metrics(
                8,
                &capability_report,
                generation,
            )),
            MeasurementEvidenceInput::host_measurement_result(&host_result_viewport_extent(
                9,
                &capability_report,
                generation,
            )),
        ],
    );

    assert_eq!(
        baseline_basis.dependency_map().entries(),
        expanded_basis.dependency_map().entries()
    );
    assert_eq!(
        baseline_basis.dependency_map().identity_digest(),
        expanded_basis.dependency_map().identity_digest()
    );
    assert_eq!(
        baseline_basis.neighborhood_class_hint(),
        expanded_basis.neighborhood_class_hint()
    );
}

#[test]
fn container_available_space_dependency_is_preserved_as_distinct_fallback_class() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let policy = UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        None,
        None,
        vec![],
    )
    .expect("container-space measurement policy should admit");

    let basis = admit_measurement_basis(
        synthetic_declaration_identity("basis-container-space"),
        UiGraphNodeIdentity::new(105),
        crate::graph::UiGraphWorldProfile::authoritative(),
        generation,
        &policy,
        &[],
    );

    assert!(basis.is_admitted());
    assert!(basis.dependency_map().entries().is_empty());
    assert_eq!(
        basis.neighborhood_class_hint(),
        UiMeasurementNeighborhoodClassHint::ContainerAvailableSpaceDependency
    );
}

#[test]
fn portal_anchor_inputs_classify_to_portal_anchor_hint() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let capability_report = capability_report(77);
    let policy = UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        None,
        vec![UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics],
    )
    .expect("portal-anchor measurement policy should admit");

    let basis = admit_measurement_basis(
        synthetic_declaration_identity("basis-portal-anchor"),
        UiGraphNodeIdentity::new(106),
        crate::graph::UiGraphWorldProfile::authoritative(),
        generation,
        &policy,
        &[
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_portal_anchor(
                10,
                &capability_report,
                generation,
            )),
        ],
    );

    assert_eq!(
        basis.neighborhood_class_hint(),
        UiMeasurementNeighborhoodClassHint::PortalAnchorDependency
    );
}

#[test]
fn scroll_container_inputs_classify_to_scroll_container_hint() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let capability_report = capability_report(77);
    let policy = UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        None,
        Some(UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis),
        vec![],
    )
    .expect("scroll-container measurement policy should admit");

    let basis = admit_measurement_basis(
        synthetic_declaration_identity("basis-scroll-container"),
        UiGraphNodeIdentity::new(107),
        crate::graph::UiGraphWorldProfile::authoritative(),
        generation,
        &policy,
        &[
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(
                &host_result_scroll_container_viewport(11, &capability_report, generation),
            ),
        ],
    );

    assert_eq!(
        basis.neighborhood_class_hint(),
        UiMeasurementNeighborhoodClassHint::ScrollContainerDependency
    );
}

fn entry_class(
    entries: &[UiMeasurementDependencyMapEntry],
    kind: UiMeasurementDependencyLineageKind,
) -> UiMeasurementNeighborhoodClassHint {
    entries
        .iter()
        .find(|entry| entry.lineage().kind() == kind)
        .expect("dependency kind should be preserved")
        .neighborhood_class_hint()
}
