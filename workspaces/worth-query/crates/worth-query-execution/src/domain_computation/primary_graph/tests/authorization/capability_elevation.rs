use std::collections::BTreeMap;

use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityEntitySelector, ApplicationCapabilityRequestContext,
    ApplicationCapabilityRequestProjection,
};
use worth_query_installation::facade::TypedApplicationValue;
use worth_relational::facade::identity::{EntityId, PartitionId};
use worth_relational::facade::symbols::ClientKey;
use worth_relational::facade::transactions::{
    AspectFieldPatch, CreateIntent, EntityMutationIntent, EntityReference, MutationIntent,
    RelationSpec, TransactionOptions, UpdateEntityFieldsIntent, WorkerIntentBatch,
};

use super::super::application_attempt::authenticated_principal;
use super::super::fixture::{
    installed_elevated_capability_world, live_scope, AccountIdentity, CapabilityAction,
    CapabilityDisclosure, CapabilityElevationApprover, CapabilityElevationIdentity,
    CapabilityElevationScenario, CapabilityElevationStatus, CapabilityElevationStatusField,
    CapabilityPurpose, CapabilityRequestContext, CapabilityTouchOperation,
    ElevatedCapabilityTouchInput, ElevatedCapabilityTouchOperation, ElevatedTouchAccountCapability,
    TouchAccountCapability,
};
use super::capability_progression::time;
use crate::domain_computation::primary_graph::{
    WorthQueryOperationAuthorizationDenialKind, WorthQueryPrincipalResolutionMode,
};

#[path = "capability_elevation/approver_conflict.rs"]
mod approver_conflict;
#[path = "capability_elevation/validity.rs"]
mod validity;

#[test]
fn exact_active_elevation_admits_and_revalidates_with_ordinary_capability_authority() {
    let mut world = installed_elevated_capability_world(CapabilityElevationScenario::Active);
    world
        .application
        .script_authorization_time([time(100), time(100)]);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let access = admit(&world, &principal, &request, Some("elevation-1")).unwrap();

    assert_eq!(access.authorization_decision_fact_count(), 2);
    let operation = world
        .application
        .installed_schema()
        .installed_operation(ElevatedCapabilityTouchOperation::reference())
        .unwrap();
    world
        .application
        .authorize_capability_operation(access, &operation, Default::default())
        .expect("the exact active elevation must survive fresh operation re-admission");
}

#[test]
fn governed_capability_requires_an_exact_elevation_selector() {
    let mut world = installed_elevated_capability_world(CapabilityElevationScenario::Active);
    world.application.script_authorization_time([time(100)]);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);

    let Err(denial) = admit(&world, &principal, &request, None) else {
        panic!("governed capability admission must require an elevation selector");
    };

    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::ElevationRequired
    );
}

#[test]
fn resource_selector_cannot_substitute_for_the_declared_elevation_identity() {
    let world = installed_elevated_capability_world(CapabilityElevationScenario::Active);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let capability = installed_capability(&world);
    let mut input = elevated_input(Some("elevation-1"));
    input.substitute_resource_selector = true;

    let Err(denial) =
        world
            .application
            .admit_capability_access(&principal, &capability, input, &request)
    else {
        panic!("a selector for another installed entity kind must not open elevation authority");
    };

    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::ElevationProjectionRejected
    );
}

#[test]
fn non_governed_capability_rejects_an_elevation_selector() {
    let world = installed_elevated_capability_world(CapabilityElevationScenario::Active);
    let capability = world
        .application
        .installed_schema()
        .capability(
            TouchAccountCapability::reference(),
            CapabilityTouchOperation::reference(),
        )
        .unwrap();
    let projection = ApplicationCapabilityRequestProjection::new(
        ApplicationCapabilityEntitySelector::new(
            AccountIdentity::reference(),
            "account-1".to_owned(),
        ),
        CapabilityAction::Touch,
        CapabilityPurpose::AccountMaintenance,
        ApplicationCapabilityRequestContext::new(CapabilityRequestContext::reference()),
    )
    .elevation(ApplicationCapabilityEntitySelector::new(
        CapabilityElevationIdentity::reference(),
        "elevation-1".to_owned(),
    ));

    let denial = crate::domain_computation::authorization::validate_elevation_projection(
        capability.contract(),
        &projection,
    )
    .unwrap_err();

    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::ElevationNotApplicable
    );
}

#[test]
fn revoked_or_wrong_grant_elevation_cannot_open_active_authority() {
    for scenario in [
        CapabilityElevationScenario::Revoked,
        CapabilityElevationScenario::WrongGrant,
    ] {
        let mut world = installed_elevated_capability_world(scenario);
        world.application.script_authorization_time([time(100)]);
        let request = live_scope();
        let principal = authenticated_principal(&world, &request);

        let Err(denial) = admit(&world, &principal, &request, Some("elevation-1")) else {
            panic!("scenario {scenario:?} must not mint elevated access");
        };

        assert_eq!(
            denial.kind(),
            WorthQueryOperationAuthorizationDenialKind::ElevationInactive,
            "scenario {scenario:?} must fail through the installed elevation rule"
        );
    }
}

#[test]
fn requester_cannot_self_approve_the_exact_elevation() {
    let mut world = installed_elevated_capability_world(CapabilityElevationScenario::SelfApproved);
    world.application.script_authorization_time([time(100)]);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);

    let Err(denial) = admit(&world, &principal, &request, Some("elevation-1")) else {
        panic!("self-approved elevation must not mint access");
    };

    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::ElevationSelfApproval
    );
}

