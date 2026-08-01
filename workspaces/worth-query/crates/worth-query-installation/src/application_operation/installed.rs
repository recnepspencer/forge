use std::marker::PhantomData;

use worth_query_declaration::facade::application_schema::{
    ApplicationSchema, ApplicationSchemaBindingIdentity, ApplicationSchemaMember,
};

use crate::application_schema::WorthQueryInstalledApplicationSchema;
use crate::authority_cryptography::{
    AuthoritySeal, AuthoritySealDomain, AuthorityTranscript, PackageAuthorityKey,
};
use crate::graph_obligation::{
    capability_requirement, WorthQueryGraphObligationInstallationDenial,
};
use worth_foundational::facade::CanonicalDigestDerivationDenial;

use super::contract_resolution::{
    ability_requirements, operation_decision_fact_budget, operation_decision_reads,
    operation_mutation_preconditions, operation_program, operation_projection_work_budget,
};
use super::{
    WorthQueryApplicationOperationCompilationSource,
    WorthQueryApplicationOperationInstallationDenial,
    WorthQueryApplicationOperationInstallationDenialKind,
    WorthQueryCompiledApplicationOperationContracts,
    WorthQueryInstalledApplicationOperationAuthorization,
};

pub struct WorthQueryInstalledApplicationOperation<Schema, Operation, Input> {
    binding_identity: ApplicationSchemaBindingIdentity,
    owner: String,
    schema_name: String,
    operation: String,
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
        if !operation_is_installed(schema, operation, input_type) {
            return Err(operation_denial(
                WorthQueryApplicationOperationInstallationDenialKind::OperationNotInstalled,
                operation,
            ));
        }
        let binding_identity = schema.binding_identity();
        let contracts =
            compile_installed_contracts(schema, &binding_identity, operation, input_type)?;
        let authority_identity = authority_identity(
            &schema.package_authority.authority_key,
            &binding_identity,
            operation,
            input_type,
            contracts.obligations().identity().digest(),
        );
        Ok(Self {
            binding_identity,
            owner: schema.owner().to_string(),
            schema_name: schema.schema_name().to_string(),
            operation: operation.to_string(),
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
}

struct InstalledAuthorizationContracts {
    mode: WorthQueryInstalledApplicationOperationAuthorization,
    abilities: Vec<super::WorthQueryInstalledAbilityRequirement>,
    capabilities: Vec<crate::graph_obligation::WorthQueryInstalledGraphCapabilityRequirement>,
}

struct InstalledProgramContracts {
    program:
        Vec<worth_query_declaration::facade::application_schema::ApplicationOperationProgramTarget>,
    decision_reads: Vec<
        worth_query_declaration::facade::application_schema::ApplicationOperationDecisionReadTarget,
    >,
    mutation_preconditions: Vec<super::WorthQueryInstalledMutationPrecondition>,
    decision_fact_budget: usize,
    projection_work_budget: usize,
}

fn operation_is_installed<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    operation: &str,
    input_type: &str,
) -> bool
where
    Schema: ApplicationSchema,
{
    schema
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
        })
}

fn compile_installed_contracts<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    binding_identity: &ApplicationSchemaBindingIdentity,
    operation: &str,
    input_type: &str,
) -> Result<
    WorthQueryCompiledApplicationOperationContracts,
    WorthQueryApplicationOperationInstallationDenial,
>
where
    Schema: ApplicationSchema,
{
    let authorization = resolve_authorization_contracts(schema, operation, input_type)?;
    let program = resolve_program_contracts(schema, operation, &authorization.abilities)?;
    WorthQueryCompiledApplicationOperationContracts::compile(
        WorthQueryApplicationOperationCompilationSource {
            binding_identity,
            operation,
            input_type,
            authorization: authorization.mode,
            ability_requirements: authorization.abilities,
            capability_requirements: authorization.capabilities,
            program: program.program,
            decision_reads: program.decision_reads,
            decision_fact_budget: program.decision_fact_budget,
            projection_work_budget: program.projection_work_budget,
            mutation_preconditions: program.mutation_preconditions,
        },
    )
    .map_err(|denial| graph_obligation_denial(operation, denial))
}

fn resolve_authorization_contracts<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    operation: &str,
    input_type: &str,
) -> Result<InstalledAuthorizationContracts, WorthQueryApplicationOperationInstallationDenial>
where
    Schema: ApplicationSchema,
{
    let abilities = ability_requirements(schema, operation)?;
    let capabilities = schema
        .capability_plan_sources()
        .filter(|source| {
            source.contract().operation() == operation
                && source.contract().input_type() == input_type
        })
        .map(|source| capability_requirement(source.identity().clone(), source.contract().clone()))
        .collect::<Vec<_>>();
    let mode = operation_authorization(operation, abilities.len(), capabilities.len())?;
    Ok(InstalledAuthorizationContracts {
        mode,
        abilities,
        capabilities,
    })
}

