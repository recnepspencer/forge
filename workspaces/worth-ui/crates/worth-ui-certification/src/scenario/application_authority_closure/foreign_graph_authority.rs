use worth_ui::facade::app::{WorthUiActiveApplicationSession, WorthUiApplicationCutoverDenial};
use worth_ui::facade::graph::UiGraphWorldDifferenceKind;

use super::authored_composition::candidate_file;
use super::candidate_catalog::admit_candidate_catalog;

pub(super) fn equal_visible_graph_evidence_cannot_cross_candidate_authority(
    session: &mut WorthUiActiveApplicationSession,
) -> bool {
    let active_generation = session.generation_identity().clone();
    let first = prepare_structural_candidate(session);
    let second = prepare_structural_candidate(session);
    let mut first = first;
    let mut second = second;

    let _origin_catalog = admit_candidate_catalog(session, &mut first);
    let foreign_catalog = admit_candidate_catalog(session, &mut second);
    let first_graph = first.candidate_graph();
    let second_graph = second.candidate_graph();
    assert_eq!(first_graph.node_count(), second_graph.node_count());
    assert_eq!(
        first_graph.compare_to(second_graph).kind(),
        UiGraphWorldDifferenceKind::SameWorldEquivalent,
        "the hostile pair must expose equivalent visible graph meaning"
    );

    let lowered = session
        .lower_prepared_replacement(*first)
        .expect("origin candidate should lower");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("origin candidate should stage");
    let boundary = session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_completion()
        .into_execution()
        .expect("empty turn should expose an activation boundary")
        .into_activation_boundary();
    let denial =
        match session.activate_prepared_replacement(pending, foreign_catalog, boundary, None) {
            Ok(_) => panic!("foreign graph authority must not cross equal visible evidence"),
            Err(denial) => denial,
        };
    assert_eq!(session.generation_identity(), &active_generation);
    matches!(
        denial,
        WorthUiApplicationCutoverDenial::PreparedApplicationGraphMismatch
    )
}

fn prepare_structural_candidate(
    session: &WorthUiActiveApplicationSession,
) -> Box<worth_ui::facade::app::WorthUiPreparedApplicationReplacement> {
    session
        .prepare_replacement(candidate_file(session.capabilities()))
        .expect("structural candidate should prepare")
}
