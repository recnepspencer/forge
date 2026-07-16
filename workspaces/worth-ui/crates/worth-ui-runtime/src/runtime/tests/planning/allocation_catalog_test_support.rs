use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::certification_support::snapshot_after_layout_admission_support;
use crate::declaration::{
    UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementConstraintModifier,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementOwnershipPosture,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, display_field_projection_context,
    host_result_portal_anchor, host_result_scroll_container_viewport, host_result_viewport_extent,
    viewport_extent_policy,
};
use crate::evidence::{admit_measurement_basis, MeasurementEvidenceInput, UiMeasurementBasis};
use crate::facade::{WorthUi, WorthUiApp};
use crate::graph::{UiGraphNodeIdentity, UiGraphSnapshot, UiGraphWorldProfile};
use crate::obligations::selection::UiSelectedObligationSet;

pub(crate) fn admitted_disjoint_planning_admissions(
    label: &str,
) -> (
    UiGraphSnapshot,
    (UiMeasurementBasis, UiSelectedObligationSet),
    (UiMeasurementBasis, UiSelectedObligationSet),
) {
    let (snapshot, mut admissions) = admitted_viewport_planning_admissions(label, 2);
    let second = admissions.pop().expect("second admission exists");
    let first = admissions.pop().expect("first admission exists");
    (snapshot, first, second)
}

pub(crate) fn admitted_viewport_planning_admissions(
    label: &str,
    count: usize,
) -> (
    UiGraphSnapshot,
    Vec<(UiMeasurementBasis, UiSelectedObligationSet)>,
) {
    admitted_planning_admissions(label, count, "operator:stack")
}

pub(crate) fn admitted_split_planning_admissions(
    label: &str,
    count: usize,
) -> (
    UiGraphSnapshot,
    Vec<(UiMeasurementBasis, UiSelectedObligationSet)>,
) {
    admitted_planning_admissions(label, count, "operator:split")
}

pub(crate) fn admitted_scroll_planning_admissions_from_settlement(
    label: &str,
    count: usize,
    settlement: &worth_ui_query_binding::WorthUiQueryMeasurementFactSettlement,
) -> (
    UiGraphSnapshot,
    Vec<(UiMeasurementBasis, UiSelectedObligationSet)>,
) {
    admitted_planning_admissions_with(
        label,
        count,
        "operator:scroll",
        Some(UiGraphWorldProfile::installed_query_basis(
            settlement.basis_authority().clone(),
        )),
        |ordinal, identity, target, app, capability, generation| {
            if ordinal == 0 {
                let viewport = host_result_viewport_extent(949, capability, generation);
                return admit_measurement_basis(
                    identity,
                    target,
                    app.graph_snapshot().world_profile().clone(),
                    generation,
                    &viewport_extent_policy(),
                    &[
                        MeasurementEvidenceInput::host_capability_report(capability),
                        MeasurementEvidenceInput::host_measurement_result(&viewport),
                    ],
                );
            }
            let viewport =
                host_result_scroll_container_viewport(950 + ordinal as u64, capability, generation);
            let outer_viewport =
                host_result_viewport_extent(960 + ordinal as u64, capability, generation);
            let policy = scroll_owner_policy();
            let dependencies = crate::declaration::declared_query_measurement_dependencies(&policy)
                .expect("scroll policy declares Query measurement dependencies");
            let query = crate::evidence::admit_declared_measurement_projection_fact_receipt(
                identity.clone(),
                generation,
                dependencies,
                settlement.resolution_mode(),
                settlement.receipt().clone(),
            )
            .expect("installed scroll content-total Query fact admits");
            admit_measurement_basis(
                identity,
                target,
                app.graph_snapshot().world_profile().clone(),
                generation,
                &policy,
                &[
                    MeasurementEvidenceInput::host_capability_report(capability),
                    MeasurementEvidenceInput::host_measurement_result(&outer_viewport),
                    MeasurementEvidenceInput::host_measurement_result(&viewport),
                    MeasurementEvidenceInput::query_projection_fact(&query),
                ],
            )
        },
    )
}

pub(crate) fn admitted_portal_planning_admissions(
    label: &str,
    count: usize,
) -> (
    UiGraphSnapshot,
    Vec<(UiMeasurementBasis, UiSelectedObligationSet)>,
) {
    admitted_planning_admissions_with(
        label,
        count,
        "operator:portal-anchor",
        None,
        |ordinal, identity, target, app, capability, generation| {
            if ordinal == 0 {
                let viewport = host_result_viewport_extent(979, capability, generation);
                return admit_measurement_basis(
                    identity,
                    target,
                    app.graph_snapshot().world_profile().clone(),
                    generation,
                    &viewport_extent_policy(),
                    &[
                        MeasurementEvidenceInput::host_capability_report(capability),
                        MeasurementEvidenceInput::host_measurement_result(&viewport),
                    ],
                );
            }
            let portal = host_result_portal_anchor(980 + ordinal as u64, capability, generation);
            let policy = portal_anchor_policy();
            admit_measurement_basis(
                identity,
                target,
                app.graph_snapshot().world_profile().clone(),
                generation,
                &policy,
                &[
                    MeasurementEvidenceInput::host_capability_report(capability),
                    MeasurementEvidenceInput::host_measurement_result(&portal),
                ],
            )
        },
    )
}

