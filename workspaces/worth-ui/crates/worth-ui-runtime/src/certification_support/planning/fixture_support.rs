use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticFamily, UiDslSemanticKey, UiDslStructuralToken,
    WorthUiSemanticArtifactDeclaration,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::capability::WorthUiQueryViewRegistration;
use crate::declaration::{
    UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementConstraintModifier,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::{admit_measurement_basis, MeasurementEvidenceInput};
use crate::facade::{WorthUi, WorthUiApp};
use crate::graph::{UiGraphNodeIdentity, UiGraphSnapshot, UiGraphWorldProfile};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactInputResolver, WorthUiBindingSemanticsLowerer,
    WorthUiCanonicalArtifactAssembler, WorthUiIdentitySeedLowerer,
    WorthUiStructuralLegalityLowerer,
};
use worth_ui_dsl::{
    WorthUiDslCompiler, WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
};

pub(super) use super::fixture_host::{
    capability_report, host_portal_anchor_result, host_scroll_viewport_result,
    host_text_intrinsic_result,
};

pub(super) fn measurement_policy(
    basis_source: Option<UiDeclaredMeasurementBasisSource>,
    bounded: bool,
) -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        bounded.then_some(UiDeclaredMeasurementConstraintModifier::Bounded),
        basis_source,
        None,
        vec![],
    )
    .expect("suite measurement policy should admit")
}

pub(super) fn container_basis(
    app: &WorthUiApp,
    root: UiGraphNodeIdentity,
    generation: UiEvidenceAuthorityGeneration,
    bounded: bool,
) -> crate::evidence::UiMeasurementBasis {
    let report = capability_report(77);
    let mut inputs = vec![MeasurementEvidenceInput::host_capability_report(&report)];
    if !bounded {
        inputs.clear();
    }
    admit_measurement_basis(
        declaration_identity_for(app, 0),
        root,
        app.graph_snapshot().world_profile().clone(),
        generation,
        &measurement_policy(None, bounded),
        &inputs,
    )
}

pub(super) fn intrinsic_basis(
    app: &WorthUiApp,
    root: UiGraphNodeIdentity,
    nodes: usize,
    generation: UiEvidenceAuthorityGeneration,
    bounded: bool,
) -> crate::evidence::UiMeasurementBasis {
    let report = capability_report(81);
    let mut inputs = vec![MeasurementEvidenceInput::host_capability_report(&report)];
    for index in 1..nodes {
        inputs.push(MeasurementEvidenceInput::child_host_measurement_result(
            graph_node_identity_for_provenance(app, index),
            &host_text_intrinsic_result(810 + index as u64, &report, generation),
        ));
    }
    admit_measurement_basis(
        declaration_identity_for(app, 0),
        root,
        app.graph_snapshot().world_profile().clone(),
        generation,
        &measurement_policy(None, bounded),
        &inputs,
    )
}

pub(super) fn query_app() -> WorthUiApp {
    let installed = worth_ui_query_binding::certification::worth_ui_installed_test_domain(
        "allocation-planning-certification",
    );
    let view = installed
        .measurement_view("workspace.view_binding.selection")
        .expect("suite Query view should install");
    WorthUi::app()
        .register_query_view(WorthUiQueryViewRegistration::new(view))
        .expect("installed suite Query view should register")
        .freeze()
        .expect("application preparation should succeed")
}

pub(super) fn artifact_from_modules<const N: usize>(
    app: &WorthUiApp,
    modules: [WorthUiRustAuthoredArtifactInputModule; N],
) -> WorthUiArtifact {
    let input = WorthUiDslCompiler::compile_rust_authored(
        &WorthUiRustAuthoredArtifactInput::from_modules(modules),
    )
    .expect("suite source compiles");
    let snapshot = app.capabilities();
    let resolved =
        WorthUiArtifactInputResolver::resolve(&input, snapshot).expect("suite input resolves");
    let structured = WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot)
        .expect("suite structure lowers");
    let bound = WorthUiBindingSemanticsLowerer::lower(&structured, snapshot)
        .expect("suite semantics lower");
    let identity_seeded = WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("suite identity seeds lower")
        .0;
    WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded).expect("suite artifact assembles")
}

