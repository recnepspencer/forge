use super::*;

#[test]
fn rebuild_declaration_executes_against_target_specific_debt() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    let request = layout_request(envelope.branch_context.clone(), envelope.commit.commit_id);
    let materialization = store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    let policy = ConservativeRetentionPolicy::new(
        Vec::new(),
        Vec::new(),
        vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
    );

    let initial_batch = store
        .plan_retention_maintenance_batch(RetentionPolicyClass::Conservative(policy.clone()))
        .unwrap();
    let initial_receipt = store.admit_maintenance_batch(initial_batch).unwrap();
    let reclaim = initial_receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| {
            matches!(
                declaration.declaration(),
                crate::MaintenanceDeclaration::Reclaim { .. }
            )
        })
        .expect("reclaim declaration")
        .clone();
    let rebuild_declaration = initial_receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| {
            matches!(
                declaration.declaration(),
                crate::MaintenanceDeclaration::Rebuild { .. }
            )
        })
        .expect("rebuild declaration")
        .declaration();
    let rebuild_id = rebuild_declaration.id().clone();
    let expected_debt_link = format!(
        "rebuild-debt:{}:{}:{}",
        "milestone_6_layout_materialization",
        format!(
            "branch:{}@{}",
            envelope.branch_context.0, envelope.commit.commit_id.0
        ),
        materialization.artifact_id()
    );

    match rebuild_declaration {
        crate::MaintenanceDeclaration::Rebuild { declaration, .. } => {
            assert_eq!(
                declaration.rebuild_target_id(),
                materialization.artifact_id()
            );
            assert_eq!(
                declaration.debt_link_artifact_id(),
                Some(expected_debt_link.as_str())
            );
        }
        _ => unreachable!("selected declaration should be a rebuild"),
    }

    let deferred = store
        .start_maintenance_declaration(
            initial_receipt
                .admitted_declarations()
                .iter()
                .find(|declaration| declaration.declaration().id() == &rebuild_id)
                .expect("rebuild admitted declaration"),
        )
        .unwrap_err();
    assert_eq!(deferred.error_kind(), "ReclaimEligibilityViolation");
    assert_eq!(
        store
            .maintenance_status(&rebuild_id)
            .unwrap()
            .execution_status(),
        MaintenanceExecutionStatus::Admitted
    );

    let reclaim_completed = store.start_maintenance_declaration(&reclaim).unwrap();
    assert_eq!(reclaim_completed.last_completed_phase(), "derived_reclaim");

    let rebuild = store
        .start_maintenance_declaration(
            initial_receipt
                .admitted_declarations()
                .iter()
                .find(|declaration| declaration.declaration().id() == &rebuild_id)
                .expect("rebuild admitted declaration"),
        )
        .unwrap();
    assert_eq!(rebuild.last_completed_phase(), "rebuild");
    assert_eq!(
        store
            .maintenance_status(&rebuild_id)
            .unwrap()
            .execution_status(),
        MaintenanceExecutionStatus::Completed
    );
    let counters = store.milestone_11_counter_contract();
    assert!(counters.maintenance_debt_link_count >= 1);
    assert_eq!(
        store
            .fetch_milestone_6_layout_support(request)
            .unwrap()
            .artifact_id(),
        materialization.artifact_id()
    );
}

