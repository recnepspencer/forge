use std::collections::BTreeSet;

use worth_ui::facade::app::WorthUiPreparedApplicationReplacement;
use worth_ui::facade::graph::{
    UiGraphAxisParticipation, UiGraphParticipationAxis, UiGraphParticipationStatus,
};
use worth_ui::facade::inspection::UiEvidenceAuthorityGeneration;
use worth_ui_host_contract::{
    UiMeasurementEvidenceFamily, UiMeasurementRequestIdentity, UiPortalAnchorRectRequest,
    UiViewportExtentRequest,
};
use worth_ui_runtime::facade::evidence::{
    certify_measurement_basis_determinism_for_active_host,
    UiMeasurementBasisCertificationHostRequest, UiMeasurementBasisCertificationScenario,
};
use worth_ui_runtime::facade::host::UiPortalAnchorCoordinateSpacePosture;
use worth_ui_runtime::facade::host::{
    UiHostMeasurementAssumptionProfile, UiHostMeasurementNeed,
    UiHostMeasurementNormalizationContext,
};
use worth_ui_test_support::{
    WorthUiActiveSessionCertificationExt, WorthUiApplicationReplacementCertificationExt,
};

pub fn admit_candidate_catalog(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    prepared: &mut WorthUiPreparedApplicationReplacement,
) -> worth_ui::facade::graph::UiAdmittedAllocationCatalogDelta {
    admit_catalog(session, prepared, Vec::new())
}

pub fn admit_candidate_catalog_with_removed_roots(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    prepared: &mut WorthUiPreparedApplicationReplacement,
    removed_roots: Vec<worth_ui::facade::graph::UiGraphNodeIdentity>,
) -> worth_ui::facade::graph::UiAdmittedAllocationCatalogDelta {
    admit_catalog(session, prepared, removed_roots)
}

fn admit_catalog(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    prepared: &mut WorthUiPreparedApplicationReplacement,
    removed_roots: Vec<worth_ui::facade::graph::UiGraphNodeIdentity>,
) -> worth_ui::facade::graph::UiAdmittedAllocationCatalogDelta {
    let candidates = candidate_inputs(prepared);
    prepared
        .commit_candidate_mount_eligibility_admissions(
            candidates
                .iter()
                .map(|(_, _, _, transition)| *transition)
                .collect(),
        )
        .expect("candidate-owned mounted transitions should commit");

    let planning_nodes = candidates
        .iter()
        .map(|(_, node, _, _)| *node)
        .collect::<BTreeSet<_>>();
    assert!(
        !planning_nodes.is_empty(),
        "mounted candidate should expose allocation-planning nodes"
    );
    let entries = measurement_entries(session, prepared, candidates);
    let partition = disjoint_partition(prepared, entries, planning_nodes);
    prepared
        .admit_candidate_allocation_catalog_delta(partition, removed_roots)
        .expect("candidate graph should admit changed allocation coverage")
}

fn candidate_inputs(
    prepared: &WorthUiPreparedApplicationReplacement,
) -> Vec<(
    worth_ui::facade::declaration::UiDeclarationIdentity,
    worth_ui::facade::graph::UiGraphNodeIdentity,
    worth_ui_runtime::facade::obligations::UiSelectedObligationSet,
    worth_ui::facade::graph::UiGraphMountEligibilityTransition,
)> {
    prepared
        .candidate_graph()
        .node_identities()
        .filter_map(|node| {
            let graph = prepared.candidate_graph();
            let record = graph
                .lookup()
                .graph_node(node)
                .expect("candidate node should remain graph-addressable");
            let declaration = record.value().declaration_identity().clone();
            let owns_measurement_policy = prepared
                .candidate_declaration_artifacts()
                .iter()
                .find(|artifact| artifact.identity() == &declaration)
                .and_then(|artifact| artifact.graph_handoff().ok())
                .is_some_and(|handoff| handoff.mosaic_sizing_contract_id().is_some());
            if !owns_measurement_policy {
                return None;
            }
            let touch = prepared
                .candidate_allocation_touch_for_node(node)
                .expect("candidate graph should mint its allocation touch");
            let selected = prepared.candidate_admission().select_obligations(&touch);
            let prior = record
                .value()
                .participation_posture()
                .axis(UiGraphParticipationAxis::Mounted);
            let transition = graph
                .mount_eligibility_transition_for_node(
                    node,
                    prior,
                    UiGraphAxisParticipation::runtime_mutation(
                        UiGraphParticipationStatus::Admitted,
                    ),
                )
                .expect("candidate graph should mint mounted transition");
            Some((declaration, node, selected, transition))
        })
        .collect()
}

