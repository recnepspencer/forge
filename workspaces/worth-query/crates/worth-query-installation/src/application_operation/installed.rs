use std::marker::PhantomData;

use worth_query_declaration::facade::application_schema::{
    ApplicationSchema, ApplicationSchemaBindingIdentity, ApplicationSchemaMember,
};

use crate::application_schema::WorthQueryInstalledApplicationSchema;
use crate::authority_cryptography::AuthoritySeal;
use crate::graph_obligation::{
    bind_capability_operation_obligations, bind_operation_obligations,
    WorthQueryApplicationOperationObligationSource, WorthQueryInstalledGraphCapabilityRequirement,
    WorthQueryInstalledGraphObligationInspection, WorthQueryInstalledGraphObligationSet,
};
use crate::installed_index::WorthQueryInstalledPackageAuthority;

use super::contract_resolution::{
    ability_requirement_meaning_matches, ability_requirements, operation_decision_fact_budget,
    operation_decision_reads, operation_decision_reads_from_members, operation_execution_posture,
    operation_mutation_preconditions, operation_program, operation_program_from_members,
    operation_projection_work_budget,
};
use super::installed_contract_support::{
    authority_identity, authority_transcript, graph_obligation_denial, operation_authorization,
    operation_capability_requirements, operation_denial,
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
    obligations: WorthQueryInstalledGraphObligationSet,
    authority_identity: AuthoritySeal,
    _marker: PhantomData<fn(Input) -> (Schema, Operation)>,
}

/// Installation-owned graph authority for an operation named by a capability.
///
/// This view intentionally does not grant executable operation authority. A
/// capability may name an operation that is used only as an authorization
/// target, such as a governed application query.
pub struct WorthQueryInstalledApplicationOperationGraphAuthority<Schema, Operation, Input> {
    binding_identity: ApplicationSchemaBindingIdentity,
    operation: String,
    contracts: Option<WorthQueryCompiledApplicationOperationContracts>,
    obligations: WorthQueryInstalledGraphObligationSet,
    authority_identity: AuthoritySeal,
    _marker: PhantomData<fn(Input) -> (Schema, Operation)>,
}

impl<Schema, Operation, Input>
    WorthQueryInstalledApplicationOperationGraphAuthority<Schema, Operation, Input>
{
    pub fn binding_identity(&self) -> &ApplicationSchemaBindingIdentity {
        &self.binding_identity
    }

    pub fn operation(&self) -> &str {
        &self.operation
    }

    pub fn authority_identity(&self) -> &str {
        self.authority_identity.as_str()
    }

    pub const fn graph_obligations(&self) -> WorthQueryInstalledGraphObligationInspection<'_> {
        self.obligations.inspect()
    }

    #[doc(hidden)]
    pub fn retain_graph_obligations_for_admission(&self) -> WorthQueryInstalledGraphObligationSet {
        self.obligations.clone()
    }

    #[doc(hidden)]
    pub fn contracts(&self) -> Option<&WorthQueryCompiledApplicationOperationContracts> {
        self.contracts.as_ref()
    }
}

impl<Schema, Operation, Input> WorthQueryInstalledApplicationOperation<Schema, Operation, Input> {
    pub(crate) fn from_installed_schema(
        schema: &WorthQueryInstalledApplicationSchema<Schema>,
        operation: &str,
    ) -> Result<Self, WorthQueryApplicationOperationInstallationDenial>
    where
        Schema: ApplicationSchema,
    {
        Self::compile_from_installed_schema(schema, operation, true)
    }

