use std::collections::BTreeSet;

use crate::application_capability::{
    ApplicationCapabilityElevationRule, ApplicationCapabilityFieldBinding,
    ApplicationCapabilityFieldDimension, ApplicationCapabilityRelationBinding,
    ApplicationCapabilityRelationDimension, ErasedApplicationCapabilityContract,
};

use super::super::{ApplicationSchemaDeclarationDenial, ApplicationSchemaMember};
use super::declared_dimensions::DeclaredCapabilityDimensions;

pub(super) fn dependencies_are_closed(
    members: &[ApplicationSchemaMember],
    dimensions: &DeclaredCapabilityDimensions<'_>,
    contract: &ErasedApplicationCapabilityContract,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    if !dimensions.context_exists(
        contract.constraints().context(),
        contract.constraints().context_type(),
    ) {
        return Err(
            ApplicationSchemaDeclarationDenial::MissingApplicationCapabilityContextDependency,
        );
    }
    if !dimensions.provenance_exists(
        contract.delegation().provenance(),
        contract.delegation().provenance_type(),
    ) {
        return Err(
            ApplicationSchemaDeclarationDenial::MissingApplicationCapabilityProvenanceDependency,
        );
    }
    if operation_exists(members, contract)
        && entity_exists(members, contract.grant_entity())
        && field_exists(members, contract.target().action().field())
        && field_exists(members, contract.target().purpose().field())
        && relation_exists(members, contract.target().resource())
        && field_dimension_exists(members, contract.target().field())
        && relation_dimension_exists(members, contract.target().relation())
        && field_dimension_exists(members, contract.constraints().amount())
        && field_exists(
            members,
            contract.constraints().currentness().active_status().field(),
        )
        && field_exists(
            members,
            contract.constraints().currentness().workflow().grant(),
        )
        && field_exists(
            members,
            contract.constraints().currentness().workflow().resource(),
        )
        && field_exists(
            members,
            contract.constraints().currentness().validity().not_before(),
        )
        && field_exists(
            members,
            contract.constraints().currentness().validity().not_after(),
        )
        && relation_exists(members, contract.delegation().parent())
        && relation_exists(members, contract.delegation().grantor())
        && relation_exists(members, contract.delegation().grantee())
        && field_exists(members, contract.delegation().limit())
        && elevation_dependencies_exist(members, dimensions, contract.elevation())
    {
        Ok(())
    } else {
        Err(ApplicationSchemaDeclarationDenial::MissingApplicationCapabilityDependency)
    }
}

pub(super) fn topology_is_valid(contract: &ErasedApplicationCapabilityContract) -> bool {
    let grant = contract.grant_entity();
    let target = contract.target();
    let constraints = contract.constraints();
    let currentness = constraints.currentness();
    let delegation = contract.delegation();
    target.action().field().entity() == grant
        && target.purpose().field().entity() == grant
        && target.resource().from() == grant
        && currentness.active_status().field().entity() == grant
        && currentness.workflow().grant().entity() == grant
        && currentness.workflow().resource().entity() == target.resource().to()
        && currentness.workflow().grant().value_type()
            == currentness.workflow().resource().value_type()
        && currentness.validity().not_before().entity() == grant
        && currentness.validity().not_after().entity() == grant
        && currentness.validity().not_before().value_type()
            == currentness.validity().not_after().value_type()
        && currentness.validity().not_before().scalar_family()
            == currentness.validity().timeline().scalar_family()
        && currentness.validity().not_after().scalar_family()
            == currentness.validity().timeline().scalar_family()
        && delegation.limit().entity() == grant
        && delegation.parent().from() == grant
        && delegation.parent().to() == grant
        && delegation.grantor().to() == grant
        && delegation.grantee().to() == grant
        && field_dimension_belongs_to(target.field(), grant)
        && field_dimension_belongs_to(constraints.amount(), grant)
        && elevation_topology_is_valid(contract)
}

