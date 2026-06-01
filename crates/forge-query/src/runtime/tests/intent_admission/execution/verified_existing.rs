use super::*;

fn seeded_existing_binding(
    workspace: &mut ForgeQueryWorkspace,
    workspace_name: &str,
) -> ForgeQueryExistingTruthTargetBinding {
    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", format!("{workspace_name}-task"))
                .aspect("title.value", "Seed title")
                .aspect("status.value", "open")
        })
        .expect("seed insert should execute");
    workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                format!("authority:{workspace_name}-task"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build")
}

#[test]
fn workspace_verify_existing_delegates_to_authoritative_mutation_intent_execution() {
    const WORKSPACE_NAME: &str = "intent-admission-verify-existing";

    let mut delegated_workspace = stateful_bridge_task_runtime()
        .workspace("intent-admission-verify-existing-delegated")
        .expect("workspace should open");
    let delegated_binding = seeded_existing_binding(&mut delegated_workspace, WORKSPACE_NAME);
    let delegated = delegated_workspace
        .verify_existing(delegated_binding, |task| {
            task.aspect("status.value", "open")
        })
        .expect("legacy verify_existing should execute");

    let mut canonical_workspace = stateful_bridge_task_runtime()
        .workspace("intent-admission-verify-existing-canonical")
        .expect("workspace should open");
    let canonical_binding = seeded_existing_binding(&mut canonical_workspace, WORKSPACE_NAME);
    let command = ForgeQueryAspectMutationBuilder::new()
        .aspect("status.value", "open")
        .build_verify_existing(canonical_binding)
        .expect("verify command should build");
    let canonical = canonical_workspace
        .write_intent(command)
        .execute()
        .expect("canonical write intent should execute");
    let delegated_assumptions = delegated
        .verified_assumption_set()
        .expect("delegated verify should retain assumption set");
    let canonical_assumptions = canonical
        .verified_assumption_set()
        .expect("canonical verify should retain assumption set");

    assert_eq!(delegated_assumptions, canonical_assumptions);
    assert_eq!(
        canonical
            .execution_provenance()
            .map(|provenance| provenance.entrypoint()),
        Some(ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite)
    );
}

#[test]
fn workspace_update_existing_verified_delegates_to_authoritative_mutation_intent_execution() {
    const WORKSPACE_NAME: &str = "intent-admission-update-existing-verified";

    let mut delegated_workspace = stateful_bridge_task_runtime()
        .workspace("intent-admission-update-existing-verified-delegated")
        .expect("workspace should open");
    let delegated_binding = seeded_existing_binding(&mut delegated_workspace, WORKSPACE_NAME);
    let delegated = delegated_workspace
        .update_existing_verified(
            delegated_binding.clone(),
            |task| task.aspect("status.value", "open"),
            |task| task.aspect("status.value", "closed"),
        )
        .expect("legacy verified update should execute");
    let delegated_probe = delegated_workspace
        .probe_existing(
            delegated_binding,
            ["status.value", "title.value", "identity.id"],
        )
        .expect("delegated post-update probe should execute");

    let mut canonical_workspace = stateful_bridge_task_runtime()
        .workspace("intent-admission-update-existing-verified-canonical")
        .expect("workspace should open");
    let canonical_binding = seeded_existing_binding(&mut canonical_workspace, WORKSPACE_NAME);
    let asserted_aspects = ForgeQueryAspectMutationBuilder::new()
        .aspect("status.value", "open")
        .finish_existing_truth_verification_aspects("backend-verified existing-truth update")
        .expect("asserted aspects should build");
    let command = ForgeQueryAspectMutationBuilder::new()
        .aspect("status.value", "closed")
        .build_update_existing_verified(canonical_binding.clone(), asserted_aspects)
        .expect("verified update command should build");
    let canonical = canonical_workspace
        .write_intent(command)
        .execute()
        .expect("canonical write intent should execute");
    let canonical_probe = canonical_workspace
        .probe_existing(
            canonical_binding,
            ["status.value", "title.value", "identity.id"],
        )
        .expect("canonical post-update probe should execute");
    let delegated_assumptions = delegated
        .verified_assumption_set()
        .expect("delegated update should retain assumption set");
    let canonical_assumptions = canonical
        .verified_assumption_set()
        .expect("canonical update should retain assumption set");

    assert_eq!(delegated_assumptions, canonical_assumptions);
    assert_eq!(
        delegated_probe
            .field("status.value")
            .map(|field| field.external_value_json()),
        canonical_probe
            .field("status.value")
            .map(|field| field.external_value_json())
    );
    assert_eq!(
        delegated_probe
            .field("title.value")
            .map(|field| field.external_value_json()),
        canonical_probe
            .field("title.value")
            .map(|field| field.external_value_json())
    );
    assert_eq!(
        delegated_probe
            .field("identity.id")
            .map(|field| field.external_value_json()),
        canonical_probe
            .field("identity.id")
            .map(|field| field.external_value_json())
    );
}

#[test]
fn workspace_delete_existing_verified_delegates_to_authoritative_mutation_intent_execution() {
    const WORKSPACE_NAME: &str = "intent-admission-delete-existing-verified";

    let mut delegated_workspace = stateful_bridge_task_runtime()
        .workspace("intent-admission-delete-existing-verified-delegated")
        .expect("workspace should open");
    let delegated_binding = seeded_existing_binding(&mut delegated_workspace, WORKSPACE_NAME);
    let delegated = delegated_workspace
        .delete_existing_verified(
            delegated_binding.clone(),
            |task| task.aspect("title.value", "Seed title"),
            |delete| delete.touch("title.value"),
        )
        .expect("legacy verified delete should execute");

    let mut canonical_workspace = stateful_bridge_task_runtime()
        .workspace("intent-admission-delete-existing-verified-canonical")
        .expect("workspace should open");
    let canonical_binding = seeded_existing_binding(&mut canonical_workspace, WORKSPACE_NAME);
    let asserted_aspects = ForgeQueryAspectMutationBuilder::new()
        .aspect("title.value", "Seed title")
        .finish_existing_truth_verification_aspects("backend-verified existing-truth delete")
        .expect("asserted aspects should build");
    let command = ForgeQueryDeleteMutationBuilder::new()
        .touch("title.value")
        .build_delete_existing_verified(canonical_binding, asserted_aspects)
        .expect("verified delete command should build");
    let canonical = canonical_workspace
        .write_intent(command)
        .execute()
        .expect("canonical write intent should execute");
    let delegated_assumptions = delegated
        .verified_assumption_set()
        .expect("delegated delete should retain assumption set");
    let canonical_assumptions = canonical
        .verified_assumption_set()
        .expect("canonical delete should retain assumption set");

    assert_eq!(delegated_assumptions, canonical_assumptions);
    assert_eq!(
        canonical
            .execution_provenance()
            .map(|provenance| provenance.entrypoint()),
        Some(ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite)
    );
}
