use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, host_result_viewport_extent_with_value, viewport_extent_policy,
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

pub(crate) fn source_backed_scaled_component_session(
    unrelated_component_count: usize,
) -> WorthUiActiveApplicationSession {
    source_backed_scaled_component_app(unrelated_component_count)
        .launch()
        .expect("scaled component source application should launch")
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

pub(crate) fn scaled_component_candidate_submission(
    session: &WorthUiActiveApplicationSession,
    source_name: &str,
    unrelated_component_count: usize,
) -> WorthUiWatchedCandidateSubmission {
    scaled_component_submission(
        source_name,
        "workspace.component.scale_target_candidate",
        unrelated_component_count,
        session.capabilities(),
    )
}

pub(crate) fn admit_candidate_catalog(
    prepared: &mut WorthUiPreparedApplicationReplacement,
) -> crate::graph::UiAdmittedAllocationCatalogDelta {
    admit_candidate_catalog_with_viewport_width(prepared, 100.0)
}

pub(crate) fn admit_candidate_catalog_with_viewport_width(
    prepared: &mut WorthUiPreparedApplicationReplacement,
    viewport_width: f32,
) -> crate::graph::UiAdmittedAllocationCatalogDelta {
    let partition = candidate_catalog_partition(prepared, viewport_width);
    prepared
        .admit_candidate_allocation_catalog_delta(partition, vec![])
        .expect("candidate graph should admit its allocation successor rows")
}

pub(crate) fn admit_first_candidate_catalog_row_with_viewport_width(
    prepared: &mut WorthUiPreparedApplicationReplacement,
    viewport_width: f32,
) -> crate::graph::UiAdmittedAllocationCatalogDelta {
    let first = candidate_catalog_partition(prepared, viewport_width)
        .into_iter()
        .next()
        .expect("candidate graph has an allocation row");
    prepared
        .admit_candidate_allocation_catalog_delta(vec![first], vec![])
        .expect("candidate graph admits one changed allocation successor row")
}

pub(crate) fn admit_candidate_complete_catalog(
    prepared: &mut WorthUiPreparedApplicationReplacement,
) -> crate::graph::UiAdmittedAllocationCatalogBasisSet {
    let partition = candidate_catalog_partition(prepared, 100.0);
    prepared
        .admit_candidate_allocation_catalog(partition)
        .expect("candidate graph should admit its complete allocation partition")
}

fn candidate_catalog_partition(
    prepared: &mut WorthUiPreparedApplicationReplacement,
    viewport_width: f32,
) -> Vec<(
    crate::evidence::UiMeasurementBasis,
    crate::obligations::selection::UiSelectedObligationSet,
)> {
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
                .expect("candidate node should admit query touch before mounted commit");
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
    let allocation_roots = prepared
        .candidate_graph()
        .allocation_planning_node_identities()
        .collect::<std::collections::BTreeSet<_>>();
    let generation =
        UiEvidenceAuthorityGeneration::new(prepared.candidate_graph().generation().as_u64());
    let entries: Vec<_> = candidate_inputs
        .into_iter()
        .filter(|(_, node, _, _)| allocation_roots.contains(node))
        .enumerate()
        .map(|(ordinal, (declaration, node, selected, _))| {
            let viewport = host_result_viewport_extent_with_value(
                9_000 + ordinal as u64,
                &report,
                generation,
                viewport_width,
                50.0,
            );
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
    let mut uncovered = prepared
        .candidate_graph()
        .allocation_planning_node_identities()
        .collect::<std::collections::BTreeSet<_>>();
    let mut remaining = entries;
    let mut partition = Vec::new();
    while !uncovered.is_empty() {
        let chosen = remaining
            .iter()
            .enumerate()
            .filter_map(|(index, (basis, selected))| {
                let neighborhood = basis
                    .admit_allocation_neighborhood(prepared.candidate_graph().snapshot(), selected)
                    .ok()?;
                let covered = neighborhood
                    .members()
                    .iter()
                    .map(|member| member.graph_node_identity())
                    .collect::<std::collections::BTreeSet<_>>();
                covered
                    .iter()
                    .all(|identity| uncovered.contains(identity))
                    .then_some((index, covered))
            })
            .max_by_key(|(_, covered)| covered.len())
            .expect("candidate allocation neighborhoods should have an exact disjoint cover");
        for identity in chosen.1 {
            uncovered.remove(&identity);
        }
        partition.push(remaining.swap_remove(chosen.0));
    }
    partition
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
        .register_theme_token(crate::capability::ThemeTokenDescriptor::define(
            crate::capability::ThemeTokenId::new("theme.removal_only")
                .expect("removal-only fixture token id is valid"),
            crate::capability::ThemeTokenFamily::text(),
            crate::capability::ThemeTokenSource::application(),
            crate::capability::ThemeTokenValue::color(
                crate::capability::ThemeColorValue::hex("#101820")
                    .expect("removal-only fixture token color is valid"),
            ),
        ))
        .register_mosaic_region_kind(source_backed_package_region())
        .register_mosaic_sizing_contract(source_backed_package_sizing())
}

fn source_backed_scaled_component_app(unrelated_component_count: usize) -> WorthUiApp {
    let builder = scaled_component_builder(unrelated_component_count);
    let snapshot = scaled_component_builder(unrelated_component_count)
        .freeze()
        .expect("scaled component snapshot should prepare");
    builder
        .with_candidate_submission(scaled_component_submission(
            "scaled-active-session-current",
            "workspace.component.scale_target_active",
            unrelated_component_count,
            snapshot.capabilities(),
        ))
        .freeze()
        .expect("scaled component source application should prepare")
}

fn scaled_component_builder(
    unrelated_component_count: usize,
) -> crate::facade::entry::WorthUiBuilder {
    let (_, _, world_profile) =
        crate::evidence::measurement::projection::fact_test_support::display_field_projection_context(
            "scaled-active-application-session",
        );
    let mut builder = WorthUi::app()
        .with_graph_world_profile(world_profile)
        .register_component(source_backed_package_component(
            "workspace.component.scale_target_active",
        ))
        .register_component(source_backed_package_component(
            "workspace.component.scale_target_candidate",
        ))
        .register_mosaic_region_kind(source_backed_package_region())
        .register_mosaic_sizing_contract(source_backed_package_sizing());
    for index in 0..unrelated_component_count {
        builder = builder.register_theme_token(scaled_unrelated_token(index));
    }
    builder
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

fn scaled_component_submission(
    source_name: &str,
    target_component_id: &str,
    unrelated_component_count: usize,
    capabilities: &crate::capability::CapabilitySnapshot,
) -> WorthUiWatchedCandidateSubmission {
    let mut declarations = vec![component_declaration(target_component_id)];
    declarations.extend(
        (0..unrelated_component_count)
            .map(scaled_unrelated_token_id)
            .map(|identity| format!("token {identity} = \"{identity}\";")),
    );
    lower_file_submission(
        WorthUiSourceProvider::in_memory(source_name)
            .with_file("app/main.wui", declarations.join("\n")),
        [WorthUiWatcherEvent::provider_revision(source_name)],
        capabilities,
    )
}

fn component_declaration(component_id: &str) -> String {
    format!(
        "component {component_id} {{ region workspace.region.primary {{ sizing workspace.sizing.mosaic_support; }} }}"
    )
}

fn scaled_unrelated_token_id(index: usize) -> String {
    format!("theme.scale.unrelated_{index}")
}

fn scaled_unrelated_token(index: usize) -> crate::capability::ThemeTokenDescriptor {
    crate::capability::ThemeTokenDescriptor::define(
        crate::capability::ThemeTokenId::new(scaled_unrelated_token_id(index))
            .expect("scaled token id is valid"),
        crate::capability::ThemeTokenFamily::text(),
        crate::capability::ThemeTokenSource::application(),
        crate::capability::ThemeTokenValue::color(
            crate::capability::ThemeColorValue::hex("#101820")
                .expect("scaled token color is valid"),
        ),
    )
}