fn elevation_dependencies_exist(
    members: &[ApplicationSchemaMember],
    dimensions: &DeclaredCapabilityDimensions<'_>,
    elevation: &ApplicationCapabilityElevationRule,
) -> bool {
    let ApplicationCapabilityElevationRule::Governed(elevation) = elevation else {
        return true;
    };
    let review = elevation.review();
    let lifecycle = elevation.lifecycle();
    [
        elevation.identity(),
        elevation.reason(),
        elevation.status(),
        elevation.validity().not_before(),
        elevation.validity().not_after(),
        review.identity(),
        review.status(),
    ]
    .into_iter()
    .chain(
        elevation
            .states()
            .values()
            .into_iter()
            .map(|state| state.field()),
    )
    .chain([review.required().field(), review.completed().field()])
    .all(|field| field_exists(members, field))
        && [
            elevation.requester(),
            elevation.approver(),
            elevation.grant(),
            review.relation(),
            review.reviewer(),
        ]
        .into_iter()
        .all(|relation| relation_exists(members, relation))
        && dimensions.entity_slot_exists(lifecycle.elevation_slot())
        && dimensions.entity_slot_exists(lifecycle.review_slot())
        && lifecycle
            .operations()
            .into_iter()
            .all(|operation| operation_binding_exists(members, operation))
}

fn elevation_topology_is_valid(contract: &ErasedApplicationCapabilityContract) -> bool {
    let ApplicationCapabilityElevationRule::Governed(elevation) = contract.elevation() else {
        return true;
    };
    let Some(principal) = capability_principal(contract) else {
        return false;
    };
    let elevation_entity = elevation.identity().entity();
    let review = elevation.review();
    let review_entity = review.identity().entity();
    let lifecycle = elevation.lifecycle();
    elevation.reason().entity() == elevation_entity
        && elevation.status().entity() == elevation_entity
        && elevation.validity().not_before().entity() == elevation_entity
        && elevation.validity().not_after().entity() == elevation_entity
        && elevation.validity().not_before().value_type()
            == elevation.validity().not_after().value_type()
        && elevation.validity().not_before().scalar_family()
            == elevation.validity().timeline().scalar_family()
        && elevation.validity().not_after().scalar_family()
            == elevation.validity().timeline().scalar_family()
        && elevation_duration_is_valid(elevation)
        && elevation
            .states()
            .values()
            .into_iter()
            .all(|state| state.field() == elevation.status())
        && elevation.requester().from() == principal
        && elevation.requester().to() == elevation_entity
        && elevation.approver().from() == principal
        && elevation.approver().to() == elevation_entity
        && elevation.grant().from() == elevation_entity
        && elevation.grant().to() == contract.grant_entity()
        && review.relation().from() == elevation_entity
        && review.relation().to() == review_entity
        && review_entity != elevation_entity
        && review.status().entity() == review_entity
        && review.reviewer().from() == principal
        && review.reviewer().to() == review_entity
        && review.required().field() == review.status()
        && review.completed().field() == review.status()
        && distinct_elevation_states(elevation)
        && review.required().value() != review.completed().value()
        && lifecycle.elevation_slot().context() == contract.constraints().context()
        && lifecycle.elevation_slot().context_type() == contract.constraints().context_type()
        && lifecycle.elevation_slot().entity() == elevation_entity
        && lifecycle.review_slot().context() == contract.constraints().context()
        && lifecycle.review_slot().context_type() == contract.constraints().context_type()
        && lifecycle.review_slot().entity() == review_entity
        && distinct_lifecycle_operations(contract)
}

fn distinct_elevation_states(
    elevation: &crate::application_capability::ApplicationCapabilityElevationDefinition,
) -> bool {
    elevation
        .states()
        .values()
        .into_iter()
        .map(|state| state.value())
        .collect::<BTreeSet<_>>()
        .len()
        == 4
}

fn elevation_duration_is_valid(
    elevation: &crate::application_capability::ApplicationCapabilityElevationDefinition,
) -> bool {
    let duration = elevation.maximum_duration();
    if duration.is_zero() {
        return false;
    }
    match elevation.validity().timeline() {
        crate::application_capability::ApplicationCapabilityValidityTimeline::UnixEpochSeconds => {
            duration.subsec_nanos() == 0
        }
        crate::application_capability::ApplicationCapabilityValidityTimeline::UnixEpochMilliseconds => {
            duration.subsec_nanos().is_multiple_of(1_000_000)
        }
    }
}

