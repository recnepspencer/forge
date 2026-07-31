use worth_query_declaration::facade::application_schema::{
    ApplicationMutationPreconditionTarget, ApplicationOperationDecisionReadTarget,
    ApplicationOperationProgramTarget, ApplicationSchema, ApplicationSchemaMember,
};

use crate::application_schema::WorthQueryInstalledApplicationSchema;

use super::installed::operation_denial;
use super::{
    WorthQueryApplicationOperationInstallationDenial,
    WorthQueryApplicationOperationInstallationDenialKind, WorthQueryInstalledAbilityRequirement,
};

pub(super) fn ability_requirement_meaning_matches(
    members: &[ApplicationSchemaMember],
    operation: &str,
    installed: &[WorthQueryInstalledAbilityRequirement],
) -> bool {
    let mut declared = members
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::OperationAbility {
                operation: candidate,
                ability,
                scope_entity,
            } if candidate == operation => Some((ability.as_str(), scope_entity.as_str())),
            _ => None,
        })
        .collect::<Vec<_>>();
    declared.sort_unstable();
    declared.dedup();
    declared.len() == installed.len()
        && declared.into_iter().all(|(ability, scope_entity)| {
            installed.iter().any(|requirement| {
                requirement.ability() == ability
                    && requirement.scope_entity() == scope_entity
                    && members.iter().any(|member| {
                        matches!(
                            member,
                            ApplicationSchemaMember::AbilityPolicy {
                                ability: candidate_ability,
                                scope_entity: candidate_scope,
                                policy,
                                paths,
                            } if candidate_ability == ability
                                && candidate_scope == scope_entity
                                && policy == requirement.policy()
                                && paths.len() == requirement.policy_paths().len()
                                && paths.iter().zip(requirement.policy_paths()).all(
                                    |(path, installed_path)| path == installed_path.path()
                                )
                        )
                    })
            })
        })
}

pub(super) fn operation_projection_work_budget(
    members: &[ApplicationSchemaMember],
    operation: &str,
) -> Option<usize> {
    members.iter().find_map(|member| match member {
        ApplicationSchemaMember::OperationProjectionWorkBudget {
            operation: installed,
            maximum_work_units,
        } if installed == operation => Some(*maximum_work_units),
        _ => None,
    })
}

pub(super) fn operation_decision_fact_budget(
    members: &[ApplicationSchemaMember],
    operation: &str,
) -> Option<usize> {
    members.iter().find_map(|member| match member {
        ApplicationSchemaMember::OperationDecisionFactBudget {
            operation: installed,
            maximum_fact_count,
        } if installed == operation => Some(*maximum_fact_count),
        _ => None,
    })
}

pub(super) fn operation_decision_reads<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    operation: &str,
) -> Vec<ApplicationOperationDecisionReadTarget>
where
    Schema: ApplicationSchema,
{
    operation_decision_reads_from_members(schema.installed_declaration().members(), operation)
}

pub(super) fn operation_decision_reads_from_members(
    members: &[ApplicationSchemaMember],
    operation: &str,
) -> Vec<ApplicationOperationDecisionReadTarget> {
    let mut reads = members
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::OperationDecisionRead {
                operation: installed,
                target,
            } if installed == operation => Some(target.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    reads.sort();
    reads.dedup();
    reads
}

pub(super) fn operation_mutation_preconditions(
    members: &[ApplicationSchemaMember],
    operation: &str,
) -> Vec<ApplicationMutationPreconditionTarget> {
    members
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::OperationMutationPrecondition {
                operation: installed,
                target,
            } if installed == operation => Some(target.clone()),
            _ => None,
        })
        .collect()
}

pub(super) fn ability_requirements<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    operation: &str,
) -> Result<
    Vec<WorthQueryInstalledAbilityRequirement>,
    WorthQueryApplicationOperationInstallationDenial,
>
where
    Schema: ApplicationSchema,
{
    ability_requirements_from_schema(schema, operation)
}

fn ability_requirements_from_schema<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    operation: &str,
) -> Result<
    Vec<WorthQueryInstalledAbilityRequirement>,
    WorthQueryApplicationOperationInstallationDenial,
>
where
    Schema: ApplicationSchema,
{
    let mut requirements = Vec::new();
    for member in schema.installed_declaration().members() {
        let requirement = match member {
            ApplicationSchemaMember::OperationAbility {
                operation: installed,
                ability,
                scope_entity,
            } if installed == operation => {
                let requirement = schema
                    .installed_ability_requirement(ability, scope_entity)
                    .cloned()
                    .ok_or_else(|| {
                        operation_denial(
                            WorthQueryApplicationOperationInstallationDenialKind::MissingAbilityPolicy,
                            operation,
                        )
                    })?;
                Some(requirement)
            }
            _ => None,
        };
        requirements.extend(requirement);
    }
    requirements.sort();
    requirements.dedup();
    Ok(requirements)
}

pub(super) fn operation_program<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    operation: &str,
) -> Vec<ApplicationOperationProgramTarget>
where
    Schema: ApplicationSchema,
{
    operation_program_from_members(schema.installed_declaration().members(), operation)
}

pub(super) fn operation_program_from_members(
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
