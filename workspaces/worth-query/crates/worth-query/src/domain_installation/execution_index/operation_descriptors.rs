use std::any::TypeId;
use std::collections::HashMap;

use super::domain_operations::InstalledDomainOperation;

type OperationIndex = HashMap<(TypeId, TypeId, TypeId), InstalledDomainOperation>;

pub(crate) struct WorthQueryDomainOperationExecutionDescriptor {
    pub(crate) domain: TypeId,
    pub(crate) operation: TypeId,
    pub(crate) family: TypeId,
    pub(crate) publishes: bool,
    pub(crate) requires_installed_read: bool,
    pub(crate) lowering_family: String,
    pub(crate) deterministic_lowering: bool,
    pub(crate) query_digest: String,
    pub(crate) result_shape_digest: String,
    pub(crate) has_workflow: bool,
    pub(crate) has_unsupported_effect_family: bool,
    pub(crate) requires_primary_mutation: bool,
    pub(crate) lookup_cost: worth_query_installation::facade::WorthQueryOperationCostClass,
    pub(crate) execution_cost: worth_query_installation::facade::WorthQueryOperationCostClass,
    pub(crate) result_width_cost: worth_query_installation::facade::WorthQueryOperationCostClass,
    pub(crate) conditional_node_count: usize,
}

pub(crate) struct WorthQueryWorkflowExecutionDescriptor {
    pub(crate) domain: TypeId,
    pub(crate) operation: TypeId,
    pub(crate) family: TypeId,
    pub(crate) lowering_family: String,
    pub(crate) deterministic_lowering: bool,
    pub(crate) has_parallel_frontier: bool,
    pub(crate) requires_installed_read: bool,
    pub(crate) query_digest: String,
    pub(crate) result_shape_digest: String,
    pub(crate) has_unsupported_effect_family: bool,
    pub(crate) lookup_cost: worth_query_installation::facade::WorthQueryOperationCostClass,
    pub(crate) execution_cost: worth_query_installation::facade::WorthQueryOperationCostClass,
    pub(crate) result_width_cost: worth_query_installation::facade::WorthQueryOperationCostClass,
    pub(crate) replay_comparator_family: Option<&'static str>,
}

pub(super) fn operation_execution_descriptors(
    operations: &OperationIndex,
) -> Vec<WorthQueryDomainOperationExecutionDescriptor> {
    operations
        .iter()
        .map(|((domain, operation, family), installed)| {
            WorthQueryDomainOperationExecutionDescriptor {
                domain: *domain,
                operation: *operation,
                family: *family,
                publishes: !matches!(
                    installed.authority.definition().semantics().publication,
                    worth_query_installation::facade::WorthQueryOperationPublicationContract::NotRequired
                ),
                requires_installed_read: !matches!(
                    installed.authority.definition().semantics().publication,
                    worth_query_installation::facade::WorthQueryOperationPublicationContract::NotRequired
                ) || installed
                        .authority
                        .definition()
                        .semantics()
                        .graph_reads
                        .roles()
                        .iter()
                        .any(|read| {
                            read.participation
                                == worth_query_installation::facade::WorthQueryOperationGraphParticipation::PrimaryLogicalGraph
                        }),
                lowering_family: installed
                    .authority
                    .definition()
                    .semantics()
                    .lowering
                    .family
                    .clone(),
                deterministic_lowering: installed
                    .authority
                    .definition()
                    .semantics()
                    .lowering
                    .deterministic,
                query_digest: installed
                    .authority
                    .definition()
                    .semantics()
                    .canonical_query
                    .query()
                    .digest()
                    .as_str()
                    .to_string(),
                result_shape_digest: installed
                    .authority
                    .definition()
                    .semantics()
                    .canonical_query
                    .result_shape()
                    .digest()
                    .as_str()
                    .to_string(),
                has_workflow: matches!(
                    installed.authority.definition().semantics().workflow,
                    worth_query_installation::facade::WorthQueryOperationWorkflowContract::Declared(_)
                ),
                has_unsupported_effect_family: matches!(
                    &installed.authority.definition().semantics().effects,
                    worth_query_installation::facade::WorthQueryOperationEffectContract::Declared { effect_families }
                        if effect_families.iter().any(|family| *family != worth_query_installation::facade::WorthQueryOperationEffectFamily::Mutation)
                ),
                requires_primary_mutation: requires_primary_mutation(
                    installed.authority.definition().semantics(),
                ),
                lookup_cost: installed.authority.definition().semantics().cost.lookup,
                execution_cost: installed.authority.definition().semantics().cost.execution,
                result_width_cost: installed
                    .authority
                    .definition()
                    .semantics()
                    .cost
                    .result_width,
                conditional_node_count: conditional_node_count(
                    installed.authority.definition().semantics(),
                ),
            }
        })
        .collect()
}

