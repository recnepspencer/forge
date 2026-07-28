use std::marker::PhantomData;

use sha2::{Digest, Sha256};
use worth_query_declaration::facade::application_schema::{
    ApplicationOperationProgramTarget, ApplicationSchema, ApplicationSchemaBindingIdentity,
    ApplicationSchemaMember,
};

use crate::application_schema::WorthQueryInstalledApplicationSchema;
use crate::installed_index::WorthQueryInstalledPackageAuthority;

use super::{
    WorthQueryApplicationOperationInstallationDenial,
    WorthQueryApplicationOperationInstallationDenialKind,
    WorthQueryCompiledApplicationOperationContracts, WorthQueryInstalledAbilityRequirement,
};

pub struct WorthQueryInstalledApplicationOperation<Schema, Operation, Input> {
    binding_identity: ApplicationSchemaBindingIdentity,
    owner: String,
    schema_name: String,
    operation: String,
    input_type: String,
    contracts: WorthQueryCompiledApplicationOperationContracts,
    authority_identity: String,
    _marker: PhantomData<fn(Input) -> (Schema, Operation)>,
}

impl<Schema, Operation, Input> WorthQueryInstalledApplicationOperation<Schema, Operation, Input> {
    pub(crate) fn from_installed_schema(
        schema: &WorthQueryInstalledApplicationSchema<Schema>,
        operation: &str,
    ) -> Result<Self, WorthQueryApplicationOperationInstallationDenial>
    where
        Schema: ApplicationSchema,
    {
        let input_type = std::any::type_name::<Input>();
        let operation_installed = schema
            .installed_declaration()
            .members()
            .iter()
            .any(|member| {
                matches!(
                    member,
                    ApplicationSchemaMember::Operation {
                        operation: installed,
                        input_type: installed_input,
                    } if installed == operation && installed_input == input_type
                )
            });
        if !operation_installed {
            return Err(operation_denial(
                WorthQueryApplicationOperationInstallationDenialKind::OperationNotInstalled,
                operation,
            ));
        }
        let abilities = ability_requirements(schema, operation)?;
        let program = operation_program(schema, operation);
        if program.is_empty() {
            return Err(operation_denial(
                WorthQueryApplicationOperationInstallationDenialKind::MissingProgram,
                operation,
            ));
        }
        let contracts =
            WorthQueryCompiledApplicationOperationContracts::compile(abilities, program);
        let binding_identity = schema.binding_identity();
        let authority_identity = authority_identity(
            &schema.package_authority.authority_nonce,
            &binding_identity,
            operation,
            input_type,
        );
        Ok(Self {
            binding_identity,
            owner: schema.owner().to_string(),
            schema_name: schema.schema_name().to_string(),
            operation: operation.to_string(),
            input_type: input_type.to_string(),
            contracts,
            authority_identity,
            _marker: PhantomData,
        })
    }

    pub fn binding_identity(&self) -> &ApplicationSchemaBindingIdentity {
        &self.binding_identity
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    pub fn operation(&self) -> &str {
        &self.operation
    }

    pub fn contracts(&self) -> &WorthQueryCompiledApplicationOperationContracts {
        &self.contracts
    }

    pub fn authority_identity(&self) -> &str {
        &self.authority_identity
    }

    pub(crate) fn meaning_matches(&self, members: &[ApplicationSchemaMember]) -> bool {
        let operation_matches = members.iter().any(|member| {
            matches!(
                member,
                ApplicationSchemaMember::Operation {
                    operation,
                    input_type,
                } if operation == &self.operation && input_type == &self.input_type
            )
        });
        operation_matches
            && operation_program_from_members(members, &self.operation) == self.contracts.program()
            && ability_requirements_from_members(members, &self.operation)
                .is_ok_and(|requirements| requirements == self.contracts.ability_requirements())
    }

    pub(crate) fn authority_matches(&self, package: &WorthQueryInstalledPackageAuthority) -> bool {
        self.authority_identity
            == authority_identity(
                &package.authority_nonce,
                &self.binding_identity,
                &self.operation,
                &self.input_type,
            )
    }
}

fn ability_requirements<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    operation: &str,
) -> Result<
    Vec<WorthQueryInstalledAbilityRequirement>,
    WorthQueryApplicationOperationInstallationDenial,
>
where
    Schema: ApplicationSchema,
{
    let requirements =
        ability_requirements_from_members(schema.installed_declaration().members(), operation)?;
    if requirements.is_empty() {
        return Err(operation_denial(
            WorthQueryApplicationOperationInstallationDenialKind::MissingAbility,
            operation,
        ));
    }
    Ok(requirements)
}

fn ability_requirements_from_members(
    members: &[ApplicationSchemaMember],
    operation: &str,
) -> Result<
    Vec<WorthQueryInstalledAbilityRequirement>,
    WorthQueryApplicationOperationInstallationDenial,
> {
    let mut requirements = Vec::new();
    for member in members {
        let requirement = match member {
            ApplicationSchemaMember::OperationAbility {
                operation: installed,
                ability,
                scope_entity,
            } if installed == operation => {
                let (policy, policy_paths) = members
                    .iter()
                    .find_map(|candidate| match candidate {
                        ApplicationSchemaMember::AbilityPolicy {
                            ability: policy_ability,
                            scope_entity: policy_scope,
                            policy,
                            paths,
                        } if policy_ability == ability && policy_scope == scope_entity => {
                            Some((policy.clone(), paths.clone()))
                        }
                        _ => None,
                    })
                    .ok_or_else(|| {
                        operation_denial(
                            WorthQueryApplicationOperationInstallationDenialKind::MissingAbilityPolicy,
                            operation,
                        )
                    })?;
                Some(WorthQueryInstalledAbilityRequirement::new(
                    ability.clone(),
                    scope_entity.clone(),
                    policy,
                    policy_paths,
                ))
            }
            _ => None,
        };
        requirements.extend(requirement);
    }
    requirements.sort();
    requirements.dedup();
    Ok(requirements)
}

fn operation_program<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    operation: &str,
) -> Vec<ApplicationOperationProgramTarget>
where
    Schema: ApplicationSchema,
{
    operation_program_from_members(schema.installed_declaration().members(), operation)
}

fn operation_program_from_members(
    members: &[ApplicationSchemaMember],
    operation: &str,
) -> Vec<ApplicationOperationProgramTarget> {
    let mut program = members
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::OperationProgram {
                operation: installed,
                target,
            } if installed == operation => Some(target.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    program.sort();
    program.dedup();
    program
}

fn authority_identity(
    nonce: &[u8; 32],
    identity: &ApplicationSchemaBindingIdentity,
    operation: &str,
    input_type: &str,
) -> String {
    let mut hash = Sha256::new();
    hash.update(b"worth-query-installed-application-operation-v1");
    hash.update(nonce);
    for value in [
        identity.package_identity(),
        identity.schema_identity().as_str(),
        operation,
        input_type,
    ] {
        hash.update(value.len().to_le_bytes());
        hash.update(value.as_bytes());
    }
    format!("{:x}", hash.finalize())
}

fn operation_denial(
    kind: WorthQueryApplicationOperationInstallationDenialKind,
    operation: &str,
) -> WorthQueryApplicationOperationInstallationDenial {
    WorthQueryApplicationOperationInstallationDenial::new(kind, operation)
}