#[test]
fn elevation_status_drift_after_admission_is_stale_at_operation_progression() {
    let mut world = installed_elevated_capability_world(CapabilityElevationScenario::Active);
    world
        .application
        .script_authorization_time([time(100), time(100)]);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let access = admit(&world, &principal, &request, Some("elevation-1")).unwrap();
    revoke_elevation(&world);
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
        panic!("revoked elevation evidence must not progress to operation authority");
    };

    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::StaleAuthorization
    );
}

#[test]
fn approver_drift_after_admission_is_stale_before_operation_authority() {
    let mut world = installed_elevated_capability_world(CapabilityElevationScenario::Active);
    world
        .application
        .script_authorization_time([time(100), time(100)]);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let access = admit(&world, &principal, &request, Some("elevation-1")).unwrap();
    add_self_approver(&world, principal.principal_entity_id());
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
        panic!("changed exact approver relationships must stale admitted elevation evidence");
    };

    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::StaleAuthorization
    );
}

fn admit(
    world: &super::super::fixture::AuthorizationWorld,
    principal: &crate::domain_computation::primary_graph::WorthQueryAuthenticatedPrincipal<
        super::super::fixture::IdentityExecutionSchema,
        super::super::fixture::Principal,
        u64,
    >,
    request: &worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope,
    elevation: Option<&str>,
) -> Result<
    crate::domain_computation::primary_graph::WorthQueryAdmittedApplicationCapabilityAccess<
        super::super::fixture::IdentityExecutionSchema,
        ElevatedTouchAccountCapability,
        ElevatedCapabilityTouchOperation,
        ElevatedCapabilityTouchInput,
    >,
    crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenial,
> {
    let capability = installed_capability(world);
    world.application.admit_capability_access(
        principal,
        &capability,
        elevated_input(elevation),
        request,
    )
}

fn installed_capability(
    world: &super::super::fixture::AuthorizationWorld,
) -> worth_query_installation::facade::WorthQueryInstalledApplicationCapability<
    super::super::fixture::IdentityExecutionSchema,
    ElevatedTouchAccountCapability,
    ElevatedCapabilityTouchOperation,
    ElevatedCapabilityTouchInput,
> {
    world
        .application
        .installed_schema()
        .capability(
            ElevatedTouchAccountCapability::reference(),
            ElevatedCapabilityTouchOperation::reference(),
        )
        .unwrap()
}

fn elevated_input(elevation: Option<&str>) -> ElevatedCapabilityTouchInput {
    ElevatedCapabilityTouchInput {
        account: "account-1".to_owned(),
        elevation: elevation.map(str::to_owned),
        substitute_resource_selector: false,
        action: CapabilityAction::Touch,
        purpose: CapabilityPurpose::AccountMaintenance,
        disclosure: CapabilityDisclosure::AccountActivity,
        amount: 50,
    }
}

fn revoke_elevation(world: &super::super::fixture::AuthorizationWorld) {
    let elevation = world
        .application
        .resolve_entity(
            CapabilityElevationIdentity::reference(),
            "elevation-1".to_owned(),
            &live_scope(),
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let field = CapabilityElevationStatusField::reference();
    let graph = world.application.runtime.primary_graph().unwrap();
    let locator = graph
        .layout()
        .field_locator(field.entity(), field.aspect(), field.field())
        .unwrap()
        .clone();
    let handle = graph.integration_handle();
    handle.with_runtime_mut(|runtime| {
        let fields = AspectFieldPatch::from(BTreeMap::from([(
            locator,
            CapabilityElevationStatus::Revoked.into_foundational_value(),
        )]));
        let mut transaction = runtime.begin_transaction(TransactionOptions::default());
        transaction.push_batch(WorkerIntentBatch::new("revoke-elevation").push(
            MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                UpdateEntityFieldsIntent {
                    entity_id: elevation.entity_id(),
                    fields,
                },
            )),
        ));
        transaction.commit().unwrap();
        handle.ensure_primary_indexes_current(runtime).unwrap();
    });
}

fn add_self_approver(world: &super::super::fixture::AuthorizationWorld, requester: EntityId) {
    let elevation = world
        .application
        .resolve_entity(
            CapabilityElevationIdentity::reference(),
            "elevation-1".to_owned(),
            &live_scope(),
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let graph = world.application.runtime.primary_graph().unwrap();
    let relation_kind = graph
        .layout()
        .relation(CapabilityElevationApprover::reference().name())
        .unwrap()
        .kind;
    let handle = graph.integration_handle();
    handle.with_runtime_mut(|runtime| {
        let mut transaction = runtime.begin_transaction(TransactionOptions::default());
        transaction.push_batch(WorkerIntentBatch::new("add-self-approver").push(
            MutationIntent::Create(CreateIntent::Relation(RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: relation_kind,
                client_key: ClientKey::raw("elevation-self-approver"),
                source: EntityReference::Existing(requester),
                target: EntityReference::Existing(elevation.entity_id()),
                fields: AspectFieldPatch::default(),
            })),
        ));
        transaction.commit().unwrap();
        handle.ensure_primary_indexes_current(runtime).unwrap();
    });
}