fn measurement_entries(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    prepared: &WorthUiPreparedApplicationReplacement,
    candidates: Vec<(
        worth_ui::facade::declaration::UiDeclarationIdentity,
        worth_ui::facade::graph::UiGraphNodeIdentity,
        worth_ui_runtime::facade::obligations::UiSelectedObligationSet,
        worth_ui::facade::graph::UiGraphMountEligibilityTransition,
    )>,
) -> Vec<(
    worth_ui_runtime::facade::evidence::UiMeasurementBasis,
    worth_ui_runtime::facade::obligations::UiSelectedObligationSet,
)> {
    let world = prepared.candidate_graph().world_profile().clone();
    let generation =
        UiEvidenceAuthorityGeneration::new(prepared.candidate_graph().generation().as_u64());
    let capability = session.host_measurement_capability();
    let report = capability.capability_report().clone();
    let profile = UiHostMeasurementAssumptionProfile::from_capability_report(&report, 1, 2, 3, 4);

    candidates
        .into_iter()
        .enumerate()
        .map(|(ordinal, (declaration, node, selected, _))| {
            let artifact = prepared
                .candidate_declaration_artifacts()
                .iter()
                .find(|artifact| artifact.identity() == &declaration)
                .expect("candidate graph declaration should remain authored");
            let policy = artifact
                .graph_handoff()
                .expect("composition root should preserve its graph handoff")
                .measurement_policy()
                .admitted()
                .cloned()
                .expect("allocation composition root should carry authored measurement policy");
            let uses_viewport_extent = policy.requires_viewport_extent_observation();
            let uses_portal_anchor = policy.requires_portal_anchor_observation();
            let mut scenario = UiMeasurementBasisCertificationScenario::new(
                declaration,
                node,
                world.clone(),
                generation,
                policy,
                report.clone(),
            );
            if uses_viewport_extent {
                scenario =
                    scenario.with_host_requests([UiMeasurementBasisCertificationHostRequest::new(
                        UiMeasurementRequestIdentity::new(9_000 + ordinal as u64),
                        UiMeasurementEvidenceFamily::ViewportExtent,
                        UiHostMeasurementNeed::ViewportExtent(UiViewportExtentRequest),
                        UiHostMeasurementNormalizationContext::viewport_logical_exact(profile),
                    )]);
            } else if uses_portal_anchor {
                scenario =
                    scenario.with_host_requests([UiMeasurementBasisCertificationHostRequest::new(
                        UiMeasurementRequestIdentity::new(9_000 + ordinal as u64),
                        UiMeasurementEvidenceFamily::PortalAnchorRect,
                        UiHostMeasurementNeed::PortalAnchorRect(UiPortalAnchorRectRequest::new(
                            ordinal as u64 + 1,
                        )),
                        UiHostMeasurementNormalizationContext::portal_anchor_logical_exact_in(
                            UiPortalAnchorCoordinateSpacePosture::PortalLayer,
                            profile,
                        ),
                    )]);
            }
            let proof =
                certify_measurement_basis_determinism_for_active_host(&scenario, &capability)
                    .expect("candidate measurement basis should use the real host evidence lane");
            (proof.first_basis().clone(), selected)
        })
        .collect()
}

fn disjoint_partition(
    prepared: &WorthUiPreparedApplicationReplacement,
    mut remaining: Vec<(
        worth_ui_runtime::facade::evidence::UiMeasurementBasis,
        worth_ui_runtime::facade::obligations::UiSelectedObligationSet,
    )>,
    mut uncovered: BTreeSet<worth_ui::facade::graph::UiGraphNodeIdentity>,
) -> Vec<(
    worth_ui_runtime::facade::evidence::UiMeasurementBasis,
    worth_ui_runtime::facade::obligations::UiSelectedObligationSet,
)> {
    let mut partition = Vec::new();
    while !uncovered.is_empty() {
        let chosen = remaining
            .iter()
            .enumerate()
            .filter_map(|(index, (basis, selected))| {
                let neighborhood = prepared
                    .admit_candidate_allocation_neighborhood(basis, selected)
                    .ok()?;
                let covered = neighborhood
                    .members()
                    .iter()
                    .map(|member| member.graph_node_identity())
                    .collect::<BTreeSet<_>>();
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
