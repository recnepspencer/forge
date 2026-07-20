use worth_ui::facade::app::WorthUiPreparedApplicationReplacement;
use worth_ui::facade::graph::{
    UiGraphAxisParticipation, UiGraphParticipationAxis, UiGraphParticipationStatus,
};
use worth_ui::facade::host::{
    WorthUiHeadlessHost, WorthUiHostCapabilityReport, WorthUiHostContract,
};
use worth_ui::facade::inspection::UiEvidenceAuthorityGeneration;
use worth_ui_runtime::facade::evidence::{
    certify_measurement_basis_determinism_for_scenarios, UiMeasurementBasisCertificationScenario,
};

pub fn admit_candidate_catalog(
    prepared: &mut WorthUiPreparedApplicationReplacement,
) -> worth_ui::facade::graph::UiAdmittedAllocationCatalogDelta {
    let candidates = prepared
        .candidate_graph()
        .node_identities()
        .filter_map(|node| {
            let graph = prepared.candidate_graph();
            let record = graph
                .lookup()
                .graph_node(node)
                .expect("allocation node should remain graph-addressable");
            let declaration = record.value().declaration_identity().clone();
            let is_source_composition_root = prepared
                .candidate_declaration_artifacts()
                .iter()
                .find(|artifact| artifact.identity() == &declaration)
                .and_then(|artifact| artifact.graph_handoff().ok())
                .is_some_and(|handoff| handoff.mosaic_sizing_contract_id().is_some());
            if !is_source_composition_root {
                return None;
            }
            let touch = prepared
                .try_candidate_query_touch_for_node(node)
                .expect("candidate graph should mint its query touch");
            let selected = prepared.candidate_admission().select_obligations(&touch);
            let prior = record
                .value()
                .participation_posture()
                .axis(UiGraphParticipationAxis::Mounted);
            let transition = graph
                .mounted_receipt_transition_for_node(
                    node,
                    prior,
                    UiGraphAxisParticipation::runtime_mutation(
                        UiGraphParticipationStatus::Admitted,
                    ),
                )
                .expect("candidate graph should mint mounted transition");
            Some((declaration, node, selected, transition))
        })
        .collect::<Vec<_>>();
    assert_eq!(
        candidates.len(),
        1,
        "one authored composition root should own mounted layout admission"
    );

    prepared
        .commit_candidate_mounted_layout_admissions(
            candidates
                .iter()
                .map(|(_, _, _, transition)| *transition)
                .collect(),
        )
        .expect("candidate-owned mounted transitions should commit");

    let graph = prepared.candidate_graph();
    let world = graph.world_profile().clone();
    let planning_nodes = graph
        .allocation_planning_node_identities()
        .collect::<std::collections::BTreeSet<_>>();
    assert!(
        !planning_nodes.is_empty(),
        "mounted candidate should expose allocation-planning nodes"
    );
    let generation =
        UiEvidenceAuthorityGeneration::new(prepared.candidate_graph().generation().as_u64());
    let report = WorthUiHostCapabilityReport::from_contract(WorthUiHostContract::headless());
    let entries = candidates
        .into_iter()
        .filter(|(_, node, _, _)| planning_nodes.contains(node))
        .map(|(declaration, node, selected, _)| {
            let artifact = prepared
                .candidate_declaration_artifacts()
                .iter()
                .find(|artifact| artifact.identity() == &declaration)
                .expect("graph declaration should remain in candidate authority");
            let handoff = artifact
                .graph_handoff()
                .expect("candidate declaration should preserve its graph handoff");
            let policy = handoff
                .measurement_policy()
                .admitted()
                .unwrap_or_else(|| {
                    panic!(
                        "allocation-planning declaration {:?} should carry measurement policy; sizing={:?}, modifier={:?}",
                        declaration,
                        handoff.mosaic_sizing_contract_id(),
                        graph
                            .lookup()
                            .graph_node(node)
                            .expect("planning node should remain graph-addressable")
                            .value()
                            .measurement_constraint_modifier(),
                    )
                })
                .clone();
            let scenario = UiMeasurementBasisCertificationScenario::new(
                declaration,
                node,
                world.clone(),
                generation,
                policy,
                report.clone(),
            );
            let proof = certify_measurement_basis_determinism_for_scenarios(
                &scenario,
                &WorthUiHeadlessHost,
                &scenario,
                &WorthUiHeadlessHost,
            )
            .expect("candidate measurement basis should materialize through certification lane");
            (proof.first_basis().clone(), selected)
        })
        .collect();

    prepared
        .admit_candidate_allocation_catalog_delta(entries, vec![])
        .expect("candidate graph should admit changed allocation coverage")
}
