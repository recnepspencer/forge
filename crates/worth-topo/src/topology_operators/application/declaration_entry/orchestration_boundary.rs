use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryDeclarationEntryOrchestrationTerminalError,
    ForgeQueryDeclarationInput,
};

use crate::facade::{topology_current_head_authoritative_context, topology_query_domain_entry};

use super::super::{
    TopologyDeclarationEntryRefusalClass, TopologyDeclarationEntryStopClass,
    TopologyMutationApplicationError, TopologyMutationFamily,
};

pub(super) fn orchestrate_topology_declaration_entry<I>(
    family: TopologyMutationFamily,
    declaration: I,
) -> Result<(), TopologyMutationApplicationError>
where
    I: ForgeQueryDeclarationInput<crate::facade::TopologyQueryDomain> + Clone,
{
    let facade = topology_current_head_declaration_entry_facade();
    let handle = topology_query_domain_entry(&facade)
        .with_operating_context(topology_current_head_authoritative_context())
        .validate()
        .expect("current-head topology declaration context should validate")
        .admit()
        .expect("current-head topology declaration context should admit");
    handle
        .orchestrate_declaration_entry(declaration)
        .map(|_| ())
        .map_err(|error| declaration_entry_error(family, error))
}

pub(super) fn topology_current_head_declaration_entry_facade() -> ForgeQueryApplicationFacade {
    ForgeQueryApplicationFacade::runtime_backed_default()
}

fn declaration_entry_error<I>(
    family: TopologyMutationFamily,
    error: ForgeQueryDeclarationEntryOrchestrationTerminalError<
        crate::facade::TopologyQueryDomain,
        I,
    >,
) -> TopologyMutationApplicationError
where
    I: ForgeQueryDeclarationInput<crate::facade::TopologyQueryDomain>,
{
    match error {
        ForgeQueryDeclarationEntryOrchestrationTerminalError::Deferred(outcome) => {
            TopologyMutationApplicationError::DeclarationEntry {
                family,
                stop_class: TopologyDeclarationEntryStopClass::Deferred,
                stop_stage: outcome.stop_stage(),
                refusal_class: None,
                reason: outcome.reason(),
            }
        }
        ForgeQueryDeclarationEntryOrchestrationTerminalError::Denied(outcome) => {
            TopologyMutationApplicationError::DeclarationEntry {
                family,
                stop_class: TopologyDeclarationEntryStopClass::Denied,
                stop_stage: outcome.stop_stage(),
                refusal_class: None,
                reason: outcome.reason(),
            }
        }
        ForgeQueryDeclarationEntryOrchestrationTerminalError::Stale(outcome) => {
            TopologyMutationApplicationError::DeclarationEntry {
                family,
                stop_class: TopologyDeclarationEntryStopClass::Stale,
                stop_stage: outcome.stop_stage(),
                refusal_class: None,
                reason: outcome.reason(),
            }
        }
        ForgeQueryDeclarationEntryOrchestrationTerminalError::RebindRequired(outcome) => {
            TopologyMutationApplicationError::DeclarationEntry {
                family,
                stop_class: TopologyDeclarationEntryStopClass::RebindRequired,
                stop_stage: outcome.stop_stage(),
                refusal_class: None,
                reason: outcome.reason(),
            }
        }
        ForgeQueryDeclarationEntryOrchestrationTerminalError::Failed(outcome) => {
            TopologyMutationApplicationError::DeclarationEntry {
                family,
                stop_class: TopologyDeclarationEntryStopClass::Failed,
                stop_stage: outcome.stop_stage(),
                refusal_class: None,
                reason: outcome.reason(),
            }
        }
        ForgeQueryDeclarationEntryOrchestrationTerminalError::Refused(outcome) => {
            TopologyMutationApplicationError::DeclarationEntry {
                family,
                stop_class: TopologyDeclarationEntryStopClass::Refused,
                stop_stage: outcome.stop_stage(),
                refusal_class: Some(TopologyDeclarationEntryRefusalClass::from(
                    outcome.refusal_class(),
                )),
                reason: outcome.reason(),
            }
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
