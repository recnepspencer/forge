use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryDeclarationEntryOrchestrationTerminalError,
    ForgeQueryDeclarationInput,
};

use crate::facade::{topology_current_head_authoritative_context, topology_query_domain_entry};

use super::super::{
    TopologyDeclarationEntryRefusalClass, TopologyDeclarationEntryStopClass, TopologyEditFamily,
    TopologyOperatorExecutionError,
};

pub(super) fn orchestrate_topology_declaration_entry<I>(
    family: TopologyEditFamily,
    declaration: I,
) -> Result<(), TopologyOperatorExecutionError>
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
    family: TopologyEditFamily,
    error: ForgeQueryDeclarationEntryOrchestrationTerminalError<
        crate::facade::TopologyQueryDomain,
        I,
    >,
) -> TopologyOperatorExecutionError
where
    I: ForgeQueryDeclarationInput<crate::facade::TopologyQueryDomain>,
{
    match error {
        ForgeQueryDeclarationEntryOrchestrationTerminalError::Deferred(outcome) => {
            TopologyOperatorExecutionError::DeclarationEntry {
                family,
                stop_class: TopologyDeclarationEntryStopClass::Deferred,
                stop_stage: outcome.stop_stage(),
                refusal_class: None,
                reason: outcome.reason(),
            }
        }
        ForgeQueryDeclarationEntryOrchestrationTerminalError::Denied(outcome) => {
            TopologyOperatorExecutionError::DeclarationEntry {
                family,
                stop_class: TopologyDeclarationEntryStopClass::Denied,
                stop_stage: outcome.stop_stage(),
                refusal_class: None,
                reason: outcome.reason(),
            }
        }
        ForgeQueryDeclarationEntryOrchestrationTerminalError::Stale(outcome) => {
            TopologyOperatorExecutionError::DeclarationEntry {
                family,
                stop_class: TopologyDeclarationEntryStopClass::Stale,
                stop_stage: outcome.stop_stage(),
                refusal_class: None,
                reason: outcome.reason(),
            }
        }
        ForgeQueryDeclarationEntryOrchestrationTerminalError::RebindRequired(outcome) => {
            TopologyOperatorExecutionError::DeclarationEntry {
                family,
                stop_class: TopologyDeclarationEntryStopClass::RebindRequired,
                stop_stage: outcome.stop_stage(),
                refusal_class: None,
                reason: outcome.reason(),
            }
        }
        ForgeQueryDeclarationEntryOrchestrationTerminalError::Failed(outcome) => {
            TopologyOperatorExecutionError::DeclarationEntry {
                family,
                stop_class: TopologyDeclarationEntryStopClass::Failed,
                stop_stage: outcome.stop_stage(),
                refusal_class: None,
                reason: outcome.reason(),
            }
        }
        ForgeQueryDeclarationEntryOrchestrationTerminalError::Refused(outcome) => {
            TopologyOperatorExecutionError::DeclarationEntry {
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
    fn declaration_entry_root_admits_the_capabilities_required_by_query_native_operator_batches() {
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
