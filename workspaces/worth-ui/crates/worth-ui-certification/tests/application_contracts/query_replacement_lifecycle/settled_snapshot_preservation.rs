use worth_query::facade::runtime;
use worth_ui::facade::app::WorthUiApplicationCutoverDenial;
use worth_ui_query_binding::{
    WorthUiInstalledQueryBindingReference, WorthUiQueryBindingSuccessionDenial,
    WorthUiQueryViewShape, WorthUiQueryWorkspaceExt, WorthUiSettledSnapshotFact,
    WorthUiSettledSnapshotProjection,
};
use worth_ui_runtime::facade::WorthUiAllocationCatalogActivationDenial;
use worth_ui_test_support::{
    WorthUiActiveSessionCertificationExt, WorthUiFrameworkTurnCertificationExt,
};

use super::scenario::{
    installed_workspace, snapshot_application, submission, FIRST_VIEW, NEXT_COMPONENT, SECOND_VIEW,
};
use super::support::{activate, lower_and_stage, prepare_catalog};
use crate::query_consumer_kit_workspace::interactive_borrowed_collection_requirements;

#[test]
fn public_changed_replacement_preserves_the_active_exact_settlement() {
    let mut workspace = installed_workspace("query-settlement-replacement-preserve");
    let installed = workspace.worth_ui().expect("Worth UI domain installed");
    let first = installed
        .measurement_view(FIRST_VIEW)
        .expect("first installed view");
    let second = installed
        .measurement_view(SECOND_VIEW)
        .expect("second installed view");
    let first_identity = first.definition().identity().clone();
    let app = snapshot_application(first, second, &mut workspace);
    let reference = app
        .resolve_query_view(&first_identity, WorthUiQueryViewShape::Collection)
        .expect("application retains the installed operation reference");
    let mut session = app.launch().expect("Query application launch");

    admit_active_settlement(
        &mut session,
        settle_snapshot(&reference, &mut workspace),
        false,
    );
    let predecessor = admit_active_settlement(
        &mut session,
        settle_snapshot(&reference, &mut workspace),
        true,
    );
    assert_coordinates(&predecessor, 2, 2);

    let mut candidate = prepare_catalog(
        &session,
        submission(
            "query-settlement-replacement-candidate",
            NEXT_COMPONENT,
            &[FIRST_VIEW],
            session.capabilities(),
        ),
    );
    candidate
        .0
        .admit_candidate_settled_query_projection(settle_snapshot(&reference, &mut workspace))
        .expect("candidate independently owns a complete settled projection");
    let pending = lower_and_stage(&session, candidate);
    let cutover = activate(&mut session, pending.0, pending.1);
    assert!(cutover.operation_live_retirement().is_empty());

    let successor_refresh = admit_active_settlement(
        &mut session,
        settle_snapshot(&reference, &mut workspace),
        true,
    );
    assert_coordinates(&successor_refresh, 3, 3);
    let _ = session.shutdown();
}

#[test]
fn installation_turnover_after_lowering_denies_before_publication() {
    let mut workspace = installed_workspace("query-settlement-replacement-stale");
    let installed = workspace.worth_ui().expect("Worth UI domain installed");
    let first = installed
        .measurement_view(FIRST_VIEW)
        .expect("first installed view");
    let second = installed
        .measurement_view(SECOND_VIEW)
        .expect("second installed view");
    let first_identity = first.definition().identity().clone();
    let app = snapshot_application(first, second, &mut workspace);
    let reference = app
        .resolve_query_view(&first_identity, WorthUiQueryViewShape::Collection)
        .expect("application retains the installed operation reference");
    let mut session = app.launch().expect("Query application launch");
    admit_active_settlement(
        &mut session,
        settle_snapshot(&reference, &mut workspace),
        false,
    );
    let active_generation = session.generation_identity().clone();

    let candidate = prepare_catalog(
        &session,
        submission(
            "query-settlement-replacement-stale-candidate",
            NEXT_COMPONENT,
            &[FIRST_VIEW],
            session.capabilities(),
        ),
    );
    let pending = lower_and_stage(&session, candidate);
    worth_query::facade::consumer_kit::advance_test_workspace_domain_installation_generation(
        &mut workspace,
    );
    assert!(
        !reference.installation_is_current(),
        "the Query test backend must make the retained application reference stale"
    );
    let stale_scan = session.inspect_query_state_residue();
    assert_eq!(stale_scan.stale_installed_reference_count(), 2);
    assert!(!stale_scan.is_clean());
    let boundary = super::support::activation_boundary(&mut session);
    let denial = match session.activate_prepared_replacement(pending.0, pending.1, boundary, None) {
        Err(denial) => denial,
        Ok(_) => panic!("stale Query authority cannot publish an application generation"),
    };

    assert!(matches!(
        denial,
        WorthUiApplicationCutoverDenial::Activation(
            WorthUiAllocationCatalogActivationDenial::QuerySuccession(
                WorthUiQueryBindingSuccessionDenial::StaleSuccessorReference
            )
        )
    ));
    assert_eq!(session.generation_identity(), &active_generation);
    let _ = session.shutdown();
}

pub(super) fn admit_active_settlement(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    projection: WorthUiSettledSnapshotProjection,
    refresh: bool,
) -> std::sync::Arc<WorthUiSettledSnapshotFact> {
    let mut admitted = None;
    let completion = session
        .execute_framework_turn(|turn| {
            turn.query_projection(|source| {
                admitted = Some(if refresh {
                    source.refresh_settled(projection)
                } else {
                    source.admit_settled(projection)
                });
            });
        })
        .expect("no mounted presentation lease is active");
    drop(completion.into_completion());
    admitted
        .expect("settlement source ran")
        .expect("active settlement transaction commits")
}

pub(super) fn settle_snapshot(
    reference: &WorthUiInstalledQueryBindingReference,
    workspace: &mut runtime::WorthQueryWorkspace,
) -> WorthUiSettledSnapshotProjection {
    reference
        .enter_snapshot_attempt(workspace)
        .expect("snapshot attempt enters exact world")
        .prepare_snapshot_consumer(interactive_borrowed_collection_requirements())
        .expect("Query mints the consumer contract")
        .execute(workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume()
        .unwrap()
        .settle()
        .unwrap()
}

fn assert_coordinates(fact: &WorthUiSettledSnapshotFact, generation: u64, order: u64) {
    assert_eq!(fact.source_generation().unwrap().as_u64(), generation);
    assert_eq!(fact.source_order().unwrap().as_u64(), order);
}
