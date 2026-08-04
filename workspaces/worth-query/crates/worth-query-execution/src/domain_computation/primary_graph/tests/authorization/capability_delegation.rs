use worth_query_installation::facade::TypedApplicationValue;

use super::super::application_attempt::authenticated_principal;
use super::super::fixture::capability::{
    CapabilityActionField, CapabilityDelegationLimitField, CapabilityDisclosureField,
    CapabilityGrantee, CapabilityGrantor, CapabilityNotAfterField, CapabilityNotBeforeField,
    CapabilityPurposeField, CapabilityResource, CapabilityStatusField, CapabilityWorkflowField,
};
use super::super::fixture::{installed_delegated_capability_world, live_scope};
use super::capability_delegation_mutation::{
    account, add_parent, field, grant, relation_kind, relation_source, replace_parent,
    replace_relation_source, replace_relation_target, update_grant_field,
};
use super::capability_progression::{admitted_capability_access, time};
use crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind;

#[test]
fn lawful_multi_link_delegation_is_one_composite_exact_authorization_fact() {
    let mut world = installed_delegated_capability_world();
    world.application.script_authorization_time([time(100)]);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);

    let access = admitted_capability_access(&world, &principal, &request, 100)
        .expect("the narrowed current child and exact parent must admit");

    assert_eq!(access.authorization_decision_fact_count(), 2);
    assert_eq!(access.relational_counters().paths_evaluated, 19);
    assert!(access.signal_dependency_count() > 0);
    let work = access.admission_canonical_work();
    assert_eq!(work.basis_preparations(), 0);
    assert_eq!(work.digest_derivations(), 0);
    assert_eq!(work.digest_text_materializations(), 0);
}

#[derive(Clone, Copy, Debug)]
enum NarrowingAxis {
    Action,
    Purpose,
    Disclosure,
    Workflow,
    ValidityStart,
    ValidityEnd,
    DownstreamDelegation,
}

#[test]
fn every_stored_narrowing_axis_is_enforced_by_production_admission() {
    for axis in [
        NarrowingAxis::Action,
        NarrowingAxis::Purpose,
        NarrowingAxis::Disclosure,
        NarrowingAxis::Workflow,
        NarrowingAxis::ValidityStart,
        NarrowingAxis::ValidityEnd,
        NarrowingAxis::DownstreamDelegation,
    ] {
        let mut world = installed_delegated_capability_world();
        world.application.script_authorization_time([time(100)]);
        widen_child_axis(&world, axis);
        let request = live_scope();
        let principal = authenticated_principal(&world, &request);

        let Err(denial) = admitted_capability_access(&world, &principal, &request, 100) else {
            panic!("a widened delegated grant must mint no access authority");
        };
        assert_eq!(
            denial.kind(),
            WorthQueryOperationAuthorizationDenialKind::DelegationRejected,
            "axis {axis:?} escaped the installed narrowing transition"
        );
    }
}

#[test]
fn width_and_cycle_attacks_are_typed_denials_before_access_authority() {
    let mut width = installed_delegated_capability_world();
    width.application.script_authorization_time([time(100)]);
    add_parent(
        &width,
        "capability-child",
        "capability-alternate",
        "width-parent",
    );
    let request = live_scope();
    let principal = authenticated_principal(&width, &request);
    let Err(denial) = admitted_capability_access(&width, &principal, &request, 100) else {
        panic!("two current parents must deny");
    };
    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::DelegationRejected
    );

    let mut cycle = installed_delegated_capability_world();
    cycle.application.script_authorization_time([time(100)]);
    add_parent(
        &cycle,
        "capability-grandparent",
        "capability-child",
        "cycle-parent",
    );
    let request = live_scope();
    let principal = authenticated_principal(&cycle, &request);
    let Err(denial) = admitted_capability_access(&cycle, &principal, &request, 100) else {
        panic!("a delegation cycle must deny before narrowing can be reused");
    };
    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::DelegationCycle
    );

    let mut depth = installed_delegated_capability_world();
    depth.application.script_authorization_time([time(100)]);
    add_parent(
        &depth,
        "capability-grandparent",
        "capability-alternate",
        "excess-depth-parent",
    );
    let request = live_scope();
    let principal = authenticated_principal(&depth, &request);
    let Err(denial) = admitted_capability_access(&depth, &principal, &request, 100) else {
        panic!("a chain beyond the installed depth must deny");
    };
    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::DelegationDepthExceeded
    );
}

#[test]
fn resource_and_grantor_grantee_link_drift_deny_at_the_transition() {
    let mut resource = installed_delegated_capability_world();
    resource.application.script_authorization_time([time(100)]);
    replace_parent_resource(&resource);
    let request = live_scope();
    let principal = authenticated_principal(&resource, &request);
    let Err(denial) = admitted_capability_access(&resource, &principal, &request, 100) else {
        panic!("a parent bound to another resource must deny");
    };
    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::DelegationRejected
    );

    let mut actors = installed_delegated_capability_world();
    actors.application.script_authorization_time([time(100)]);
    replace_child_grantor_with_grantee(&actors);
    let request = live_scope();
    let principal = authenticated_principal(&actors, &request);
    let Err(denial) = admitted_capability_access(&actors, &principal, &request, 100) else {
        panic!("a child grantor who is not the parent grantee must deny");
    };
    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::DelegationRejected
    );
}

