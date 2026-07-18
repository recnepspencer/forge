use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, host_result_viewport_extent, viewport_extent_policy,
};
use crate::evidence::{admit_measurement_basis, MeasurementEvidenceInput};
use crate::facade::{
    WorthUi, WorthUiActiveApplicationSession, WorthUiApp, WorthUiPreparedApplicationReplacement,
};
use crate::runtime::tests::source_ingress_boundary_test_support::{
    lower_file_submission, source_backed_package_component, source_backed_package_region,
    source_backed_package_sizing,
};
use crate::runtime::{
    WorthUiSourceProvider, WorthUiWatchedCandidateSubmission, WorthUiWatcherEvent,
};

pub(crate) fn source_backed_component_session() -> WorthUiActiveApplicationSession {
    source_backed_component_app()
        .launch()
        .expect("component source application should launch")
}

pub(crate) fn source_backed_component_session_with_host<Adapter>(
    adapter: Adapter,
) -> WorthUiActiveApplicationSession
where
    Adapter: crate::facade::host_observation::WorthUiOperationalHostAdapter + 'static,
{
    source_backed_component_app_with_host(adapter)
        .launch()
        .expect("component source application should launch with configured host")
}

pub(crate) fn source_backed_component_app() -> WorthUiApp {
    let builder = component_builder();
    let snapshot = component_builder()
        .freeze()
        .expect("component snapshot should prepare");
    builder
        .with_candidate_submission(component_submission(
            "active-session-current",
            "workspace.component.active_session_current",
            snapshot.capabilities(),
        ))
        .freeze()
        .expect("component source application should prepare")
}

fn source_backed_component_app_with_host<Adapter>(adapter: Adapter) -> WorthUiApp
where
    Adapter: crate::facade::host_observation::WorthUiOperationalHostAdapter + 'static,
{
    let builder = component_builder().with_host(adapter);
    let snapshot = component_builder()
        .freeze()
        .expect("component snapshot should prepare");
    builder
        .with_candidate_submission(component_submission(
            "active-session-host-current",
            "workspace.component.active_session_current",
            snapshot.capabilities(),
        ))
        .freeze()
        .expect("component source application should prepare with configured host")
}

pub(crate) fn component_candidate_submission(
    session: &WorthUiActiveApplicationSession,
    source_name: &str,
    component_id: &str,
) -> WorthUiWatchedCandidateSubmission {
    component_submission(source_name, component_id, session.capabilities())
}

pub(crate) fn admit_candidate_catalog(
    prepared: &mut WorthUiPreparedApplicationReplacement,
) -> crate::graph::UiAdmittedAllocationCatalogBasisSet {
    let report = capability_report(77);
    let world_profile = prepared.candidate_graph().world_profile().clone();
    let candidate_inputs = prepared
        .candidate_graph()
        .node_identities()
        .map(|node| {
            let declaration = prepared
                .candidate_graph()
                .lookup()
                .graph_node(node)
                .expect("candidate node remains graph-addressable")
                .value()
                .declaration_identity()
                .clone();
            let touch = prepared
                .try_candidate_query_touch_for_node(node)
                .expect("query touch should admit before mounted transition commit");
            let selected = prepared.candidate_admission().select_obligations(&touch);
            let prior = prepared
                .candidate_graph()
                .lookup()
                .graph_node(node)
                .expect("candidate node remains graph-addressable")
                .value()
                .participation_posture()
                .axis(crate::graph::UiGraphParticipationAxis::Mounted);
            let transition = prepared
                .candidate_graph()
                .mounted_receipt_transition_for_node(
                    node,
                    prior,
                    crate::graph::UiGraphAxisParticipation::runtime_mutation(
                        crate::graph::UiGraphParticipationStatus::Admitted,
                    ),
                )
                .expect("candidate graph should mint its mounted transition");
            (declaration, node, selected, transition)
        })
        .collect::<Vec<_>>();
    prepared
        .commit_candidate_mounted_layout_admissions(
            candidate_inputs
                .iter()
                .map(|(_, _, _, transition)| *transition)
                .collect(),
        )
        .expect("candidate-mounted proof should admit layout participation");
    let generation =
        UiEvidenceAuthorityGeneration::new(prepared.candidate_graph().generation().as_u64());
    let entries = candidate_inputs
        .into_iter()
        .enumerate()
        .map(|(ordinal, (declaration, node, selected, _))| {
            let viewport = host_result_viewport_extent(9_000 + ordinal as u64, &report, generation);
            let basis = admit_measurement_basis(
                declaration,
                node,
                world_profile.clone(),
                generation,
                &viewport_extent_policy(),
                &[
                    MeasurementEvidenceInput::host_capability_report(&report),
                    MeasurementEvidenceInput::host_measurement_result(&viewport),
                ],
            );
            (basis, selected)
        })
        .collect();
    prepared
        .admit_candidate_allocation_catalog(entries)
        .expect("candidate graph should admit its complete allocation catalog")
}

fn component_builder() -> crate::facade::entry::WorthUiBuilder {
    let (_, _, world_profile) =
        crate::evidence::measurement::projection::fact_test_support::display_field_projection_context(
            "active-application-session",
        );
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .register_component(source_backed_package_component(
            "workspace.component.active_session_current",
        ))
        .register_component(source_backed_package_component(
            "workspace.component.active_session_candidate",
        ))
        .register_mosaic_region_kind(source_backed_package_region())
        .register_mosaic_sizing_contract(source_backed_package_sizing())
}

fn component_submission(
    source_name: &str,
    component_id: &str,
    capabilities: &crate::capability::CapabilitySnapshot,
) -> WorthUiWatchedCandidateSubmission {
    lower_file_submission(
        WorthUiSourceProvider::in_memory(source_name).with_file(
            "app/main.wui",
            format!(
                "component {component_id} {{ region workspace.region.primary {{ sizing workspace.sizing.mosaic_support; }} }}"
            ),
        ),
        [WorthUiWatcherEvent::provider_revision(source_name)],
        capabilities,
    )
}
