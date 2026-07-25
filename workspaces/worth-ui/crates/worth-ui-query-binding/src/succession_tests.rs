use crate::{
    WorthUiQueryBindingPlan, WorthUiQueryBindingSuccessionChange, WorthUiQueryViewShape,
    WorthUiQueryWorkspaceExt,
};

#[test]
fn regional_succession_carries_shared_truth_across_different_identity_universes() {
    let mut workspace = crate::snapshot_refresh_isolation_tests::installed_workspace();
    let installed = workspace.worth_ui().unwrap();
    let removed_view = installed.measurement_view("dashboard.removed").unwrap();
    let shared_view = installed.measurement_view("dashboard.shared").unwrap();
    let added_view = installed.measurement_view("dashboard.added").unwrap();
    let removed_identity = removed_view.definition().identity().clone();
    let shared_identity = shared_view.definition().identity().clone();
    let added_identity = added_view.definition().identity().clone();

    let active_plan = WorthUiQueryBindingPlan::default()
        .register_view(removed_view)
        .unwrap()
        .register_view(shared_view.clone())
        .unwrap();
    let candidate_plan = WorthUiQueryBindingPlan::default()
        .register_view(shared_view)
        .unwrap()
        .register_view(added_view)
        .unwrap();
    let removed = reference(&active_plan, &removed_identity);
    let active_shared = reference(&active_plan, &shared_identity);
    let candidate_shared = reference(&candidate_plan, &shared_identity);
    let added = reference(&candidate_plan, &added_identity);
    assert_eq!(
        active_shared, candidate_shared,
        "an unchanged installed reference is the carry basis"
    );

    let mut active = active_plan.prepare_downstream_state();
    active
        .admit_settled_snapshot(crate::snapshot_refresh_isolation_tests::settle(
            &removed,
            &mut workspace,
        ))
        .unwrap();
    let active_shared_fact = active
        .admit_settled_snapshot(crate::snapshot_refresh_isolation_tests::settle(
            &active_shared,
            &mut workspace,
        ))
        .unwrap();

    let mut candidate = candidate_plan.prepare_downstream_state();
    let displaced_shared_fact = candidate
        .admit_settled_snapshot(crate::snapshot_refresh_isolation_tests::settle(
            &candidate_shared,
            &mut workspace,
        ))
        .unwrap();
    let added_fact = candidate
        .admit_settled_snapshot(crate::snapshot_refresh_isolation_tests::settle(
            &added,
            &mut workspace,
        ))
        .unwrap();
    assert_ne!(
        active_shared_fact.settlement_reference(),
        displaced_shared_fact.settlement_reference()
    );

    let prepared = candidate
        .prepare_regional_succession(
            &active,
            [WorthUiQueryBindingSuccessionChange::new(
                Some(removed.clone()),
                Some(added.clone()),
            )],
        )
        .unwrap();
    let retirement = prepared.commit_once(&mut active);

    assert!(retirement.is_empty());
    assert_eq!(
        active
            .settled_snapshot_fact_for(&candidate_shared)
            .unwrap()
            .settlement_reference(),
        active_shared_fact.settlement_reference(),
        "the active shared fact, not the candidate duplicate, survives"
    );
    assert_eq!(
        active
            .settled_snapshot_fact_for(&added)
            .unwrap()
            .settlement_reference(),
        added_fact.settlement_reference(),
        "the added identity keeps candidate-owned truth"
    );
    assert!(active.settled_snapshot_fact_for(&removed).is_err());
}

fn reference(
    plan: &WorthUiQueryBindingPlan,
    identity: &crate::WorthUiQueryViewIdentity,
) -> crate::WorthUiInstalledQueryBindingReference {
    plan.resolve_definition(identity, WorthUiQueryViewShape::Collection)
        .unwrap()
}
