use crate::application_capability::{
    ApplicationCapabilityFieldBinding, ApplicationCapabilityFieldDimension,
    ApplicationCapabilityRelationBinding, ApplicationCapabilityRelationDimension,
    ErasedApplicationCapabilityContract,
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
        && field_exists(members, contract.constraints().workflow_stage())
        && field_exists(members, contract.constraints().validity().not_before())
        && field_exists(members, contract.constraints().validity().not_after())
        && relation_exists(members, contract.delegation().parent())
        && relation_exists(members, contract.delegation().grantor())
        && relation_exists(members, contract.delegation().grantee())
        && field_exists(members, contract.delegation().limit())
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
    let delegation = contract.delegation();
    target.action().field().entity() == grant
        && target.purpose().field().entity() == grant
        && target.resource().from() == grant
        && constraints.workflow_stage().entity() == grant
        && constraints.validity().not_before().entity() == grant
        && constraints.validity().not_after().entity() == grant
        && delegation.limit().entity() == grant
        && delegation.parent().from() == grant
        && delegation.parent().to() == grant
        && delegation.grantor().to() == grant
        && delegation.grantee().to() == grant
        && field_dimension_belongs_to(target.field(), grant)
        && field_dimension_belongs_to(constraints.amount(), grant)
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
                value_type,
                ..
            } if entity == binding.entity()
                && aspect == binding.aspect()
                && field == binding.field()
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
