use std::marker::PhantomData;

use worth_query_declaration::facade::application_schema::{
    ApplicationSchema, ApplicationSchemaBindingIdentity, ApplicationSchemaMember,
};

use crate::application_schema::WorthQueryInstalledApplicationSchema;
use crate::authority_cryptography::{
    AuthoritySeal, AuthoritySealDomain, AuthorityTranscript, PackageAuthorityKey,
};
use crate::installed_index::WorthQueryInstalledPackageAuthority;

use super::contract_resolution::{
    ability_requirement_meaning_matches, ability_requirements, operation_decision_fact_budget,
    operation_decision_reads, operation_decision_reads_from_members,
    operation_mutation_preconditions, operation_program, operation_program_from_members,
    operation_projection_work_budget,
};
use super::{
    WorthQueryApplicationOperationInstallationDenial,
    WorthQueryApplicationOperationInstallationDenialKind,
    WorthQueryCompiledApplicationOperationContracts,
};

pub struct WorthQueryInstalledApplicationOperation<Schema, Operation, Input> {
    binding_identity: ApplicationSchemaBindingIdentity,
    owner: String,
    schema_name: String,
    operation: String,
    input_type: String,
    contracts: WorthQueryCompiledApplicationOperationContracts,
    authority_identity: AuthoritySeal,
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
        let decision_reads = operation_decision_reads(schema, operation);
        let mutation_preconditions = super::precondition_contract::compile_precondition_contract(
            operation_mutation_preconditions(schema.installed_declaration().members(), operation),
            &decision_reads,
            &abilities,
        )
        .map_err(|()| {
            operation_denial(
                WorthQueryApplicationOperationInstallationDenialKind::InvalidMutationPreconditionContract,
                operation,
            )
        })?;
        if program.is_empty() && decision_reads.is_empty() {
            return Err(operation_denial(
                WorthQueryApplicationOperationInstallationDenialKind::MissingProgram,
                operation,
            ));
        }
        let decision_fact_budget =
            operation_decision_fact_budget(schema.installed_declaration().members(), operation)
                .ok_or_else(|| {
                    operation_denial(
                WorthQueryApplicationOperationInstallationDenialKind::MissingDecisionFactBudget,
                operation,
            )
                })?;
        let projection_work_budget =
            operation_projection_work_budget(schema.installed_declaration().members(), operation)
                .ok_or_else(|| {
                operation_denial(
                WorthQueryApplicationOperationInstallationDenialKind::MissingProjectionWorkBudget,
                operation,
            )
            })?;
        let contracts = WorthQueryCompiledApplicationOperationContracts::compile(
            abilities,
            program,
            decision_reads,
            decision_fact_budget,
            projection_work_budget,
            mutation_preconditions,
        );
        let binding_identity = schema.binding_identity();
        let authority_identity = authority_identity(
            &schema.package_authority.authority_key,
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
        self.authority_identity.as_str()
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
        let Some(decision_fact_budget) = operation_decision_fact_budget(members, &self.operation)
        else {
            return false;
        };
        let Some(projection_work_budget) =
            operation_projection_work_budget(members, &self.operation)
        else {
            return false;
        };
        operation_matches
            && ability_requirement_meaning_matches(
                members,
                &self.operation,
                self.contracts.ability_requirements(),
            )
            && {
                let requirements = self.contracts.ability_requirements().to_vec();
                let decision_reads =
                    operation_decision_reads_from_members(members, &self.operation);
                let Ok(mutation_preconditions) =
                    super::precondition_contract::compile_precondition_contract(
                        operation_mutation_preconditions(members, &self.operation),
                        &decision_reads,
                        &requirements,
                    )
                else {
                    return false;
                };
                WorthQueryCompiledApplicationOperationContracts::compile(
                    requirements,
                    operation_program_from_members(members, &self.operation),
                    decision_reads,
                    decision_fact_budget,
                    projection_work_budget,
                    mutation_preconditions,
                ) == self.contracts
            }
    }

    pub(crate) fn authority_matches(&self, package: &WorthQueryInstalledPackageAuthority) -> bool {
        authority_transcript(
            &package.authority_key,
            &self.binding_identity,
            &self.operation,
            &self.input_type,
        )
        .verifies(&self.authority_identity)
    }
}

fn authority_identity(
    key: &PackageAuthorityKey,
    identity: &ApplicationSchemaBindingIdentity,
    operation: &str,
    input_type: &str,
) -> AuthoritySeal {
    authority_transcript(key, identity, operation, input_type).finish()
}

fn authority_transcript(
    key: &PackageAuthorityKey,
    identity: &ApplicationSchemaBindingIdentity,
    operation: &str,
    input_type: &str,
) -> AuthorityTranscript {
    let mut transcript =
        AuthorityTranscript::new(key, AuthoritySealDomain::InstalledApplicationOperation);
    transcript.bytes("package", identity.package_identity().bytes());
    transcript.bytes("schema", identity.schema_identity().bytes());
    transcript.text("operation", operation);
    transcript.text("input-type", input_type);
    transcript
}

pub(super) fn operation_denial(
    kind: WorthQueryApplicationOperationInstallationDenialKind,
    operation: &str,
) -> WorthQueryApplicationOperationInstallationDenial {
    WorthQueryApplicationOperationInstallationDenial::new(kind, operation)
}