fn conditional_node_count(
    semantics: &worth_query_installation::facade::WorthQueryDomainOperationSemanticClosure,
) -> usize {
    semantics.conditional_nodes.len()
        + match &semantics.workflow {
            worth_query_installation::facade::WorthQueryOperationWorkflowContract::Declared(
                workflow,
            ) => workflow
                .stages()
                .iter()
                .map(|stage| stage.semantics().conditional_nodes.len())
                .sum(),
            worth_query_installation::facade::WorthQueryOperationWorkflowContract::NotRequired => 0,
        }
}

fn requires_primary_mutation(
    semantics: &worth_query_installation::facade::WorthQueryDomainOperationSemanticClosure,
) -> bool {
    let has_effect = matches!(
        semantics.effects,
        worth_query_installation::facade::WorthQueryOperationEffectContract::Declared { .. }
    );
    if !has_effect {
        return false;
    }
    match &semantics.touches {
        worth_query_installation::facade::WorthQueryOperationTouchContract::NotRequired => true,
        worth_query_installation::facade::WorthQueryOperationTouchContract::Declared {
            graph_roles,
            ..
        } => graph_roles.iter().any(|role| {
            semantics.graph_reads.roles().iter().any(|read| {
                read.role == *role
                    && read.participation
                        == worth_query_installation::facade::WorthQueryOperationGraphParticipation::PrimaryLogicalGraph
            })
        }),
    }
}

pub(super) fn workflow_execution_descriptors(
    operations: &OperationIndex,
) -> Vec<WorthQueryWorkflowExecutionDescriptor> {
    operations
        .iter()
        .filter_map(|((domain, operation, family), installed)| {
            let workflow = match &installed.authority.definition().semantics().workflow {
                worth_query_installation::facade::WorthQueryOperationWorkflowContract::Declared(
                    workflow,
                ) => workflow,
                worth_query_installation::facade::WorthQueryOperationWorkflowContract::NotRequired => {
                    return None;
                }
            };
            Some(WorthQueryWorkflowExecutionDescriptor {
                    domain: *domain,
                    operation: *operation,
                    family: *family,
                    lowering_family: installed
                        .authority
                        .definition()
                        .semantics()
                        .lowering
                        .family
                        .clone(),
                    deterministic_lowering: installed
                        .authority
                        .definition()
                        .semantics()
                        .lowering
                        .deterministic,
                    has_parallel_frontier: workflow.has_parallel_frontier(),
                    requires_installed_read: installed
                        .authority
                        .definition()
                        .semantics()
                        .graph_reads
                        .roles()
                        .iter()
                        .any(|read| {
                            read.participation
                                == worth_query_installation::facade::WorthQueryOperationGraphParticipation::PrimaryLogicalGraph
                        }),
                    query_digest: installed
                        .authority
                        .definition()
                        .semantics()
                        .canonical_query
                        .query()
                        .digest()
                        .as_str()
                        .to_string(),
                    result_shape_digest: installed
                        .authority
                        .definition()
                        .semantics()
                        .canonical_query
                        .result_shape()
                        .digest()
                        .as_str()
                        .to_string(),
                    has_unsupported_effect_family: workflow.stages().iter().any(|stage| {
                        stage.semantics().effect_roles.iter().any(|family| {
                            *family
                                != worth_query_installation::facade::WorthQueryOperationEffectFamily::Mutation
                        })
                    }),
                    lookup_cost: installed.authority.definition().semantics().cost.lookup,
                    execution_cost: installed.authority.definition().semantics().cost.execution,
                    result_width_cost: installed
                        .authority
                        .definition()
                        .semantics()
                        .cost
                        .result_width,
                    replay_comparator_family: match installed.authority.definition().semantics().replay {
                        worth_query_installation::facade::WorthQueryOperationReplayContract::CertReplayable { comparator }
                        | worth_query_installation::facade::WorthQueryOperationReplayContract::CertReplayableWithNoise { comparator, .. } => Some(comparator.family),
                        _ => None,
                    },
            })
        })
        .collect()
}
