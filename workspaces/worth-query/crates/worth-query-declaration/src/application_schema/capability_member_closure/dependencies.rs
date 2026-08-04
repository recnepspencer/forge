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
        && elevation_dependencies_exist(members, contract.elevation())
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
    elevation: &ApplicationCapabilityElevationRule,
) -> bool {
    let ApplicationCapabilityElevationRule::Governed(elevation) = elevation else {
        return true;
    };
    let review = elevation.review();
    [
        elevation.identity(),
        elevation.reason(),
        elevation.status(),
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
    elevation.reason().entity() == elevation_entity
        && elevation.status().entity() == elevation_entity
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
        && review.status().entity() == review_entity
        && review.reviewer().from() == principal
        && review.reviewer().to() == review_entity
        && review.required().field() == review.status()
        && review.completed().field() == review.status()
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