    pub(crate) fn graph_authority_from_installed_schema<Capability>(
        schema: &WorthQueryInstalledApplicationSchema<Schema>,
        capability: &crate::application_capability::WorthQueryInstalledApplicationCapability<
            Schema,
            Capability,
            Operation,
            Input,
        >,
    ) -> Result<
        WorthQueryInstalledApplicationOperationGraphAuthority<Schema, Operation, Input>,
        WorthQueryApplicationOperationInstallationDenial,
    >
    where
        Schema: ApplicationSchema,
    {
        let operation = capability.contract().operation();
        match Self::compile_from_installed_schema(schema, operation, false) {
            Ok(installed) => Ok(WorthQueryInstalledApplicationOperationGraphAuthority {
                binding_identity: installed.binding_identity,
                operation: installed.operation,
                contracts: Some(installed.contracts),
                obligations: installed.obligations,
                authority_identity: installed.authority_identity,
                _marker: PhantomData,
            }),
            Err(denial)
                if matches!(
                    denial.kind(),
                    WorthQueryApplicationOperationInstallationDenialKind::MissingProgram
                        | WorthQueryApplicationOperationInstallationDenialKind::MissingDecisionFactBudget
                        | WorthQueryApplicationOperationInstallationDenialKind::MissingProjectionWorkBudget
                ) =>
            {
                let binding_identity = schema.binding_identity();
                let requirement = WorthQueryInstalledGraphCapabilityRequirement::new(
                    capability.identity().clone(),
                    capability.contract().clone(),
                );
                let obligations = bind_capability_operation_obligations(
                    &binding_identity,
                    operation,
                    capability.contract().input_type(),
                    requirement,
                )
                .map_err(|denial| graph_obligation_denial(operation, denial))?;
                let authority_identity = authority_identity(
                    &schema.package_authority.authority_key,
                    &binding_identity,
                    operation,
                    capability.contract().input_type(),
                    obligations.identity(),
                );
                Ok(WorthQueryInstalledApplicationOperationGraphAuthority {
                    binding_identity,
                    operation: operation.to_owned(),
                    contracts: None,
                    obligations,
                    authority_identity,
                    _marker: PhantomData,
                })
            }
            Err(denial) => Err(denial),
        }
    }

    fn compile_from_installed_schema(
        schema: &WorthQueryInstalledApplicationSchema<Schema>,
        operation: &str,
        require_executable_program: bool,
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
        let authorization = operation_authorization(
            operation,
            abilities.len(),
            schema.installed_capability_count_for_operation(operation, input_type),
        )?;
        let program = operation_program(schema, operation, input_type);
        let decision_reads = operation_decision_reads(schema, operation, input_type);
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
        if require_executable_program && program.is_empty() && decision_reads.is_empty() {
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
            authorization,
            abilities,
            program,
            decision_reads,
            decision_fact_budget,
            projection_work_budget,
            schema.progression_support_fact_count(operation, input_type),
            mutation_preconditions,
            operation_execution_posture(
                schema.installed_declaration().members(),
                operation,
                input_type,
            ),
        );
        let binding_identity = schema.binding_identity();
        let capability_requirements =
            operation_capability_requirements(schema, operation, input_type);
        let obligations = bind_operation_obligations(
            &binding_identity,
            operation,
            input_type,
            WorthQueryApplicationOperationObligationSource {
                authorization,
                ability_requirements: contracts.ability_requirements(),
                capability_requirements: &capability_requirements,
                graph_reads: contracts.graph_reads(),
                touches: contracts.touches(),
                effects: contracts.effects(),
                invariants: contracts.invariants(),
                invariant_execution: contracts.invariant_execution(),
                resources: contracts.resources(),
            },
        )
        .map_err(|denial| graph_obligation_denial(operation, denial))?;
        let authority_identity = authority_identity(
            &schema.package_authority.authority_key,
            &binding_identity,
            operation,
            input_type,
            obligations.identity(),
        );
        Ok(Self {
            binding_identity,
            owner: schema.owner().to_string(),
            schema_name: schema.schema_name().to_string(),
            operation: operation.to_string(),
            input_type: input_type.to_string(),
            contracts,
            obligations,
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

    pub const fn execution_posture(
        &self,
    ) -> super::WorthQueryInstalledApplicationOperationExecutionPosture {
        self.contracts.execution_posture()
    }

    pub fn authority_identity(&self) -> &str {
        self.authority_identity.as_str()
    }

    #[doc(hidden)]
    pub fn authority_identity_bytes(&self) -> [u8; 32] {
        *self.authority_identity.bytes()
    }

    pub const fn graph_obligations(&self) -> WorthQueryInstalledGraphObligationInspection<'_> {
        self.obligations.inspect()
    }

    #[doc(hidden)]
    pub fn retain_graph_obligations_for_admission(&self) -> WorthQueryInstalledGraphObligationSet {
        self.obligations.clone()
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
                let decision_reads = operation_decision_reads_from_members(
                    members,
                    &self.operation,
                    &self.input_type,
                );
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
                    self.contracts.authorization(),
                    requirements,
                    operation_program_from_members(members, &self.operation, &self.input_type),
                    decision_reads,
                    decision_fact_budget,
                    projection_work_budget,
                    self.contracts.additional_authorization_fact_count(),
                    mutation_preconditions,
                    operation_execution_posture(members, &self.operation, &self.input_type),
                ) == self.contracts
            }
    }

    pub(crate) fn authority_matches(&self, package: &WorthQueryInstalledPackageAuthority) -> bool {
        authority_transcript(
            &package.authority_key,
            &self.binding_identity,
            &self.operation,
            &self.input_type,
            self.obligations.identity(),
        )
        .verifies(&self.authority_identity)
    }
}