pub(super) fn control_app(
    world_profile: UiGraphWorldProfile,
    operator_token: &str,
    nodes: usize,
    bounded: bool,
) -> WorthUiApp {
    multi_control_app(
        world_profile,
        operator_token,
        nodes,
        bounded,
        "allocation_suite_control.wui",
    )
}

pub(super) fn peer_app(
    world_profile: UiGraphWorldProfile,
    operator_token: &str,
    nodes: usize,
    bounded: bool,
) -> WorthUiApp {
    multi_control_app(
        world_profile,
        operator_token,
        nodes,
        bounded,
        "allocation_suite_peer.wui",
    )
}

fn multi_control_app(
    world_profile: UiGraphWorldProfile,
    operator_token: &str,
    nodes: usize,
    bounded: bool,
    module_path: &str,
) -> WorthUiApp {
    let mut module = WorthUiRustAuthoredArtifactInputModule::new(format!("app/{module_path}"));
    for index in 0..nodes {
        let mut declaration = WorthUiSemanticArtifactDeclaration::new(
            UiDslSemanticKey::new(format!("planning.suite.node.{index}")),
            UiDslSemanticFamily::Control,
        )
        .with_structural_token(UiDslStructuralToken::new("control:primary"))
        .with_structural_token(UiDslStructuralToken::new("slot:footer"))
        .with_structural_token(UiDslStructuralToken::new(operator_token))
        .with_posture_token(UiDslPostureToken::new("touch:press"));
        if bounded {
            declaration = declaration
                .with_posture_token(UiDslPostureToken::new("measurement:constraint:bounded"));
        }
        module = module.with_semantic_declaration(declaration);
    }
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_rust_authored_input(WorthUiRustAuthoredArtifactInput::from_modules([module]))
        .freeze()
        .expect("application preparation should succeed")
}

pub(super) fn declaration_identity_for(
    app: &WorthUiApp,
    index: usize,
) -> crate::declaration::UiDeclarationIdentity {
    app.declaration_artifacts()[index].identity().clone()
}

pub(super) fn graph_node_identity_for_provenance(
    app: &WorthUiApp,
    declaration_index: usize,
) -> UiGraphNodeIdentity {
    let artifact = app
        .declaration_artifacts()
        .iter()
        .find(|artifact| {
            artifact
                .provenance()
                .source_provenance()
                .declaration_index()
                == declaration_index
        })
        .expect("suite declaration artifact should exist");
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("suite declaration should project one node")
}

pub(super) fn snapshot_with_admitted_layout(
    app: &WorthUiApp,
    admitted_nodes: &[UiGraphNodeIdentity],
) -> UiGraphSnapshot {
    crate::certification_support::layout_admission::snapshot_after_layout_admission_support(
        app,
        admitted_nodes,
    )
}

pub(super) fn structural_touch_for_node(
    app: &WorthUiApp,
    graph_node_identity: UiGraphNodeIdentity,
) -> crate::obligations::touch::UiGraphTouchDescriptor {
    let declaration_identity = app
        .graph()
        .lookup()
        .graph_node(graph_node_identity)
        .expect("certification node remains in graph authority")
        .value()
        .declaration_identity()
        .clone();
    let artifact = app
        .declaration_artifacts()
        .iter()
        .find(|artifact| artifact.identity() == &declaration_identity)
        .expect("certification node retains declaration authority");
    let origin = app
        .graph()
        .touches()
        .declaration_change_receipt(artifact)
        .expect("certification declaration admits structural change authority");
    app.graph()
        .touches()
        .from_node(
            origin,
            crate::obligations::touch::UiGraphTouchTiming::PostMutation,
            graph_node_identity,
            crate::obligations::touch::UiGraphTouchAspects::new()
                .structural(crate::obligations::touch::UiGraphTouchAspectPosture::Invalidated),
        )
        .expect("certification structural change admits a graph touch")
}
