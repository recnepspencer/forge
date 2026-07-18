use worth_ui_inspection::{UiEvidenceExpansionOutcome, UiEvidenceRichness};

use crate::facade::WorthUiApplicationReplacementPreparation;
use crate::runtime::tests::active_application_session_test_support::{
    admit_candidate_catalog, component_candidate_submission, source_backed_component_session,
};

#[test]
fn candidate_evidence_stays_isolated_until_successful_cutover() {
    let mut session = source_backed_component_session();
    let outcome = session
        .prepare_replacement(component_candidate_submission(
            &session,
            "active-session-candidate-evidence",
            "workspace.component.active_session_candidate",
        ))
        .expect("candidate application should prepare");
    let WorthUiApplicationReplacementPreparation::Prepared(mut prepared) = outcome else {
        panic!("structurally distinct candidate should not converge as a no-op");
    };
    let catalog = admit_candidate_catalog(&mut prepared);
    let candidate_node = prepared
        .candidate_graph()
        .node_identities()
        .next()
        .expect("candidate graph should contain a node");
    let candidate_ref = prepared
        .candidate_graph()
        .evidence_ref_for_node(candidate_node)
        .expect("candidate graph should derive its evidence ref");

    let candidate_expansion =
        prepared.expand_candidate_evidence_ref(candidate_ref, UiEvidenceRichness::summary());
    let active_expansion =
        session.expand_evidence_ref(candidate_ref, UiEvidenceRichness::summary());

    assert!(candidate_expansion.outcome().is_available());
    assert!(matches!(
        active_expansion.outcome(),
        UiEvidenceExpansionOutcome::WrongGeneration { .. }
    ));

    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("candidate application should lower");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("candidate application should stage");
    let boundary = session
        .execute_framework_turn(|_| {})
        .into_completion()
        .into_execution()
        .expect("empty framework turn should expose an activation boundary")
        .into_activation_boundary();
    session
        .activate_prepared_replacement(pending, catalog, boundary, None)
        .expect("candidate application should cut over");

    let active_expansion =
        session.expand_evidence_ref(candidate_ref, UiEvidenceRichness::summary());
    assert!(active_expansion.outcome().is_available());
}
