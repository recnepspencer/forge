use forge_query::facade::consumer_kit::{
    graph_obligation_consumer_kit, ForgeQueryBoundaryAuditSourceSet,
    ForgeQueryGraphObligationConsumerKitError,
    ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ForgeQueryGraphObligationExecutionBackedAdoptionProof,
    ForgeQueryGraphObligationLocalCeremonyAudit, ForgeQueryGraphObligationResidueManifest,
    ForgeQueryGraphObligationSelectorCoverageDeclaration, ForgeQueryGraphObligationSupportPin,
};
use forge_query::facade::runtime::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationRegistration, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportMatrix, ForgeQueryGraphObligationSupportMatrixRow,
    ForgeQueryGraphObligationSupportStatus,
};

use super::execution::{
    composition_context_registrations, composition_participation_registrations,
    composition_topology_registrations, live_view_state_binding_registrations,
    mounted_interaction_registrations, primitive_construction_registrations,
    primitive_content_anatomy_registrations, primitive_event_dispatch_registrations,
};
use super::{
    WorthUiLiveViewStateBindingGraphPosture, WorthUiPrimitiveContentGraphPosture,
    WorthUiPrimitiveEventGraphDispatchPosture, WorthUiQueryGraphTouchDescriptor,
};

pub type WorthUiQueryGraphAdoptionProof = ForgeQueryGraphObligationExecutionBackedAdoptionProof;

pub fn primitive_construction_adoption_proof(
) -> Result<WorthUiQueryGraphAdoptionProof, ForgeQueryGraphObligationConsumerKitError> {
    let touch = WorthUiQueryGraphTouchDescriptor::primitive_construction(
        "worth.surface.query_graph.primitive_construction.adoption",
        [
            crate::runtime::WorthUiRuntimeFactId::primitive_construction(
                "worth.surface.query_graph.primitive_construction.adoption",
            ),
        ],
    );
    prove_lane_adoption(
        "worth-ui-primitive-construction",
        "worth-ui-primitive-construction",
        touch,
        primitive_construction_registrations(),
    )
}

pub fn composition_topology_adoption_proof(
) -> Result<WorthUiQueryGraphAdoptionProof, ForgeQueryGraphObligationConsumerKitError> {
    let root = "composition.root.surface.worth.surface.query_graph.composition.adoption";
    let touch = WorthUiQueryGraphTouchDescriptor::composition_topology(
        root,
        [
            crate::runtime::WorthUiRuntimeFactId::composition_root(root),
            crate::runtime::WorthUiRuntimeFactId::composition_node(
                "composition.node.query_graph.adoption",
            ),
            crate::runtime::WorthUiRuntimeFactId::composition_edge(
                "composition.edge.query_graph.adoption",
            ),
        ],
    );
    prove_lane_adoption(
        "worth-ui-composition-topology",
        "worth-ui-composition-topology",
        touch,
        composition_topology_registrations(),
    )
}

pub fn composition_context_adoption_proof(
) -> Result<WorthUiQueryGraphAdoptionProof, ForgeQueryGraphObligationConsumerKitError> {
    let root = "composition.root.surface.worth.surface.query_graph.context.adoption";
    let touch = WorthUiQueryGraphTouchDescriptor::composition_context_propagation(
        root,
        [
            crate::runtime::WorthUiRuntimeFactId::composition_root(root),
            crate::runtime::WorthUiRuntimeFactId::composition_context("composition.node.context"),
            crate::runtime::WorthUiRuntimeFactId::composition_context_propagation(root),
        ],
    );
    prove_lane_adoption(
        "worth-ui-composition-context",
        "worth-ui-composition-context",
        touch,
        composition_context_registrations(),
    )
}

pub fn composition_participation_adoption_proof(
) -> Result<WorthUiQueryGraphAdoptionProof, ForgeQueryGraphObligationConsumerKitError> {
    let root = "composition.root.surface.worth.surface.query_graph.participation.adoption";
    let touch = WorthUiQueryGraphTouchDescriptor::composition_participation(
        root,
        [
            crate::runtime::WorthUiRuntimeFactId::composition_root(root),
            crate::runtime::WorthUiRuntimeFactId::composition_participation(root),
        ],
    );
    prove_lane_adoption(
        "worth-ui-composition-participation",
        "worth-ui-composition-participation",
        touch,
        composition_participation_registrations(),
    )
}

pub fn mounted_interaction_adoption_proof(
) -> Result<WorthUiQueryGraphAdoptionProof, ForgeQueryGraphObligationConsumerKitError> {
    let touch = WorthUiQueryGraphTouchDescriptor::mounted_interaction_activation(
        &crate::capability::SurfaceId::new("worth.surface.query_graph.adoption")
            .expect("adoption fixture surface id is valid"),
        "worth.interaction.query-graph.adoption",
        [],
        crate::runtime::WorthUiInteractionOperabilityBasis::Enabled,
        crate::runtime::WorthUiInteractionReadiness::Enabled,
        crate::runtime::WorthUiInteractionKind::Submit,
        &crate::runtime::WorthUiInteractionTarget::Surface(
            "worth.surface.query_graph.adoption".to_owned(),
        ),
        crate::runtime::WorthUiPrimitiveFocusPosture::Focusable,
    );
    prove_lane_adoption(
        "worth-ui-mounted-interaction",
        "worth-ui-mounted-interaction-activation",
        touch,
        mounted_interaction_registrations(),
    )
}