fn distinct_lifecycle_operations(contract: &ErasedApplicationCapabilityContract) -> bool {
    let ApplicationCapabilityElevationRule::Governed(elevation) = contract.elevation() else {
        return true;
    };
    let operations = elevation.lifecycle().operations();
    operations
        .iter()
        .map(|operation| (operation.operation(), operation.input_type()))
        .collect::<BTreeSet<_>>()
        .len()
        == operations.len()
        && operations.into_iter().all(|operation| {
            operation.operation() != contract.operation()
                || operation.input_type() != contract.input_type()
        })
}

fn capability_principal(contract: &ErasedApplicationCapabilityContract) -> Option<&str> {
    contract
        .composition()
        .decision()
        .allow()
        .graph()
        .requirements()
        .first()
        .and_then(|requirement| requirement.clauses().first())
        .map(|clause| clause.path().principal_entity())
}

fn operation_exists(
    members: &[ApplicationSchemaMember],
    contract: &ErasedApplicationCapabilityContract,
) -> bool {
    members.iter().any(|member| {
        matches!(
            member,
            ApplicationSchemaMember::Operation {
                operation,
                input_type,
            } if operation == contract.operation() && input_type == contract.input_type()
        )
    })
}

fn operation_binding_exists(
    members: &[ApplicationSchemaMember],
    binding: &crate::application_capability::ApplicationCapabilityOperationBinding,
) -> bool {
    members.iter().any(|member| {
        matches!(
            member,
            ApplicationSchemaMember::Operation {
                operation,
                input_type,
            } if operation == binding.operation()
                && input_type == binding.input_type()
        )
    })
}

fn entity_exists(members: &[ApplicationSchemaMember], entity: &str) -> bool {
    members.iter().any(
        |member| matches!(member, ApplicationSchemaMember::Entity { entity: found } if found == entity),
    )
}

fn field_exists(
    members: &[ApplicationSchemaMember],
    binding: &ApplicationCapabilityFieldBinding,
) -> bool {
    members.iter().any(|member| {
        matches!(
            member,
            ApplicationSchemaMember::Field {
                entity,
                aspect,
                field,
                scalar_family,
                value_type,
                ..
            } if entity == binding.entity()
                && aspect == binding.aspect()
                && field == binding.field()
                && *scalar_family == binding.scalar_family()
                && value_type == binding.value_type()
        )
    })
}

fn relation_exists(
    members: &[ApplicationSchemaMember],
    binding: &ApplicationCapabilityRelationBinding,
) -> bool {
    members.iter().any(|member| {
        matches!(
            member,
            ApplicationSchemaMember::Relation { relation, from, to }
                if relation == binding.relation()
                    && from == binding.from()
                    && to == binding.to()
        )
    })
}

fn field_dimension_exists(
    members: &[ApplicationSchemaMember],
    dimension: &ApplicationCapabilityFieldDimension,
) -> bool {
    match dimension {
        ApplicationCapabilityFieldDimension::NotApplicable => true,
        ApplicationCapabilityFieldDimension::Bound(field) => field_exists(members, field),
    }
}

fn relation_dimension_exists(
    members: &[ApplicationSchemaMember],
    dimension: &ApplicationCapabilityRelationDimension,
) -> bool {
    match dimension {
        ApplicationCapabilityRelationDimension::NotApplicable => true,
        ApplicationCapabilityRelationDimension::Bound(relation) => {
            relation_exists(members, relation)
        }
    }
}

fn field_dimension_belongs_to(
    dimension: &ApplicationCapabilityFieldDimension,
    grant_entity: &str,
) -> bool {
    match dimension {
        ApplicationCapabilityFieldDimension::NotApplicable => true,
        ApplicationCapabilityFieldDimension::Bound(field) => field.entity() == grant_entity,
    }
}
