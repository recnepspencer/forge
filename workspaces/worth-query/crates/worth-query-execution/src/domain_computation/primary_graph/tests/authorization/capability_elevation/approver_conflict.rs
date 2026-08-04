use worth_relational::facade::identity::PartitionId;
use worth_relational::facade::symbols::ClientKey;
use worth_relational::facade::transactions::{
    AspectFieldPatch, CreateIntent, EntityReference, MutationIntent, RelationSpec,
    TransactionOptions, WorkerIntentBatch,
};

use super::super::super::application_attempt::authenticated_principal;
use super::super::super::fixture::capability::CapabilityConflictingBeneficiary;
use super::super::super::fixture::{
    installed_elevated_capability_world, live_scope, AccountIdentity, CapabilityElevationScenario,
    ElevatedCapabilityTouchOperation, PrincipalIdentityField,
};
use super::super::capability_progression::time;
use crate::domain_computation::primary_graph::{
    WorthQueryOperationAuthorizationDenialKind, WorthQueryPrincipalResolutionMode,
};

#[test]
fn exact_conflicted_approver_cannot_open_elevation_authority() {
    let mut world =
        installed_elevated_capability_world(CapabilityElevationScenario::ConflictedApprover);
    world.application.script_authorization_time([time(100)]);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);

    let Err(denial) = super::admit(&world, &principal, &request, Some("elevation-1")) else {
        panic!("a conflict held by the exact approver must deny elevation admission");
    };

    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::ElevationApproverConflict
    );
}

#[test]
fn approver_conflict_drift_stales_admitted_elevation_authority() {
    let mut world = installed_elevated_capability_world(CapabilityElevationScenario::Active);
    world
        .application
        .script_authorization_time([time(100), time(100)]);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let access = super::admit(&world, &principal, &request, Some("elevation-1")).unwrap();
    add_approver_conflict(&world);
    let operation = world
        .application
        .installed_schema()
        .installed_operation(ElevatedCapabilityTouchOperation::reference())
        .unwrap();

    let Err(denial) =
        world
            .application
            .authorize_capability_operation(access, &operation, Default::default())
    else {
        panic!("new conflict truth for the exact approver must stale retained authority");
    };

    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::StaleAuthorization
    );
}

fn add_approver_conflict(world: &super::super::super::fixture::AuthorizationWorld) {
    let scope = live_scope();
    let approver = world
        .application
        .resolve_entity(
            PrincipalIdentityField::reference(),
            2_u64,
            &scope,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let account = world
        .application
        .resolve_entity(
            AccountIdentity::reference(),
            "account-1".to_owned(),
            &scope,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let graph = world.application.runtime.primary_graph().unwrap();
    let relation_kind = graph
        .layout()
        .relation(CapabilityConflictingBeneficiary::reference().name())
        .unwrap()
        .kind;
    let handle = graph.integration_handle();
    handle.with_runtime_mut(|runtime| {
        let mut transaction = runtime.begin_transaction(TransactionOptions::default());
        transaction.push_batch(WorkerIntentBatch::new("add-approver-conflict").push(
            MutationIntent::Create(CreateIntent::Relation(RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: relation_kind,
                client_key: ClientKey::raw("elevation-approver-conflict-drift"),
                source: EntityReference::Existing(approver.entity_id()),
                target: EntityReference::Existing(account.entity_id()),
                fields: AspectFieldPatch::default(),
            })),
        ));
        transaction.commit().unwrap();
        handle.ensure_primary_indexes_current(runtime).unwrap();
    });
}
