use std::collections::BTreeSet;

use crate::application_capability::{
    ApplicationCapabilityElevationRule, ErasedApplicationCapabilityContract,
};

use super::member_closure::ClosureIndex;
use super::{ApplicationSchemaDeclarationDenial, ApplicationSchemaMember};

mod composition;
mod declared_dimensions;
mod dependencies;

use composition::composition_is_closed;
use declared_dimensions::DeclaredCapabilityDimensions;
use dependencies::{dependencies_are_closed, topology_is_valid};

const MAXIMUM_CAPABILITY_CONTRACTS: usize = 1_024;

pub(super) fn validate_application_capability_members(
    members: &[ApplicationSchemaMember],
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    let dimensions = DeclaredCapabilityDimensions::validate(members)?;
    let closure = ClosureIndex::new(members);
    let contracts = capability_contracts(members);
    if contracts.len() > MAXIMUM_CAPABILITY_CONTRACTS {
        return Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability);
    }
    if !lifecycle_operations_have_one_owner(&contracts) {
        return Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability);
    }
    let mut identities = BTreeSet::new();
    for contract in contracts {
        if !identities.insert((contract.name(), contract.capability_type())) {
            return Err(ApplicationSchemaDeclarationDenial::DuplicateApplicationCapability);
        }
        validate_contract(members, &closure, &dimensions, contract)?;
    }
    Ok(())
}

fn lifecycle_operations_have_one_owner(contracts: &[&ErasedApplicationCapabilityContract]) -> bool {
    let mut operations = BTreeSet::new();
    contracts.iter().all(|contract| {
        let ApplicationCapabilityElevationRule::Governed(elevation) = contract.elevation() else {
            return true;
        };
        elevation
            .lifecycle()
            .operations()
            .into_iter()
            .all(|operation| operations.insert((operation.operation(), operation.input_type())))
    })
}

fn capability_contracts(
    members: &[ApplicationSchemaMember],
) -> Vec<&ErasedApplicationCapabilityContract> {
    members
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::ApplicationCapability { contract } => Some(contract),
            _ => None,
        })
        .collect()
}

fn validate_contract(
    members: &[ApplicationSchemaMember],
    closure: &ClosureIndex<'_>,
    dimensions: &DeclaredCapabilityDimensions<'_>,
    contract: &ErasedApplicationCapabilityContract,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    dependencies_are_closed(members, dimensions, contract)?;
    if !topology_is_valid(contract) || !composition_is_closed(closure, dimensions, contract) {
        return Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability);
    }
    Ok(())
}
