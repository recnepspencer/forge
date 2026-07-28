use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, host_result_viewport_extent_with_value, viewport_extent_policy,
};
use crate::evidence::{admit_measurement_basis, MeasurementEvidenceInput};
use crate::facade::WorthUiPreparedApplicationReplacement;

type CandidateCatalogEntry = (
    crate::evidence::UiMeasurementBasis,
    crate::obligations::selection::UiSelectedObligationSet,
);

struct CandidateCatalogSeed {
    declaration: crate::declaration::UiDeclarationIdentity,
    node: crate::graph::UiGraphNodeIdentity,
    selected: crate::obligations::selection::UiSelectedObligationSet,
    transition: crate::graph::UiGraphMountEligibilityTransition,
}

pub(crate) fn admit_candidate_catalog(
    prepared: &mut WorthUiPreparedApplicationReplacement,
) -> crate::graph::UiAdmittedAllocationCatalogDelta {
    admit_candidate_catalog_with_viewport_width(prepared, 100.0)
}

fn admit_candidate_catalog_with_viewport_width(
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
) -> Vec<CandidateCatalogEntry> {
    let seeds = admit_candidate_mount_eligibility(prepared);
    let entries = candidate_measurement_entries(prepared, seeds, viewport_width);
    exact_disjoint_catalog_partition(prepared, entries)
}

fn admit_candidate_mount_eligibility(
    prepared: &mut WorthUiPreparedApplicationReplacement,
) -> Vec<CandidateCatalogSeed> {
    let nodes = prepared
        .candidate_graph()
        .node_identities()
        .collect::<Vec<_>>();
    let seeds = nodes
        .into_iter()
        .map(|node| candidate_catalog_seed(prepared, node))
        .collect::<Vec<_>>();
    prepared
        .commit_candidate_mount_eligibility_admissions(
            seeds.iter().map(|seed| seed.transition).collect(),
        )
        .expect("candidate-mounted proof should admit layout participation");
    seeds
}

fn candidate_catalog_seed(
    prepared: &mut WorthUiPreparedApplicationReplacement,
    node: crate::graph::UiGraphNodeIdentity,
) -> CandidateCatalogSeed {
    let graph_node = prepared
        .candidate_graph()
        .lookup()
        .graph_node(node)
        .expect("candidate node remains graph-addressable")
        .value();
    let declaration = graph_node.declaration_identity().clone();
    let prior = graph_node
        .participation_posture()
        .axis(crate::graph::UiGraphParticipationAxis::Mounted);
    let touch = prepared
        .try_candidate_query_touch_for_node(node)
        .expect("candidate node should admit query touch before mounted commit");
    let selected = prepared.candidate_admission().select_obligations(&touch);
    let transition = prepared
        .candidate_graph()
        .mount_eligibility_transition_for_node(
            node,
            prior,
            crate::graph::UiGraphAxisParticipation::runtime_mutation(
                crate::graph::UiGraphParticipationStatus::Admitted,
            ),
        )
        .expect("candidate graph should mint its mounted transition");
    CandidateCatalogSeed {
        declaration,
        node,
        selected,
        transition,
    }
}

fn candidate_measurement_entries(
    prepared: &WorthUiPreparedApplicationReplacement,
    seeds: Vec<CandidateCatalogSeed>,
    viewport_width: f32,
) -> Vec<CandidateCatalogEntry> {
    let report = capability_report(77);
    let world_profile = prepared.candidate_graph().world_profile().clone();
    let allocation_roots = prepared
        .candidate_graph()
        .allocation_planning_node_identities()
        .collect::<std::collections::BTreeSet<_>>();
    let generation =
        UiEvidenceAuthorityGeneration::new(prepared.candidate_graph().generation().as_u64());
    seeds
        .into_iter()
        .filter(|seed| allocation_roots.contains(&seed.node))
        .enumerate()
        .map(|(ordinal, seed)| {
            let viewport = host_result_viewport_extent_with_value(
                9_000 + ordinal as u64,
                &report,
                generation,
                viewport_width,
                50.0,
            );
            let basis = admit_measurement_basis(
                seed.declaration,
                seed.node,
                world_profile.clone(),
                generation,
                &viewport_extent_policy(),
                &[
                    MeasurementEvidenceInput::host_capability_report(&report),
                    MeasurementEvidenceInput::host_measurement_result(&viewport),
                ],
            );
            (basis, seed.selected)
        })
        .collect()
}

fn exact_disjoint_catalog_partition(
    prepared: &WorthUiPreparedApplicationReplacement,
    entries: Vec<CandidateCatalogEntry>,
) -> Vec<CandidateCatalogEntry> {
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