fn portal_anchor_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        Some(UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired),
        vec![crate::declaration::UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics],
    )
    .expect("portal-anchor catalog policy admits")
}

fn scroll_owner_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        Some(UiDeclaredMeasurementBasisSource::ScrollViewport),
        Some(UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis),
        vec![crate::declaration::UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent],
    )
    .expect("scroll-owned catalog policy admits")
}

fn admitted_planning_admissions(
    label: &str,
    count: usize,
    operator: &str,
) -> (
    UiGraphSnapshot,
    Vec<(UiMeasurementBasis, UiSelectedObligationSet)>,
) {
    admitted_planning_admissions_with(
        label,
        count,
        operator,
        None,
        |_, identity, target, app, capability, generation| {
            let viewport_extent = host_result_viewport_extent(900, capability, generation);
            admit_measurement_basis(
                identity,
                target,
                app.graph_snapshot().world_profile().clone(),
                generation,
                &viewport_extent_policy(),
                &[
                    MeasurementEvidenceInput::host_capability_report(capability),
                    MeasurementEvidenceInput::host_measurement_result(&viewport_extent),
                ],
            )
        },
    )
}

fn admitted_planning_admissions_with(
    label: &str,
    count: usize,
    operator: &str,
    world_profile: Option<crate::graph::UiGraphWorldProfile>,
    basis: impl Fn(
        usize,
        crate::declaration::UiDeclarationIdentity,
        UiGraphNodeIdentity,
        &WorthUiApp,
        &worth_ui_host_contract::WorthUiHostCapabilityReport,
        UiEvidenceAuthorityGeneration,
    ) -> UiMeasurementBasis,
) -> (
    UiGraphSnapshot,
    Vec<(UiMeasurementBasis, UiSelectedObligationSet)>,
) {
    assert!(count > 0);
    let world_profile = world_profile.unwrap_or_else(|| {
        let (_, _, world_profile) = display_field_projection_context(label);
        world_profile
    });
    let package = (0..count.saturating_sub(1)).fold(
        WorthUiDslPackage::named("worth-ui.runtime.allocation-planning.catalog"),
        |package, ordinal| {
            package.with_semantic_artifact_spec(control_spec(
                ordinal,
                &format!("slot:{ordinal}"),
                operator,
            ))
        },
    );
    let app = WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_dsl_package(package)
        .freeze();
    let first_identity = app
        .declaration_artifacts()
        .iter()
        .find(|artifact| {
            artifact.family().is_ok_and(|family| {
                family.kind() == crate::declaration::UiDeclarationFamilyKind::Page
            })
        })
        .expect("application root page is admitted")
        .identity()
        .clone();
    let mut identities = vec![first_identity];
    identities
        .extend((0..count.saturating_sub(1)).map(|ordinal| declaration_identity(&app, ordinal)));
    let nodes = identities
        .iter()
        .map(|identity| graph_node(&app, identity))
        .collect::<Vec<_>>();
    let snapshot = snapshot_after_layout_admission_support(&app, &nodes);
    let capability = capability_report(77);
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let selected = |target| {
        let touch = app.try_query_touch_for_node(target).expect("touch admits");
        app.admission().select_obligations(&touch)
    };
    let admissions = identities
        .into_iter()
        .zip(nodes)
        .enumerate()
        .map(|(ordinal, (identity, node))| {
            (
                basis(ordinal, identity, node, &app, &capability, generation),
                selected(node),
            )
        })
        .collect();
    (snapshot, admissions)
}

fn control_spec(ordinal: usize, slot: &str, operator: &str) -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new(format!("allocation_planning.control.{ordinal}")),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/allocation_catalog_test_support.wui", ordinal),
    )
    .with_structural_token(UiDslStructuralToken::new("control:primary"))
    .with_structural_token(UiDslStructuralToken::new(slot))
    .with_structural_token(UiDslStructuralToken::new(operator))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
}

fn declaration_identity(
    app: &WorthUiApp,
    ordinal: usize,
) -> crate::declaration::UiDeclarationIdentity {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == "app/allocation_catalog_test_support.wui"
                && provenance.declaration_index() == ordinal
        })
        .expect("catalog declaration identity is frozen")
        .identity()
        .clone()
}

fn graph_node(
    app: &WorthUiApp,
    declaration: &crate::declaration::UiDeclarationIdentity,
) -> UiGraphNodeIdentity {
    app.graph()
        .lookup()
        .declaration_instances(declaration)
        .value()[0]
}