fn resolve_program_contracts<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    operation: &str,
    abilities: &[super::WorthQueryInstalledAbilityRequirement],
) -> Result<InstalledProgramContracts, WorthQueryApplicationOperationInstallationDenial>
where
    Schema: ApplicationSchema,
{
    let members = schema.installed_declaration().members();
    let program = operation_program(schema, operation);
    let decision_reads = operation_decision_reads(schema, operation);
    if program.is_empty() && decision_reads.is_empty() {
        return Err(operation_denial(
            WorthQueryApplicationOperationInstallationDenialKind::MissingProgram,
            operation,
        ));
    }
    let mutation_preconditions = super::precondition_contract::compile_precondition_contract(
        operation_mutation_preconditions(members, operation),
        &decision_reads,
        abilities,
    )
    .map_err(|()| {
        operation_denial(
            WorthQueryApplicationOperationInstallationDenialKind::InvalidMutationPreconditionContract,
            operation,
        )
    })?;
    let decision_fact_budget =
        operation_decision_fact_budget(members, operation).ok_or_else(|| {
            operation_denial(
                WorthQueryApplicationOperationInstallationDenialKind::MissingDecisionFactBudget,
                operation,
            )
        })?;
    let projection_work_budget =
        operation_projection_work_budget(members, operation).ok_or_else(|| {
            operation_denial(
                WorthQueryApplicationOperationInstallationDenialKind::MissingProjectionWorkBudget,
                operation,
            )
        })?;
    Ok(InstalledProgramContracts {
        program,
        decision_reads,
        mutation_preconditions,
        decision_fact_budget,
        projection_work_budget,
    })
}

fn operation_authorization(
    operation: &str,
    ability_count: usize,
    capability_count: usize,
) -> Result<
    WorthQueryInstalledApplicationOperationAuthorization,
    WorthQueryApplicationOperationInstallationDenial,
> {
    match (ability_count > 0, capability_count > 0) {
        (true, true) => Err(operation_denial(
            WorthQueryApplicationOperationInstallationDenialKind::ConflictingAuthorizationContract,
            operation,
        )),
        (true, false) => Ok(WorthQueryInstalledApplicationOperationAuthorization::Abilities),
        (false, true) => Ok(WorthQueryInstalledApplicationOperationAuthorization::Capability),
        (false, false) => Ok(WorthQueryInstalledApplicationOperationAuthorization::Principal),
    }
}

fn authority_identity(
    key: &PackageAuthorityKey,
    identity: &ApplicationSchemaBindingIdentity,
    operation: &str,
    input_type: &str,
    obligation_identity: &worth_foundational::facade::CanonicalDigestId,
) -> AuthoritySeal {
    authority_transcript(key, identity, operation, input_type, obligation_identity).finish()
}

fn authority_transcript(
    key: &PackageAuthorityKey,
    identity: &ApplicationSchemaBindingIdentity,
    operation: &str,
    input_type: &str,
    obligation_identity: &worth_foundational::facade::CanonicalDigestId,
) -> AuthorityTranscript {
    let mut transcript =
        AuthorityTranscript::new(key, AuthoritySealDomain::InstalledApplicationOperation);
    transcript.bytes("package", identity.package_identity().bytes());
    transcript.bytes("schema", identity.schema_identity().bytes());
    transcript.text("operation", operation);
    transcript.text("input-type", input_type);
    transcript.bytes("graph-obligations", obligation_identity.bytes());
    transcript
}

fn graph_obligation_denial(
    operation: &str,
    denial: WorthQueryGraphObligationInstallationDenial,
) -> WorthQueryApplicationOperationInstallationDenial {
    let kind = match denial {
        WorthQueryGraphObligationInstallationDenial::InvalidContract => {
            WorthQueryApplicationOperationInstallationDenialKind::InvalidGraphObligationContract
        }
        WorthQueryGraphObligationInstallationDenial::Canonical(
            CanonicalDigestDerivationDenial::EntryLimitExceeded { .. },
        ) => WorthQueryApplicationOperationInstallationDenialKind::CanonicalEntryBudgetExceeded,
        WorthQueryGraphObligationInstallationDenial::Canonical(
            CanonicalDigestDerivationDenial::EncodedByteLimitExceeded { .. },
        ) => {
            WorthQueryApplicationOperationInstallationDenialKind::CanonicalEncodedByteBudgetExceeded
        }
        WorthQueryGraphObligationInstallationDenial::Canonical(_) => {
            WorthQueryApplicationOperationInstallationDenialKind::CanonicalDigestSlotRejected
        }
    };
    operation_denial(kind, operation)
}

pub(super) fn operation_denial(
    kind: WorthQueryApplicationOperationInstallationDenialKind,
    operation: &str,
) -> WorthQueryApplicationOperationInstallationDenial {
    WorthQueryApplicationOperationInstallationDenial::new(kind, operation)
}

#[cfg(test)]
mod authorization_mode_tests {
    use super::*;

    #[test]
    fn operation_authorization_mode_is_an_exclusive_installed_lattice() {
        assert_eq!(
            operation_authorization("operation", 0, 0).unwrap(),
            WorthQueryInstalledApplicationOperationAuthorization::Principal
        );
        assert_eq!(
            operation_authorization("operation", 1, 0).unwrap(),
            WorthQueryInstalledApplicationOperationAuthorization::Abilities
        );
        assert_eq!(
            operation_authorization("operation", 0, 1).unwrap(),
            WorthQueryInstalledApplicationOperationAuthorization::Capability
        );
        let denial = operation_authorization("operation", 1, 1).unwrap_err();
        assert_eq!(
            denial.kind(),
            WorthQueryApplicationOperationInstallationDenialKind::ConflictingAuthorizationContract
        );
    }
}