#[test]
fn every_parent_currentness_drift_stales_retained_authority() {
    for drift in [
        ParentDrift::Revocation,
        ParentDrift::Expiry,
        ParentDrift::Workflow,
        ParentDrift::Resource,
        ParentDrift::Grantee,
        ParentDrift::Substitute,
    ] {
        let mut world = installed_delegated_capability_world();
        world
            .application
            .script_authorization_time([time(100), time(100)]);
        let request = live_scope();
        let principal = authenticated_principal(&world, &request);
        let access = admitted_capability_access(&world, &principal, &request, 100).unwrap();
        apply_parent_drift(&world, drift);
        let operation = world
            .application
            .installed_schema()
            .installed_operation(super::super::fixture::CapabilityTouchOperation::reference())
            .unwrap();
        let Err(denial) = world.application.authorize_capability_operation(
            access,
            &operation,
            Default::default(),
        ) else {
            panic!("retained authority must stale before a parent can be refreshed");
        };
        assert_eq!(
            denial.kind(),
            WorthQueryOperationAuthorizationDenialKind::StaleAuthorization,
            "drift {drift:?} did not invalidate the composite fact"
        );
    }
}

#[derive(Clone, Copy, Debug)]
enum ParentDrift {
    Revocation,
    Expiry,
    Workflow,
    Resource,
    Grantee,
    Substitute,
}

fn apply_parent_drift(world: &super::super::fixture::AuthorizationWorld, drift: ParentDrift) {
    match drift {
        ParentDrift::Revocation => update_grant_field(
            world,
            "capability-parent",
            field(world, CapabilityStatusField::reference()),
            super::super::fixture::CapabilityStatus::Revoked.into_foundational_value(),
        ),
        ParentDrift::Expiry => update_grant_field(
            world,
            "capability-parent",
            field(world, CapabilityNotAfterField::reference()),
            99_u64.into_foundational_value(),
        ),
        ParentDrift::Workflow => update_grant_field(
            world,
            "capability-parent",
            field(world, CapabilityWorkflowField::reference()),
            "closed".to_owned().into_foundational_value(),
        ),
        ParentDrift::Resource => replace_parent_resource(world),
        ParentDrift::Grantee => replace_parent_grantee(world),
        ParentDrift::Substitute => replace_parent(
            world,
            "capability-child",
            "capability-parent",
            "capability-alternate",
        ),
    }
}

fn replace_parent_resource(world: &super::super::fixture::AuthorizationWorld) {
    let kind = relation_kind(world, CapabilityResource::reference().name());
    replace_relation_target(
        world,
        kind,
        grant(world, "capability-parent"),
        account(world, "account-1"),
        account(world, "account-2"),
        "parent-other-resource",
    );
}

fn replace_child_grantor_with_grantee(world: &super::super::fixture::AuthorizationWorld) {
    let child = grant(world, "capability-child");
    let grantor_kind = relation_kind(world, CapabilityGrantor::reference().name());
    let grantee_kind = relation_kind(world, CapabilityGrantee::reference().name());
    replace_relation_source(
        world,
        grantor_kind,
        relation_source(world, grantor_kind, child),
        relation_source(world, grantee_kind, child),
        child,
        "child-wrong-grantor",
    );
}

fn replace_parent_grantee(world: &super::super::fixture::AuthorizationWorld) {
    let child = grant(world, "capability-child");
    let parent = grant(world, "capability-parent");
    let grantee_kind = relation_kind(world, CapabilityGrantee::reference().name());
    replace_relation_source(
        world,
        grantee_kind,
        relation_source(world, grantee_kind, parent),
        relation_source(world, grantee_kind, child),
        parent,
        "parent-wrong-grantee",
    );
}

fn widen_child_axis(world: &super::super::fixture::AuthorizationWorld, axis: NarrowingAxis) {
    let (field, value) = match axis {
        NarrowingAxis::Action => (
            field(world, CapabilityActionField::reference()),
            super::super::fixture::CapabilityAction::Inspect.into_foundational_value(),
        ),
        NarrowingAxis::Purpose => (
            field(world, CapabilityPurposeField::reference()),
            super::super::fixture::CapabilityPurpose::Audit.into_foundational_value(),
        ),
        NarrowingAxis::Disclosure => (
            field(world, CapabilityDisclosureField::reference()),
            super::super::fixture::CapabilityDisclosure::PrivateLabel.into_foundational_value(),
        ),
        NarrowingAxis::Workflow => (
            field(world, CapabilityWorkflowField::reference()),
            "closed".to_owned().into_foundational_value(),
        ),
        NarrowingAxis::ValidityStart => (
            field(world, CapabilityNotBeforeField::reference()),
            89_u64.into_foundational_value(),
        ),
        NarrowingAxis::ValidityEnd => (
            field(world, CapabilityNotAfterField::reference()),
            111_u64.into_foundational_value(),
        ),
        NarrowingAxis::DownstreamDelegation => (
            field(world, CapabilityDelegationLimitField::reference()),
            2_u64.into_foundational_value(),
        ),
    };
    let grant = match axis {
        NarrowingAxis::Action
        | NarrowingAxis::Purpose
        | NarrowingAxis::Disclosure
        | NarrowingAxis::Workflow => "capability-parent",
        NarrowingAxis::ValidityStart
        | NarrowingAxis::ValidityEnd
        | NarrowingAxis::DownstreamDelegation => "capability-child",
    };
    update_grant_field(world, grant, field, value);
}