pub fn primitive_event_dispatch_adoption_proof(
) -> Result<WorthUiQueryGraphAdoptionProof, ForgeQueryGraphObligationConsumerKitError> {
    let touch = WorthUiQueryGraphTouchDescriptor::primitive_event_dispatch(
        "worth.surface.query_graph.event_dispatch.adoption",
        "worth.interaction.query_graph.event_dispatch.adoption",
        [
            crate::runtime::WorthUiRuntimeFactId::primitive_event_region(
                "worth.surface.query_graph.event_dispatch.adoption",
            ),
        ],
        WorthUiPrimitiveEventGraphDispatchPosture::EnabledHit,
    );
    prove_lane_adoption(
        "worth-ui-primitive-event-dispatch",
        "worth-ui-primitive-event-dispatch",
        touch,
        primitive_event_dispatch_registrations(),
    )
}

#[allow(dead_code)]
pub fn primitive_content_anatomy_adoption_proof(
) -> Result<WorthUiQueryGraphAdoptionProof, ForgeQueryGraphObligationConsumerKitError> {
    let surface = "worth.surface.query_graph.content_anatomy.adoption";
    let touch = WorthUiQueryGraphTouchDescriptor::primitive_content_anatomy(
        surface,
        [
            crate::runtime::WorthUiRuntimeFactId::primitive_content(surface),
            crate::runtime::WorthUiRuntimeFactId::density_token(
                &crate::capability::DensityTokenId::new(
                    "validation.density.primitive.content.icon.default",
                )
                .expect("adoption fixture density token id is valid"),
            ),
        ],
        WorthUiPrimitiveContentGraphPosture::NativeVector,
    );
    prove_lane_adoption(
        "worth-ui-primitive-content",
        "worth-ui-primitive-content-anatomy",
        touch,
        primitive_content_anatomy_registrations(),
    )
}

#[allow(dead_code)]
pub fn live_view_state_binding_adoption_proof(
) -> Result<WorthUiQueryGraphAdoptionProof, ForgeQueryGraphObligationConsumerKitError> {
    let live_view = "validation.live_view.query_graph.adoption";
    let touch = WorthUiQueryGraphTouchDescriptor::live_view_state_binding(
        live_view,
        17,
        [
            crate::runtime::WorthUiRuntimeFactId::live_view_declaration(live_view),
            crate::runtime::WorthUiRuntimeFactId::live_view_state_binding(
                "validation.live_view.query_graph.adoption:first_name",
            ),
            crate::runtime::WorthUiRuntimeFactId::live_view_state_value(
                "validation.state.contact.first_name",
            ),
        ],
        WorthUiLiveViewStateBindingGraphPosture::Admitted,
    );
    prove_lane_adoption(
        "worth-ui-live-view-state-binding",
        "worth-ui-live-view-state-binding",
        touch,
        live_view_state_binding_registrations(),
    )
}

fn prove_lane_adoption(
    consumer_name: &'static str,
    runtime_family: &'static str,
    touch: WorthUiQueryGraphTouchDescriptor,
    registrations: Vec<(
        super::WorthUiQueryGraphObligationSemantic,
        ForgeQueryGraphObligationRegistration,
    )>,
) -> Result<WorthUiQueryGraphAdoptionProof, ForgeQueryGraphObligationConsumerKitError> {
    let registrations = registrations
        .into_iter()
        .map(|(_, registration)| registration)
        .collect::<Vec<_>>();
    let selector_coverage = ForgeQueryGraphObligationSelectorCoverageDeclaration::required(
        registrations
            .iter()
            .enumerate()
            .map(|(index, registration)| {
                (
                    format!("worth ui query graph operation selector {index}"),
                    registration.touch_selector().clone(),
                )
            }),
    );
    graph_obligation_consumer_kit(consumer_name)
        .register_obligations(
            ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                runtime_family,
                registrations,
            )?,
        )
        .declare_selector_coverage(selector_coverage)
        .pin_support(ForgeQueryGraphObligationSupportPin::supported(
            ForgeQueryGraphObligationKind::ALL
                .map(|kind| (kind, ForgeQueryGraphObligationSupportLane::PreviewIntent)),
        ))
        .against_support_matrix(worth_ui_support_matrix())
        .audit_local_ceremony(evaluated_clean_audit())
        .account_for_residue(ForgeQueryGraphObligationResidueManifest::empty())
        .prove_execution_with(
            touch.descriptor(),
            &ForgeQueryGraphObligationOperatingWorldDescriptor::preview(),
        )?
        .prove_adoption_with_execution()
}

fn worth_ui_support_matrix() -> ForgeQueryGraphObligationSupportMatrix {
    ForgeQueryGraphObligationSupportMatrix::new(
        ForgeQueryGraphObligationKind::ALL
            .into_iter()
            .map(|kind| {
                ForgeQueryGraphObligationSupportMatrixRow::new(
                    kind,
                    ForgeQueryGraphObligationSupportLane::PreviewIntent,
                    ForgeQueryGraphObligationSupportStatus::Supported,
                )
            })
            .collect(),
    )
}

fn evaluated_clean_audit() -> ForgeQueryGraphObligationLocalCeremonyAudit {
    ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(
        &ForgeQueryBoundaryAuditSourceSet::new("worth-ui").source(
            "crates/worth-ui/src/runtime/query_graph",
            "mounted interaction activation uses query consumer kit adoption",
        ),
    )
}
