mod aftermath_compilation;
mod reinstallation_match;

use std::marker::PhantomData;

use worth_query_declaration::facade::application_schema::{
    ApplicationSchema, ApplicationSchemaBindingIdentity,
};

use self::aftermath_compilation::compile_operation_aftermath;
use super::contract_resolution::{
    ability_requirements, operation_decision_fact_budget, operation_decision_reads,
    operation_execution_posture, operation_external_effect, operation_mutation_preconditions,
    operation_program, operation_projection_work_budget,
};
use super::contracts::WorthQueryApplicationOperationContractSources;
use super::installed_contract_support::{
    authority_identity, graph_obligation_denial, operation_authorization,
    operation_capability_requirements, operation_denial,
};
use super::operation_declaration_resolution::{
    resolve_operation_declaration, ResolvedApplicationOperationDeclaration,
};
use super::{
    WorthQueryApplicationOperationInstallationDenial,
    WorthQueryApplicationOperationInstallationDenialKind,
    WorthQueryCompiledApplicationOperationContracts,
};
use crate::application_schema::WorthQueryInstalledApplicationSchema;
use crate::authority_cryptography::AuthoritySeal;
use crate::graph_obligation::{
    bind_capability_operation_obligations, bind_operation_obligations,
    WorthQueryApplicationOperationObligationSource, WorthQueryInstalledGraphCapabilityRequirement,
    WorthQueryInstalledGraphObligationInspection, WorthQueryInstalledGraphObligationSet,
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
}

impl<Schema, Operation, Input> WorthQueryInstalledApplicationOperation<Schema, Operation, Input> {
    pub(crate) fn from_installed_schema(
        schema: &WorthQueryInstalledApplicationSchema<Schema>,
        operation: &str,
    ) -> Result<Self, WorthQueryApplicationOperationInstallationDenial>
    where
        Schema: ApplicationSchema,
    {
        let declaration = resolve_operation_declaration::<Schema, Input>(schema, operation)?;
        Self::install_executable_operation(schema, &declaration)
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
        let declaration = resolve_operation_declaration::<Schema, Input>(
            schema,
            capability.contract().operation(),
        )?;
        let binding_identity = schema.binding_identity();
        let requirement = WorthQueryInstalledGraphCapabilityRequirement::new(
            capability.identity().clone(),
            capability.contract().clone(),
        );
        let obligations = bind_capability_operation_obligations(
            &binding_identity,
            declaration.operation(),
            declaration.input_type(),
            requirement,
        )
        .map_err(|denial| graph_obligation_denial(declaration.operation(), denial))?;
        let authority_identity = authority_identity(
            &schema.package_authority.authority_key,
            &binding_identity,
            declaration.operation(),
            declaration.input_type(),
            obligations.identity(),
        );
        Ok(WorthQueryInstalledApplicationOperationGraphAuthority {
            binding_identity,
            operation: declaration.operation().to_owned(),
            obligations,
            authority_identity,
            _marker: PhantomData,
        })
    }

    fn install_executable_operation(
        schema: &WorthQueryInstalledApplicationSchema<Schema>,
        declaration: &ResolvedApplicationOperationDeclaration,
    ) -> Result<Self, WorthQueryApplicationOperationInstallationDenial>
    where
        Schema: ApplicationSchema,
    {
        let operation = declaration.operation();
        let input_type = declaration.input_type();
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
        // One resolution of the escaping lane, shared by the aftermath install
        // and the compiled contracts. The aftermath's external posture is a
        // projection of this value, never a second declaration (Q8.25-C1).
        let external_effect =
            operation_external_effect(schema.installed_declaration().members(), operation)
                .map_err(|denial| operation_denial(denial.installation_kind(), operation))?;
        let decision_reads_for_coverage = &decision_reads;
        let aftermath = compile_operation_aftermath(
            schema,
            operation,
            decision_reads_for_coverage,
            &external_effect,
        )?;
        let contracts = WorthQueryCompiledApplicationOperationContracts::compile(
            WorthQueryApplicationOperationContractSources {
                authorization,
                ability_requirements: abilities,
                program,
                decision_reads,
                decision_fact_budget,
                projection_work_budget,
                additional_authorization_fact_count: schema
                    .progression_support_fact_count(operation, input_type),
                mutation_preconditions,
                execution_posture: operation_execution_posture(
                    schema.installed_declaration().members(),
                    operation,
                    input_type,
                ),
                external_effect,
                aftermath,
            },
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
}
