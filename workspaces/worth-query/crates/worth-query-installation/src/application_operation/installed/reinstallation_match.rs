//! Whether an already-installed operation still matches a re-presented schema.
//!
//! Reinstallation must refuse silent drift. Both checks below recompute from
//! the newly presented members and the presented package authority, and
//! compare against what installation already sealed.

use worth_query_declaration::facade::application_schema::ApplicationSchemaMember;

use crate::installed_index::WorthQueryInstalledPackageAuthority;

use super::super::contract_resolution::{
    ability_requirement_meaning_matches, operation_aftermath, operation_decision_fact_budget,
    operation_decision_reads_from_members, operation_execution_posture, operation_external_effect,
    operation_mutation_preconditions, operation_program_from_members,
    operation_projection_work_budget,
};
use super::super::contracts::WorthQueryApplicationOperationContractSources;
use super::super::installed_contract_support::authority_transcript;
use super::super::WorthQueryCompiledApplicationOperationContracts;
use super::WorthQueryInstalledApplicationOperation;

impl<Schema, Operation, Input> WorthQueryInstalledApplicationOperation<Schema, Operation, Input> {
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
            && self.recompiled_contracts_match(
                members,
                decision_fact_budget,
                projection_work_budget,
            )
    }

    fn recompiled_contracts_match(
        &self,
        members: &[ApplicationSchemaMember],
        decision_fact_budget: usize,
        projection_work_budget: usize,
    ) -> bool {
        let requirements = self.contracts.ability_requirements().to_vec();
        let decision_reads =
            operation_decision_reads_from_members(members, &self.operation, &self.input_type);
        let Ok(mutation_preconditions) =
            super::super::precondition_contract::compile_precondition_contract(
                operation_mutation_preconditions(members, &self.operation),
                &decision_reads,
                &requirements,
            )
        else {
            return false;
        };
        let Ok(external_effect) = operation_external_effect(members, &self.operation) else {
            return false;
        };
        let Ok(aftermath) = recompile_aftermath(
            &self.binding_identity,
            &self.operation,
            members,
            &decision_reads,
            &external_effect,
        ) else {
            return false;
        };
        WorthQueryCompiledApplicationOperationContracts::compile(
            WorthQueryApplicationOperationContractSources {
                authorization: self.contracts.authorization(),
                ability_requirements: requirements,
                program: operation_program_from_members(members, &self.operation, &self.input_type),
                decision_reads,
                decision_fact_budget,
                projection_work_budget,
                additional_authorization_fact_count: self
                    .contracts
                    .additional_authorization_fact_count(),
                mutation_preconditions,
                execution_posture: operation_execution_posture(
                    members,
                    &self.operation,
                    &self.input_type,
                ),
                external_effect,
                aftermath,
            },
        ) == self.contracts
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

/// Recompiles the aftermath from the candidate members for a match comparison.
///
/// The escaping lane is resolved from the same candidate members, so a
/// reinstallation that moved the external effect recompiles to a different
/// aftermath identity and fails to match (Q8.25-C1).
fn recompile_aftermath(
    binding: &worth_query_declaration::facade::application_schema::ApplicationSchemaBindingIdentity,
    operation: &str,
    members: &[ApplicationSchemaMember],
    decision_reads: &[worth_query_declaration::facade::application_schema::ApplicationOperationDecisionReadTarget],
    external_effect: &crate::application_aftermath::InstalledExternalEffectContract,
) -> Result<Option<crate::application_aftermath::WorthQueryInstalledAftermathContract>, ()> {
    use worth_query_declaration::facade::application_schema::ApplicationOperationDecisionReadTarget;

    let Some(declared) = operation_aftermath(members, operation).map_err(|_| ())? else {
        return Ok(None);
    };
    let declared_reads =
        crate::application_aftermath::OperationDeclaredReadFields::from_field_slots(
            decision_reads.iter().filter_map(|target| match target {
                ApplicationOperationDecisionReadTarget::Field { field, .. } => Some(field.as_str()),
                _ => None,
            }),
        );
    let catalog = crate::application_aftermath::derived_lowering_catalog(binding, &declared)
        .map_err(|_| ())?;
    crate::application_aftermath::install_application_aftermath(
        crate::application_aftermath::OperationAftermathInstallation {
            binding,
            operation_slot: operation,
            declared: &declared,
            declared_reads: &declared_reads,
            external_effect,
            lowering_catalog: &catalog,
        },
    )
    .map(Some)
    .map_err(|_| ())
}
