use forge_query::facade::{ForgeQueryApplicationFacade, ForgeQueryDeclarationInput};

use crate::query_domain::{
    topology_current_head_authoritative_context, topology_query_domain_entry, TopologyQueryDomain,
};
use crate::topology_operators::{
    topology_operator_contribution_workflow, TopologyOperatorContributionCheckedOutcome,
    TopologyOperatorContributionDeclaration, TopologyOperatorWorkflowHandleExt,
};

use super::super::{
    TopologyDeclarationEntryStopClass, TopologyMutationApplicationError, TopologyMutationFamily,
    TopologyRetainedApplicationHandoff,
};

pub(super) fn orchestrate_topology_declaration_entry<I>(
    family: TopologyMutationFamily,
    declaration: I,
) -> Result<TopologyRetainedApplicationHandoff<I>, TopologyMutationApplicationError>
where
    I: ForgeQueryDeclarationInput<TopologyQueryDomain>
        + TopologyOperatorContributionDeclaration
        + Clone,
{
    let facade = topology_current_head_declaration_entry_facade();
    let handle = topology_query_domain_entry(&facade)
        .with_operating_context(topology_current_head_authoritative_context())
        .validate()
        .expect("current-head topology declaration context should validate")
        .admit()
        .expect("current-head topology declaration context should admit");
    let artifact = handle
        .orchestrate_topology_operator_with_contributions(topology_operator_contribution_workflow(
            declaration,
        ))
        .map_err(|outcome| contribution_error(family, &handle, outcome))?;
    Ok(TopologyRetainedApplicationHandoff::new(artifact))
}

pub(super) fn topology_current_head_declaration_entry_facade() -> ForgeQueryApplicationFacade {
    ForgeQueryApplicationFacade::runtime_backed_default()
}

fn contribution_error<I>(
    family: TopologyMutationFamily,
    handle: &crate::query_domain::TopologyCurrentHeadConfiguredDomainHandle,
    outcome: TopologyOperatorContributionCheckedOutcome<I>,
) -> TopologyMutationApplicationError
where
    I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
{
    let stop_stage = match &outcome {
        forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Bound(_) => None,
        forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Deferred(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::DeclarationDenied(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::ContributionDenied(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Stale(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::RebindRequired(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Unsupported(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Failed(value) => {
            Some(value.stop_stage())
        }
    };
    let fallback_stop_class = contribution_stop_class(&outcome);
    let fallback_reason = contribution_reason(&outcome).to_string();
    let graph_obligation_envelope_digest = outcome
        .graph_obligation_dispatch()
        .and_then(|dispatch| dispatch.envelope_digest())
        .map(str::to_string);
    let brief = handle.recover_topology_operator_contribution_checked(outcome);
    let stop_class = brief
        .as_ref()
        .map(|value| TopologyDeclarationEntryStopClass::from(value.stop_kind()))
        .unwrap_or(fallback_stop_class);
    let reason = brief
        .as_ref()
        .map(|value| value.reason().to_string())
        .unwrap_or(fallback_reason);
    TopologyMutationApplicationError::DeclarationEntry {
        family,
        stop_class,
        stop_stage,
        refusal_class: None,
        recovery: brief,
        graph_obligation_envelope_digest,
        reason,
    }
}

fn contribution_reason<I>(outcome: &TopologyOperatorContributionCheckedOutcome<I>) -> &str
where
    I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
{
    match outcome {
        forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Bound(_) => {
            "topology contribution-composed orchestration should not use the non-bound error lane"
        }
        forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Deferred(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::DeclarationDenied(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::ContributionDenied(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Stale(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::RebindRequired(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Unsupported(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Failed(value) => value.reason(),
    }
}

fn contribution_stop_class<I>(
    outcome: &TopologyOperatorContributionCheckedOutcome<I>,
) -> TopologyDeclarationEntryStopClass
where
    I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
{
    match outcome {
        forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Bound(_) => {
            TopologyDeclarationEntryStopClass::Failed
        }
        forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Deferred(_) => {
            TopologyDeclarationEntryStopClass::Deferred
        }
        forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::DeclarationDenied(_)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::ContributionDenied(_) => {
            TopologyDeclarationEntryStopClass::Denied
        }
        forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Stale(_) => {
            TopologyDeclarationEntryStopClass::Stale
        }
        forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::RebindRequired(_) => {
            TopologyDeclarationEntryStopClass::RebindRequired
        }
        forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Unsupported(_) => {
            TopologyDeclarationEntryStopClass::Unsupported
        }
        forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Failed(_) => {
            TopologyDeclarationEntryStopClass::Failed
        }
    }
}

#[cfg(test)]
mod tests {
    use forge_query::facade::{ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily};

    use super::topology_current_head_declaration_entry_facade;

    #[test]
    fn declaration_entry_root_admits_the_capabilities_required_by_query_native_mutation_orchestration(
    ) {
        let support = topology_current_head_declaration_entry_facade().support_report();

        assert!(support
            .admitted_capability_families()
            .contains(&ForgeQueryCapabilityFamily::WorkflowOrchestration));
        assert!(support
            .admitted_capability_families()
            .contains(&ForgeQueryCapabilityFamily::IdentityEvolution));
        assert!(support.section_postures().iter().any(|posture| {
            posture.section() == ForgeQueryConfigSectionFamily::Relational && posture.enabled()
        }));
    }
}
