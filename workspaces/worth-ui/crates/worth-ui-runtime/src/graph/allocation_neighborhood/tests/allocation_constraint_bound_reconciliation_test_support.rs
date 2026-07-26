#![cfg(any(test, feature = "certification-support"))]

use crate::facade::WorthUiRustAuthoredDeclarationFixture;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};

use crate::declaration::{
    UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementConstraintModifier,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementOwnershipPosture,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::facade::WorthUi;
use crate::graph::{UiGraphNodeIdentity, UiGraphWorldProfile};

pub(crate) fn scroll_bound_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        Some(UiDeclaredMeasurementBasisSource::ScrollViewport),
        Some(UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis),
        vec![],
    )
    .expect("scroll bound policy should admit")
}

pub(crate) fn bounded_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        None,
        None,
        vec![],
    )
    .expect("bounded policy should admit")
}

pub(crate) fn peer_app(
    world_profile: UiGraphWorldProfile,
    operator_token: &str,
    bounded_flags: &[bool; 3],
) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.runtime.graph.allocation-constraint-bounds",
            )
            .with_semantic_artifact_spec(control_spec(
                "workflow_editor.root",
                0,
                operator_token,
                bounded_flags[0],
            ))
            .with_semantic_artifact_spec(control_spec(
                "workflow_editor.peer.a",
                1,
                operator_token,
                bounded_flags[1],
            ))
            .with_semantic_artifact_spec(control_spec(
                "workflow_editor.peer.b",
                2,
                operator_token,
                bounded_flags[2],
            )),
        )
        .freeze()
        .expect("application preparation should succeed")
}

pub(crate) fn graph_node_identity_for_provenance(
    app: &crate::facade::WorthUiApp,
    declaration_index: usize,
) -> UiGraphNodeIdentity {
    let artifact = app
        .declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == "app/allocation_constraint_bound_reconciliation_tests.wui"
                && provenance.declaration_index() == declaration_index
        })
        .expect("expected declaration artifact for requested provenance row");
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should project one graph node")
}

fn control_spec(
    semantic_key: &str,
    declaration_index: usize,
    operator_token: &str,
    bounded: bool,
) -> UiDslSemanticArtifactSpec {
    let spec = UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new(semantic_key),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored(
            "app/allocation_constraint_bound_reconciliation_tests.wui",
            declaration_index,
        ),
    )
    .with_structural_token(UiDslStructuralToken::new("control:primary"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_structural_token(UiDslStructuralToken::new(operator_token))
    .with_posture_token(UiDslPostureToken::new("touch:press"));
    if bounded {
        spec.with_posture_token(UiDslPostureToken::new("measurement:constraint:bounded"))
    } else {
        spec
    }
}
