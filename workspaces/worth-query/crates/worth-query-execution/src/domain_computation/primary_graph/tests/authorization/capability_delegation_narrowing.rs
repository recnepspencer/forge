use worth_query_installation::facade::TypedApplicationValue;

use super::super::application_attempt::authenticated_principal;
use super::super::fixture::capability::{
    CapabilityActionField, CapabilityAmountField, CapabilityDelegationLimitField,
    CapabilityDisclosureField, CapabilityNotAfterField, CapabilityNotBeforeField,
    CapabilityPurposeField, CapabilityRelated, CapabilityWorkflowField,
};
use super::super::fixture::{installed_delegated_capability_world, live_scope};
use super::capability_delegation_mutation::{
    account, field, grant, relation_kind, replace_relation_target, update_grant_field,
};
use super::capability_progression::{admitted_capability_access, time};
use crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind;

#[derive(Clone, Copy, Debug)]
enum NarrowingAxis {
    Action,
    Purpose,
    Disclosure,
    RelatedRelationship,
    Amount,
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
        NarrowingAxis::RelatedRelationship,
        NarrowingAxis::Amount,
        NarrowingAxis::Workflow,
        NarrowingAxis::ValidityStart,
        NarrowingAxis::ValidityEnd,
        NarrowingAxis::DownstreamDelegation,
    ] {
        let world = installed_delegated_capability_world();
        world.authorization_time.script([time(100)]);
        break_narrowing_axis(&world, axis);
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

fn break_narrowing_axis(world: &super::super::fixture::AuthorizationWorld, axis: NarrowingAxis) {
    if matches!(axis, NarrowingAxis::RelatedRelationship) {
        replace_parent_related_relationship(world);
        return;
    }
    let (field, value, grant) = match axis {
        NarrowingAxis::Action => (
            field(world, CapabilityActionField::reference()),
            super::super::fixture::CapabilityAction::Inspect.into_foundational_value(),
            "capability-parent",
        ),
        NarrowingAxis::Purpose => (
            field(world, CapabilityPurposeField::reference()),
            super::super::fixture::CapabilityPurpose::Audit.into_foundational_value(),
            "capability-parent",
        ),
        NarrowingAxis::Disclosure => (
            field(world, CapabilityDisclosureField::reference()),
            super::super::fixture::CapabilityDisclosure::PrivateLabel.into_foundational_value(),
            "capability-parent",
        ),
        NarrowingAxis::Amount => (
            field(world, CapabilityAmountField::reference()),
            76_u64.into_foundational_value(),
            "capability-child",
        ),
        NarrowingAxis::Workflow => (
            field(world, CapabilityWorkflowField::reference()),
            "closed".to_owned().into_foundational_value(),
            "capability-parent",
        ),
        NarrowingAxis::ValidityStart => (
            field(world, CapabilityNotBeforeField::reference()),
            89_u64.into_foundational_value(),
            "capability-child",
        ),
        NarrowingAxis::ValidityEnd => (
            field(world, CapabilityNotAfterField::reference()),
            111_u64.into_foundational_value(),
            "capability-child",
        ),
        NarrowingAxis::DownstreamDelegation => (
            field(world, CapabilityDelegationLimitField::reference()),
            2_u64.into_foundational_value(),
            "capability-child",
        ),
        NarrowingAxis::RelatedRelationship => unreachable!("handled above"),
    };
    update_grant_field(world, grant, field, value);
}

fn replace_parent_related_relationship(world: &super::super::fixture::AuthorizationWorld) {
    let parent = grant(world, "capability-parent");
    replace_relation_target(
        world,
        relation_kind(world, CapabilityRelated::reference().name()),
        parent,
        account(world, "account-2"),
        account(world, "account-1"),
        "parent-wider-related-relationship",
    );
}
